// ============ IMPORTS ============
use hyprland::{prelude::*, data::{Workspaces, Workspace}};





// ============ FUNCTIONS ============
pub fn current_workspace() -> i32 { Workspace::get_active().expect("Failed To Get Current Workspace").id }
pub fn workspace_count() -> usize { Workspaces::get().expect("Failed To Get Workspace Amount").into_iter().len() }
