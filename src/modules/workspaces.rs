// ============ IMPORTS ============
use iced::widget::button;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct WorkspaceData
{
    pub is_hovering_workspace: bool,
    pub visible_workspaces: Vec<i32>,
    pub current_workspace: i32 
}

#[derive(Clone)]
pub enum UserWorkspaceAction
{
    ChangeWithIndex(i32),
    MoveNext,
    MovePrev
}





// ============ CRATES ============
use crate::helpers::style::{UserStyle, orient_text, set_style};
use crate::AppData;





// ============ FUNCTIONS ============
pub fn define_workspaces_style(app: &AppData, status: button::Status, i: &i32) -> iced::widget::button::Style
{
    let hovered = app.ron_config.workspace_button_hovered_color;
    let hovered_text = app.ron_config.workspace_button_hovered_text_color;
    let pressed = app.ron_config.workspace_button_pressed_color;

    let normal = if app.modules_data.workspace_data.current_workspace == *i 
    { app.ron_config.workspace_button_selected_color }
    else 
    { app.ron_config.workspace_button_color };

    let normal_text = if app.modules_data.workspace_data.current_workspace == *i 
    { app.ron_config.workspace_selected_text_color }
    else
    { app.ron_config.workspace_text_color };

    let border_size = app.ron_config.workspace_border_size;
    let border_color = app.ron_config.workspace_border_color;
    let border_radius = app.ron_config.workspace_border_radius;
    let normal_gradient = if app.modules_data.workspace_data.current_workspace == *i
    { app.ron_config.workspace_button_selected_gradient_color.clone() }
    else
    { app.ron_config.workspace_button_gradient_color.clone() };
    set_style(UserStyle {status, hovered, hovered_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient, hovered_gradient: app.ron_config.workspace_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.workspace_button_pressed_gradient_color.clone()})
}


pub fn define_workspaces_text(app: &AppData, id: i32) -> String
{
    let string_not_oriented = if id == app.modules_data.workspace_data.current_workspace 
    {
        if let Some(selected) = &app.ron_config.workspace_selected_text 
        {
            let safe_id = id.saturating_sub(1) as usize;
            selected.get(safe_id).cloned().unwrap_or_else(|| id.to_string()) 
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



pub fn define_workspaces_size(app: &AppData, id: i32) -> (u32, u32)
{
    let width = if let Some(value) = app.ron_config.workspace_different_selected_width && id == app.modules_data.workspace_data.current_workspace
    {
        value
    } 
    else 
    {
        app.ron_config.workspace_width
    };

    let height = if let Some(value) = app.ron_config.workspace_different_selected_height && id == app.modules_data.workspace_data.current_workspace
    {
        value
    } 
    else 
    {
        app.ron_config.workspace_height
    };

    (width, height)
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::modules::workspaces::WorkspaceData;
    use crate::helpers::color::ColorType;
    use iced::{widget::button, Background, Color};

    fn make_app(current: i32) -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.modules_data.workspace_data = WorkspaceData
        {
            is_hovering_workspace: false,
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
 
    fn make_style_app(current: i32) -> AppData
    {
        let mut app = make_app(current);   // re-uses the existing make_app helper
        app.ron_config.workspace_button_color = ColorType::RGB([0, 0, 200]);
        app.ron_config.workspace_button_selected_color = ColorType::RGB([255, 0, 0]);
        app.ron_config.workspace_button_hovered_color = ColorType::RGB([0, 200, 0]);
        app.ron_config.workspace_button_pressed_color = ColorType::RGB([0, 100, 0]);
        app
    }
 
    #[test]
    fn workspace_style_current_workspace_uses_selected_color()
    {
        let style = define_workspaces_style(&make_style_app(2), button::Status::Active, &2);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(255, 0, 0))));
    }
 
    #[test]
    fn workspace_style_non_current_uses_normal_color()
    {
        let style = define_workspaces_style(&make_style_app(1), button::Status::Active, &3);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 0, 200))));
    }
 
    #[test]
    fn workspace_style_selected_and_non_selected_differ()
    {
        let app      = make_style_app(1);
        let selected = define_workspaces_style(&app, button::Status::Active, &1);
        let other    = define_workspaces_style(&app, button::Status::Active, &2);
        assert_ne!(selected.background, other.background);
    }
 
    #[test]
    fn workspace_style_hovered_uses_hovered_color()
    {
        let style = define_workspaces_style(&make_style_app(1), button::Status::Hovered, &2);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 200, 0))));
    }
 
    #[test]
    fn workspace_style_pressed_uses_pressed_color()
    {
        let style = define_workspaces_style(&make_style_app(1), button::Status::Pressed, &2);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 100, 0))));
    }
}
