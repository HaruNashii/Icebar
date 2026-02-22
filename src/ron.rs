// ============ IMPORTS ============
use ron::from_str;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use iced_layershell::reexport::Anchor;





// ============ STRUCTS/ENUM ============
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct BarConfig
{
    // ================= GENERAL =================
    pub display: Option<String>,
    pub bar_position: String,
    pub bar_size: [u32;2],
    pub bar_general_padding: u16,
    pub bar_background_color_rgba: [u8;4],
    pub font_family: String,
    pub font_style: String,


    // ================= MODULES =================
    pub left_modules: Vec<String>,
    pub center_modules: Vec<String>,
    pub right_modules: Vec<String>,


    // ================= MODULES CONFIGS =================
    pub force_static_position_context_menu: Option<(i32, i32)>,
    pub reverse_scroll_on_workspace: bool,
    pub incremental_steps_output: u8,
    pub incremental_steps_input: u8,


    // ================= FORMATS =================
    pub output_volume_format: [String;6],
    pub output_volume_muted_format: String,
    pub input_volume_format: [String;6],
    pub input_volume_muted_format: String,
    pub clock_format: String,
    pub clock_alt_format: String,


    // ================= TRAY (STYLE) =================
    pub tray_icon_size: u32,
    pub tray_button_size: u16,
    pub tray_spacing: u32,
    pub tray_background_color_rgba: [u8;4],
    pub tray_button_color_rgb: [u8;3],
    pub tray_button_text_color_rgb: [u8;3],
    pub tray_button_hovered_color_rgb: [u8;3],
    pub tray_button_hovered_text_color_rgb: [u8;3],
    pub tray_button_pressed_color_rgb: [u8;3],
    pub tray_border_color_rgba: [u8;4],
    pub tray_border_size: f32,
    pub tray_border_radius: [u32;4],


    // ================= CLOCK (STYLE) =================
    pub clock_text_size: u32,
    pub clock_background_color_rgba: [u8;4],
    pub clock_button_color_rgb: [u8;3],
    pub clock_button_text_color_rgb: [u8;3],
    pub clock_button_hovered_color_rgb: [u8;3],
    pub clock_button_hovered_text_color_rgb: [u8;3],
    pub clock_button_pressed_color_rgb: [u8;3],
    pub clock_border_color_rgba: [u8;4],
    pub clock_border_size: f32,
    pub clock_border_radius: [u32;4],


    // ================= VOLUME/OUTPUT (STYLE) =================
    pub volume_output_text_size: u32,
    pub volume_output_background_color_rgba: [u8;4],
    pub volume_output_button_color_rgb: [u8;3],
    pub volume_output_button_text_color_rgb: [u8;3],
    pub volume_output_button_hovered_color_rgb: [u8;3],
    pub volume_output_button_hovered_text_color_rgb: [u8;3],
    pub volume_output_button_pressed_color_rgb: [u8;3],
    pub volume_output_border_color_rgba: [u8;4],
    pub volume_output_border_size: f32,
    pub volume_output_border_radius: [u32;4],


    // ================= VOLUME/INPUT (STYLE) =================
    pub volume_input_text_size: u32,
    pub volume_input_background_color_rgba: [u8;4],
    pub volume_input_button_color_rgb: [u8;3],
    pub volume_input_button_text_color_rgb: [u8;3],
    pub volume_input_button_hovered_color_rgb: [u8;3],
    pub volume_input_button_hovered_text_color_rgb: [u8;3],
    pub volume_input_button_pressed_color_rgb: [u8;3],
    pub volume_input_border_color_rgba: [u8;4],
    pub volume_input_border_size: f32,
    pub volume_input_border_radius: [u32;4],


    // ================= HYPR WORKSPACES (STYLE) =================
    pub hypr_workspace_text_size: u32,
    pub hypr_workspace_text: Vec<String>,
    pub hypr_workspace_selected_text: Option<Vec<String>>,
    pub hypr_workspace_spacing: u32,
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


    // ================= CONTEXT MENU (STYLE) =================
    pub context_menu_background_color_rgba: [u8;4],
    pub context_menu_background_size: u16,
    pub context_menu_background_border_color_rgba: [u8;4],
    pub context_menu_background_border_size: f32,
    pub context_menu_background_border_radius: [u32;4],

    pub context_menu_text_size: u32,
    pub context_menu_width: u32,
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
        _ => 
        {
            println!("\n\n\n\n WARNING!!!! BAR POSITION PARSED IN THE CONFIG FILE IS NOT VALID, USING THE DEFAULT OPTION...\n\n\n\n");
            Anchor::Top | Anchor::Left | Anchor::Right
        }
    };
    (bar_config, anchor_position)
}
