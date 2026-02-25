// ============ IMPORTS ============
use serde::{Deserialize, Serialize};





// ============ ENUM/STRUCT, ETC ============
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct CustomModule
{
    pub name: String,
    pub text: String,
    pub text_size: u32,
    pub height: u32,
    pub background_color_rgba: [u8;4],
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
            height: 30,
            background_color_rgba: [30, 30, 36, 0],
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
