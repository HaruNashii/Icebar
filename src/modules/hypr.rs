// ============ IMPORTS ============
use hyprland::{prelude::*, data::{Workspaces, Workspace}};






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
