// ============ IMPORTS ============
use hyprland::shared::HyprDataActiveOptional;
use iced::widget::button;





// ============ CRATES ============
use crate::helpers::style::{UserStyle, orient_text, set_style};
use crate::AppData;





// ============ STRUCTS ============
#[derive(Default, Clone)]
pub struct FocusedWindowData
{
    pub title: String,
}





// ============ FUNCTIONS ============
pub fn read_focused_window_hypr() -> Option<String>
{
    let client = hyprland::data::Client::get_active().ok()??;
    Some(client.title)
}



pub fn read_focused_window_sway() -> Option<String>
{
    let mut conn = swayipc::Connection::new().ok()?;
    let tree     = conn.get_tree().ok()?;
    find_focused_sway(&tree)
}


fn find_focused_sway(node: &swayipc::Node) -> Option<String>
{
    use swayipc::NodeType;

    if node.focused
    {
        // Only return a title for actual windows, not workspaces/outputs
        return match node.node_type
        {
            NodeType::Con | NodeType::FloatingCon => node.name.clone(),
            _                                     => None,
        };
    }
    for child in &node.nodes
    {
        if let Some(title) = find_focused_sway(child) { return Some(title); }
    }
    for child in &node.floating_nodes
    {
        if let Some(title) = find_focused_sway(child) { return Some(title); }
    }
    None
}


pub fn read_focused_window_niri() -> Option<String>
{
    use niri_ipc::{Request, Response, socket::Socket};
    let mut socket = Socket::connect().ok()?;
    let reply      = socket.send(Request::FocusedWindow).ok()?;
    match reply
    {
        Ok(Response::FocusedWindow(Some(w))) => w.title,
        _=> None,
    }
}



pub fn define_focused_window_text(app: &AppData) -> String
{
    let title = &app.modules_data.focused_window_data.title;
    if title.is_empty()
    {
        if app.ron_config.dont_show_focused_window_if_empty { return String::new(); };
        return orient_text(&app.ron_config.text_when_focused_window_is_empty, &app.ron_config.focused_window_text_orientation);
    };
    let text  = app.ron_config.focused_window_format.replace("{title}", title);
    orient_text(&text, &app.ron_config.focused_window_text_orientation)
}



pub fn define_focused_window_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    set_style(UserStyle
    {
        status,
        normal:            app.ron_config.focused_window_button_color,
        normal_text:       app.ron_config.focused_window_text_color,
        hovered:           app.ron_config.focused_window_button_hovered_color,
        hovered_text:      app.ron_config.focused_window_button_hovered_text_color,
        pressed:           app.ron_config.focused_window_button_pressed_color,
        border_color: app.ron_config.focused_window_border_color,
        border_size:       app.ron_config.focused_window_border_size,
        border_radius:     app.ron_config.focused_window_border_radius,
    })
}




// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
 
    // ---- helpers ------------------------------------------------------------
    fn make_node(name: Option<&str>, focused: bool, nodes: Vec<swayipc::Node>, floating: Vec<swayipc::Node>) -> swayipc::Node
    {
        let name_val = match name
        {
            Some(n) => format!("\"{}\"", n),
            None    => "null".into(),
        };
        let nodes_json    = serde_json::to_string(&nodes).unwrap();
        let floating_json = serde_json::to_string(&floating).unwrap();
    
        let json = format!(r#"{{
            "id": 0,
            "name": {name_val},
            "focused": {focused},
            "type": "con",
            "border": "normal",
            "current_border_width": 0,
            "layout": "none",
            "orientation": "none",
            "percent": null,
            "focus": [],
            "rect": {{"x":0,"y":0,"width":0,"height":0}},
            "window_rect": {{"x":0,"y":0,"width":0,"height":0}},
            "deco_rect": {{"x":0,"y":0,"width":0,"height":0}},
            "geometry": {{"x":0,"y":0,"width":0,"height":0}},
            "urgent": false,
            "marks": [],
            "fullscreen_mode": 0,
            "nodes": {nodes_json},
            "floating_nodes": {floating_json},
            "sticky": false
        }}"#);
    
        serde_json::from_str(&json).unwrap()
    } 

 
    fn make_app(title: &str) -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.modules_data.focused_window_data.title        = title.into();
        app.ron_config.focused_window_format              = "{title}".into();
        app.ron_config.text_when_focused_window_is_empty  = "Desktop".into();
        app.ron_config.dont_show_focused_window_if_empty  = false;
        app
    }
 
    // ---- find_focused_sway --------------------------------------------------
 
    #[test]
    fn find_focused_sway_returns_title_of_focused_node()
    {
        let node = make_node(Some("My Window"), true, vec![], vec![]);
        assert_eq!(find_focused_sway(&node), Some("My Window".into()));
    }
 
    #[test]
    fn find_focused_sway_unfocused_root_returns_none()
    {
        let node = make_node(Some("Root"), false, vec![], vec![]);
        assert!(find_focused_sway(&node).is_none());
    }
 
    #[test]
    fn find_focused_sway_focused_node_with_no_name_returns_none()
    {
        let node = make_node(None, true, vec![], vec![]);
        assert!(find_focused_sway(&node).is_none());
    }
 
    #[test]
    fn find_focused_sway_finds_focused_child()
    {
        let child  = make_node(Some("Child"), true,  vec![], vec![]);
        let parent = make_node(None,          false, vec![child], vec![]);
        assert_eq!(find_focused_sway(&parent), Some("Child".into()));
    }
 
    #[test]
    fn find_focused_sway_finds_deeply_nested_focused_node()
    {
        let leaf = make_node(Some("Deep"), true,  vec![], vec![]);
        let mid  = make_node(None,         false, vec![leaf], vec![]);
        let root = make_node(None,         false, vec![mid],  vec![]);
        assert_eq!(find_focused_sway(&root), Some("Deep".into()));
    }
 
    #[test]
    fn find_focused_sway_returns_none_when_nothing_focused()
    {
        let a    = make_node(Some("A"), false, vec![], vec![]);
        let b    = make_node(Some("B"), false, vec![], vec![]);
        let root = make_node(None,      false, vec![a, b], vec![]);
        assert!(find_focused_sway(&root).is_none());
    }
 
    #[test]
    fn find_focused_sway_returns_first_focused_among_siblings()
    {
        let first  = make_node(Some("First"),  true, vec![], vec![]);
        let second = make_node(Some("Second"), true, vec![], vec![]);
        let root   = make_node(None, false, vec![first, second], vec![]);
        assert_eq!(find_focused_sway(&root), Some("First".into()));
    }
 
    #[test]
    fn find_focused_sway_checks_floating_nodes()
    {
        let floating = make_node(Some("Float"), true, vec![], vec![]);
        let root     = make_node(None, false, vec![], vec![floating]);
        assert_eq!(find_focused_sway(&root), Some("Float".into()));
    }
 
    #[test]
    fn find_focused_sway_prefers_regular_child_over_floating()
    {
        // Regular nodes are searched first
        let regular  = make_node(Some("Regular"),  true, vec![], vec![]);
        let floating = make_node(Some("Floating"), true, vec![], vec![]);
        let root     = make_node(None, false, vec![regular], vec![floating]);
        assert_eq!(find_focused_sway(&root), Some("Regular".into()));
    }
 
    #[test]
    fn find_focused_sway_floating_found_when_no_regular_focused()
    {
        let regular  = make_node(Some("Regular"),  false, vec![], vec![]);
        let floating = make_node(Some("Floating"), true,  vec![], vec![]);
        let root     = make_node(None, false, vec![regular], vec![floating]);
        assert_eq!(find_focused_sway(&root), Some("Floating".into()));
    }
 
    #[test]
    fn find_focused_sway_empty_tree_returns_none()
    {
        let root = make_node(None, false, vec![], vec![]);
        assert!(find_focused_sway(&root).is_none());
    }
 
    // ---- define_focused_window_text -----------------------------------------
 
    #[test]
    fn focused_window_text_replaces_title_placeholder()
    {
        let app = make_app("Firefox");
        assert_eq!(define_focused_window_text(&app), "Firefox");
    }
 
    #[test]
    fn focused_window_text_custom_format()
    {
        let mut app = make_app("Alacritty");
        app.ron_config.focused_window_format = "[ {title} ]".into();
        assert_eq!(define_focused_window_text(&app), "[ Alacritty ]");
    }
 
    #[test]
    fn focused_window_text_empty_title_returns_fallback()
    {
        let app = make_app("");
        assert_eq!(define_focused_window_text(&app), "Desktop");
    }
 
    #[test]
    fn focused_window_text_empty_title_dont_show_flag_returns_empty()
    {
        let mut app = make_app("");
        app.ron_config.dont_show_focused_window_if_empty = true;
        assert_eq!(define_focused_window_text(&app), "");
    }
 
    #[test]
    fn focused_window_text_dont_show_flag_ignored_when_title_not_empty()
    {
        let mut app = make_app("Vim");
        app.ron_config.dont_show_focused_window_if_empty = true;
        assert_eq!(define_focused_window_text(&app), "Vim");
    }
 
    #[test]
    fn focused_window_text_fallback_uses_orientation()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty  = "Hi".into();
        app.ron_config.focused_window_text_orientation    = TextOrientation::Vertical;
        let result = define_focused_window_text(&app);
        assert!(result.contains('\n'));
    }
 
    #[test]
    fn focused_window_text_title_uses_orientation()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("abc");
        app.ron_config.focused_window_text_orientation = TextOrientation::Vertical;
        let result = define_focused_window_text(&app);
        assert!(result.contains('\n'));
    }
 
    #[test]
    fn focused_window_text_format_with_no_placeholder_returns_format_string()
    {
        let mut app = make_app("Anything");
        app.ron_config.focused_window_format = "static".into();
        assert_eq!(define_focused_window_text(&app), "static");
    }
 
    #[test]
    fn focused_window_text_multiple_title_placeholders_all_replaced()
    {
        let mut app = make_app("Vim");
        app.ron_config.focused_window_format = "{title} | {title}".into();
        assert_eq!(define_focused_window_text(&app), "Vim | Vim");
    }
 
    // ---- find_focused_sway: extra cases -------------------------------------

    #[test]
    fn find_focused_sway_unicode_title()
    {
        let node = make_node(Some("日本語ウィンドウ"), true, vec![], vec![]);
        assert_eq!(find_focused_sway(&node), Some("日本語ウィンドウ".into()));
    }

    #[test]
    fn find_focused_sway_title_with_spaces()
    {
        let node = make_node(Some("My Text Editor"), true, vec![], vec![]);
        assert_eq!(find_focused_sway(&node), Some("My Text Editor".into()));
    }

    #[test]
    fn find_focused_sway_title_with_special_chars()
    {
        let node = make_node(Some("file.rs — VSCode"), true, vec![], vec![]);
        assert_eq!(find_focused_sway(&node), Some("file.rs — VSCode".into()));
    }

    #[test]
    fn find_focused_sway_whitespace_only_name_returned_as_is()
    {
        // Whitespace is a valid name string — we don't trim it
        let node = make_node(Some("   "), true, vec![], vec![]);
        assert_eq!(find_focused_sway(&node), Some("   ".into()));
    }

    #[test]
    fn find_focused_sway_three_levels_deep()
    {
        let leaf  = make_node(Some("Leaf"),  true,  vec![], vec![]);
        let mid1  = make_node(None,          false, vec![leaf], vec![]);
        let mid2  = make_node(None,          false, vec![mid1], vec![]);
        let root  = make_node(None,          false, vec![mid2], vec![]);
        assert_eq!(find_focused_sway(&root), Some("Leaf".into()));
    }

    #[test]
    fn find_focused_sway_focused_node_is_root_even_with_children()
    {
        // Root itself is focused — should return root's title without descending
        let child = make_node(Some("Child"), false, vec![], vec![]);
        let root  = make_node(Some("Root"),  true,  vec![child], vec![]);
        assert_eq!(find_focused_sway(&root), Some("Root".into()));
    }

    #[test]
    fn find_focused_sway_second_sibling_focused()
    {
        let first  = make_node(Some("First"),  false, vec![], vec![]);
        let second = make_node(Some("Second"), true,  vec![], vec![]);
        let root   = make_node(None, false, vec![first, second], vec![]);
        assert_eq!(find_focused_sway(&root), Some("Second".into()));
    }

    #[test]
    fn find_focused_sway_multiple_floating_returns_first_focused()
    {
        let f1   = make_node(Some("Float1"), true,  vec![], vec![]);
        let f2   = make_node(Some("Float2"), false, vec![], vec![]);
        let root = make_node(None, false, vec![], vec![f1, f2]);
        assert_eq!(find_focused_sway(&root), Some("Float1".into()));
    }

    #[test]
    fn find_focused_sway_focused_child_in_floating_subtree()
    {
        let deep     = make_node(Some("Deep Float"), true,  vec![], vec![]);
        let floating = make_node(None,               false, vec![deep], vec![]);
        let root     = make_node(None, false, vec![], vec![floating]);
        assert_eq!(find_focused_sway(&root), Some("Deep Float".into()));
    }

    #[test]
    fn find_focused_sway_empty_string_name_treated_as_some()
    {
        // Some("") is different from None — we return it
        let node = make_node(Some(""), true, vec![], vec![]);
        assert_eq!(find_focused_sway(&node), Some("".into()));
    }

    // ---- define_focused_window_text: extra cases ----------------------------

    #[test]
    fn focused_window_text_whitespace_title_is_not_empty()
    {
        // "   " is not empty() so it goes through the normal format path
        let app = make_app("   ");
        assert_eq!(define_focused_window_text(&app), "   ");
    }

    #[test]
    fn focused_window_text_title_with_unicode()
    {
        let app = make_app("ターミナル");
        assert_eq!(define_focused_window_text(&app), "ターミナル");
    }

    #[test]
    fn focused_window_text_title_with_brackets_in_format()
    {
        let mut app = make_app("Vim");
        app.ron_config.focused_window_format = "[{title}]".into();
        assert_eq!(define_focused_window_text(&app), "[Vim]");
    }

    #[test]
    fn focused_window_text_empty_format_string_returns_empty()
    {
        let mut app = make_app("Firefox");
        app.ron_config.focused_window_format = "".into();
        assert_eq!(define_focused_window_text(&app), "");
    }

    #[test]
    fn focused_window_text_fallback_empty_returns_empty_string_when_flag_false()
    {
        // text_when_focused_window_is_empty itself is empty
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty = "".into();
        app.ron_config.dont_show_focused_window_if_empty = false;
        assert_eq!(define_focused_window_text(&app), "");
    }

    #[test]
    fn focused_window_text_very_long_title()
    {
        let long  = "a".repeat(500);
        let app   = make_app(&long);
        let result = define_focused_window_text(&app);
        assert_eq!(result, long);
    }

    #[test]
    fn focused_window_text_title_containing_placeholder_not_double_replaced()
    {
        // If the title itself contains "{title}", it must not be replaced again
        let mut app = make_app("{title}");
        app.ron_config.focused_window_format = "{title}".into();
        // str::replace replaces the first occurrence — result is "{title}" expanded once
        // i.e. the title value "{title}" is placed where {title} was
        assert_eq!(define_focused_window_text(&app), "{title}");
    }

    #[test]
    fn focused_window_text_custom_fallback_text()
    {
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty  = "~ desktop ~".into();
        app.ron_config.dont_show_focused_window_if_empty  = false;
        assert_eq!(define_focused_window_text(&app), "~ desktop ~");
    }

    // ---- FocusedWindowData --------------------------------------------------

    #[test]
    fn focused_window_data_default_title_is_empty()
    {
        assert_eq!(FocusedWindowData::default().title, "");
    }

    #[test]
    fn focused_window_data_clone_is_independent()
    {
        let mut original = FocusedWindowData { title: "Firefox".into() };
        let clone        = original.clone();
        original.title   = "Vim".into();
        assert_eq!(clone.title, "Firefox");
    }

    #[test]
    fn focused_window_data_title_can_be_set_and_read()
    {
        let data = FocusedWindowData { title: "Alacritty".into() };
        assert_eq!(data.title, "Alacritty");
    }

    #[test]
    fn focused_window_data_title_unicode()
    {
        let data = FocusedWindowData { title: "ターミナル".into() };
        assert_eq!(data.title, "ターミナル");
    }

    // ---- define_focused_window_text: title present --------------------------

    #[test]
    fn text_basic_title_substitution()
    {
        let app = make_app("Firefox");
        assert_eq!(define_focused_window_text(&app), "Firefox");
    }

    #[test]
    fn text_custom_format_with_prefix()
    {
        let mut app = make_app("Vim");
        app.ron_config.focused_window_format = " {title}".into();
        assert_eq!(define_focused_window_text(&app), " Vim");
    }

    #[test]
    fn text_custom_format_with_suffix()
    {
        let mut app = make_app("Vim");
        app.ron_config.focused_window_format = "{title} — icebar".into();
        assert_eq!(define_focused_window_text(&app), "Vim — icebar");
    }

    #[test]
    fn text_custom_format_wrapping_brackets()
    {
        let mut app = make_app("Alacritty");
        app.ron_config.focused_window_format = "[ {title} ]".into();
        assert_eq!(define_focused_window_text(&app), "[ Alacritty ]");
    }

    #[test]
    fn text_multiple_placeholders_all_replaced()
    {
        let mut app = make_app("Vim");
        app.ron_config.focused_window_format = "{title} | {title}".into();
        assert_eq!(define_focused_window_text(&app), "Vim | Vim");
    }

    #[test]
    fn text_empty_format_string_returns_empty()
    {
        let mut app = make_app("Firefox");
        app.ron_config.focused_window_format = "".into();
        assert_eq!(define_focused_window_text(&app), "");
    }

    #[test]
    fn text_format_with_no_placeholder_returns_literal()
    {
        let mut app = make_app("Anything");
        app.ron_config.focused_window_format = "static label".into();
        assert_eq!(define_focused_window_text(&app), "static label");
    }

    #[test]
    fn text_unicode_title_passes_through()
    {
        let app = make_app("ターミナル");
        assert_eq!(define_focused_window_text(&app), "ターミナル");
    }

    #[test]
    fn text_very_long_title_not_truncated()
    {
        let long = "a".repeat(500);
        let app  = make_app(&long);
        assert_eq!(define_focused_window_text(&app), long);
    }

    #[test]
    fn text_dont_show_flag_ignored_when_title_present()
    {
        let mut app = make_app("Vim");
        app.ron_config.dont_show_focused_window_if_empty = true;
        assert_eq!(define_focused_window_text(&app), "Vim");
    }

    // ---- define_focused_window_text: empty title ----------------------------

    #[test]
    fn text_empty_title_returns_fallback_string()
    {
        let app = make_app("");
        assert_eq!(define_focused_window_text(&app), "Desktop");
    }

    #[test]
    fn text_empty_title_dont_show_flag_returns_empty_string()
    {
        let mut app = make_app("");
        app.ron_config.dont_show_focused_window_if_empty = true;
        assert_eq!(define_focused_window_text(&app), "");
    }

    #[test]
    fn text_empty_title_custom_fallback()
    {
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty = "~ desktop ~".into();
        assert_eq!(define_focused_window_text(&app), "~ desktop ~");
    }

    #[test]
    fn text_empty_title_fallback_itself_empty()
    {
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty = "".into();
        app.ron_config.dont_show_focused_window_if_empty = false;
        assert_eq!(define_focused_window_text(&app), "");
    }

    #[test]
    fn text_empty_title_unicode_fallback()
    {
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty = "デスクトップ".into();
        assert_eq!(define_focused_window_text(&app), "デスクトップ");
    }

    // whitespace title is NOT considered empty by is_empty()
    #[test]
    fn text_whitespace_title_goes_through_format_not_fallback()
    {
        let app = make_app("   ");
        assert_eq!(define_focused_window_text(&app), "   ");
    }

    // ---- define_focused_window_text: orientation ----------------------------

    #[test]
    fn text_title_vertical_orientation_contains_newline()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("abc");
        app.ron_config.focused_window_text_orientation = TextOrientation::Vertical;
        assert!(define_focused_window_text(&app).contains('\n'));
    }

    #[test]
    fn text_fallback_vertical_orientation_contains_newline()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty  = "Hi".into();
        app.ron_config.focused_window_text_orientation    = TextOrientation::Vertical;
        assert!(define_focused_window_text(&app).contains('\n'));
    }

    #[test]
    fn text_horizontal_orientation_no_newline()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("Firefox");
        app.ron_config.focused_window_text_orientation = TextOrientation::Horizontal;
        assert!(!define_focused_window_text(&app).contains('\n'));
    }

    #[test]
    fn text_fallback_horizontal_orientation_no_newline()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("");
        app.ron_config.text_when_focused_window_is_empty  = "Desktop".into();
        app.ron_config.focused_window_text_orientation    = TextOrientation::Horizontal;
        assert!(!define_focused_window_text(&app).contains('\n'));
    }
}
