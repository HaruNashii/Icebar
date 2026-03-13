// ============ IMPORTS ============
use iced_layershell::{application, settings::{LayerShellSettings, Settings, StartMode}};
use iced::{Font, font::Family};
use std::time::Instant;







// ============ CRATES ============
use crate::{helpers::{fs::check_if_config_file_exists, misc::{is_active_module, validade_bar_size_and_margin}, monitor::get_monitor_res, string::weight_from_str, style::{UserStyle, set_style, style} }};
use crate::modules::{data::{Modules, ModulesData}, tray::{self, TrayEvent, start_tray}};
use crate::ron::{read_ron_config, BarConfig};
use crate::subscription::subscription;
use crate::update::update;
use crate::view::view;





// ============ MOD'S ============
mod context_menu;
mod subscription;
mod modules;
mod helpers;
mod update;
mod view;
mod ron;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
struct AppData
{
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
    let (ron_config, anchor_position,  current_clock_timezone, active_modules) = read_ron_config();
    let monitor_res = get_monitor_res(ron_config.display.clone());
    if is_active_module(&active_modules, Modules::Tray) { start_tray(); }
    let ron_config_clone = ron_config.clone();
    let (bar_size, exclusive_zone, floating_space) = validade_bar_size_and_margin(&ron_config);
    let start_mode = match ron_config.display { Some(output) => StartMode::TargetScreen(output), None => StartMode::Active };
    let font_name = ron_config.font_family;

    let modules_data = ModulesData
    {
        active_modules: active_modules.clone(),
        ..Default::default()
    };
    let app_data = AppData
    {
        default_font: Font { family: Family::Name(Box::leak(font_name.into_boxed_str())), weight: weight_from_str(&ron_config.font_style), ..iced::Font::DEFAULT}, 
        monitor_size: (monitor_res.0, monitor_res.1),
        custom_module_last_run: vec![Instant::now(); ron_config.custom_modules.len()],
        network_icons: ron_config.network_level_format,
        connection_type_icons: ron_config.network_connection_type_icons,
        ron_config: ron_config_clone, 
        current_clock_timezone,
        modules_data,
        ..Default::default()
    };



    application(move || app_data.clone(), namespace, update, view).style(style).subscription(subscription).settings(Settings
    {
        layer_settings: LayerShellSettings
        {
            exclusive_zone: exclusive_zone as i32,
            size: Some((bar_size.0, bar_size.1)),
            layer: iced_layershell::reexport::Layer::Top,
            margin: floating_space,
            anchor: anchor_position,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    }).run()
}
fn namespace() -> String { String::from("icebar") }
