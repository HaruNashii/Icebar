use std::pin::Pin;

// ============ IMPORTS ============
use hyprland::{data::{Workspace, Workspaces}, dispatch::*, event_listener::EventListener, prelude::*};





// ============ CRATES ============
use crate::{modules::workspaces::UserWorkspaceAction, update::Message};





// ============ FUNCTIONS ============
pub fn hypr_event_subscription() -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        yield Message::UpdateHyprWorkspaces;
        yield Message::UpdateFocusedWindowHypr;
        loop
        {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
 
            let tx_ws  = tx.clone();
            let tx_win = tx.clone();
 
            // EventListener::start_listener() is blocking —
            // run it on a dedicated OS thread so it never touches the async runtime
            std::thread::spawn(move ||
            {
                let mut listener = EventListener::new();
 
                // ── workspace events ──────────────────────────────────────
                let t = tx_ws.clone();
                listener.add_workspace_changed_handler(move |_| { let _ = t.send(Message::UpdateHyprWorkspaces); });
 
                let t = tx_ws.clone();
                listener.add_workspace_added_handler(move |_| { let _ = t.send(Message::UpdateHyprWorkspaces); });
 
                let t = tx_ws.clone();
                listener.add_workspace_deleted_handler(move |_| { let _ = t.send(Message::UpdateHyprWorkspaces); });
 
                let t = tx_ws.clone();
                listener.add_workspace_moved_handler(move |_| { let _ = t.send(Message::UpdateHyprWorkspaces); });
 
                let t = tx_ws.clone();
                listener.add_workspace_renamed_handler(move |_| { let _ = t.send(Message::UpdateHyprWorkspaces); });
 
                // ── focused window events ─────────────────────────────────
                let t = tx_win.clone();
                listener.add_active_window_changed_handler(move |_| { let _ = t.send(Message::UpdateFocusedWindowHypr); });
 
                let t = tx_win.clone();
                listener.add_window_opened_handler(move |_| { let _ = t.send(Message::UpdateFocusedWindowHypr); });
 
                let t = tx_win.clone();
                listener.add_window_closed_handler(move |_| { let _ = t.send(Message::UpdateFocusedWindowHypr); });
 
                let t = tx_win.clone();
                listener.add_window_moved_handler(move |_| { let _ = t.send(Message::UpdateFocusedWindowHypr); });
 
                let t = tx_win.clone();
                listener.add_window_title_changed_handler(move |_| { let _ = t.send(Message::UpdateFocusedWindowHypr); });
 
                // Blocks until the compositor socket closes
                if let Err(e) = listener.start_listener()
                {
                    eprintln!("[icebar] hypr event listener error: {e}");
                }
            });
            drop(tx);
            while let Some(msg) = rx.recv().await
            {
                yield msg;
            }
 
            eprintln!("[icebar] hypr event listener stopped — reconnecting in 2s");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    })
}
 


pub fn workspace_count() -> Vec<i32>
{ 
    let result_workspaces = Workspaces::get();
    if let Ok(all_workspaces) = result_workspaces
    {
        all_workspaces.iter().map(|item| item.id).collect()
    }
    else
    {
        Vec::new()
    }
}



pub fn current_workspace() -> i32 
{ 
    let result = Workspace::get_active();
    if let Ok(value) = result 
    {
        value.id 
    }
    else
    {
        0
    }
}



pub fn change_workspace_hypr(action: UserWorkspaceAction)
{
    match action 
    {
        UserWorkspaceAction::ChangeWithIndex(id) =>
        {
            let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id))); 
        }
        UserWorkspaceAction::MoveNext =>
        {
            let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(1))); 
        }
        UserWorkspaceAction::MovePrev =>
        {
            let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(-1))); 
        }
    }
}
