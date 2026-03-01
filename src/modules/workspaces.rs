use iced::widget::button;

use crate::{AppData, helpers::style::{UserStyle, set_style}};


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

