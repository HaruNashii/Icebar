// ============ IMPORTS ============
use hyprland::{prelude::*, data::{Workspaces, Workspace}};






// ============ FUNCTIONS ============
pub fn workspace_count() -> Vec<i32>
{ 
    let result = Workspaces::get();
    if let Ok(values) = result 
    {
        let mut return_vec = Vec::new();
        for item in &values 
        {
            return_vec.push(item.id)
        }
        return_vec
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
