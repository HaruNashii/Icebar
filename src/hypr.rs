use hyprland::data::{Workspaces, Workspace};
use hyprland::prelude::*;





#[derive(Default, Clone)]
pub struct HyprlandData
{
    pub _current_workspace: i32,
    pub workspace_count: usize
}





pub fn current_workspace() -> i32 { Workspace::get_active().expect("Failed To Get Current Workspace").id }
pub fn workspace_count() -> usize { Workspaces::get().expect("Failed To Get Workspace Amount").into_iter().len() }
pub fn get_hyprland_data() -> HyprlandData { HyprlandData { _current_workspace: current_workspace(), workspace_count: workspace_count() } }
