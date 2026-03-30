// ============ IMPORTS ============
use std::pin::Pin;





// ============ CRATES ============
use crate::{modules::workspaces::UserWorkspaceAction, update::Message};



// ============ ENUM/STRUCT, ETC ============
// desktops property: a(uss) = array of (position: u32, id: String, name: String)
#[derive(Debug, zbus::zvariant::Type, zbus::zvariant::OwnedValue, zbus::zvariant::Value, serde::Deserialize, serde::Serialize)]
struct Desktop { position: u32, id: String, name: String }

#[zbus::proxy(interface = "org.kde.KWin.VirtualDesktopManager", default_service = "org.kde.KWin", default_path = "/VirtualDesktopManager")]
trait VirtualDesktopManager
{
    #[zbus(property, name = "current")]
    fn current(&self) -> zbus::Result<String>;

    #[zbus(property, name = "current")]
    fn set_current(&self, id: &str) -> zbus::Result<()>;

    #[zbus(property, name = "desktops")]
    fn desktops(&self) -> zbus::Result<Vec<Desktop>>;

    #[zbus(signal, name = "currentChanged")]
    fn kwin_current_changed(&self, desktop_id: String) -> zbus::Result<()>;

    #[zbus(signal, name = "desktopCreated")]
    fn kwin_desktop_created(&self, desktop_id: String, desktop: Desktop) -> zbus::Result<()>;

    #[zbus(signal, name = "desktopRemoved")]
    fn kwin_desktop_removed(&self, desktop_id: String) -> zbus::Result<()>;
}





// ============ FUNCTIONS ============
pub fn plasma_event_subscription() -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        yield Message::UpdatePlasmaWorkspaces;
        loop
        {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

            tokio::spawn(async move
            {
                let conn = match zbus::Connection::session().await
                {
                    Ok(c) => c,
                    Err(e) => { eprintln!("[plasma] D-Bus connection failed: {e}"); return; }
                };
                let proxy = match VirtualDesktopManagerProxy::new(&conn).await
                {
                    Ok(p) => p,
                    Err(e) => { eprintln!("[plasma] proxy failed: {e}"); return; }
                };

                let mut current_stream = match proxy.receive_kwin_current_changed().await
                {
                    Ok(s) => s,
                    Err(e) => { eprintln!("[plasma] currentChanged subscribe failed: {e}"); return; }
                };
                let mut created_stream = match proxy.receive_kwin_desktop_created().await
                {
                    Ok(s) => s,
                    Err(e) => { eprintln!("[plasma] desktopCreated subscribe failed: {e}"); return; }
                };
                let mut removed_stream = match proxy.receive_kwin_desktop_removed().await
                {
                    Ok(s) => s,
                    Err(e) => { eprintln!("[plasma] desktopRemoved subscribe failed: {e}"); return; }
                };

                loop
                {
                    tokio::select!
                    {
                        Some(_) = futures_util::StreamExt::next(&mut current_stream) => { let _ = tx.send(Message::UpdatePlasmaWorkspaces); }
                        Some(_) = futures_util::StreamExt::next(&mut created_stream) => { let _ = tx.send(Message::UpdatePlasmaWorkspaces); }
                        Some(_) = futures_util::StreamExt::next(&mut removed_stream) => { let _ = tx.send(Message::UpdatePlasmaWorkspaces); }
                    }
                }
            });

            while let Some(msg) = rx.recv().await { yield msg; }

            eprintln!("[plasma] event listener stopped — reconnecting in 2s");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    })
}



