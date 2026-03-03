// ============ IMPORTS ============
use hyprland::{dispatch::*, prelude::*, data::{Workspaces, Workspace}};





// ============ CRATES ============
use crate::modules::workspaces::UserWorkspaceAction;





// ============ FUNCTIONS ============
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
            let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id as i32))); 
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
