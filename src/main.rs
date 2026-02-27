// ============ IMPORTS ============
use iced::{Font, font::Family, mouse::ScrollDelta};
use iced_layershell::{application, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};






// ============ CRATES ============
use crate::{modules::{clock::ClockData, data::{Modules, ModulesData}, tray::{self, TrayEvent, start_tray}, volume::VolumeData }, ron::BarPosition};
use crate::helpers::{misc::is_active_module, style::{style, set_style, UserStyle}, string::weight_from_str, workspaces::WorkspaceData, fs::check_if_config_file_exists, monitor::get_monitor_res, };
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
#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message
{
    CreateCustomModuleCommand((Option<usize>, Vec<String>, String, bool, bool)),
    MenuLoaded(String, String, Vec<tray::MenuItem>),
    MouseWheelScrolled(ScrollDelta),
    CommandFinished(usize, String),
    WorkspaceButtonPressed(usize),
    IsHoveringVolumeOutput(bool),
    IsHoveringVolumeInput(bool),
    IsHoveringWorkspace(bool),
    CursorMoved(iced::Point),
    TrayIconClicked(usize),
    MuteAudioPressedOutput,
    MuteAudioPressedInput,
    TrayEvent(TrayEvent),
    ToggleAltClock,
    Nothing,
    Tick
}

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
        workspace_data: WorkspaceData::default(),
        volume_data: VolumeData::default(), 
        clock_data: ClockData::default(), 
        tray_icons: Vec::new(),
        active_modules
    };
    let app_data = AppData
    {
        default_font: Font { family: Family::Name(Box::leak(font_name.into_boxed_str())), weight: weight_from_str(&ron_config.font_style), ..iced::Font::DEFAULT}, 
        monitor_size: (monitor_res.0, monitor_res.1),
        cached_continuous_outputs: Vec::new(),
        cached_command_outputs: Vec::new(),
        is_hovering_volume_output: false, 
        is_hovering_volume_input: false, 
        is_hovering_workspace: false, 
        ron_config: ron_config_clone, 
        is_showing_alt_clock: false,
        mouse_position: (0, 0),
        modules_data,
    };

    let exclusive_zone = match ron_config.bar_position 
    {
        BarPosition::Up | BarPosition::Down => ron_config.bar_size[1],
        BarPosition::Left | BarPosition::Right => ron_config.bar_size[0]
    };
    let default_font = app_data.default_font;
    application(move || app_data.clone(), namespace, update, view).default_font(default_font).style(style).subscription(subscription).settings(Settings
    {
        layer_settings: LayerShellSettings
        {
            size: Some((ron_config.bar_size[0], ron_config.bar_size[1])),
            exclusive_zone: exclusive_zone as i32,
            anchor: anchor_position,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    }).run()
}
fn namespace() -> String { String::from("icebar") }
