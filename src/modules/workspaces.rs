// ============ IMPORTS ============
use iced::widget::button;





// ============ CONFIG ============
use serde::{Deserialize, Serialize};
use crate::helpers::style::{TextOrientation, SideOption};
use crate::helpers::color::{ColorType, Gradient};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct WorkspaceConfig
{
    pub niri_workspaces_update_interval:          u64,
    pub reverse_scroll_on_workspace:              bool,
    pub persistent_workspaces:                    Option<u8>,
    pub workspace_height:                         u32,
    pub workspace_width:                          u32,
    pub workspace_different_selected_width:       Option<u32>,
    pub workspace_different_selected_height:      Option<u32>,
    pub workspace_text_size:                      u32,
    pub workspace_unique_text:                    Option<String>,
    pub workspace_selected_unique_text:           Option<String>,
    pub workspace_text:                           Vec<String>,
    pub workspace_selected_text:                  Option<Vec<String>>,
    pub workspace_text_color:                     ColorType,
    pub workspace_selected_text_color:            ColorType,
    pub workspace_text_orientation:               TextOrientation,
    pub workspace_spacing:                        u32,
    pub workspace_padding:                        u16,
    pub workspace_button_color:                   ColorType,
    pub workspace_button_selected_color:          ColorType,
    pub workspace_button_hovered_color:           ColorType,
    pub workspace_button_hovered_text_color:      ColorType,
    pub workspace_button_pressed_text_color:      ColorType,
    pub workspace_button_pressed_color:           ColorType,
    pub workspace_border_color:                   ColorType,
    pub workspace_border_size:                    f32,
    pub workspace_border_radius:                  [f32; 4],
    pub workspace_side_separator:                 Option<SideOption>,
    pub workspace_side_separator_color:           ColorType,
    pub workspace_side_separator_width:           f32,
    pub workspace_side_separator_height:          f32,
    pub workspace_button_gradient_color:          Option<Gradient>,
    pub workspace_button_selected_gradient_color: Option<Gradient>,
    pub workspace_button_hovered_gradient_color:  Option<Gradient>,
    pub workspace_button_pressed_gradient_color:  Option<Gradient>,
    pub workspace_button_shadow_color:            Option<ColorType>,
    pub workspace_button_shadow_x:                f32,
    pub workspace_button_shadow_y:                f32,
    pub workspace_button_shadow_blur:             f32
}

impl Default for WorkspaceConfig
{
    fn default() -> Self
    {
        Self
        {
            niri_workspaces_update_interval:          225,
            reverse_scroll_on_workspace:              false,
            persistent_workspaces:                    None,
            workspace_height:                         30,
            workspace_width:                          30,
            workspace_different_selected_width:       None,
            workspace_different_selected_height:      None,
            workspace_unique_text:                    None,
            workspace_selected_unique_text:           None,
            workspace_text_size:                      15,
            workspace_text:                           vec!["1".into(),"2".into(),"3".into(),"4".into(),"5".into(),"6".into(),"7".into(),"8".into(),"9".into(),"10".into()],
            workspace_text_color:                     ColorType::RGB([255, 255, 255]),
            workspace_selected_text_color:            ColorType::RGB([255, 255, 255]),
            workspace_text_orientation:               TextOrientation::Horizontal,
            workspace_selected_text:                  Some(vec!["●".into(),"●".into(),"●".into(),"●".into(),"●".into(),"●".into(),"●".into(),"●".into(),"●".into(),"●".into()]),
            workspace_spacing:                        3,
            workspace_padding:                        0,
            workspace_button_color:                   ColorType::RGB([45, 40, 55]),
            workspace_button_selected_color:          ColorType::RGB([150, 40, 80]),
            workspace_button_hovered_color:           ColorType::RGB([140, 35, 75]),
            workspace_button_hovered_text_color:      ColorType::RGB([255, 255, 255]),
            workspace_button_pressed_text_color:      ColorType::RGB([255, 255, 255]),
            workspace_button_pressed_color:           ColorType::RGB([90, 25, 50]),
            workspace_border_color:                   ColorType::RGB([120, 90, 135]),
            workspace_border_size:                    1.0,
            workspace_border_radius:                  [3.0, 3.0, 3.0, 3.0],
            workspace_side_separator:                 None,
            workspace_side_separator_color:           ColorType::RGB([75, 75, 75]),
            workspace_side_separator_width:           1.,
            workspace_side_separator_height:          16.,
            workspace_button_gradient_color:          None,
            workspace_button_selected_gradient_color: None,
            workspace_button_hovered_gradient_color:  None,
            workspace_button_pressed_gradient_color:  None,
            workspace_button_shadow_color:            None,
            workspace_button_shadow_x:                0.0,
            workspace_button_shadow_y:                0.0,
            workspace_button_shadow_blur:             0.0
        }
    }
}





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

