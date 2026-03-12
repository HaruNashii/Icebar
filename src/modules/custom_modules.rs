// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use iced::widget::button;





// ============ CRATES ============
use crate::helpers::{string::ellipsize, style::{TextOrientation, UserStyle, set_style}};
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct CustomModule
{
    pub name: String,
    pub text: String,
    pub text_size: u32,
    pub text_color_rgb: [u8;3],
    pub text_orientation: TextOrientation,
    pub height: u32,
    pub button_color_rgb: [u8;3],
    pub button_text_color_rgb: [u8;3],
    pub button_hovered_color_rgb: [u8;3],
    pub button_hovered_text_color_rgb: [u8;3],
    pub button_pressed_color_rgb: [u8;3],
    pub border_color_rgba: [u8;4],
    pub border_size: f32,
    pub border_radius: [f32;4],
    pub dont_show_if_any_output_is_empty: bool,
    pub display_err_output_if_failed: bool,
    pub use_output_as_text: bool,
    pub use_continous_output_as_text: bool,
    pub all_output_as_text_format: String,
    pub output_text_limit_len: usize,
    pub command_to_exec_on_left_click: Vec<String>,
    pub command_to_exec_on_right_click: Vec<String>,
    pub continous_command_interval: u64,
    pub continous_command: Vec<String>
}





// ============ FUNCTIONS ============
impl Default for CustomModule
{
    fn default() -> Self
    {
        Self 
        {
            name: "Default Custom Module".to_string(),
            text: "".to_string(),
            text_size: 10,
            text_color_rgb: [255, 255, 255],
            text_orientation: TextOrientation::Horizontal,
            height: 30,
            button_color_rgb: [60, 50, 70],
            button_text_color_rgb: [220, 220, 230],
            button_hovered_color_rgb: [110, 40, 80],
            button_hovered_text_color_rgb: [255, 255, 255],
            button_pressed_color_rgb: [70, 20, 40],
            border_color_rgba: [90, 70, 100, 100],
            border_size: 1.0,
            border_radius: [3., 3., 3., 3.],
            display_err_output_if_failed: true,
            dont_show_if_any_output_is_empty: false,
            use_output_as_text: false,
            use_continous_output_as_text: false,
            all_output_as_text_format: "Undefined".to_string(),
            output_text_limit_len: 100,
            command_to_exec_on_left_click: vec![], 
            command_to_exec_on_right_click: vec![],
            continous_command_interval: 500,
            continous_command: vec![]
        }
    }
}



pub fn define_custom_module_style(custom_module: &CustomModule, status: button::Status) -> iced::widget::button::Style
{
    let hovered = custom_module.button_hovered_color_rgb; 
    let hovered_text = custom_module.button_hovered_text_color_rgb; 
    let pressed = custom_module.button_pressed_color_rgb; 
    let normal = custom_module.button_color_rgb; 
    let normal_text = custom_module.button_text_color_rgb; 
    let border_size = custom_module.border_size; 
    let border_color_rgba = custom_module.border_color_rgba; 
    let border_radius = custom_module.border_radius;
    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
}



pub fn define_custom_module_text(index: usize, custom_module: &CustomModule, app: &AppData) -> String
{
    // COMMAND_OUTPUT
    if custom_module.use_output_as_text && !custom_module.all_output_as_text_format.is_empty()
    {
        let output_text = app.cached_command_outputs.get(index).map(String::as_str).unwrap_or("");
        let output_text = ellipsize(&app.ron_config.ellipsis_text, output_text, custom_module.output_text_limit_len);
        if custom_module.dont_show_if_any_output_is_empty && output_text.is_empty() { return String::new() };
        custom_module.all_output_as_text_format.replace("{text}", &custom_module.text).replace("{output}", &output_text).replace('\n', "")
    }
    // CONTINOUS_OUTPUT
    else if custom_module.use_continous_output_as_text && !custom_module.all_output_as_text_format.is_empty() && !&app.cached_continuous_outputs.is_empty() && (app.cached_continuous_outputs.len() - 1) >= index
    {
        let output_text = ellipsize(&app.ron_config.ellipsis_text, &app.cached_continuous_outputs[index], custom_module.output_text_limit_len);
        if custom_module.dont_show_if_any_output_is_empty && output_text.is_empty() { return String::new() };
        custom_module.all_output_as_text_format.replace("{text}", &custom_module.text).replace("{continous_output}", &output_text).replace('\n', "")
    }
    // NO OUTPUT JUST TEXT
    else 
    {
        custom_module.text.clone()
    }
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
 
    fn make_module() -> CustomModule
    {
        CustomModule
        {
            text: "MyText".into(),
            use_output_as_text: false,
            use_continous_output_as_text: false,
            all_output_as_text_format: String::new(),
            output_text_limit_len: 100,
            dont_show_if_any_output_is_empty: false,
            ..Default::default()
        }
    }
 
    // ---- define_custom_module_text ------------------------------------------
 
    #[test]
    fn custom_module_text_plain_returns_text_field()
    {
        let app = AppData::default();
        let m = make_module();
        assert_eq!(define_custom_module_text(0, &m, &app), "MyText");
    }
 
    #[test]
    fn custom_module_text_with_command_output()
    {
        let mut app = AppData::default();
        app.cached_command_outputs = vec!["CmdOut".to_string()];
        let m = CustomModule
        {
            use_output_as_text: true,
            all_output_as_text_format: "{output}".into(),
            ..make_module()
        };
        assert_eq!(define_custom_module_text(0, &m, &app), "CmdOut");
    }
 
    #[test]
    fn custom_module_text_format_replaces_text_and_output()
    {
        let mut app = AppData::default();
        app.cached_command_outputs = vec!["99%".to_string()];
        let m = CustomModule
        {
            text: "CPU".into(),
            use_output_as_text: true,
            all_output_as_text_format: "{text}: {output}".into(),
            ..make_module()
        };
        assert_eq!(define_custom_module_text(0, &m, &app), "CPU: 99%");
    }
 
    #[test]
    fn custom_module_text_dont_show_if_empty()
    {
        let mut app = AppData::default();
        app.cached_command_outputs = vec!["".to_string()];
        let m = CustomModule
        {
            use_output_as_text: true,
            all_output_as_text_format: "{output}".into(),
            dont_show_if_any_output_is_empty: true,
            ..make_module()
        };
        assert_eq!(define_custom_module_text(0, &m, &app), "");
    }
 
    #[test]
    fn custom_module_text_with_continuous_output()
    {
        let mut app = AppData::default();
        app.cached_continuous_outputs = vec!["live_data".to_string()];
        let m = CustomModule
        {
            use_continous_output_as_text: true,
            all_output_as_text_format: "{continous_output}".into(),
            ..make_module()
        };
        assert_eq!(define_custom_module_text(0, &m, &app), "live_data");
    }
 
    #[test]
    fn custom_module_text_strips_newlines_from_output()
    {
        let mut app = AppData::default();
        app.cached_command_outputs = vec!["line1\nline2".to_string()];
        let m = CustomModule
        {
            use_output_as_text: true,
            all_output_as_text_format: "{output}".into(),
            ..make_module()
        };
        assert_eq!(define_custom_module_text(0, &m, &app), "line1line2");
    }
}
