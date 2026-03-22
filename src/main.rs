// ============ IMPORTS ============
use iced_layershell::{daemon, settings::{StartMode, LayerShellSettings, Settings}};
use std::{sync::OnceLock, collections::HashMap, time::{Instant, Duration}};
use iced::Font;





// ============ CRATES ============
use crate::helpers::{font::build_font, fs::check_if_config_file_exists, misc::{define_bar_anchor_position, is_active_module, validate_bar_data}, monitor::get_monitor_res, string::{intern_string, weight_from_str}, style::{UserStyle, set_style, style} };
use crate::modules::{custom_modules::CustomModuleData, network::NetworkData, clock::ClockData, image::{ImageData, preload_image}, data::{Modules, ModulesData}, tray::{self, TrayEvent, start_tray}};
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
mod warning;
mod update;
mod view;
mod ron;





// ============ ENUM/STRUCT, ETC ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowInfo 
{
    MainBar,
    Warning,
    ContextMenu
}

#[derive(Default, Clone)]
pub struct AppData
{ 
    ids: HashMap<iced::window::Id, WindowInfo>,
    monitor_size: (u32, u32),
    context_menu_data: ContextMenuData,
    modules_data: ModulesData,
    warning_err: String,
    config_parsed_failed: bool,
    ron_config: BarConfig,
    default_font: Font,
}






// ============ FUNCTIONS ============
#[tokio::main]
pub async fn main() -> Result<(), iced_layershell::Error>
{
    check_if_config_file_exists();
    let (ron_config, current_clock_timezone, active_modules, (mut config_parsed_failed, mut warning_err)) = read_ron_config();
    let preloaded_images = preload_image(&mut warning_err, &mut config_parsed_failed, &ron_config.image.images);
    let anchor_position = define_bar_anchor_position(&ron_config.general.bar_position);
    let monitor_res = get_monitor_res(ron_config.general.display.clone());
    if is_active_module(&active_modules, Modules::Tray) { start_tray(); }
    let ron_config_clone = ron_config.clone();
    let font_name = ron_config.general.font_family;
    let start_mode = match ron_config.general.display { Some(output) => StartMode::TargetScreen(output), None => StartMode::Active };




    let modules_data = ModulesData
    {
        active_modules: active_modules.clone(),
        clock_data: ClockData { current_clock_timezone, ..Default::default() },
        network_data: NetworkData 
        {
            connection_type_icons: ron_config.network.network_connection_type_icons,
            network_icons: ron_config.network.network_level_format,
            ..Default::default()
        },
        custom_module_data: CustomModuleData
        {
            custom_module_last_run: vec![Instant::now() - Duration::from_secs(3600); ron_config.custom_module.custom_modules.len()],
            ..Default::default()
        },
        image_data: ImageData
        {
            preloaded_images_handle: preloaded_images
        },
        ..Default::default()
    };

    let mut app_data = AppData
    {
        warning_err,
        config_parsed_failed,
        default_font: build_font(&font_name, &ron_config.general.font_style),
        monitor_size: (monitor_res.0, monitor_res.1),
        ron_config: ron_config_clone, 
        modules_data,
        ..Default::default()
    };
    let validated_bar_data = validate_bar_data(&mut app_data);

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
