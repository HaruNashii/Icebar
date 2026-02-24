// ============ IMPORTS ============
use iced_layershell::reexport::Anchor;
use serde::{Deserialize, Serialize};
use std::fs;





// ============ STRUCTS/ENUM'S ============
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BarPosition
{
    Up,
    Down
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub border_radius: [u32;4],
    pub use_output_as_text: bool,
    pub output_as_text_format: String,
    pub output_text_limit_len: usize,
    pub command_to_exec_on_left_click: Vec<String>,
    pub command_to_exec_on_right_click: Vec<String>
}

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
            border_radius: [6, 6, 6, 6],
            use_output_as_text: false,
            output_as_text_format: "{name}, {output}".to_string(),
            output_text_limit_len: 100,
            command_to_exec_on_left_click: vec![], 
            command_to_exec_on_right_click: vec![]
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BarConfig
{
    // ================= GENERAL =================
    pub display: Option<String>,
    pub bar_position: BarPosition,
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
    pub persistent_workspaces: Option<u8>,
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
    pub tray_height: u32,
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
    pub clock_height: u32,
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
    pub volume_output_height: u32,
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
    pub volume_input_height: u32,
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


    // ================= HYPR/SWAY WORKSPACES (STYLE) =================
    pub workspace_height: u32,
    pub workspace_text_size: u32,
    pub workspace_text: Vec<String>,
    pub workspace_selected_text: Option<Vec<String>>,
    pub workspace_spacing: u32,
    pub workspace_background_color_rgba: [u8;4],
    pub workspace_button_color_rgb: [u8;3],
    pub workspace_button_text_color_rgb: [u8;3],
    pub workspace_button_selected_color_rgb: [u8;3],
    pub workspace_button_hovered_color_rgb: [u8;3],
    pub workspace_button_hovered_text_color_rgb: [u8;3],
    pub workspace_button_pressed_color_rgb: [u8;3],
    pub workspace_border_color_rgba: [u8;4],
    pub workspace_border_size: f32,
    pub workspace_border_radius: [u32;4],


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
    pub context_menu_border_radius: [u32;4],

    // ================= CUSTOM MODULES =================
    pub custom_modules_spacing: u32,
    pub custom_modules: Vec<CustomModule>
}





