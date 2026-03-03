// ============ IMPORTS ============
use iced::widget::button;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct WorkspaceData
{
    pub visible_workspaces: Vec<i32>,
    pub current_workspace: i32 
}

#[derive(Clone)]
pub enum UserWorkspaceAction
{
    ChangeWithIndex(usize),
    MoveNext,
    MovePrev
}





// ============ CRATES ============
use crate::helpers::style::{UserStyle, set_style};
use crate::AppData;





// ============ FUNCTIONS ============
pub fn define_workspaces_style(app: &AppData, status: button::Status, i: &i32) -> iced::widget::button::Style
{
    let hovered = app.ron_config.workspace_button_hovered_color_rgb;
    let hovered_text = app.ron_config.workspace_button_hovered_text_color_rgb;
    let pressed = app.ron_config.workspace_button_pressed_color_rgb;
    let normal = if app.modules_data.workspace_data.current_workspace == *i 
    { app.ron_config.workspace_button_selected_color_rgb }
    else 
    { app.ron_config.workspace_button_color_rgb };
    let normal_text = app.ron_config.workspace_button_text_color_rgb;
    let border_size = app.ron_config.workspace_border_size;
    let border_color_rgba = app.ron_config.workspace_border_color_rgba;
    let border_radius = app.ron_config.workspace_border_radius;
    set_style(UserStyle {status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius})
}


pub fn define_workspaces_text(app: &AppData, id: i32) -> String
{
    if id == app.modules_data.workspace_data.current_workspace 
    {
        if let Some(selected) = &app.ron_config.workspace_selected_text 
        {
            selected.get((id - 1) as usize).cloned().unwrap_or_else(|| id.to_string()) 
        } 
        else 
        {
            id.to_string() 
        }
    } 
    else 
    { 
        app.ron_config.workspace_text.get((id - 1) as usize).cloned().unwrap_or_else(|| id.to_string()) 
    }
}



pub fn define_workspaces_padding(app: &AppData, id: i32) -> u16
{
    if let Some(value) = app.ron_config.workspace_different_selected_width && id == app.modules_data.workspace_data.current_workspace
    {
        value 
    } 
    else 
    {
        app.ron_config.workspace_width 
    }
}

