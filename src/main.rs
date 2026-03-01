// ============ IMPORTS ============
use iced_layershell::{application, settings::{LayerShellSettings, Settings, StartMode}};
use iced::{Font, font::Family};






// ============ CRATES ============
use crate::helpers::{misc::is_active_module, style::{style, set_style, UserStyle}, string::weight_from_str, fs::check_if_config_file_exists, monitor::get_monitor_res, };
use crate::modules::{data::{Modules, ModulesData}, tray::{self, TrayEvent, start_tray}};
use crate::ron::{BarPosition, read_ron_config, BarConfig};
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
    cached_continuous_outputs: Vec<String>,
    cached_command_outputs: Vec<String>,
    is_hovering_volume_output: bool,
    is_hovering_volume_input: bool,
    is_hovering_workspace: bool,
    is_showing_alt_clock: bool,
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
    let (ron_config, anchor_position, active_modules) = read_ron_config();
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
        default_font: Font { family: Family::Name(Box::leak(font_name.into_boxed_str())), weight: weight_from_str(&ron_config.font_style), ..iced::Font::DEFAULT}, 
        monitor_size: (monitor_res.0, monitor_res.1),
        ron_config: ron_config_clone, 
        modules_data,
        ..Default::default()
    };

    let (exclusive_zone, (floating_space_up, floating_space_right, floating_space_down, floating_space_left)) = match ron_config.bar_position 
    {
        BarPosition::Up => 
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On Top!!!") }
            (ron_config.bar_size[1] + ron_config.increased_exclusive_bar_zone, (ron_config.floating_space, 0, 0 ,0))
        },
        BarPosition::Right =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Right!!!") }
            (ron_config.bar_size[0] + ron_config.increased_exclusive_bar_zone, (0, ron_config.floating_space, 0, 0))
        }
        BarPosition::Down =>
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On The Bottom!!!") }
            (ron_config.bar_size[1] + ron_config.increased_exclusive_bar_zone, (0, 0, ron_config.floating_space, 0))
        }
        BarPosition::Left =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Left!!!") }
            (ron_config.bar_size[0] + ron_config.increased_exclusive_bar_zone, (0, 0, 0, ron_config.floating_space))
        }
    };

    let default_font = app_data.default_font;
    application(move || app_data.clone(), namespace, update, view).default_font(default_font).style(style).subscription(subscription).settings(Settings
    {
        layer_settings: LayerShellSettings
        {
            size: Some((ron_config.bar_size[0], ron_config.bar_size[1])),
            exclusive_zone: exclusive_zone as i32,
            margin: (floating_space_up, floating_space_right, floating_space_down, floating_space_left),
            anchor: anchor_position,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    }).run()
}
fn namespace() -> String { String::from("icebar") }