#[derive(Clone)]
pub struct WorkspaceStyle
{
    pub hovered_text:       crate::helpers::color::ColorType,
    pub pressed_text:       crate::helpers::color::ColorType,
    pub normal_text:        crate::helpers::color::ColorType,
    pub border_size:        f32,
    pub border_color:       crate::helpers::color::ColorType,
    pub border_radius:      [f32; 4],
    pub normal_background:  Option<iced::Background>,
    pub hovered_background: Option<iced::Background>,
    pub pressed_background: Option<iced::Background>,
    pub shadow_color:       Option<crate::helpers::color::ColorType>,
    pub shadow_x:           f32,
    pub shadow_y:           f32,
    pub shadow_blur:        f32
}





// ============ CRATES ============
use crate::helpers::style::{UserStyle, orient_text, set_style, match_color_or_gradient};
use crate::AppData;





// ============ FUNCTIONS ============
pub fn build_workspace_style(app: &AppData, workspace_id: i32) -> WorkspaceStyle
{
    let is_selected = app.modules_data.workspace_data.current_workspace == workspace_id;
    let cfg         = &app.ron_config.workspace;

    let normal_color = if is_selected { cfg.workspace_button_selected_color } else { cfg.workspace_button_color };
    let normal_grad  = if is_selected { cfg.workspace_button_selected_gradient_color.clone() } else { cfg.workspace_button_gradient_color.clone() };

    WorkspaceStyle
    {
        hovered_text:       cfg.workspace_button_hovered_text_color,
        pressed_text:       cfg.workspace_button_pressed_text_color,
        normal_text:        if is_selected { cfg.workspace_selected_text_color } else { cfg.workspace_text_color },
        border_size:        cfg.workspace_border_size,
        border_color:       cfg.workspace_border_color,
        border_radius:      cfg.workspace_border_radius,
        normal_background:  match_color_or_gradient(normal_grad.as_ref(),                                         normal_color),
        hovered_background: match_color_or_gradient(cfg.workspace_button_hovered_gradient_color.as_ref(), cfg.workspace_button_hovered_color),
        pressed_background: match_color_or_gradient(cfg.workspace_button_pressed_gradient_color.as_ref(), cfg.workspace_button_pressed_color),
        shadow_color:       cfg.workspace_button_shadow_color,
        shadow_x:           cfg.workspace_button_shadow_x,
        shadow_y:           cfg.workspace_button_shadow_y,
        shadow_blur:        cfg.workspace_button_shadow_blur
    }
}



pub fn define_workspaces_style(ws: WorkspaceStyle, status: button::Status) -> iced::widget::button::Style
{
    set_style(UserStyle
    {
        status,
        hovered_text:       ws.hovered_text,
        pressed_text:       ws.pressed_text,
        normal_text:        ws.normal_text,
        border_color:       ws.border_color,
        border_size:        ws.border_size,
        border_radius:      ws.border_radius,
        normal_background:  ws.normal_background,
        hovered_background: ws.hovered_background,
        pressed_background: ws.pressed_background,
        shadow_color:       ws.shadow_color,
        shadow_x:           ws.shadow_x,
        shadow_y:           ws.shadow_y,
        shadow_blur:        ws.shadow_blur
    })
}


pub fn define_workspaces_text(app: &AppData, id: i32) -> String
{
    let string_not_oriented = if id == app.modules_data.workspace_data.current_workspace 
    {
        if let Some(unique_selected_text) = &app.ron_config.workspace.workspace_selected_unique_text
        {
            if unique_selected_text.contains("{workspace_number}")
            {
                unique_selected_text.replace("{workspace_number}", &id.to_string())
            }
            else
            {
                unique_selected_text.to_string()
            }
        }
        else
        {
            if let Some(selected) = &app.ron_config.workspace.workspace_selected_text 
            {
                let safe_id = id.saturating_sub(1) as usize;
                selected.get(safe_id).cloned().unwrap_or_else(|| id.to_string()) 
            } 
            else 
            {
                id.to_string() 
            }
        }
    } 
    else 
    { 
        if let Some(unique_text) = &app.ron_config.workspace.workspace_unique_text
        {
            if unique_text.contains("{workspace_number}")
            {
                unique_text.replace("{workspace_number}", &id.to_string())
            }
            else
            {
                unique_text.to_string()
            }
        }
        else
        {
            app.ron_config.workspace.workspace_text.get(id.saturating_sub(1) as usize).cloned().unwrap_or_else(|| id.to_string()) 
        }
    };

    orient_text(&string_not_oriented, &app.ron_config.workspace.workspace_text_orientation)
}