pub async fn get_plasma_workspaces() -> (i32, Vec<i32>, Vec<String>)
{
    let conn = match zbus::Connection::session().await
    {
        Ok(c) => c,
        Err(e) => { eprintln!("[plasma] D-Bus failed: {e}"); return (0, Vec::new(), Vec::new()); }
    };
    let proxy = match VirtualDesktopManagerProxy::new(&conn).await
    {
        Ok(p) => p,
        Err(e) => { eprintln!("[plasma] proxy failed: {e}"); return (0, Vec::new(), Vec::new()); }
    };

    let current_id = match proxy.current().await
    {
        Ok(id) => id,
        Err(e) => { eprintln!("[plasma] current() failed: {e}"); return (0, Vec::new(), Vec::new()); }
    };

    let mut desktops = match proxy.desktops().await
    {
        Ok(d) => d,
        Err(e) => { eprintln!("[plasma] desktops() failed: {e}"); return (0, Vec::new(), Vec::new()); }
    };


    desktops.sort_by_key(|d| d.position);
    let (current, list, ids) = resolve_workspaces(&current_id, &desktops);
    (current, list, ids)
}



pub async fn change_workspace_plasma(action: UserWorkspaceAction, ids: Vec<String>)
{
    let conn  = match zbus::Connection::session().await { Ok(c) => c, Err(_) => return };
    let proxy = match VirtualDesktopManagerProxy::new(&conn).await { Ok(p) => p, Err(_) => return };

    let current_id = proxy.current().await.unwrap_or_default();
    let target_id  = resolve_target_id(&action, &current_id, &ids);

    if let Some(id) = target_id && let Err(e) = proxy.set_current(&id).await
    {
        eprintln!("[plasma] set_current failed: {e}");
    }
}



fn resolve_workspaces(current_id: &str, desktops: &[Desktop]) -> (i32, Vec<i32>, Vec<String>)
{
    let ids: Vec<String> = desktops.iter().map(|d| d.id.clone()).collect();
    let current          = ids.iter().position(|id| id == current_id).map(|i| i as i32 + 1).unwrap_or(0);
    let list: Vec<i32>   = (1..=ids.len() as i32).collect();
    (current, list, ids)
}



