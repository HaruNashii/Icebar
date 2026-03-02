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
    pub use_output_as_text: bool,
    pub use_continous_output_as_text: bool,
    pub all_output_as_text_format: String,
    pub output_text_limit_len: usize,
    pub command_to_exec_on_left_click: Vec<String>,
    pub command_to_exec_on_right_click: Vec<String>,
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
            use_output_as_text: false,
            use_continous_output_as_text: false,
            all_output_as_text_format: "Undefined".to_string(),
            output_text_limit_len: 100,
            command_to_exec_on_left_click: vec![], 
            command_to_exec_on_right_click: vec![],
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
        custom_module.all_output_as_text_format.replace("{text}", &custom_module.text).replace("{output}", &output_text).replace('\n', "")
    }
    // CONTINOUS_OUTPUT
    else if custom_module.use_continous_output_as_text && !custom_module.all_output_as_text_format.is_empty() && !&app.cached_continuous_outputs.is_empty() && (app.cached_continuous_outputs.len() - 1) >= index
    {
        let output_text = ellipsize(&app.ron_config.ellipsis_text, &app.cached_continuous_outputs[index], custom_module.output_text_limit_len);
        custom_module.all_output_as_text_format.replace("{text}", &custom_module.text).replace("{continous_output}", &output_text).replace('\n', "")
    }
    // NO OUTPUT JUST TEXT
    else 
    {
        custom_module.text.clone()
    }
}