pub fn define_workspaces_size(app: &AppData, id: i32) -> (u32, u32)
{
    let width = if let Some(value) = app.ron_config.workspace.workspace_different_selected_width && id == app.modules_data.workspace_data.current_workspace
    {
        value
    } 
    else 
    {
        app.ron_config.workspace.workspace_width
    };

    let height = if let Some(value) = app.ron_config.workspace.workspace_different_selected_height && id == app.modules_data.workspace_data.current_workspace
    {
        value
    } 
    else 
    {
        app.ron_config.workspace.workspace_height
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
            visible_workspaces: vec![1, 2, 3]
        };
        app.ron_config.workspace.workspace_text = vec!["ws1".into(), "ws2".into(), "ws3".into()];
        app.ron_config.workspace.workspace_selected_text = Some(vec!["[1]".into(), "[2]".into(), "[3]".into()]);
        app.ron_config.workspace.workspace_width = 10;
        app.ron_config.workspace.workspace_different_selected_width = Some(20);
        app
    }
 
 
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
        let text = define_workspaces_text(&app, 10);
        assert_eq!(text, "10");
    }
 
    #[test]
    fn workspace_text_no_selected_vec_falls_back_to_id()
    {
        let mut app = make_app(2);
        app.ron_config.workspace.workspace_selected_text = None;
        let text = define_workspaces_text(&app, 2);
        assert_eq!(text, "2");
    }
 
    #[test]
    fn workspace_text_id_1_returns_first_element()
    {
        let app = make_app(3);
        assert_eq!(define_workspaces_text(&app, 1), "ws1");
    }
 
    #[test]
    fn workspace_text_all_three_workspaces_correct()
    {
        let app = make_app(99);
        assert_eq!(define_workspaces_text(&app, 1), "ws1");
        assert_eq!(define_workspaces_text(&app, 2), "ws2");
        assert_eq!(define_workspaces_text(&app, 3), "ws3");
    }
 
    #[test]
    fn workspace_text_selected_text_none_falls_back_for_current()
    {
        let mut app = make_app(2);
        app.ron_config.workspace.workspace_selected_text = None;
        assert_eq!(define_workspaces_text(&app, 2), "2");
    }
 
    fn make_style_app(current: i32) -> AppData
    {
        let mut app = make_app(current);
        app.ron_config.workspace.workspace_button_color = ColorType::RGB([0, 0, 200]);
        app.ron_config.workspace.workspace_button_selected_color = ColorType::RGB([255, 0, 0]);
        app.ron_config.workspace.workspace_button_hovered_color = ColorType::RGB([0, 200, 0]);
        app.ron_config.workspace.workspace_button_pressed_color = ColorType::RGB([0, 100, 0]);
        app
    }
 
    #[test]
    fn workspace_style_current_workspace_uses_selected_color()
    {
        let style = define_workspaces_style(build_workspace_style(&make_style_app(2), 2), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(255, 0, 0))));
    }
 
    #[test]
    fn workspace_style_non_current_uses_normal_color()
    {
        let style = define_workspaces_style(build_workspace_style(&make_style_app(1), 3), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 0, 200))));
    }
 
    #[test]
    fn workspace_style_selected_and_non_selected_differ()
    {
        let app      = make_style_app(1);
        let selected = define_workspaces_style(build_workspace_style(&app, 1), button::Status::Active);
        let other    = define_workspaces_style(build_workspace_style(&app, 2), button::Status::Active);
        assert_ne!(selected.background, other.background);
    }
 
    #[test]
    fn workspace_style_hovered_uses_hovered_color()
    {
        let style = define_workspaces_style(build_workspace_style(&make_style_app(1), 2), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 200, 0))));
    }
 
    #[test]
    fn workspace_style_pressed_uses_pressed_color()
    {
        let style = define_workspaces_style(build_workspace_style(&make_style_app(1), 2), button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 100, 0))));
    }
}
