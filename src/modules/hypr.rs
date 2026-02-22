// ============ IMPORTS ============
use hyprland::{prelude::*, data::{Workspaces, Workspace}};





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct UserHyprData
{
    pub workspace_count: usize,
    pub current_workspace: i32 
}





// ============ FUNCTIONS ============
pub fn workspace_count() -> usize 
{ 
    let result = Workspaces::get();
    if let Ok(value) = result 
    {
        value.into_iter().len() 
    }
    else
    {
        0
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
