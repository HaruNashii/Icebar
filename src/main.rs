// ============ IMPORTS ============
use iced_layershell::{daemon, settings::{StartMode, LayerShellSettings, Settings}};
use std::{sync::OnceLock, collections::HashMap, time::Instant};
use iced::Font;





// ============ CRATES ============
use crate::helpers::{font::build_font, fs::check_if_config_file_exists, misc::{define_bar_anchor_position, is_active_module, validate_bar_data}, monitor::get_monitor_res, string::{intern_string, weight_from_str}, style::{UserStyle, set_style, style} };
use crate::modules::{image::{PreloadedImage, preload_image}, data::{Modules, ModulesData}, tray::{self, TrayEvent, start_tray}};
use crate::ron::{read_ron_config, BarConfig};
use crate::context_menu::ContextMenuData;
use crate::subscription::subscription;
use crate::update::update;
use crate::view::view;





// ============ STATIC'S ============
pub static MAIN_ID: OnceLock<iced::window::Id> = OnceLock::new();





// ============ MOD'S ============
mod context_menu;
mod subscription;
mod modules;
mod helpers;
mod update;
mod view;
mod ron;





// ============ ENUM/STRUCT, ETC ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowInfo 
{
    MainBar,
    ContextMenu
}

#[derive(Default, Clone)]
pub struct AppData
{ 
    ids: HashMap<iced::window::Id, WindowInfo>,
    data: ContextMenuData,

    volume_output_raw: f32,
    volume_input_raw: f32,

    preloaded_images_handle: Vec<Option<(PreloadedImage, usize)>>,
    cpu_snapshot: Option<crate::modules::cpu::CpuSnapshot>,
    current_clock_timezone: Option<(String, u32)>,
    is_hovering_media_player_meta_data: bool,
    cached_continuous_outputs: Vec<String>,
    custom_module_last_run: Vec<Instant>,
    cached_command_outputs: Vec<String>,
    is_showing_alt_network_module: bool,
    is_hovering_volume_output: bool,
    is_hovering_volume_input: bool,
    is_hovering_workspace: bool,
    is_showing_alt_clock: bool,
    connection_type_icons: [String;3],
    network_icons: [String;4],
    volume_output_is_muted: bool,
    volume_input_is_muted: bool,
    mouse_position: (i32, i32),
    modules_data: ModulesData,
    monitor_size: (u32, u32),
    ron_config: BarConfig,
    default_font: Font,
}






// ============ FUNCTIONS ============
#[tokio::main]
pub async fn main() -> Result<(), iced_layershell::Error>
{
    check_if_config_file_exists();
    let (ron_config, current_clock_timezone, active_modules) = read_ron_config();
    let preloaded_images = preload_image(&ron_config.images);
    let validated_bar_data = validate_bar_data(&ron_config);
    let anchor_position = define_bar_anchor_position(&ron_config.bar_position);
    let monitor_res = get_monitor_res(ron_config.display.clone());
    if is_active_module(&active_modules, Modules::Tray) { start_tray(); }
    let ron_config_clone = ron_config.clone();
    let font_name = ron_config.font_family;
    let start_mode = match ron_config.display { Some(output) => StartMode::TargetScreen(output), None => StartMode::Active };



    let modules_data = ModulesData
    {
        active_modules: active_modules.clone(),
        ..Default::default()
    };
    let app_data = AppData
    {
        preloaded_images_handle: preloaded_images,
        default_font: build_font(&font_name, &ron_config.font_style),
        monitor_size: (monitor_res.0, monitor_res.1),
        custom_module_last_run: vec![Instant::now(); ron_config.custom_modules.len()],
        network_icons: ron_config.network_level_format,
        connection_type_icons: ron_config.network_connection_type_icons,
        ron_config: ron_config_clone, 
        current_clock_timezone,
        modules_data,
        ..Default::default()
    };


    daemon(move || app_data.clone(), namespace, update, view).style(style).subscription(subscription).settings(Settings
    {
        layer_settings: LayerShellSettings
        {
            exclusive_zone: validated_bar_data.exclusive_zone,
            size: Some(validated_bar_data.bar_size),
            layer: iced_layershell::reexport::Layer::Top,
            margin: validated_bar_data.floating_space,
            anchor: anchor_position,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    }).run()
}
fn namespace() -> String { String::from("icebar") }
pub fn id_info(app: &AppData, id: iced::window::Id) -> Option<WindowInfo> { app.ids.get(&id).cloned() }