// ============ FUNCTIONS ============
impl Default for BarConfig
{
    fn default() -> Self
    {
         Self 
        {
            display: None,
            bar_position: BarPosition::Up,
            bar_size: [0, 45],
            bar_general_padding: 6,
            bar_background_color_rgba: [18, 18, 22, 92],
            font_family: "JetBrains Mono".into(),
            font_style: "Normal".into(),

            left_modules: vec![],
            center_modules: vec!["clock".to_string()],
            right_modules: vec!["tray".to_string(), "volume/output".to_string(), "volume/input".to_string()],

            force_static_position_context_menu: None,
            reverse_scroll_on_workspace: false,
            persistent_workspaces: None,
            incremental_steps_output: 10,
            incremental_steps_input: 10,

            output_volume_format: 
            [
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
            ],
            output_volume_muted_format: "Muted".into(),
            input_volume_format:
            [
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
                "{}%".to_string(),
            ],
            input_volume_muted_format: "Muted".into(),

            clock_format: "󰥔  %H:%M".into(),
            clock_alt_format: "󰃭  %a %d %b |  󰥔  %H:%M:%S".into(),


            // ================= TRAY (STYLE) =================
            tray_height: 30,
            tray_icon_size: 18,
            tray_button_size: 5,
            tray_spacing: 8,
            tray_background_color_rgba: [30, 30, 36, 0],
            tray_button_color_rgb: [60, 50, 70],
            tray_button_text_color_rgb: [220, 220, 230],
            tray_button_hovered_color_rgb: [110, 40, 80],
            tray_button_hovered_text_color_rgb: [255, 255, 255],
            tray_button_pressed_color_rgb: [70, 20, 40],
            tray_border_color_rgba: [90, 70, 100, 100],
            tray_border_size: 1.0,
            tray_border_radius: [6, 6, 6, 6],
            
            // ================= CLOCK (STYLE) =================
            clock_height: 30,
            clock_text_size: 15,
            clock_background_color_rgba: [25, 25, 30, 95],
            clock_button_color_rgb: [50, 45, 60],
            clock_button_text_color_rgb: [235, 235, 240],
            clock_button_hovered_color_rgb: [130, 35, 70],
            clock_button_hovered_text_color_rgb: [255, 255, 255],
            clock_button_pressed_color_rgb: [80, 25, 45],
            clock_border_color_rgba: [120, 80, 130, 100],
            clock_border_size: 1.0,
            clock_border_radius: [8, 8, 8, 8],
            
            // ================= VOLUME/OUTPUT (STYLE) =================
            volume_output_height: 30,
            volume_output_text_size: 15,
            volume_output_background_color_rgba: [30, 30, 36, 95],
            volume_output_button_color_rgb: [55, 45, 65],
            volume_output_button_text_color_rgb: [220, 220, 230],
            volume_output_button_hovered_color_rgb: [150, 45, 85],
            volume_output_button_hovered_text_color_rgb: [255, 255, 255],
            volume_output_button_pressed_color_rgb: [85, 30, 50],
            volume_output_border_color_rgba: [110, 80, 120, 100],
            volume_output_border_size: 1.0,
            volume_output_border_radius: [6, 6, 6, 6],
            
            // ================= VOLUME/INPUT (STYLE) =================
            volume_input_height: 30,
            volume_input_text_size: 15,
            volume_input_background_color_rgba: [30, 30, 36, 95],
            volume_input_button_color_rgb: [55, 45, 65],
            volume_input_button_text_color_rgb: [220, 220, 230],
            volume_input_button_hovered_color_rgb: [150, 45, 85],
            volume_input_button_hovered_text_color_rgb: [255, 255, 255],
            volume_input_button_pressed_color_rgb: [85, 30, 50],
            volume_input_border_color_rgba: [110, 80, 120, 100],
            volume_input_border_size: 1.0,
            volume_input_border_radius: [6, 6, 6, 6],
            
            // ================= HYPR WORKSPACES (STYLE) =================
            workspace_height: 30,
            workspace_text_size: 15,
            workspace_text: vec![
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
            workspace_selected_text: Some(vec![
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
            workspace_background_color_rgba: [28, 28, 34, 95],
            workspace_button_color_rgb: [45, 40, 55],
            workspace_button_text_color_rgb: [200, 200, 210],
            workspace_button_selected_color_rgb: [150, 40, 80],
            workspace_button_hovered_color_rgb: [140, 35, 75],
            workspace_button_hovered_text_color_rgb: [255, 255, 255],
            workspace_button_pressed_color_rgb: [90, 25, 50],
            workspace_border_color_rgba: [120, 90, 135, 100],
            workspace_border_size: 1.0,
            workspace_border_radius: [6, 6, 6, 6],

            
            // ================= CONTEXT MENU (STYLE) =================
            context_menu_background_color_rgba: [20, 20, 24, 98],
            context_menu_background_size: 5,
            context_menu_background_border_color_rgba: [255, 255, 255, 100],
            context_menu_background_border_size: 1.0,
            context_menu_background_border_radius: [6, 6, 6, 6],
            
            context_menu_text_size: 15,
            context_menu_width: 200,
            context_menu_button_color_rgb: [45, 40, 55],
            context_menu_button_text_color_rgb: [230, 230, 240],
            context_menu_button_hovered_color_rgb: [150, 40, 80],
            context_menu_button_hovered_text_color_rgb: [255, 255, 255],
            context_menu_button_pressed_color_rgb: [85, 30, 55],
            context_menu_border_color_rgba: [130, 90, 140, 100],
            context_menu_border_size: 1.0,
            context_menu_border_radius: [8, 8, 8, 8],

            // ================= CUSTOM MODULES =================
            custom_modules_spacing: 10,
            custom_modules: Vec::new()
        }
    }
}



pub fn read_ron_config() -> (BarConfig, Anchor, Vec<String>, Vec<String>)
{
    println!("\n=== READING CONFIG FILE ===");
    let home_path = home::home_dir().expect("Failed To Get Home Directory");
    let path = home_path.join(".config/icebar/config.ron");

    let bar_config: BarConfig = fs::read_to_string(&path).map_err(|e| {panic!("Failed to read config: {e}"); }).and_then(|content| 
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

    let anchor_position = match bar_config.bar_position
    {
        BarPosition::Up => Anchor::Top | Anchor::Left | Anchor::Right,
        BarPosition::Down => Anchor::Bottom | Anchor::Left | Anchor::Right,
    };

    let mut active_modules = Vec::new();
    let mut inactive_modules = Vec::new();
    let all_possible_modules = ["tray", "hypr/workspaces", "sway/workspaces", "clock", "volume/output", "volume/input", "custom_modules"];
    let all_possible_position = [&bar_config.left_modules, &bar_config.center_modules, &bar_config.right_modules];
    for module in all_possible_modules
    {
        for position in all_possible_position
        {
            for item in position 
            {
                if *item == module
                {
                    active_modules.push(module.to_string());
                }
                else
                {
                    inactive_modules.push(module.to_string());
                }
            }
        }
    }

    (bar_config, anchor_position, active_modules, inactive_modules)
}
