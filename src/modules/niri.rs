// ============ IMPORTS ============
use niri_ipc::{Action, Request, Response, Workspace, WorkspaceReferenceArg, socket::Socket};





// ============ FUNCTIONS ============
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
            let _ = socket.send(Request::Action(Action::FocusWorkspace{reference: WorkspaceReferenceArg::Index(id as u8)}));
        }
        UserWorkspaceAction::MoveNext =>
        {
            let _ = socket.send(Request::Action(Action::FocusWorkspaceDown{}));
        }
        UserWorkspaceAction::MovePrev =>
        {
            let _ = socket.send(Request::Action(Action::FocusWorkspaceUp{}));
        }
    }
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
