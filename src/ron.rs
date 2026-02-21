// ============ IMPORTS ============
use ron::from_str;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use iced_layershell::reexport::Anchor;





// ============ STRUCTS/ENUM ============
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct BarConfig
{
   pub display: Option<String>,
   pub bar_position: String,
   pub bar_size: [u32;2],
   pub bar_general_padding: u16,
   pub bar_background_color_rgba: [u8;4],

   pub left_modules: Vec<String>,
   pub center_modules: Vec<String>,
   pub right_modules: Vec<String>,

   pub reverse_scroll_on_workspace: bool,
   pub incremental_steps_output: u8,
   pub incremental_steps_input: u8,

   pub output_volume_format: String,
   pub output_volume_muted_format: String,
   pub input_volume_format: String,
   pub input_volume_muted_format: String,
   pub clock_format: String,
   pub clock_alt_format: String,

   pub tray_background_color_rgba: [u8;4],
   pub tray_button_color_rgb: [u8;3],
   pub tray_button_text_color_rgb: [u8;3],
   pub tray_button_hovered_color_rgb: [u8;3],
   pub tray_button_hovered_text_color_rgb: [u8;3],
   pub tray_button_pressed_color_rgb: [u8;3],
   pub tray_border_color_rgba: [u8;4],
   pub tray_border_size: f32,
   pub tray_border_radius: [u32;4],

   pub clock_background_color_rgba: [u8;4],
   pub clock_button_color_rgb: [u8;3],
   pub clock_button_text_color_rgb: [u8;3],
   pub clock_button_hovered_color_rgb: [u8;3],
   pub clock_button_hovered_text_color_rgb: [u8;3],
   pub clock_button_pressed_color_rgb: [u8;3],
   pub clock_border_color_rgba: [u8;4],
   pub clock_border_size: f32,
   pub clock_border_radius: [u32;4],

   pub volume_output_background_color_rgba: [u8;4],
   pub volume_output_button_color_rgb: [u8;3],
   pub volume_output_button_text_color_rgb: [u8;3],
   pub volume_output_button_hovered_color_rgb: [u8;3],
   pub volume_output_button_hovered_text_color_rgb: [u8;3],
   pub volume_output_button_pressed_color_rgb: [u8;3],
   pub volume_output_border_color_rgba: [u8;4],
   pub volume_output_border_size: f32,
   pub volume_output_border_radius: [u32;4],

   pub volume_input_background_color_rgba: [u8;4],
   pub volume_input_button_color_rgb: [u8;3],
   pub volume_input_button_text_color_rgb: [u8;3],
   pub volume_input_button_hovered_color_rgb: [u8;3],
   pub volume_input_button_hovered_text_color_rgb: [u8;3],
   pub volume_input_button_pressed_color_rgb: [u8;3],
   pub volume_input_border_color_rgba: [u8;4],
   pub volume_input_border_size: f32,
   pub volume_input_border_radius: [u32;4],

   pub hypr_workspace_text: [String;10],
   pub hypr_workspace_background_color_rgba: [u8;4],
   pub hypr_workspace_button_color_rgb: [u8;3],
   pub hypr_workspace_button_text_color_rgb: [u8;3],
   pub hypr_workspace_button_selected_color_rgb: [u8;3],
   pub hypr_workspace_button_hovered_color_rgb: [u8;3],
   pub hypr_workspace_button_hovered_text_color_rgb: [u8;3],
   pub hypr_workspace_button_pressed_color_rgb: [u8;3],
   pub hypr_workspace_border_color_rgba: [u8;4],
   pub hypr_workspace_border_size: f32,
   pub hypr_workspace_border_radius: [u32;4],

   pub context_menu_width: u32,
   pub context_menu_background_color_rgba: [u8;4],
   pub context_menu_button_color_rgb: [u8;3],
   pub context_menu_button_text_color_rgb: [u8;3],
   pub context_menu_button_hovered_color_rgb: [u8;3],
   pub context_menu_button_hovered_text_color_rgb: [u8;3],
   pub context_menu_button_pressed_color_rgb: [u8;3],
   pub context_menu_border_color_rgba: [u8;4],
   pub context_menu_border_size: f32,
   pub context_menu_border_radius: [u32;4]

}





// ============ FUNCTIONS ============
pub fn read_ron_config() -> (BarConfig, Anchor)
{
    let home_path = home::home_dir().expect("Failed To Get Home Directory").display().to_string();
    let ron_config_file_dir = format!("{}/.config/icebar/config.ron", home_path);
    let ron_file_config_path = Path::new(&ron_config_file_dir);
    let ron_content = fs::read_to_string(ron_file_config_path).expect("Couldn't Read Config File");
    let bar_config: BarConfig = from_str(&ron_content).expect("Coudln't Translate Config File");
    let anchor_position = match bar_config.bar_position.as_str()
    {
        "Up" => Anchor::Top | Anchor::Left | Anchor::Right,
        "Down" => Anchor::Bottom | Anchor::Left | Anchor::Right,
        "Left" => Anchor::Left | Anchor::Top | Anchor::Bottom,
        "Right" => Anchor::Right | Anchor::Top | Anchor::Bottom,
        _ => Anchor::Top | Anchor::Left | Anchor::Right,
    };
    (bar_config, anchor_position)
}
