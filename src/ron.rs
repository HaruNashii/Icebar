use ron::from_str;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use iced_layershell::reexport::Anchor;





#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct BarConfig
{
   pub bar_position: String,
   pub bar_size: [u32;2],
   pub bar_general_padding: u16,
   pub bar_background_color_rgba: [u8;4],

   pub left_modules: Vec<String>,
   pub center_modules: Vec<String>,
   pub right_modules: Vec<String>,

   pub volume_format: String,
   pub volume_muted_format: String,

   pub clock_format: String,
   pub clock_alt_format: String,

   pub tray_background_color_rgba: [u8;4],
   pub tray_button: [u8;3],
   pub tray_button_text: [u8;3],
   pub tray_button_hovered: [u8;3],
   pub tray_button_hovered_text: [u8;3],
   pub tray_button_pressed: [u8;3],
   pub tray_border_color: [u8;3],
   pub tray_border_size: f32,

   pub clock_background_color_rgba: [u8;4],
   pub clock_button: [u8;3],
   pub clock_button_text: [u8;3],
   pub clock_button_hovered: [u8;3],
   pub clock_button_hovered_text: [u8;3],
   pub clock_button_pressed: [u8;3],
   pub clock_border_color: [u8;3],
   pub clock_border_size: f32,

   pub volume_output_background_color_rgba: [u8;4],
   pub volume_output_button: [u8;3],
   pub volume_output_button_text: [u8;3],
   pub volume_output_button_hovered: [u8;3],
   pub volume_output_button_hovered_text: [u8;3],
   pub volume_output_button_pressed: [u8;3],
   pub volume_output_border_color: [u8;3],
   pub volume_output_border_size: f32,

   pub hypr_workspace_background_color_rgba: [u8;4],
   pub hypr_workspace_button: [u8;3],
   pub hypr_workspace_button_text: [u8;3],
   pub hypr_workspace_button_hovered: [u8;3],
   pub hypr_workspace_button_hovered_text: [u8;3],
   pub hypr_workspace_button_pressed: [u8;3],
   pub hypr_workspace_border_color: [u8;3],
   pub hypr_workspace_border_size: f32,

   pub context_menu_width: u32,
   pub context_menu_background_color_rgba: [u8;4],
   pub context_menu_button: [u8;3],
   pub context_menu_button_text: [u8;3],
   pub context_menu_button_hovered: [u8;3],
   pub context_menu_button_hovered_text: [u8;3],
   pub context_menu_button_pressed: [u8;3],
   pub context_menu_border_color: [u8;3],
   pub context_menu_border_size: f32

}

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
