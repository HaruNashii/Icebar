// ============ IMPORTS ============
use std::pin::Pin;
use niri_ipc::{Action, Request, Response, Workspace, WorkspaceReferenceArg, socket::Socket};





// ============ CRATES ============
use crate::modules::workspaces::UserWorkspaceAction;





// ============ FUNCTIONS ============
pub fn workspace_count() -> Vec<i32>
{
    let workspaces = niri_ipc_workspaces_setup();
    let mut idxs: Vec<i32> = workspaces.iter().map(|w| w.idx as i32).collect();
    idxs.sort_unstable();
    idxs.dedup();
    idxs
}



pub fn current_workspace() -> i32
{
    let workspaces = niri_ipc_workspaces_setup();
    let result_focused_idx = workspaces.iter().find(|w| w.is_focused).map(|w| w.idx);
    if let Some(focused_idx) = result_focused_idx
    {
        focused_idx as i32
    }
    else
    {
        0
    }
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
            let result = tokio::task::spawn_blocking(||
            {
                let mut socket = Socket::connect().ok()?;
                match socket.send(Request::EventStream)
                {
                    Ok(_)  => Some(socket),
                    Err(_) => None
                }
            }).await;

            let socket = match result
            {
                Ok(Some(s)) => s,
                _ =>
                {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    continue;
                }
            };

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<crate::update::Message>();

            // The thread below exits when tx.send() returns Err, which happens as soon
            // as rx is dropped (i.e. when this stream generator is dropped or the outer
            // loop restarts). The blocking read_next() call may delay the exit by one
            // event, but no new threads will be spawned until the old one has finished.
            std::thread::spawn(move ||
            {
                let mut read_next = socket.read_events();
                while let Ok(event) = read_next()
                {
                    use niri_ipc::Event;
                    let msg = match event
                    {
                        Event::WorkspacesChanged { .. }
                        | Event::WorkspaceActivated { .. }
                        | Event::WorkspaceActiveWindowChanged { .. } =>
                            Some(crate::update::Message::UpdateNiriWorkspaces),

                        Event::WindowFocusChanged { .. }
                        | Event::WindowsChanged { .. }
                        | Event::WindowOpenedOrChanged { .. }
                        | Event::WindowClosed { .. } =>
                            Some(crate::update::Message::UpdateFocusedWindowNiri),

                        _ => None
                    };
                    if let Some(m) = msg
                        && tx.send(m).is_err() { break; }
                }
            });

            while let Some(msg) = rx.recv().await
            {
                yield msg;
            }

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
