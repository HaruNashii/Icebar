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
use crate::helpers::style::{UserStyle, orient_text, set_style};
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
    let string_not_oriented = if id == app.modules_data.workspace_data.current_workspace 
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
    };

    orient_text(&string_not_oriented, &app.ron_config.workspace_text_orientation)
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





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::modules::workspaces::WorkspaceData;
 
    fn make_app(current: i32) -> AppData
    {
        let mut app = AppData::default();
        app.modules_data.workspace_data = WorkspaceData
        {
            current_workspace: current,
            visible_workspaces: vec![1, 2, 3],
        };
        app.ron_config.workspace_text = vec!["ws1".into(), "ws2".into(), "ws3".into()];
        app.ron_config.workspace_selected_text = Some(vec!["[1]".into(), "[2]".into(), "[3]".into()]);
        app.ron_config.workspace_width = 10;
        app.ron_config.workspace_different_selected_width = Some(20);
        app
    }
 
    // ---- define_workspaces_text ---------------------------------------------
 
    #[test]
    fn workspace_text_returns_normal_text_for_inactive()
    {
        let app = make_app(1);
        let text = define_workspaces_text(&app, 2);
        assert_eq!(text, "ws2");
    }
 
    #[test]
    fn workspace_text_returns_selected_text_for_active()
    {
        let app = make_app(2);
        let text = define_workspaces_text(&app, 2);
        assert_eq!(text, "[2]");
    }
 
    #[test]
    fn workspace_text_falls_back_to_id_when_out_of_bounds()
    {
        let app = make_app(1);
        // workspace 10 doesn't exist in the vec
        let text = define_workspaces_text(&app, 10);
        assert_eq!(text, "10");
    }
 
    #[test]
    fn workspace_text_no_selected_vec_falls_back_to_id()
    {
        let mut app = make_app(2);
        app.ron_config.workspace_selected_text = None;
        let text = define_workspaces_text(&app, 2);
        assert_eq!(text, "2");
    }
 
    // ---- define_workspaces_padding ------------------------------------------
 
    #[test]
    fn workspace_padding_returns_different_width_for_selected()
    {
        let app = make_app(1);
        assert_eq!(define_workspaces_padding(&app, 1), 20);
    }
 
    #[test]
    fn workspace_padding_returns_default_width_for_unselected()
    {
        let app = make_app(1);
        assert_eq!(define_workspaces_padding(&app, 2), 10);
    }
 
    #[test]
    fn workspace_padding_no_different_selected_width_always_returns_default()
    {
        let mut app = make_app(1);
        app.ron_config.workspace_different_selected_width = None;
        assert_eq!(define_workspaces_padding(&app, 1), 10);
        assert_eq!(define_workspaces_padding(&app, 2), 10);
    }
        #[test]
    fn workspace_text_id_1_returns_first_element()
    {
        let app = make_app(3);
        // workspace 1 is not current (current=3), should return workspace_text[0]
        assert_eq!(define_workspaces_text(&app, 1), "ws1");
    }
 
    #[test]
    fn workspace_text_all_three_workspaces_correct()
    {
        let app = make_app(99); // nothing is selected
        assert_eq!(define_workspaces_text(&app, 1), "ws1");
        assert_eq!(define_workspaces_text(&app, 2), "ws2");
        assert_eq!(define_workspaces_text(&app, 3), "ws3");
    }
 
    #[test]
    fn workspace_text_selected_text_none_falls_back_for_current()
    {
        let mut app = make_app(2);
        app.ron_config.workspace_selected_text = None;
        // Should fall back to id.to_string() when no selected_text provided
        assert_eq!(define_workspaces_text(&app, 2), "2");
    }
}
