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
pub fn workspace_count() -> usize { Workspaces::get().expect("Failed To Get Workspace Amount").into_iter().len() }
pub fn current_workspace() -> i32 { Workspace::get_active().expect("Failed To Get Current Workspace").id }
