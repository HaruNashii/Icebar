// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use std::{fs, collections::HashSet};





// ============ CRATES ============
use crate::modules::{custom_modules::CustomModule, data::Modules};
use crate::helpers::{color::ColorType, ron_general::apply_general_settings, style::{SideOption, TextOrientation}};





// ============ STRUCTS/ENUM'S ============
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum BarPosition
{
    Up,
    Down,
    Left, 
    Right
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ActionOnClick 
{
    Nothing,
    DefaultAction,
    CustomAction(Vec<String>),
    CycleClockTimezones,
    ToggleAltClockAndCycleClockTimezones
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BarConfig
{
    // ================= GENERAL =================
    pub display: Option<String>,
    pub bar_position: BarPosition,
    pub floating_space: i32,
    pub increased_exclusive_bar_zone: i32,
    pub bar_check_reload_interval_ms: Option<u64>,
    pub bar_side_spaces_size: u32,
    pub bar_size: [u32;2],
    pub bar_border_radius: [f32;4],
    pub bar_border_size: f32,
    pub bar_border_color: ColorType,
    pub bar_background_color: ColorType,
    pub font_family: String,
    pub font_style: String,

    // ================= GENERAL STYLE =================
    pub general_padding: Option<u16>,
    pub general_text_size: Option<u32>,
    pub general_text_color: Option<ColorType>,
    pub general_text_orientation: Option<TextOrientation>,
    pub general_button_color: Option<ColorType>,
    pub general_button_hovered_color: Option<ColorType>,
    pub general_button_hovered_text_color: Option<ColorType>,
    pub general_button_pressed_color: Option<ColorType>,
    pub general_border_color: Option<ColorType>,
    pub general_border_size: Option<f32>,
    pub general_border_radius: Option<[f32;4]>,
    pub general_side_separator: Option<SideOption>,
    pub general_side_separator_color: Option<ColorType>,
    pub general_side_separator_width: Option<f32>,
    pub general_side_separator_height: Option<f32>,
    pub general_alt_side_separator: Option<SideOption>,
    pub general_alt_side_separator_color: Option<ColorType>,
    pub general_alt_side_separator_width: Option<f32>,
    pub general_alt_side_separator_height: Option<f32>,
    pub general_alt_padding: Option<u16>,
    pub general_alt_text_size: Option<u32>,
    pub general_alt_text_color: Option<ColorType>,
    pub general_alt_text_orientation: Option<TextOrientation>,
    pub general_alt_button_color: Option<ColorType>,
    pub general_alt_button_hovered_color: Option<ColorType>,
    pub general_alt_button_hovered_text_color: Option<ColorType>,
    pub general_alt_button_pressed_color: Option<ColorType>,
    pub general_alt_border_color: Option<ColorType>,
    pub general_alt_border_size: Option<f32>,
    pub general_alt_border_radius: Option<[f32;4]>,


    // ================= MODULES =================
    pub left_modules: Vec<Modules>,
    pub center_modules: Vec<Modules>,
    pub right_modules: Vec<Modules>,


    // ================= MODULES CONFIGS =================
    pub clock_timezones: Option<Vec<String>>,
    pub ellipsis_text: String,
    pub player: String, 
    pub dont_show_metadata_if_empty: bool,
    pub dont_show_focused_window_if_empty: bool,
    pub text_when_metadata_is_empty: String,
    pub text_when_focused_window_is_empty: String,
    pub media_player_metadata_text_limit_len: usize,
    pub focused_window_text_limit_len: usize,
    pub spacing_between_all_modules: u32,
    pub force_static_position_context_menu: Option<(i32, i32)>,
    pub reverse_scroll_on_workspace: bool,
    pub persistent_workspaces: Option<u8>,
    pub incremental_steps_output: u8,
    pub incremental_steps_input: u8,
    pub action_on_left_click_cpu: ActionOnClick, 
    pub action_on_right_click_cpu: ActionOnClick, 
    pub action_on_left_click_cpu_temp: ActionOnClick, 
    pub action_on_right_click_cpu_temp: ActionOnClick, 
    pub action_on_left_click_media_player_metadata: ActionOnClick, 
    pub action_on_right_click_media_player_metadata: ActionOnClick, 
    pub action_on_left_click_clock: ActionOnClick, 
    pub action_on_right_click_clock: ActionOnClick, 
    pub action_on_left_click_network: ActionOnClick, 
    pub action_on_right_click_network: ActionOnClick, 
    pub action_on_left_click_volume_output: ActionOnClick, 
    pub action_on_right_click_volume_output: ActionOnClick, 
    pub action_on_left_click_volume_input: ActionOnClick, 
    pub action_on_right_click_volume_input: ActionOnClick, 


    // ================= UPDATE INTERVAL =================
    pub media_player_metadata_update_interval: u64,
    pub niri_workspaces_update_interval: u64,
    pub clock_update_interval: u64,
    pub cpu_update_interval: u64,
    pub cpu_temp_update_interval: u64,
    pub ram_update_interval: u64,
    pub focused_window_update_interval: u64,


    // ================= FORMATS =================
    pub media_player_buttons_format: [String;4],
    pub media_player_metadata_format: String, 
    pub network_module_format: String,
    pub alt_network_module_format: String,
    pub network_disconnected_text: String,
    pub network_level_format: [String;4], 
    pub alt_network_level_format: [String;4], 
    pub network_connection_type_icons: [String;3],
    pub alt_network_connection_type_icons: [String;3],
    pub output_volume_format: [String;6],
    pub output_volume_muted_format: String,
    pub input_volume_format: [String;6],
    pub input_volume_muted_format: String,
    pub clock_format: String,
    pub clock_alt_format: String,
    pub cpu_format: String,


    // ================= SIDE SEPARATOR CONFIGS =================
    pub clock_side_separator: Option<SideOption>,
    pub clock_side_separator_color: ColorType,
    pub clock_side_separator_width: f32,
    pub clock_side_separator_height: f32,

    pub alt_clock_side_separator: Option<SideOption>,
    pub alt_clock_side_separator_color: ColorType,
    pub alt_clock_side_separator_width: f32,
    pub alt_clock_side_separator_height: f32,

    pub tray_side_separator:        Option<SideOption>,
    pub tray_side_separator_color:  ColorType,
    pub tray_side_separator_width:  f32,
    pub tray_side_separator_height: f32,
 
    pub workspace_side_separator:        Option<SideOption>,
    pub workspace_side_separator_color:  ColorType,
    pub workspace_side_separator_width:  f32,
    pub workspace_side_separator_height: f32,
 
    pub media_player_metadata_side_separator:        Option<SideOption>,
    pub media_player_metadata_side_separator_color:  ColorType,
    pub media_player_metadata_side_separator_width:  f32,
    pub media_player_metadata_side_separator_height: f32,
 
    pub media_player_buttons_side_separator:        Option<SideOption>,
    pub media_player_buttons_side_separator_color:  ColorType,
    pub media_player_buttons_side_separator_width:  f32,
    pub media_player_buttons_side_separator_height: f32,
 
    pub focused_window_side_separator:        Option<SideOption>,
    pub focused_window_side_separator_color:  ColorType,
    pub focused_window_side_separator_width:  f32,
    pub focused_window_side_separator_height: f32,
 
    pub cpu_side_separator:        Option<SideOption>,
    pub cpu_side_separator_color:  ColorType,
    pub cpu_side_separator_width:  f32,
    pub cpu_side_separator_height: f32,
 
    pub cpu_temp_side_separator:        Option<SideOption>,
    pub cpu_temp_side_separator_color:  ColorType,
    pub cpu_temp_side_separator_width:  f32,
    pub cpu_temp_side_separator_height: f32,
 
    pub ram_side_separator:        Option<SideOption>,
    pub ram_side_separator_color:  ColorType,
    pub ram_side_separator_width:  f32,
    pub ram_side_separator_height: f32,
 
    pub network_side_separator:        Option<SideOption>,
    pub network_side_separator_color:  ColorType,
    pub network_side_separator_width:  f32,
    pub network_side_separator_height: f32,

    pub alt_network_side_separator:        Option<SideOption>,
    pub alt_network_side_separator_color:  ColorType,
    pub alt_network_side_separator_width:  f32,
    pub alt_network_side_separator_height: f32,
 
    pub volume_output_side_separator:        Option<SideOption>,
    pub volume_output_side_separator_color:  ColorType,
    pub volume_output_side_separator_width:  f32,
    pub volume_output_side_separator_height: f32,

    pub muted_volume_output_side_separator:        Option<SideOption>,
    pub muted_volume_output_side_separator_color:  ColorType,
    pub muted_volume_output_side_separator_width:  f32,
    pub muted_volume_output_side_separator_height: f32,

    pub volume_input_side_separator:        Option<SideOption>,
    pub volume_input_side_separator_color:  ColorType,
    pub volume_input_side_separator_width:  f32,
    pub volume_input_side_separator_height: f32,
 
    pub muted_volume_input_side_separator:        Option<SideOption>,
    pub muted_volume_input_side_separator_color:  ColorType,
    pub muted_volume_input_side_separator_width:  f32,
    pub muted_volume_input_side_separator_height: f32,


    // ================= TRAY (STYLE) =================
    pub tray_icon_size: u32,
    pub tray_spacing: u32,
    pub tray_button_size: u16,
    pub tray_button_color: ColorType,
    pub tray_button_hovered_color: ColorType,
    pub tray_button_hovered_text_color: ColorType,
    pub tray_button_pressed_color: ColorType,
    pub tray_border_color: ColorType,
    pub tray_border_size: f32,
    pub tray_border_radius: [f32;4],


    // ================= FOCUSED WINDOW (STYLE) =================
    pub focused_window_padding: u16,
    pub focused_window_format:                        String,
    pub focused_window_text_size:                     u32,
    pub focused_window_text_color:                ColorType,
    pub focused_window_text_orientation:              TextOrientation,
    pub focused_window_button_color:              ColorType,
    pub focused_window_button_hovered_color:      ColorType,
    pub focused_window_button_hovered_text_color: ColorType,
    pub focused_window_button_pressed_color:      ColorType,
    pub focused_window_border_color:             ColorType,
    pub focused_window_border_size:                   f32,
    pub focused_window_border_radius:                 [f32; 4],


    // ================= CPU (STYLE) =================
    pub cpu_padding: u16,
    pub cpu_text_size: u32,
    pub cpu_text_color: ColorType,
    pub cpu_text_orientation: TextOrientation,
    pub cpu_button_color: ColorType,
    pub cpu_button_hovered_color: ColorType,
    pub cpu_button_hovered_text_color: ColorType,
    pub cpu_button_pressed_color: ColorType,
    pub cpu_border_color: ColorType,
    pub cpu_border_size: f32,
    pub cpu_border_radius: [f32; 4],


    // ================= CPU TEMP (STYLE) =================
    pub cpu_temp_padding: u16,
    pub cpu_temp_format:                        String,
    pub cpu_temp_text_size:                     u32,
    pub cpu_temp_text_color:                ColorType,
    pub cpu_temp_text_orientation:              TextOrientation,
    pub cpu_temp_button_color:              ColorType,
    pub cpu_temp_button_hovered_color:      ColorType,
    pub cpu_temp_button_hovered_text_color: ColorType,
    pub cpu_temp_button_pressed_color:      ColorType,
    pub cpu_temp_border_color:             ColorType,
    pub cpu_temp_border_size:                   f32,
    pub cpu_temp_border_radius:                 [f32; 4],


    // ================= RAM (STYLE) =================
    pub ram_padding: u16,
    pub ram_format:                        String,
    pub ram_text_size:                     u32,
    pub ram_text_color:                ColorType,
    pub ram_text_orientation:              TextOrientation,
    pub ram_button_color:              ColorType,
    pub ram_button_hovered_color:      ColorType,
    pub ram_button_hovered_text_color: ColorType,
    pub ram_button_pressed_color:      ColorType,
    pub ram_border_color:             ColorType,
    pub ram_border_size:                   f32,
    pub ram_border_radius:                 [f32; 4],


    // ================= MEDIA PLAYER (STYLE) =================
    pub media_player_metadata_padding: u16,
    pub media_player_metadata_text_size: u32,
    pub media_player_metadata_text_color: ColorType,
    pub media_player_metadata_text_orientation: TextOrientation,
    pub media_player_metadata_button_color: ColorType,
    pub media_player_metadata_button_hovered_color: ColorType,
    pub media_player_metadata_button_hovered_text_color: ColorType,
    pub media_player_metadata_button_pressed_color: ColorType,
    pub media_player_metadata_border_color: ColorType,
    pub media_player_metadata_border_size: f32,
    pub media_player_metadata_border_radius: [f32;4],


    // ================= MEDIA PLAYER BUTTONS (STYLE) =================
    pub media_player_button_text_color: ColorType,
    pub media_player_button_padding: u16,
    pub media_player_button_spacing: u32,
    pub media_player_button_text_size: u32,
    pub media_player_button_text_orientation: TextOrientation,
    pub media_player_button_color: ColorType,
    pub media_player_button_hovered_color: ColorType,
    pub media_player_button_hovered_text_color: ColorType,
    pub media_player_button_pressed_color: ColorType,
    pub media_player_button_border_color: ColorType,
    pub media_player_button_border_size: f32,
    pub media_player_button_border_radius: [f32;4],


    // ================= NETWORK (STYLE) =================
    pub network_padding: u16,
    pub network_text_size: u32,
    pub network_text_color: ColorType,
    pub network_text_orientation: TextOrientation,
    pub network_button_color: ColorType,
    pub network_button_hovered_color: ColorType,
    pub network_button_hovered_text_color: ColorType,
    pub network_button_pressed_color: ColorType,
    pub network_border_color: ColorType,
    pub network_border_size: f32,
    pub network_border_radius: [f32;4],


    // ================= ALT NETWORK (STYLE) =================
    pub alt_network_padding: u16,
    pub alt_network_text_size: u32,
    pub alt_network_text_color: ColorType,
    pub alt_network_text_orientation: TextOrientation,
    pub alt_network_button_color: ColorType,
    pub alt_network_button_hovered_color: ColorType,
    pub alt_network_button_hovered_text_color: ColorType,
    pub alt_network_button_pressed_color: ColorType,
    pub alt_network_border_color: ColorType,
    pub alt_network_border_size: f32,
    pub alt_network_border_radius: [f32;4],


    // ================= CLOCK (STYLE) =================
    pub clock_padding: u16,
    pub clock_text_size: u32,
    pub clock_text_color: ColorType,
    pub clock_text_orientation: TextOrientation,
    pub clock_button_color: ColorType,
    pub clock_button_hovered_color: ColorType,
    pub clock_button_hovered_text_color: ColorType,
    pub clock_button_pressed_color: ColorType,
    pub clock_border_color: ColorType,
    pub clock_border_size: f32,
    pub clock_border_radius: [f32;4],


    // ================= ALT CLOCK (STYLE) =================
    pub alt_clock_padding: u16,
    pub alt_clock_text_size: u32,
    pub alt_clock_text_color: ColorType,
    pub alt_clock_text_orientation: TextOrientation,
    pub alt_clock_button_color: ColorType,
    pub alt_clock_button_hovered_color: ColorType,
    pub alt_clock_button_hovered_text_color: ColorType,
    pub alt_clock_button_pressed_color: ColorType,
    pub alt_clock_border_color: ColorType,
    pub alt_clock_border_size: f32,
    pub alt_clock_border_radius: [f32;4],


    // ================= VOLUME/OUTPUT (STYLE) =================
    pub volume_output_padding: u16,
    pub volume_output_text_size: u32,
    pub volume_output_text_color: ColorType,
    pub volume_output_text_orientation: TextOrientation,
    pub volume_output_button_color: ColorType,
    pub volume_output_button_hovered_color: ColorType,
    pub volume_output_button_hovered_text_color: ColorType,
    pub volume_output_button_pressed_color: ColorType,
    pub volume_output_border_color: ColorType,
    pub volume_output_border_size: f32,
    pub volume_output_border_radius: [f32;4],


    // ================= MUTED VOLUME/OUTPUT (STYLE) =================
    pub muted_volume_output_padding: u16,
    pub muted_volume_output_text_size: u32,
    pub muted_volume_output_text_color: ColorType,
    pub muted_volume_output_text_orientation: TextOrientation,
    pub muted_volume_output_button_color: ColorType,
    pub muted_volume_output_button_hovered_color: ColorType,
    pub muted_volume_output_button_hovered_text_color: ColorType,
    pub muted_volume_output_button_pressed_color: ColorType,
    pub muted_volume_output_border_color: ColorType,
    pub muted_volume_output_border_size: f32,
    pub muted_volume_output_border_radius: [f32;4],


    // ================= VOLUME/INPUT (STYLE) =================
    pub volume_input_padding: u16,
    pub volume_input_text_size: u32,
    pub volume_input_text_color: ColorType,
    pub volume_input_text_orientation: TextOrientation,
    pub volume_input_button_color: ColorType,
    pub volume_input_button_hovered_color: ColorType,
    pub volume_input_button_hovered_text_color: ColorType,
    pub volume_input_button_pressed_color: ColorType,
    pub volume_input_border_color: ColorType,
    pub volume_input_border_size: f32,
    pub volume_input_border_radius: [f32;4],


    // ================= VOLUME/INPUT (STYLE) =================
    pub muted_volume_input_padding: u16,
    pub muted_volume_input_text_size: u32,
    pub muted_volume_input_text_color: ColorType,
    pub muted_volume_input_text_orientation: TextOrientation,
    pub muted_volume_input_button_color: ColorType,
    pub muted_volume_input_button_hovered_color: ColorType,
    pub muted_volume_input_button_hovered_text_color: ColorType,
    pub muted_volume_input_button_pressed_color: ColorType,
    pub muted_volume_input_border_color: ColorType,
    pub muted_volume_input_border_size: f32,
    pub muted_volume_input_border_radius: [f32;4],


    // ================= HYPR/SWAY WORKSPACES (STYLE) =================
    pub workspace_padding: u16,
    pub workspace_height: u32,
    pub workspace_width: u32,
    pub workspace_different_selected_width: Option<u32>,
    pub workspace_different_selected_height: Option<u32>,
    pub workspace_text_size: u32,
    pub workspace_text: Vec<String>,
    pub workspace_text_color: ColorType,
    pub workspace_selected_text_color: ColorType,
    pub workspace_text_orientation: TextOrientation,
    pub workspace_selected_text: Option<Vec<String>>,
    pub workspace_spacing: u32,
    pub workspace_button_color: ColorType,
    pub workspace_button_selected_color: ColorType,
    pub workspace_button_hovered_color: ColorType,
    pub workspace_button_hovered_text_color: ColorType,
    pub workspace_button_pressed_color: ColorType,
    pub workspace_border_color: ColorType,
    pub workspace_border_size: f32,
    pub workspace_border_radius: [f32;4],


    // ================= CONTEXT MENU (STYLE) =================
    pub context_menu_background_color: ColorType,
    pub context_menu_background_size: u16,
    pub context_menu_background_border_color: ColorType,
    pub context_menu_background_border_size: f32,
    pub context_menu_background_border_radius: [f32;4],

    pub context_menu_text_size: u32,
    pub context_menu_text_color: ColorType,
    pub context_menu_orientation: TextOrientation,
    pub context_menu_size: u32,
    pub context_menu_item_size: u32,
    pub context_menu_button_color: ColorType,
    pub context_menu_button_hovered_color: ColorType,
    pub context_menu_button_hovered_text_color: ColorType,
    pub context_menu_button_pressed_color: ColorType,
    pub context_menu_border_color: ColorType,
    pub context_menu_border_size: f32,
    pub context_menu_border_radius: [f32;4],

    // ================= CUSTOM MODULES =================
    pub custom_modules_spacing: u32,
    pub custom_modules: Vec<CustomModule>
}





// ============ DEFAULT ============
impl Default for BarConfig
{
    fn default() -> Self
    {
        Self 
        {
            // ================= GENERAL =================
            display: None,
            bar_position: BarPosition::Up,
            floating_space: 0,
            increased_exclusive_bar_zone: 0,
            bar_check_reload_interval_ms: Some(500),
            bar_side_spaces_size: 0,
            bar_size: [0, 35],
            bar_border_radius: [0., 0., 0., 0.],
            bar_border_size: 1.0,
            bar_border_color: ColorType::RGB([90, 70, 100]),
            bar_background_color: ColorType::RGBA([18, 18, 22, 92]),
            font_family: "JetBrains Mono".into(),
            font_style: "Normal".into(),


            // ================= GENERAL STYLE =================
            general_padding: None,
            general_text_size: None,
            general_text_orientation: None,
            general_text_color: None,
            general_button_color: None,
            general_button_hovered_color: None,
            general_button_hovered_text_color: None,
            general_button_pressed_color: None,
            general_border_color: None,
            general_border_size: None,
            general_border_radius: None,
            general_side_separator: None,
            general_side_separator_color: None,
            general_side_separator_width: None,
            general_side_separator_height: None,
            general_alt_side_separator: None,
            general_alt_side_separator_color: None,
            general_alt_side_separator_width: None,
            general_alt_side_separator_height: None,
            general_alt_padding: None,
            general_alt_text_size: None,
            general_alt_text_orientation: None,
            general_alt_text_color: None,
            general_alt_button_color: None,
            general_alt_button_hovered_color: None,
            general_alt_button_hovered_text_color: None,
            general_alt_button_pressed_color: None,
            general_alt_border_color: None,
            general_alt_border_size: None,
            general_alt_border_radius: None,


            // ================= MODULES =================
            left_modules: Vec::new(),
            center_modules: Vec::new(),
            right_modules: Vec::new(),


            // ================= MODULES CONFIGS =================
            clock_timezones: None,
            ellipsis_text: "...".to_string(),
            player: "spotify".to_string(),
            dont_show_metadata_if_empty: false,
            dont_show_focused_window_if_empty: false,
            text_when_metadata_is_empty: "No Media Found.".to_string(),
            text_when_focused_window_is_empty: "No Window Focused".to_string(),
            media_player_metadata_text_limit_len: 25,
            focused_window_text_limit_len: 25,
            spacing_between_all_modules: 5,
            force_static_position_context_menu: None,
            reverse_scroll_on_workspace: false,
            persistent_workspaces: None,
            incremental_steps_output: 10,
            incremental_steps_input: 10,
            action_on_left_click_media_player_metadata: ActionOnClick::DefaultAction, 
            action_on_right_click_media_player_metadata: ActionOnClick::DefaultAction, 
            action_on_left_click_cpu: ActionOnClick::DefaultAction, 
            action_on_right_click_cpu: ActionOnClick::DefaultAction, 
            action_on_left_click_cpu_temp: ActionOnClick::DefaultAction, 
            action_on_right_click_cpu_temp: ActionOnClick::DefaultAction, 
            action_on_left_click_clock: ActionOnClick::DefaultAction, 
            action_on_right_click_clock: ActionOnClick::DefaultAction, 
            action_on_left_click_network: ActionOnClick::DefaultAction, 
            action_on_right_click_network: ActionOnClick::DefaultAction, 
            action_on_left_click_volume_output: ActionOnClick::DefaultAction, 
            action_on_right_click_volume_output: ActionOnClick::DefaultAction, 
            action_on_left_click_volume_input: ActionOnClick::DefaultAction, 
            action_on_right_click_volume_input: ActionOnClick::DefaultAction, 


            // ================= UPDATE INTERVAL =================
            media_player_metadata_update_interval: 750,
            niri_workspaces_update_interval: 225,
            clock_update_interval: 400,
            cpu_update_interval: 1050,
            cpu_temp_update_interval: 1050,
            ram_update_interval: 1050,
            focused_window_update_interval: 500,


            // ================= FORMATS =================
            media_player_buttons_format: ["󰒮".to_string(), "⏸".to_string(), "▶".to_string(), "󰒭".to_string()],
            media_player_metadata_format: "{{artist}} | {{album}} | {{title}}".to_string(),
            network_disconnected_text: "No Connection Found.".to_string(),
            network_module_format: "{level} ".to_string(),
            alt_network_module_format: "{level} | {connection_type} | {id}".to_string(),
            network_level_format: 
            [
                "󰖩".to_string(),
                "󱚵".to_string(),
                "󱚼".to_string(),
                "󰖪".to_string()
            ],
            alt_network_level_format: 
            [
                "󰖩".to_string(),
                "󱚵".to_string(),
                "󱚼".to_string(),
                "󰖪".to_string()
            ],
            network_connection_type_icons: 
            [
                "󰈀".to_string(), 
                "".to_string(), 
                "?".to_string()
            ],
            alt_network_connection_type_icons: 
            [
                "󰈀".to_string(), 
                "".to_string(), 
                "?".to_string()
            ],
            output_volume_format: 
            [
                "   {}%".to_string(), 
                "󰖀   {}%".to_string(), 
                "   {}%".to_string(), 
                "   {}%".to_string(), 
                "   {}%".to_string(), 
                "   + {}%".to_string()
            ],
            input_volume_format:
            [
                "   {}%".to_string(), 
                "  {}%".to_string(), 
                "  {}%".to_string(), 
                "  {}%".to_string(), 
                "  {}%".to_string(), 
                "󰢴  {}%".to_string()
            ],
            output_volume_muted_format: "   Muted".into(),
            input_volume_muted_format: "   Muted".into(),
            clock_format: "󰥔  %H:%M".into(),
            clock_alt_format: "󰃭  %a %d %b |  󰥔  %H:%M:%S".into(),
            cpu_format: "CPU: {usage}%".to_string(),
            focused_window_format: "{title}".to_string(),
            cpu_temp_format: " {temp}°C".to_string(),
            ram_format: " {used}MB / {total}MB ({percent}%)".to_string(),


            // ================= SIDE SEPARATOR CONFIGS =================
            clock_side_separator: None,
            clock_side_separator_color: ColorType::RGB([75, 75, 75]),
            clock_side_separator_width: 1.,
            clock_side_separator_height: 16.,

            alt_clock_side_separator: None,
            alt_clock_side_separator_color: ColorType::RGB([75, 75, 75]),
            alt_clock_side_separator_width: 1.,
            alt_clock_side_separator_height: 16.,

            tray_side_separator:        None,
            tray_side_separator_color: ColorType::RGB([75, 75, 75]),
            tray_side_separator_width:  1.,
            tray_side_separator_height: 16.,
 
            workspace_side_separator:        None,
            workspace_side_separator_color: ColorType::RGB([75, 75, 75]),
            workspace_side_separator_width:  1.,
            workspace_side_separator_height: 16.,
 
            media_player_metadata_side_separator:        None,
            media_player_metadata_side_separator_color: ColorType::RGB([75, 75, 75]),
            media_player_metadata_side_separator_width:  1.,
            media_player_metadata_side_separator_height: 16.,
 
            media_player_buttons_side_separator:        None,
            media_player_buttons_side_separator_color: ColorType::RGB([75, 75, 75]),
            media_player_buttons_side_separator_width:  1.,
            media_player_buttons_side_separator_height: 16.,
 
            focused_window_side_separator:        None,
            focused_window_side_separator_color: ColorType::RGB([75, 75, 75]),
            focused_window_side_separator_width:  1.,
            focused_window_side_separator_height: 16.,
 
            cpu_side_separator:        None,
            cpu_side_separator_color: ColorType::RGB([75, 75, 75]),
            cpu_side_separator_width:  1.,
            cpu_side_separator_height: 16.,
 
            cpu_temp_side_separator:        None,
            cpu_temp_side_separator_color: ColorType::RGB([75, 75, 75]),
            cpu_temp_side_separator_width:  1.,
            cpu_temp_side_separator_height: 16.,
 
            ram_side_separator:        None,
            ram_side_separator_color: ColorType::RGB([75, 75, 75]),
            ram_side_separator_width:  1.,
            ram_side_separator_height: 16.,
 
            network_side_separator:        None,
            network_side_separator_color: ColorType::RGB([75, 75, 75]),
            network_side_separator_width:  1.,
            network_side_separator_height: 16.,

            alt_network_side_separator:        None,
            alt_network_side_separator_color: ColorType::RGB([75, 75, 75]),
            alt_network_side_separator_width:  1.,
            alt_network_side_separator_height: 16.,

            volume_output_side_separator:        None,
            volume_output_side_separator_color: ColorType::RGB([75, 75, 75]),
            volume_output_side_separator_width:  1.,
            volume_output_side_separator_height: 20.,
 
            muted_volume_output_side_separator:        None,
            muted_volume_output_side_separator_color: ColorType::RGB([75, 75, 75]),
            muted_volume_output_side_separator_width:  1.,
            muted_volume_output_side_separator_height: 20.,
 
            volume_input_side_separator:        None,
            volume_input_side_separator_color: ColorType::RGB([75, 75, 75]),
            volume_input_side_separator_width:  1.,
            volume_input_side_separator_height: 20.,

            muted_volume_input_side_separator:        None,
            muted_volume_input_side_separator_color: ColorType::RGB([75, 75, 75]),
            muted_volume_input_side_separator_width:  1.,
            muted_volume_input_side_separator_height: 20.,


            // ================= TRAY (STYLE) =================
            tray_icon_size: 18,
            tray_spacing: 5,
            tray_button_size: 5,
            tray_button_color: ColorType::RGB([60, 50, 70]),
            tray_button_hovered_color: ColorType::RGB([110, 40, 80]),
            tray_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            tray_button_pressed_color: ColorType::RGB([70, 20, 40]),
            tray_border_color: ColorType::RGB([90, 70, 100]),
            tray_border_size: 1.0,
            tray_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= FOCUSED WINDOW (STYLE) =================
            focused_window_padding: 0,
            focused_window_text_size:                     12,
            focused_window_text_color: ColorType::RGB([220, 220, 220]),
            focused_window_text_orientation:              TextOrientation::Horizontal,
            focused_window_button_color: ColorType::RGB([40, 40, 50]),
            focused_window_button_hovered_color: ColorType::RGB([60, 60, 75]),
            focused_window_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            focused_window_button_pressed_color: ColorType::RGB([30, 30, 40]),
            focused_window_border_color: ColorType::RGB([80, 80, 100]),
            focused_window_border_size:                   1.0,
            focused_window_border_radius:                 [3.0, 3.0, 3.0, 3.0],


            // ================= CPU (STYLE) =================
            cpu_padding: 0,
            cpu_text_size: 12,
            cpu_text_color: ColorType::RGB([220, 220, 220]),
            cpu_text_orientation: TextOrientation::Horizontal,
            cpu_button_color: ColorType::RGB([40, 40, 50]),
            cpu_button_hovered_color: ColorType::RGB([60, 60, 75]),
            cpu_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            cpu_button_pressed_color: ColorType::RGB([30, 30, 40]),
            cpu_border_color: ColorType::RGB([80, 80, 100]),
            cpu_border_size: 1.0,
            cpu_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= CPU TEMP (STYLE) =================
            cpu_temp_padding: 0,
            cpu_temp_text_size:                     12,
            cpu_temp_text_color: ColorType::RGB([220, 220, 220]),
            cpu_temp_text_orientation:              TextOrientation::Horizontal,
            cpu_temp_button_color: ColorType::RGB([40, 40, 50]),
            cpu_temp_button_hovered_color: ColorType::RGB([60, 60, 75]),
            cpu_temp_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            cpu_temp_button_pressed_color: ColorType::RGB([30, 30, 40]),
            cpu_temp_border_color: ColorType::RGB([80, 80, 100]),
            cpu_temp_border_size:                   1.0,
            cpu_temp_border_radius:                 [3.0, 3.0, 3.0, 3.0],


            // ================= RAM (STYLE) =================
            ram_padding: 0,
            ram_text_size:                     12,
            ram_text_color: ColorType::RGB([220, 220, 220]),
            ram_text_orientation:              TextOrientation::Horizontal,
            ram_button_color: ColorType::RGB([40, 40, 50]),
            ram_button_hovered_color: ColorType::RGB([60, 60, 75]),
            ram_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            ram_button_pressed_color: ColorType::RGB([30, 30, 40]),
            ram_border_color: ColorType::RGB([80, 80, 100]),
            ram_border_size:                   1.0,
            ram_border_radius:                 [3.0, 3.0, 3.0, 3.0],


            // ================= MEDIA PLAYER (STYLE) =================
            media_player_metadata_padding: 0,
            media_player_metadata_text_size: 15,
            media_player_metadata_text_color: ColorType::RGB([255, 255, 255]),
            media_player_metadata_text_orientation: TextOrientation::Horizontal,
            media_player_metadata_button_color: ColorType::RGB([50, 45, 60]),
            media_player_metadata_button_hovered_color: ColorType::RGB([130, 35, 70]),
            media_player_metadata_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            media_player_metadata_button_pressed_color: ColorType::RGB([80, 25, 45]),
            media_player_metadata_border_color: ColorType::RGB([120, 80, 130]),
            media_player_metadata_border_size: 1.0,
            media_player_metadata_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= MEDIA PLAYER BUTTONS (STYLE) =================
            media_player_button_text_color: ColorType::RGB([255, 255, 255]),
            media_player_button_padding: 0,
            media_player_button_spacing: 5,
            media_player_button_text_size: 15,
            media_player_button_text_orientation: TextOrientation::Horizontal,
            media_player_button_color: ColorType::RGB([50, 45, 60]),
            media_player_button_hovered_color: ColorType::RGB([130, 35, 70]),
            media_player_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            media_player_button_pressed_color: ColorType::RGB([80, 25, 45]),
            media_player_button_border_color: ColorType::RGB([120, 80, 130]),
            media_player_button_border_size: 1.0,
            media_player_button_border_radius: [3.0, 3.0, 3.0, 3.0],
        

            // ================= NETWORK (STYLE) =================
            network_padding: 0,
            network_text_size: 15,
            network_text_color: ColorType::RGB([255, 255, 255]),
            network_text_orientation: TextOrientation::Horizontal,
            network_button_color: ColorType::RGB([50, 45, 60]),
            network_button_hovered_color: ColorType::RGB([130, 35, 70]),
            network_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            network_button_pressed_color: ColorType::RGB([80, 25, 45]),
            network_border_color: ColorType::RGB([120, 80, 130]),
            network_border_size: 1.0,
            network_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= NETWORK (STYLE) =================
            alt_network_padding: 0,
            alt_network_text_size: 15,
            alt_network_text_color: ColorType::RGB([255, 255, 255]),
            alt_network_text_orientation: TextOrientation::Horizontal,
            alt_network_button_color: ColorType::RGB([150, 50, 80]),
            alt_network_button_hovered_color: ColorType::RGB([130, 35, 70]),
            alt_network_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            alt_network_button_pressed_color: ColorType::RGB([80, 25, 45]),
            alt_network_border_color: ColorType::RGB([120, 80, 130]),
            alt_network_border_size: 1.0,
            alt_network_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= CLOCK (STYLE) =================
            clock_padding: 0,
            clock_text_size: 15,
            clock_text_color: ColorType::RGB([255, 255, 255]),
            clock_text_orientation: TextOrientation::Horizontal,
            clock_button_color: ColorType::RGB([50, 45, 60]),
            clock_button_hovered_color: ColorType::RGB([130, 35, 70]),
            clock_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            clock_button_pressed_color: ColorType::RGB([80, 25, 45]),
            clock_border_color: ColorType::RGB([120, 80, 130]),
            clock_border_size: 1.0,
            clock_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= ALT CLOCK (STYLE) =================
            alt_clock_padding: 0,
            alt_clock_text_size: 15,
            alt_clock_text_color: ColorType::RGB([255, 255, 255]),
            alt_clock_text_orientation: TextOrientation::Horizontal,
            alt_clock_button_color: ColorType::RGB([150, 40, 80]),
            alt_clock_button_hovered_color: ColorType::RGB([130, 35, 70]),
            alt_clock_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            alt_clock_button_pressed_color: ColorType::RGB([80, 25, 45]),
            alt_clock_border_color: ColorType::RGB([120, 80, 130]),
            alt_clock_border_size: 1.0,
            alt_clock_border_radius: [3.0, 3.0, 3.0, 3.0],
            

            // ================= VOLUME/OUTPUT (STYLE) =================
            volume_output_padding: 0,
            volume_output_text_size: 15,
            volume_output_text_color: ColorType::RGB([255, 255, 255]),
            volume_output_text_orientation: TextOrientation::Horizontal,
            volume_output_button_color: ColorType::RGB([55, 45, 65]),
            volume_output_button_hovered_color: ColorType::RGB([150, 45, 85]),
            volume_output_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            volume_output_button_pressed_color: ColorType::RGB([85, 30, 50]),
            volume_output_border_color: ColorType::RGB([110, 80, 120]),
            volume_output_border_size: 1.0,
            volume_output_border_radius: [3.0, 3.0, 3.0, 3.0],

            // ================= MUTED VOLUME/OUTPUT (STYLE) =================
            muted_volume_output_padding: 0,
            muted_volume_output_text_size: 15,
            muted_volume_output_text_color: ColorType::RGB([255, 255, 255]),
            muted_volume_output_text_orientation: TextOrientation::Horizontal,
            muted_volume_output_button_color: ColorType::RGB([150, 40, 80]),
            muted_volume_output_button_hovered_color: ColorType::RGB([150, 45, 85]),
            muted_volume_output_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            muted_volume_output_button_pressed_color: ColorType::RGB([85, 30, 50]),
            muted_volume_output_border_color: ColorType::RGB([110, 80, 120]),
            muted_volume_output_border_size: 1.0,
            muted_volume_output_border_radius: [3.0, 3.0, 3.0, 3.0],
            

            // ================= VOLUME/INPUT (STYLE) =================
            volume_input_padding: 0,
            volume_input_text_size: 15,
            volume_input_text_color: ColorType::RGB([255, 255, 255]),
            volume_input_text_orientation: TextOrientation::Horizontal,
            volume_input_button_color: ColorType::RGB([55, 45, 65]),
            volume_input_button_hovered_color: ColorType::RGB([150, 45, 85]),
            volume_input_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            volume_input_button_pressed_color: ColorType::RGB([85, 30, 50]),
            volume_input_border_color: ColorType::RGB([110, 80, 120]),
            volume_input_border_size: 1.0,
            volume_input_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= VOLUME/INPUT (STYLE) =================
            muted_volume_input_padding: 0,
            muted_volume_input_text_size: 15,
            muted_volume_input_text_color: ColorType::RGB([255, 255, 255]),
            muted_volume_input_text_orientation: TextOrientation::Horizontal,
            muted_volume_input_button_color: ColorType::RGB([150, 40, 80]),
            muted_volume_input_button_hovered_color: ColorType::RGB([150, 45, 85]),
            muted_volume_input_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            muted_volume_input_button_pressed_color: ColorType::RGB([85, 30, 50]),
            muted_volume_input_border_color: ColorType::RGB([110, 80, 120]),
            muted_volume_input_border_size: 1.0,
            muted_volume_input_border_radius: [3.0, 3.0, 3.0, 3.0],
            

            // ================= HYPR WORKSPACES (STYLE) =================
            workspace_padding: 0,
            workspace_height: 30,
            workspace_width: 30,
            workspace_different_selected_width: None,
            workspace_different_selected_height: None,
            workspace_text_size: 15,
            workspace_selected_text_color: ColorType::RGB([255, 255, 255]),
            workspace_text_color: ColorType::RGB([255, 255, 255]),
            workspace_text_orientation: TextOrientation::Horizontal,
            workspace_text: vec!
            [
                "1".into(),
                "2".into(),
                "3".into(),
                "4".into(),
                "5".into(),
                "6".into(),
                "7".into(),
                "8".into(),
                "9".into(),
                "10".into(),
            ],
            workspace_selected_text: Some(vec!
            [
                "●".into(),
                "●".into(),
                "●".into(),
                "●".into(),
                "●".into(),
                "●".into(),
                "●".into(),
                "●".into(),
                "●".into(),
                "●".into(),
            ]),
            workspace_spacing: 3,
            workspace_button_color: ColorType::RGB([45, 40, 55]),
            workspace_button_selected_color: ColorType::RGB([150, 40, 80]),
            workspace_button_hovered_color: ColorType::RGB([140, 35, 75]),
            workspace_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            workspace_button_pressed_color: ColorType::RGB([90, 25, 50]),
            workspace_border_color: ColorType::RGB([120, 90, 135]),
            workspace_border_size: 1.0,
            workspace_border_radius: [3.0, 3.0, 3.0, 3.0],

            
            // ================= CONTEXT MENU (STYLE) =================
            context_menu_background_color: ColorType::RGBA([20, 20, 24, 98]),
            context_menu_background_size: 5,
            context_menu_background_border_color: ColorType::RGB([255, 255, 255]),
            context_menu_background_border_size: 1.0,
            context_menu_background_border_radius: [3.0, 3.0, 3.0, 3.0],
            
            context_menu_text_size: 15,
            context_menu_text_color: ColorType::RGB([255, 255, 255]),
            context_menu_orientation: TextOrientation::Vertical,
            context_menu_size: 300,
            context_menu_item_size: 30,
            context_menu_button_color: ColorType::RGB([45, 40, 55]),
            context_menu_button_hovered_color: ColorType::RGB([150, 40, 80]),
            context_menu_button_hovered_text_color: ColorType::RGB([255, 255, 255]),
            context_menu_button_pressed_color: ColorType::RGB([85, 30, 55]),
            context_menu_border_color: ColorType::RGB([130, 90, 140]),
            context_menu_border_size: 1.0,
            context_menu_border_radius: [3.0, 3.0, 3.0, 3.0],


            // ================= CUSTOM MODULES =================
            custom_modules_spacing: 10,
            custom_modules: Vec::new()
        }
    }
}





// ============ FUNCTIONS ============
pub fn read_ron_config() -> (BarConfig, Option<(String, u32)>, HashSet<Modules>)
{
    println!("\n=== READING CONFIG FILE ===");
    let home_path = home::home_dir().expect("Failed To Get Home Directory");
    let path = home_path.join(".config/icebar/config.ron");

    let mut bar_config: BarConfig = fs::read_to_string(&path).map_err(|e| {panic!("Failed to read config: {e}"); }).and_then(|content| 
    {
            println!("Config loaded successfully!!!.");
            ron::from_str::<BarConfig>(&content).map_err(|e| 
            {
                println!("\n=== PARSING CONFIG FILE ===");
                eprintln!("WARNING!!!: Config Parse Failed!!");
                eprintln!("WARNING!!!: Your 'config.ron' syntax maybe wrong!!!");
                panic!("\n\nRON parse error:\n{e}\n\n\n");
            })
    }).unwrap();

    apply_general_settings(&mut bar_config);


    let current_time_zone = if let Some(ref time_zone) = bar_config.clock_timezones && !time_zone.is_empty()
    {
        Some((time_zone[0].clone(), 0))
    }
    else
    {
        None
    };

    let mut active_modules: HashSet<Modules> = HashSet::new();
    let all_possible_default_modules = [Modules::FocusedWindowSway, Modules::FocusedWindowHypr, Modules::FocusedWindowNiri, Modules::CpuTemp, Modules::Ram, Modules::Cpu, Modules::NiriWorkspaces, Modules::MediaPlayerMetaData, Modules::MediaPlayerButtons, Modules::Network, Modules::HyprWorkspaces, Modules::SwayWorkspaces, Modules::VolumeOutput, Modules::VolumeInput, Modules::Clock, Modules::Tray];
    let all_possible_position = [&bar_config.left_modules, &bar_config.center_modules, &bar_config.right_modules];
    for position in all_possible_position
    {
        for item in position
        {
            if let Modules::CustomModule(_) = item
            {
                active_modules.insert(item.to_owned());
            }
            for module in &all_possible_default_modules
            {
                if *item == *module
                {
                    active_modules.insert(module.to_owned());
                }
            }
        }
    }

    println!("Active modules: {:?}", active_modules);

    (bar_config, current_time_zone, active_modules)
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
 
    #[test]
    fn bar_config_default_has_nonzero_bar_height()
    {
        let config = BarConfig::default();
        assert!(config.bar_size[1] > 0, "default bar height should be > 0");
    }
 
    #[test]
    fn bar_config_default_has_nonempty_font_family()
    {
        let config = BarConfig::default();
        assert!(!config.font_family.is_empty());
    }
 
    #[test]
    fn bar_config_default_has_nonempty_font_style()
    {
        let config = BarConfig::default();
        assert!(!config.font_style.is_empty());
    }
 
    #[test]
    fn bar_config_default_workspace_text_has_ten_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.workspace_text.len(), 10);
    }
 
    #[test]
    fn bar_config_default_workspace_selected_text_has_ten_entries()
    {
        let config = BarConfig::default();
        let selected = config.workspace_selected_text.unwrap();
        assert_eq!(selected.len(), 10);
    }
 
    #[test]
    fn bar_config_default_output_volume_format_has_six_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.output_volume_format.len(), 6);
    }
 
    #[test]
    fn bar_config_default_input_volume_format_has_six_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.input_volume_format.len(), 6);
    }
 
    #[test]
    fn bar_config_default_media_player_buttons_format_has_four_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.media_player_buttons_format.len(), 4);
    }
 
    #[test]
    fn bar_config_default_network_level_format_has_four_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.network_level_format.len(), 4);
    }
 
    #[test]
    fn bar_config_default_connection_type_icons_has_three_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.network_connection_type_icons.len(), 3);
    }
 
    #[test]
    fn bar_config_default_position_is_up()
    {
        assert_eq!(BarConfig::default().bar_position, BarPosition::Up);
    }
 
    #[test]
    fn bar_config_default_clock_timezones_is_none()
    {
        assert!(BarConfig::default().clock_timezones.is_none());
    }
 
    #[test]
    fn bar_config_default_ellipsis_text_is_three_dots()
    {
        assert_eq!(BarConfig::default().ellipsis_text, "...");
    }
 
    #[test]
    fn bar_config_default_incremental_steps_are_nonzero()
    {
        let config = BarConfig::default();
        assert!(config.incremental_steps_output > 0);
        assert!(config.incremental_steps_input > 0);
    }

        // ---- alt_network default arrays ----------------------------------------
 
    #[test]
    fn bar_config_default_alt_network_level_format_has_four_entries()
    {
        assert_eq!(BarConfig::default().alt_network_level_format.len(), 4);
    }
 
    #[test]
    fn bar_config_default_alt_network_connection_type_icons_has_three_entries()
    {
        assert_eq!(BarConfig::default().alt_network_connection_type_icons.len(), 3);
    }
 
    // ---- Option fields default to None / Some sentinel ---------------------
 
    #[test]
    fn bar_config_default_display_is_none()
    {
        assert!(BarConfig::default().display.is_none());
    }
 
    #[test]
    fn bar_config_default_persistent_workspaces_is_none()
    {
        assert!(BarConfig::default().persistent_workspaces.is_none());
    }
 
    #[test]
    fn bar_config_default_force_static_position_context_menu_is_none()
    {
        assert!(BarConfig::default().force_static_position_context_menu.is_none());
    }
 
    // ---- String defaults are non-empty -------------------------------------
 
    #[test]
    fn bar_config_default_ellipsis_text_is_nonempty()
    {
        assert!(!BarConfig::default().ellipsis_text.is_empty());
    }
 
    #[test]
    fn bar_config_default_network_disconnected_text_is_nonempty()
    {
        assert!(!BarConfig::default().network_disconnected_text.is_empty());
    }
 
    #[test]
    fn bar_config_default_clock_format_is_nonempty()
    {
        assert!(!BarConfig::default().clock_format.is_empty());
    }
 
    #[test]
    fn bar_config_default_network_module_format_is_nonempty()
    {
        assert!(!BarConfig::default().network_module_format.is_empty());
    }
 
    // ---- Numeric defaults are sane -----------------------------------------
 
    #[test]
    fn bar_config_default_incremental_steps_output_is_positive()
    {
        assert!(BarConfig::default().incremental_steps_output > 0);
    }
 
    #[test]
    fn bar_config_default_incremental_steps_input_is_positive()
    {
        assert!(BarConfig::default().incremental_steps_input > 0);
    }
 
    #[test]
    fn bar_config_default_context_menu_item_size_is_positive()
    {
        assert!(BarConfig::default().context_menu_item_size > 0);
    }
 
    #[test]
    fn bar_config_default_context_menu_size_is_positive()
    {
        assert!(BarConfig::default().context_menu_size > 0);
    }
 
    #[test]
    fn bar_config_default_tray_icon_size_is_positive()
    {
        assert!(BarConfig::default().tray_icon_size > 0);
    }
 
    // ---- Vec defaults are empty (user configures modules) ------------------
 
    #[test]
    fn bar_config_default_custom_modules_is_empty()
    {
        assert!(BarConfig::default().custom_modules.is_empty());
    }
 
    #[test]
    fn bar_config_default_left_modules_is_empty()
    {
        assert!(BarConfig::default().left_modules.is_empty());
    }
 
    #[test]
    fn bar_config_default_center_modules_is_empty()
    {
        assert!(BarConfig::default().center_modules.is_empty());
    }
 
    #[test]
    fn bar_config_default_right_modules_is_empty()
    {
        assert!(BarConfig::default().right_modules.is_empty());
    }
 
    // ---- BarPosition equality ----------------------------------------------
 
    #[test]
    fn bar_position_same_variants_are_equal()
    {
        assert_eq!(BarPosition::Up,    BarPosition::Up);
        assert_eq!(BarPosition::Down,  BarPosition::Down);
        assert_eq!(BarPosition::Left,  BarPosition::Left);
        assert_eq!(BarPosition::Right, BarPosition::Right);
    }
 
    #[test]
    fn bar_position_different_variants_are_not_equal()
    {
        assert_ne!(BarPosition::Up,   BarPosition::Down);
        assert_ne!(BarPosition::Left, BarPosition::Right);
        assert_ne!(BarPosition::Up,   BarPosition::Left);
    }
}