pub fn resolve_target_id(action: &UserWorkspaceAction, current_id: &str, ids: &[String]) -> Option<String>
{
    let current_pos = ids.iter().position(|id| id == current_id).unwrap_or(0);
    match action
    {
        UserWorkspaceAction::ChangeWithIndex(i) => ids.get(*i as usize - 1).cloned(),
        UserWorkspaceAction::MoveNext           => ids.get(current_pos + 1).cloned(),
        UserWorkspaceAction::MovePrev           => if current_pos > 0 { ids.get(current_pos - 1).cloned() } else { None },
    }
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;

    fn make_desktops(entries: &[(u32, &str, &str)]) -> Vec<Desktop>
    {
        entries.iter().map(|(pos, id, name)| Desktop { position: *pos, id: id.to_string(), name: name.to_string() }).collect()
    }

    fn ids(entries: &[(u32, &str, &str)]) -> Vec<String>
    {
        entries.iter().map(|(_, id, _)| id.to_string()).collect()
    }


    // ---- resolve_workspaces ------------------------------------------------

    #[test]
    fn resolve_workspaces_current_is_first()
    {
        let desktops = make_desktops(&[(0, "aaa", "Desktop 1"), (1, "bbb", "Desktop 2"), (2, "ccc", "Desktop 3")]);
        let (current, list, out_ids) = resolve_workspaces("aaa", &desktops);
        assert_eq!(current, 1);
        assert_eq!(list, vec![1, 2, 3]);
        assert_eq!(out_ids, vec!["aaa", "bbb", "ccc"]);
    }

    #[test]
    fn resolve_workspaces_current_is_middle()
    {
        let desktops = make_desktops(&[(0, "aaa", "D1"), (1, "bbb", "D2"), (2, "ccc", "D3")]);
        let (current, list, _) = resolve_workspaces("bbb", &desktops);
        assert_eq!(current, 2);
        assert_eq!(list, vec![1, 2, 3]);
    }

    #[test]
    fn resolve_workspaces_current_is_last()
    {
        let desktops = make_desktops(&[(0, "aaa", "D1"), (1, "bbb", "D2"), (2, "ccc", "D3")]);
        let (current, _, _) = resolve_workspaces("ccc", &desktops);
        assert_eq!(current, 3);
    }

    #[test]
    fn resolve_workspaces_unknown_id_returns_zero()
    {
        let desktops = make_desktops(&[(0, "aaa", "D1"), (1, "bbb", "D2")]);
        let (current, list, _) = resolve_workspaces("zzz", &desktops);
        assert_eq!(current, 0);
        assert_eq!(list, vec![1, 2]);
    }

    #[test]
    fn resolve_workspaces_empty_desktops()
    {
        let (current, list, ids) = resolve_workspaces("aaa", &[]);
        assert_eq!(current, 0);
        assert!(list.is_empty());
        assert!(ids.is_empty());
    }

    #[test]
    fn resolve_workspaces_single_desktop_is_current()
    {
        let desktops = make_desktops(&[(0, "only", "D1")]);
        let (current, list, out_ids) = resolve_workspaces("only", &desktops);
        assert_eq!(current, 1);
        assert_eq!(list, vec![1]);
        assert_eq!(out_ids, vec!["only"]);
    }

    #[test]
    fn resolve_workspaces_list_is_always_1_to_n()
    {
        let desktops = make_desktops(&[(0, "a", ""), (1, "b", ""), (2, "c", ""), (3, "d", ""), (4, "e", "")]);
        let (_, list, _) = resolve_workspaces("a", &desktops);
        assert_eq!(list, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn resolve_workspaces_ids_preserve_order()
    {
        let desktops = make_desktops(&[(0, "first", ""), (1, "second", ""), (2, "third", "")]);
        let (_, _, out_ids) = resolve_workspaces("first", &desktops);
        assert_eq!(out_ids, vec!["first", "second", "third"]);
    }


    // ---- resolve_target_id -------------------------------------------------

    #[test]
    fn change_with_index_selects_correct_id()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::ChangeWithIndex(2), "aaa", &ids), Some("bbb".into()));
    }

    #[test]
    fn change_with_index_1_selects_first()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::ChangeWithIndex(1), "ccc", &ids), Some("aaa".into()));
    }

    #[test]
    fn change_with_index_out_of_range_returns_none()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::ChangeWithIndex(99), "aaa", &ids), None);
    }

    #[test]
    fn move_next_from_first_goes_to_second()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MoveNext, "aaa", &ids), Some("bbb".into()));
    }

    #[test]
    fn move_next_from_last_returns_none()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MoveNext, "ccc", &ids), None);
    }

    #[test]
    fn move_prev_from_last_goes_to_middle()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MovePrev, "ccc", &ids), Some("bbb".into()));
    }

    #[test]
    fn move_prev_from_first_returns_none()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MovePrev, "aaa", &ids), None);
    }

    #[test]
    fn move_prev_from_second_returns_first()
    {
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MovePrev, "bbb", &ids), Some("aaa".into()));
    }

    #[test]
    fn move_next_on_single_desktop_returns_none()
    {
        let ids = ids(&[(0, "only", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MoveNext, "only", &ids), None);
    }

    #[test]
    fn move_prev_on_single_desktop_returns_none()
    {
        let ids = ids(&[(0, "only", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MovePrev, "only", &ids), None);
    }

    #[test]
    fn unknown_current_id_move_next_returns_second()
    {
        // unknown id → position 0 → next is index 1
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", ""), (2, "ccc", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MoveNext, "zzz", &ids), Some("bbb".into()));
    }

    #[test]
    fn unknown_current_id_move_prev_returns_none()
    {
        // unknown id → position 0 → prev is out of bounds
        let ids = ids(&[(0, "aaa", ""), (1, "bbb", "")]);
        assert_eq!(resolve_target_id(&UserWorkspaceAction::MovePrev, "zzz", &ids), None);
    }
}
