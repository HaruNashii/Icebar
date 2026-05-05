// ============ IMPORTS ============
use std::pin::Pin;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use niri_ipc::{Action, Request, Response, Workspace, WorkspaceReferenceArg, socket::Socket};



// ============ CRATES ============
use crate::modules::workspaces::UserWorkspaceAction;



// ============ FUNCTIONS ============
pub fn current_workspace_and_count() -> (i32, Vec<i32>)
{
    let workspaces = niri_ipc_workspaces_setup();
    let current = workspaces.iter().find(|w| w.is_focused).map(|w| w.idx as i32).unwrap_or(0);
    let mut idxs: Vec<i32> = workspaces.iter().map(|w| w.idx as i32).collect();
    idxs.sort_unstable();
    idxs.dedup();
    (current, idxs)
}



pub fn change_workspace_niri(action: UserWorkspaceAction)
{
    let result_socket = Socket::connect();
    let mut socket = if let Ok(socket) = result_socket
    {
        socket
    }
    else
    {
        eprintln!("Failed To Connect To Niri Socket");
        return;
    };

    match action
    {
        UserWorkspaceAction::ChangeWithIndex(id) =>
        {
            let safe_id = id.clamp(1, 255) as u8;
            let _ = socket.send(Request::Action(Action::FocusWorkspace { reference: WorkspaceReferenceArg::Index(safe_id) }));
        }
        UserWorkspaceAction::MoveNext =>
        {
            let _ = socket.send(Request::Action(Action::FocusWorkspaceDown {}));
        }
        UserWorkspaceAction::MovePrev =>
        {
            let _ = socket.send(Request::Action(Action::FocusWorkspaceUp {}));
        }
    }
}



pub fn niri_event_subscription() -> Pin<Box<dyn futures::Stream<Item = crate::update::Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        yield crate::update::Message::UpdateNiriWorkspaces;
        yield crate::update::Message::UpdateFocusedWindowNiri;

        loop
        {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<crate::update::Message>();
            let tx_thread = tx.clone();
            let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let stop_thread = stop.clone();

            std::thread::spawn(move ||
            {
                let socket_path = match std::env::var("NIRI_SOCKET")
                {
                    Ok(p)  => p,
                    Err(e) =>
                    {
                        eprintln!("[icebar] NIRI_SOCKET not set: {e}");
                        return;
                    }
                };

                let stream = match UnixStream::connect(&socket_path)
                {
                    Ok(s)  => s,
                    Err(e) =>
                    {
                        eprintln!("[icebar] niri socket connect failed: {e}");
                        return;
                    }
                };

                let request = match serde_json::to_string(&Request::EventStream)
                {
                    Ok(r)  => r,
                    Err(e) =>
                    {
                        eprintln!("[icebar] niri request serialize failed: {e}");
                        return;
                    }
                };

                use std::io::Write;
                if let Err(e) = (&stream).write_all(format!("{request}\n").as_bytes())
                {
                    eprintln!("[icebar] niri event stream request failed: {e}");
                    return;
                }

                let reader = BufReader::new(&stream);
                let mut lines = reader.lines();

                if let Some(first) = lines.next()
                {
                    match first
                    {
                        Ok(_)  => {}
                        Err(e) =>
                        {
                            eprintln!("[icebar] niri event stream handshake failed: {e}");
                            return;
                        }
                    }
                }

                for line_result in lines
                {
                    if stop_thread.load(std::sync::atomic::Ordering::Relaxed) { break; }

                    let line = match line_result
                    {
                        Ok(l)  => l,
                        Err(e) =>
                        {
                            eprintln!("[icebar] niri socket read error: {e}");
                            break;
                        }
                    };

                    let value: serde_json::Value = match serde_json::from_str(&line)
                    {
                        Ok(v)  => v,
                        Err(e) =>
                        {
                            eprintln!("[icebar] niri event parse error: {e}");
                            continue;
                        }
                    };

                    let event_key = match value.as_object().and_then(|o| o.keys().next())
                    {
                        Some(k) => k.as_str(),
                        None    => continue
                    };

                    let msg = match event_key
                    {
                        "WorkspacesChanged"
                        | "WorkspaceActivated"
                        | "WorkspaceActiveWindowChanged" =>
                            Some(crate::update::Message::UpdateNiriWorkspaces),

                        "WindowFocusChanged"
                        | "WindowsChanged"
                        | "WindowOpenedOrChanged"
                        | "WindowClosed" =>
                            Some(crate::update::Message::UpdateFocusedWindowNiri),

                        _ => None
                    };

                    if let Some(m) = msg
                        && tx_thread.send(m).is_err() { break; }
                }
            });

            drop(tx);
            while let Some(msg) = rx.recv().await
            {
                yield msg;
            }

            stop.store(true, std::sync::atomic::Ordering::Relaxed);
            eprintln!("[icebar] niri event stream ended — reconnecting in 2s");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    })
}



fn niri_ipc_workspaces_setup() -> Vec<Workspace>
{
    let result_socket = Socket::connect();
    let mut socket = if let Ok(socket) = result_socket
    {
        socket
    }
    else
    {
        eprintln!("Failed To Connect To Niri Socket");
        return Vec::new();
    };

    let result_reply = socket.send(Request::Workspaces);
    let replay = if let Ok(replay) = result_reply
    {
        replay
    }
    else
    {
        eprintln!("Failed to request workspaces");
        return Vec::new();
    };

    let response = if let Ok(replay) = replay
    {
        replay
    }
    else
    {
        eprintln!("Failed to request workspaces");
        return Vec::new();
    };

    match response
    {
        Response::Workspaces(ws) => ws,
        _ =>
        {
            eprintln!("Unexpected response type");
            Vec::new()
        }
    }
}
