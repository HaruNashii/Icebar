// ============ IMPORTS ============
use iced::{Task, mouse::ScrollDelta, widget::image};
use std::{sync::Once, time::{Duration, Instant}};
use iced_layershell::to_layer_message;
use chrono::Datelike;





// ============ CRATES ============
use crate::modules::{power_profile::{read_power_profile, cycle_power_profile}, plasma, image::preload_image, clock::cycle_clock_timezones, clock::get_current_time, data::Modules, hypr::{self, change_workspace_hypr}, media_player::{MediaPlayerAction, get_player_data_with_format, media_player_action}, network::NetworkData, niri::{self, change_workspace_niri}, sway::{self, change_workspace_sway}, tray::{load_tray_menu, MenuItem, TrayEvent}, volume, workspaces::UserWorkspaceAction};
use crate::volume_mixer::{MixerKind, MixerState, create_output_mixer_window, create_input_mixer_window, task_set_device_volume, task_toggle_device_mute, task_set_default_device, task_set_app_volume, task_toggle_app_mute};
use crate::helpers::{string::format_volume, misc::{is_active_module, validate_bar_data, define_bar_anchor_position}, workspaces::build_workspace_list, font::build_font, fs::check_if_config_file_exists, monitor::get_monitor_res};
use crate::modules::focused_window::{read_focused_window_hypr, read_focused_window_sway, read_focused_window_niri};
use crate::calendar::{CalendarView, DayClickAction, create_calendar_window};
use crate::context_menu::{create_context_menu, get_context_menu_size};
use crate::{warning::create_warning, MAIN_ID, AppData, WindowInfo};
use crate::ron::read_ron_config;





// ============ STATICS ============
static WARNING_ONCE: Once = Once::new();





// ============ ENUM/STRUCT, ETC ============
#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message
{
    TrayAction(String, String, i32, String),
    MouseButtonClicked,
    CloseContextMenu,
    CloseContextMenuAndCalendar,
    CloseWarning,

    MediaPlayerDataFetched(crate::modules::media_player::MediaPlayerData),
    CreateCustomModuleCommand((Option<usize>, Vec<String>, String, bool, bool)),
    MenuLoaded(String, String, Vec<MenuItem>),
    ContinuousCommandFinished(usize, String),
    ToggleAltClockAndCycleClockTimeZones,
    IsHoveringMediaPlayerMetaData(bool),
    TrayIconClicked(String),
    MouseWheelScrolled(ScrollDelta),
    CommandFinished(usize, String),
    WorkspaceButtonPressed(i32),
    IsHoveringVolumeOutput(bool),
    IsHoveringVolumeInput(bool),
    NetworkUpdated(NetworkData),
    IsHoveringWorkspace(bool),
    MediaPlayerClickPlayPause,
    CursorMoved(iced::Point),
    MuteAudioPressedOutput,
    MuteAudioPressedInput,
    MediaPlayerClickNext,
    MediaPlayerClickPrev,
    TrayEvent(TrayEvent),
    CycleClockTimeZones,
    ToggleAltNetwork,
    ToggleAltClock,
    ConfigChanged,
    Nothing,


    BarEnter,
    BarLeave,
    BarVisibilityTick,


    ShowCalendar,
    CloseCalendar,
    CalendarPrev,
    CalendarNext,
    CalendarSetView(CalendarView),
    CalendarDaySelected(u32),
    CalendarMonthSelected(u32),
    CalendarYearSelected(i32),

    ShowVolumeOutputMixer,
    ShowVolumeInputMixer,
    CloseVolumeOutputMixer,
    CloseVolumeInputMixer,
    MixerStateUpdated(MixerState),
    SetDeviceVolume(MixerKind, u32, u8),
    ToggleDeviceMute(MixerKind, u32),
    SetDefaultDevice(MixerKind, String),
    SetAppVolume(MixerKind, u32, u8),
    ToggleAppMute(MixerKind, u32),
    ToggleOutputDeviceCategory,
    ToggleInputDeviceCategory,
    ToggleOutputAppCategory,
    ToggleInputAppCategory,

    Tick,
    VolumeUpdated(f32, bool, f32, bool),
    FocusedWindowSwayFetched(Option<String>),
    FocusedWindowNiriFetched(Option<String>),
    FocusedWindowHyprFetched(Option<String>),
    SwayWorkspacesFetched(i32, Vec<i32>),
    NiriWorkspacesFetched(i32, Vec<i32>),
    HyprWorkspacesFetched(i32, Vec<i32>),
    PlasmaWorkspacesFetched(i32, Vec<i32>, Vec<String>),


    UpdatePowerProfile,
    CyclePowerProfile,

    PowerProfileFetched(Option<crate::modules::power_profile::PowerProfile>),

    CpuTextChanged(String),
    RamTextChanged(String),
    DiskTextChanged(String),
    CpuTempTextChanged(String),
    NetworkSpeedChanged(u64, u64),

    UpdateFocusedWindowNiri,
    UpdateFocusedWindowSway,
    UpdateFocusedWindowHypr,
    UpdateMediaPlayerMetadata,
    UpdatePlasmaWorkspaces,
    UpdateNiriWorkspaces,
    UpdateSwayWorkspaces,
    UpdateHyprWorkspaces,
    UpdateClock
}





// ============ FUNCTIONS ============
pub fn update(app: &mut AppData, message: Message) -> Task<Message>
{
    if app.config_parsed_failed
    {
        let has_warning_window = app.ids.iter().any(|(_, info)| *info == WindowInfo::Warning);
        if !has_warning_window
        {
            return create_warning(app);
        }
    };

    match message
    {
        Message::TrayAction(service, path, id, label) =>
        {
            println!("\n===# Menu Action Activated!!! #===");
            println!("Label: {label}");
            println!("Service: {service}");
            println!("Menu Path: {path}");
            println!("Id: {id}");

            app.context_menu_data.context_menu_is_open = false;
            let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::ContextMenu).map(|(id, _)| *id).collect();
            for id in &window_ids_to_close { app.ids.remove(id); }
            let close_tasks = Task::batch(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            let activate_task = Task::perform
            (
                async move { let _ = crate::tray::activate_menu_item(&service, &path, id).await; },
                |_| Message::Nothing
            );
            return Task::batch([close_tasks, activate_task]);
        }

        Message::CursorMoved(position) =>
        {
            let new_pos = (position.x as i32, position.y as i32);
            if new_pos != app.context_menu_data.mouse_position
            {
                app.context_menu_data.mouse_position = new_pos;
            }
            if app.context_menu_data.context_menu_is_open
            {
                let (width, height) = get_context_menu_size(&app.context_menu_data, &app.ron_config);
                app.context_menu_data.cursor_is_inside_menu = position.x >= 0.0 && position.y >= 0.0 && position.x <= width as f32 && position.y <= height as f32;
            }
            if app.modules_data.calendar_data.is_open
            {
                let [cw, ch] = app.ron_config.calendar_window.calendar_window_size;
                app.modules_data.calendar_data.cursor_inside = position.x >= 0.0 && position.y >= 0.0 && position.x <= cw as f32 && position.y <= ch as f32;
            }
            app.modules_data.calendar_data.mouse_pos = (position.x as i32, position.y as i32);

            if app.modules_data.volume_mixer_data.output_mixer_open
            {
                let [mw, mh] = app.ron_config.volume_output_mixer.mixer_window_size;
                app.modules_data.volume_mixer_data.output_cursor_inside = position.x >= 0.0 && position.y >= 0.0 && position.x <= mw as f32 && position.y <= mh as f32;
            }
            if app.modules_data.volume_mixer_data.input_mixer_open
            {
                let [mw, mh] = app.ron_config.volume_input_mixer.mixer_window_size;
                app.modules_data.volume_mixer_data.input_cursor_inside = position.x >= 0.0 && position.y >= 0.0 && position.x <= mw as f32 && position.y <= mh as f32;
            }
            app.modules_data.volume_mixer_data.mouse_pos = (position.x as i32, position.y as i32);
        }

        Message::MouseButtonClicked =>
        {
            let mut tasks: Vec<Task<Message>> = Vec::new();

            let has_context_menu = app.ids.values().any(|v| *v == WindowInfo::ContextMenu);
            if has_context_menu
            {
                app.context_menu_data.context_menu_is_open = false;
                if !app.context_menu_data.cursor_is_inside_menu
                {
                    let ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::ContextMenu).map(|(id, _)| *id).collect();
                    for id in &ids_to_close { app.ids.remove(id); }
                    tasks.extend(ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
                }
            }

            let has_calendar = app.ids.values().any(|v| *v == WindowInfo::Calendar);
            if has_calendar && !app.modules_data.calendar_data.cursor_inside
            {
                app.modules_data.calendar_data.is_open = false;
                let ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Calendar).map(|(id, _)| *id).collect();
                for id in &ids_to_close { app.ids.remove(id); }
                tasks.extend(ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }

            let has_output_mixer = app.ids.values().any(|v| *v == WindowInfo::VolumeOutputMixer);
            if has_output_mixer && !app.modules_data.volume_mixer_data.output_cursor_inside
            {
                app.modules_data.volume_mixer_data.output_mixer_open = false;
                let ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeOutputMixer).map(|(id, _)| *id).collect();
                for id in &ids_to_close { app.ids.remove(id); }
                tasks.extend(ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }

            let has_input_mixer = app.ids.values().any(|v| *v == WindowInfo::VolumeInputMixer);
            if has_input_mixer && !app.modules_data.volume_mixer_data.input_cursor_inside
            {
                app.modules_data.volume_mixer_data.input_mixer_open = false;
                let ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeInputMixer).map(|(id, _)| *id).collect();
                for id in &ids_to_close { app.ids.remove(id); }
                tasks.extend(ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }

            if !tasks.is_empty() { return Task::batch(tasks); }
        }

        Message::CloseContextMenu =>
        {
            app.context_menu_data.context_menu_is_open = false;
            let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::ContextMenu).map(|(id, _)| *id).collect();
            for id in &window_ids_to_close { app.ids.remove(id); }
            maybe_restart_hide_timer(app);
            return Task::batch(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
        }

        Message::CloseCalendar =>
        {
            app.modules_data.calendar_data.is_open = false;
            let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Calendar).map(|(id, _)| *id).collect();
            for id in &window_ids_to_close { app.ids.remove(id); }
            maybe_restart_hide_timer(app);
            return Task::batch(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
        }

        Message::CloseVolumeOutputMixer =>
        {
            app.modules_data.volume_mixer_data.output_mixer_open = false;
            let ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeOutputMixer).map(|(id, _)| *id).collect();
            for id in &ids { app.ids.remove(id); }
            maybe_restart_hide_timer(app);
            return Task::batch(ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
        }

        Message::CloseVolumeInputMixer =>
        {
            app.modules_data.volume_mixer_data.input_mixer_open = false;
            let ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeInputMixer).map(|(id, _)| *id).collect();
            for id in &ids { app.ids.remove(id); }
            maybe_restart_hide_timer(app);
            return Task::batch(ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
        }

        Message::CloseContextMenuAndCalendar =>
        {
            let mut tasks: Vec<Task<Message>> = Vec::new();

            app.context_menu_data.context_menu_is_open = false;
            let context_ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::ContextMenu).map(|(id, _)| *id).collect();
            for id in &context_ids { app.ids.remove(id); }
            tasks.extend(context_ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));

            app.modules_data.calendar_data.is_open = false;
            let calendar_ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Calendar).map(|(id, _)| *id).collect();
            for id in &calendar_ids { app.ids.remove(id); }
            tasks.extend(calendar_ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));

            app.modules_data.volume_mixer_data.output_mixer_open = false;
            let out_mixer_ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeOutputMixer).map(|(id, _)| *id).collect();
            for id in &out_mixer_ids { app.ids.remove(id); }
            tasks.extend(out_mixer_ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));

            app.modules_data.volume_mixer_data.input_mixer_open = false;
            let in_mixer_ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeInputMixer).map(|(id, _)| *id).collect();
            for id in &in_mixer_ids { app.ids.remove(id); }
            tasks.extend(in_mixer_ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));

            maybe_restart_hide_timer(app);

            return Task::batch(tasks);
        }

        Message::CloseWarning =>
        {
            app.config_parsed_failed = false;
            let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Warning).map(|(id, _)| *id).collect();
            for id in &window_ids_to_close { app.ids.remove(id); }
            return Task::batch(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
        }

        Message::IsHoveringVolumeOutput(bool)        => { app.modules_data.volume_data.is_hovering_volume_output = bool; }
        Message::IsHoveringVolumeInput(bool)         => { app.modules_data.volume_data.is_hovering_volume_input = bool; }
        Message::IsHoveringWorkspace(bool)           => { app.modules_data.workspace_data.is_hovering_workspace = bool; }
        Message::IsHoveringMediaPlayerMetaData(bool) => { app.modules_data.media_player_data.is_hovering_media_player_meta_data = bool; }
        Message::MuteAudioPressedOutput              => { return volume::volume(volume::VolumeAction::MuteOutput); }
        Message::MuteAudioPressedInput               => { return volume::volume(volume::VolumeAction::MuteInput); }
        Message::ToggleAltClock                      => { app.modules_data.clock_data.is_showing_alt_clock = !app.modules_data.clock_data.is_showing_alt_clock; }
        Message::CommandFinished(index, text)           => { if index > 1024 { return Task::none(); }; if app.modules_data.custom_module_data.cached_command_outputs.len() <= index { app.modules_data.custom_module_data.cached_command_outputs.resize(index + 1, String::new()); } app.modules_data.custom_module_data.cached_command_outputs[index] = text; }
        Message::ContinuousCommandFinished(index, text) => { if index > 1024 { return Task::none(); }; if app.modules_data.custom_module_data.cached_continuous_outputs.len() <= index { app.modules_data.custom_module_data.cached_continuous_outputs.resize(index + 1, String::new()); } app.modules_data.custom_module_data.cached_continuous_outputs[index] = text; }
        Message::MediaPlayerClickNext                => return media_player_action(&app.ron_config.media_player_metadata.player, MediaPlayerAction::Next),
        Message::MediaPlayerClickPlayPause           => return media_player_action(&app.ron_config.media_player_metadata.player, MediaPlayerAction::PlayPause),
        Message::MediaPlayerClickPrev                => return media_player_action(&app.ron_config.media_player_metadata.player, MediaPlayerAction::Prev),
        Message::CycleClockTimeZones                 => cycle_clock_timezones(app),
        Message::ToggleAltClockAndCycleClockTimeZones => { app.modules_data.clock_data.is_showing_alt_clock = !app.modules_data.clock_data.is_showing_alt_clock; cycle_clock_timezones(app); }

        Message::CpuTextChanged(text)     => { app.modules_data.cpu_text      = text; }
        Message::RamTextChanged(text)     => { app.modules_data.ram_text      = text; }
        Message::DiskTextChanged(text)    => { app.modules_data.disk_text     = text; }
        Message::CpuTempTextChanged(text) => { app.modules_data.cpu_temp_text = text; }

        Message::NetworkSpeedChanged(rx, tx) =>
        {
            app.modules_data.network_data.rx_bytes_per_sec = rx;
            app.modules_data.network_data.tx_bytes_per_sec = tx;
        }

        Message::UpdatePowerProfile => return Task::perform(tokio::task::spawn_blocking(read_power_profile), |r| Message::PowerProfileFetched(r.ok().flatten())),

        Message::PowerProfileFetched(Some(p)) =>
        {
            app.modules_data.power_profile_data.current_profile = p;
        }

        Message::PowerProfileFetched(None) => {}

        Message::CyclePowerProfile =>
        {
            if let Some(new_profile) = cycle_power_profile(&app.modules_data.power_profile_data.current_profile)
            {
                app.modules_data.power_profile_data.current_profile = new_profile;
            }
        }

        Message::FocusedWindowNiriFetched(title) => { app.modules_data.focused_window_data.title = title.unwrap_or_default(); }
        Message::FocusedWindowSwayFetched(title) => { app.modules_data.focused_window_data.title = title.unwrap_or_default(); }
        Message::FocusedWindowHyprFetched(title) => { app.modules_data.focused_window_data.title = title.unwrap_or_default(); }
        Message::UpdateFocusedWindowNiri         => { return Task::perform(tokio::task::spawn_blocking(read_focused_window_niri), |result| Message::FocusedWindowNiriFetched(result.ok().flatten())); }
        Message::UpdateFocusedWindowSway         => { return Task::perform(tokio::task::spawn_blocking(read_focused_window_sway), |result| Message::FocusedWindowSwayFetched(result.ok().flatten())); }
        Message::UpdateFocusedWindowHypr         => { return Task::perform(read_focused_window_hypr(), Message::FocusedWindowHyprFetched); }
        Message::MediaPlayerDataFetched(data)    => { app.modules_data.media_player_data = data; }
        Message::SwayWorkspacesFetched(current, list) => { app.modules_data.workspace_data.current_workspace = current; app.modules_data.workspace_data.visible_workspaces = list; }
        Message::NiriWorkspacesFetched(current, list) => { app.modules_data.workspace_data.current_workspace = current; app.modules_data.workspace_data.visible_workspaces = list; }
        Message::HyprWorkspacesFetched(current, list) => { app.modules_data.workspace_data.current_workspace = current; app.modules_data.workspace_data.visible_workspaces = list; }

        Message::PlasmaWorkspacesFetched(current, list, ids) =>
        {
            app.modules_data.workspace_data.current_workspace  = current;
            app.modules_data.workspace_data.visible_workspaces = list;
            app.modules_data.plasma_desktop_ids = ids;
        }

        Message::UpdatePlasmaWorkspaces =>
        {
            let persistent = app.ron_config.workspace.persistent_workspaces;
            return Task::perform(plasma::get_plasma_workspaces(), move |(current, counts, ids)|
            {
                Message::PlasmaWorkspacesFetched(current, build_workspace_list(&counts, persistent), ids)
            });
        }

        Message::WorkspaceButtonPressed(id) =>
        {
            if is_active_module(&app.modules_data.active_modules, Modules::HyprWorkspaces)
            {
                change_workspace_hypr(UserWorkspaceAction::ChangeWithIndex(id));
            }
            else if is_active_module(&app.modules_data.active_modules, Modules::SwayWorkspaces)
            {
                return Task::perform(
                    tokio::task::spawn_blocking(move || change_workspace_sway(UserWorkspaceAction::ChangeWithIndex(id))),
                    |_| Message::Nothing
                );
            }
            else if is_active_module(&app.modules_data.active_modules, Modules::NiriWorkspaces)
            {
                return Task::perform(
                    tokio::task::spawn_blocking(move || change_workspace_niri(UserWorkspaceAction::ChangeWithIndex(id))),
                    |_| Message::Nothing
                );
            }
            else if is_active_module(&app.modules_data.active_modules, Modules::PlasmaWorkspaces)
            {
                let ids = app.modules_data.plasma_desktop_ids.clone();
                return Task::perform(
                    plasma::change_workspace_plasma(UserWorkspaceAction::ChangeWithIndex(id), ids),
                    |_| Message::Nothing
                );
            }
        }

        Message::UpdateHyprWorkspaces =>
        {
            let persistent = app.ron_config.workspace.persistent_workspaces;
            return Task::perform(tokio::task::spawn_blocking(move || { (hypr::current_workspace(), hypr::workspace_count()) }), move |result|
            {
                let (current, counts) = result.unwrap_or((0, vec![]));
                Message::HyprWorkspacesFetched(current, build_workspace_list(&counts, persistent))
            });
        }

        Message::UpdateSwayWorkspaces =>
        {
            let persistent = app.ron_config.workspace.persistent_workspaces;
            return Task::perform(tokio::task::spawn_blocking(move || { (sway::current_workspace(), sway::workspace_count()) }), move |result|
            {
                let (current, counts) = result.unwrap_or((0, vec![]));
                Message::SwayWorkspacesFetched(current, build_workspace_list(&counts, persistent))
            });
        }

        Message::NetworkUpdated(data) =>
        {
            app.modules_data.network_data.connection_type    = data.connection_type;
            app.modules_data.network_data.network_level      = data.network_level;
            app.modules_data.network_data.network_speed      = data.network_speed;
            app.modules_data.network_data.id                 = data.id;
            app.modules_data.network_data.iface              = data.iface;
            app.modules_data.network_data.rx_bytes_per_sec   = data.rx_bytes_per_sec;
            app.modules_data.network_data.tx_bytes_per_sec   = data.tx_bytes_per_sec;
        }

        Message::UpdateMediaPlayerMetadata =>
        {
            let player = app.ron_config.media_player_metadata.player.clone();
            let format = app.ron_config.media_player_metadata.media_player_metadata_format.clone();
            return Task::perform
            (
                async move { get_player_data_with_format(&player, &format).await },
                Message::MediaPlayerDataFetched
            );
        }

        Message::VolumeUpdated(out_vol, out_muted, in_vol, in_muted) =>
        {
            app.modules_data.volume_data.volume_output_raw = out_vol;
            app.modules_data.volume_data.volume_input_raw  = in_vol;

            let (output_str, _) = format_volume(out_vol, out_muted, app.ron_config.volume_output.output_volume_unique_format.clone(), &app.ron_config.volume_output.output_volume_format, &app.ron_config.volume_output.output_volume_muted_format);
            app.modules_data.volume_data.output_volume_level  = output_str;
            app.modules_data.volume_data.volume_output_is_muted = out_muted;

            let (input_str, _) = format_volume(in_vol, in_muted, app.ron_config.volume_input.input_volume_unique_format.clone(), &app.ron_config.volume_input.input_volume_format, &app.ron_config.volume_input.input_volume_muted_format);
            app.modules_data.volume_data.input_volume_level  = input_str;
            app.modules_data.volume_data.volume_input_is_muted = in_muted;
        }

        Message::UpdateNiriWorkspaces =>
        {
            WARNING_ONCE.call_once(||
            {
                if app.ron_config.workspace.persistent_workspaces.is_some()
                {
                    println!("\n=== Niri Workspaces Warning ===");
                    for _ in 0..3
                    {
                        println!("Warning!!!: Persistent Elements Defined But Niri Doesn't Support Persistent Workspaces.");
                    }
                    println!("\n");
                }
            });

            return Task::perform(tokio::task::spawn_blocking(|| { (niri::current_workspace(), niri::workspace_count()) }), |result|
            {
                let (current, counts) = result.unwrap_or((0, vec![]));
                Message::NiriWorkspacesFetched(current, build_workspace_list(&counts, None))
            });
        }

        Message::UpdateClock =>
        {
            let format_to_send = if app.modules_data.clock_data.is_showing_alt_clock
            {
                &app.ron_config.clock.clock_alt_format
            }
            else
            {
                &app.ron_config.clock.clock_format
            };
            app.modules_data.clock_data.current_time = get_current_time(format_to_send, &app.modules_data.clock_data.current_clock_timezone)
        }

        Message::ToggleAltNetwork =>
        {
            app.modules_data.network_data.is_showing_alt_network_module = !app.modules_data.network_data.is_showing_alt_network_module;
            if app.modules_data.network_data.is_showing_alt_network_module
            {
                app.modules_data.network_data.connection_type_icons = app.ron_config.alt_network.alt_network_connection_type_icons.clone();
                app.modules_data.network_data.network_icons         = app.ron_config.alt_network.alt_network_level_format.clone();
            }
            else
            {
                app.modules_data.network_data.connection_type_icons = app.ron_config.network.network_connection_type_icons.clone();
                app.modules_data.network_data.network_icons         = app.ron_config.network.network_level_format.clone();
            }
        }

        Message::ConfigChanged =>
        {
            let Some(&id) = MAIN_ID.get() else { return Task::none(); };
            println!("\n=== CONFIG RELOAD ===");
            println!("[icebar] config.ron changed — reloading in place...");
            check_if_config_file_exists(app.cli_data.config.clone());
            let (new_config, current_clock_timezone, active_modules, (mut config_parsed_failed, mut warning_err)) = read_ron_config(app.cli_data.config.clone());
            let preloaded_images = preload_image(&mut warning_err, &mut config_parsed_failed, &new_config.image.images);
            let new_anchor       = define_bar_anchor_position(&new_config.general.bar_position);
            let monitor_res      = get_monitor_res(new_config.general.display.clone());
            let font_name        = new_config.general.font_family.clone();
            let new_font         = if font_name != app.ron_config.general.font_family { build_font(&font_name, &new_config.general.font_style) } else { app.default_font };
            let mut modules_data = app.modules_data.clone();

            modules_data.active_modules                              = active_modules.clone();
            modules_data.clock_data.current_clock_timezone          = current_clock_timezone;
            modules_data.network_data.connection_type_icons         = new_config.network.network_connection_type_icons.clone();
            modules_data.network_data.network_icons                 = new_config.network.network_level_format.clone();
            modules_data.custom_module_data.custom_module_last_run  = vec![Instant::now() - Duration::from_secs(3600); new_config.custom_module.custom_modules.len()];
            modules_data.image_data.preloaded_images_handle         = preloaded_images;

            let old_config_parse_status = app.config_parsed_failed;

            *app = AppData
            {
                warning_err,
                config_parsed_failed,
                ids:              app.ids.clone(),
                default_font:     new_font,
                monitor_size:     monitor_res,
                ron_config:       new_config,
                modules_data,
                cli_data:         app.cli_data.clone(),
                bar_visible:      app.bar_visible,
                cursor_on_bar:    app.cursor_on_bar,
                hide_timer:       app.hide_timer,
                show_timer:       app.show_timer,
                context_menu_data: app.context_menu_data.clone()
            };

            let bar_data_validated = validate_bar_data(app);
            let mut bar_size       = bar_data_validated.bar_size;
            if bar_size.0 == 0 { bar_size.0 = app.monitor_size.0; }
            if bar_size.1 == 0 { bar_size.1 = app.monitor_size.1; }

            let config_parsed_failed_final = app.config_parsed_failed;

            let mut task_vec = vec!
            [
                Task::done(Message::SizeChange          { id, size:      bar_size }),
                Task::done(Message::AnchorChange        { id, anchor:    new_anchor }),
                Task::done(Message::MarginChange        { id, margin:    bar_data_validated.floating_space }),
                Task::done(Message::ExclusiveZoneChange { id, zone_size: bar_data_validated.exclusive_zone })
            ];

            if !config_parsed_failed_final && old_config_parse_status
            {
                let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Warning).map(|(id, _)| *id).collect();
                for id in &window_ids_to_close { app.ids.remove(id); }
                task_vec.extend(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }

            let (output_str, _) = format_volume(app.modules_data.volume_data.volume_output_raw, app.modules_data.volume_data.volume_output_is_muted, app.ron_config.volume_output.output_volume_unique_format.clone(), &app.ron_config.volume_output.output_volume_format, &app.ron_config.volume_output.output_volume_muted_format);
            app.modules_data.volume_data.output_volume_level = output_str;

            let (input_str, _) = format_volume(app.modules_data.volume_data.volume_input_raw, app.modules_data.volume_data.volume_input_is_muted, app.ron_config.volume_input.input_volume_unique_format.clone(), &app.ron_config.volume_input.input_volume_format, &app.ron_config.volume_input.input_volume_muted_format);
            app.modules_data.volume_data.input_volume_level = input_str;

            println!("\n=== CONFIG RELOAD ===");
            println!("Reloaded Successfully");

            if crate::helpers::misc::is_active_module(&app.modules_data.active_modules, crate::modules::data::Modules::Tray)
            {
                crate::modules::tray::start_tray(app.ron_config.tray.tray_attention_icon);
            }

            return Task::batch(task_vec);
        }

        Message::MouseWheelScrolled(delta) =>
        {
            let y = match delta
            {
                ScrollDelta::Pixels { y, .. } => y,
                ScrollDelta::Lines  { y, .. } => y * 15.0
            };

            if app.modules_data.media_player_data.is_hovering_media_player_meta_data
            {
                if y > 0. { return media_player_action(&app.ron_config.media_player_metadata.player, MediaPlayerAction::VolumeUp); }
                if y < 0. { return media_player_action(&app.ron_config.media_player_metadata.player, MediaPlayerAction::VolumeDown); }
            }

            if app.modules_data.volume_data.is_hovering_volume_output
            {
                if y > 0. { return volume::volume(volume::VolumeAction::IncreaseOutput(app.ron_config.volume_output.incremental_steps_output)); }
                if y < 0. { return volume::volume(volume::VolumeAction::DecreaseOutput(app.ron_config.volume_output.incremental_steps_output)); }
            }

            if app.modules_data.volume_data.is_hovering_volume_input
            {
                if y > 0. { return volume::volume(volume::VolumeAction::IncreaseInput(app.ron_config.volume_input.incremental_steps_input)); }
                if y < 0. { return volume::volume(volume::VolumeAction::DecreaseInput(app.ron_config.volume_input.incremental_steps_input)); }
            }

            if app.modules_data.workspace_data.is_hovering_workspace
            {
                let hypr_active   = is_active_module(&app.modules_data.active_modules, Modules::HyprWorkspaces);
                let sway_active   = is_active_module(&app.modules_data.active_modules, Modules::SwayWorkspaces);
                let niri_active   = is_active_module(&app.modules_data.active_modules, Modules::NiriWorkspaces);
                let plasma_active = is_active_module(&app.modules_data.active_modules, Modules::PlasmaWorkspaces);

                if y > 0.
                {
                    if app.ron_config.workspace.reverse_scroll_on_workspace
                    {
                        if hypr_active        { change_workspace_hypr(UserWorkspaceAction::MoveNext); }
                        else if sway_active   { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_sway(UserWorkspaceAction::MoveNext)), |_| Message::Nothing); }
                        else if niri_active   { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_niri(UserWorkspaceAction::MoveNext)), |_| Message::Nothing); }
                        else if plasma_active { let ids = app.modules_data.plasma_desktop_ids.clone(); return Task::perform(plasma::change_workspace_plasma(UserWorkspaceAction::MoveNext, ids), |_| Message::Nothing); }
                    }
                    else if hypr_active        { change_workspace_hypr(UserWorkspaceAction::MovePrev); }
                    else if sway_active        { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_sway(UserWorkspaceAction::MovePrev)), |_| Message::Nothing); }
                    else if niri_active        { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_niri(UserWorkspaceAction::MovePrev)), |_| Message::Nothing); }
                    else if plasma_active      { let ids = app.modules_data.plasma_desktop_ids.clone(); return Task::perform(plasma::change_workspace_plasma(UserWorkspaceAction::MovePrev, ids), |_| Message::Nothing); }
                }

                if y < 0.
                {
                    if app.ron_config.workspace.reverse_scroll_on_workspace
                    {
                        if hypr_active        { change_workspace_hypr(UserWorkspaceAction::MovePrev); }
                        else if sway_active   { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_sway(UserWorkspaceAction::MovePrev)), |_| Message::Nothing); }
                        else if niri_active   { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_niri(UserWorkspaceAction::MovePrev)), |_| Message::Nothing); }
                        else if plasma_active { let ids = app.modules_data.plasma_desktop_ids.clone(); return Task::perform(plasma::change_workspace_plasma(UserWorkspaceAction::MovePrev, ids), |_| Message::Nothing); }
                    }
                    else if hypr_active        { change_workspace_hypr(UserWorkspaceAction::MoveNext); }
                    else if sway_active        { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_sway(UserWorkspaceAction::MoveNext)), |_| Message::Nothing); }
                    else if niri_active        { return Task::perform(tokio::task::spawn_blocking(|| change_workspace_niri(UserWorkspaceAction::MoveNext)), |_| Message::Nothing); }
                    else if plasma_active      { let ids = app.modules_data.plasma_desktop_ids.clone(); return Task::perform(plasma::change_workspace_plasma(UserWorkspaceAction::MoveNext, ids), |_| Message::Nothing); }
                }
            }
        }

        Message::Tick =>
        {
            let mut tasks = Vec::new();
            for module_name in &app.modules_data.active_modules
            {
                if let Modules::CustomModule(index) = module_name
                {
                    let index = *index;
                    let Some(module) = app.ron_config.custom_module.custom_modules.get(index) else { continue; };
                    if module.continous_command.is_empty() { continue; }
                    if index > 1024 { continue; }
                    if app.modules_data.custom_module_data.custom_module_last_run.len() <= index { app.modules_data.custom_module_data.custom_module_last_run.resize(index + 1, Instant::now() - Duration::from_secs(3600)); }
                    if app.modules_data.custom_module_data.custom_module_last_run[index].elapsed() < Duration::from_millis(module.continous_command_interval.max(1)) { continue; }
                    app.modules_data.custom_module_data.custom_module_last_run[index] = Instant::now();

                    if let Some((program, args)) = module.continous_command.split_first()
                    {
                        let program     = program.clone();
                        let args        = args.to_vec();
                        let display_err = module.display_err_output_if_failed;

                        tasks.push(Task::perform
                        (
                            async move
                            {
                                let result = tokio::time::timeout(Duration::from_secs(10), tokio::process::Command::new(program).args(args).output()).await;
                                let out = result.ok().and_then(|r| r.ok());
                                out.map
                                (|o|
                                {
                                    if o.stdout.is_empty() && display_err
                                    {
                                        String::from_utf8_lossy(&o.stderr).into()
                                    }
                                    else
                                    {
                                        String::from_utf8_lossy(&o.stdout).into()
                                    }
                                }).unwrap_or_default()
                            },
                            move |text| Message::ContinuousCommandFinished(index, text)
                        ));
                    }
                }
            }
            return Task::batch(tasks);
        }

        Message::CreateCustomModuleCommand((output_index, command_vec, custom_name, is_left_click, output_as_text)) =>
        {
            if let Some((program, args)) = command_vec.split_first()
            {
                let program = program.clone();
                let args    = args.to_vec();

                println!("\n=== Custom Module ===");
                if custom_name.is_empty() { if is_left_click { println!("Custom Module Button Was *Left* Clicked!!"); } else { println!("Custom Module Button Was *Right* Clicked!!"); } } else if is_left_click { println!("Your '{custom_name}' Button Was *Left* Clicked!!"); } else { println!("Your '{custom_name}' Button Was *Right* Clicked!!"); }

                if output_as_text
                {
                    return Task::perform
                    (
                        async move
                        {
                            let result = tokio::time::timeout(
                                std::time::Duration::from_secs(10),
                                tokio::process::Command::new(program).args(args).output()
                            ).await;
                            let output = result.ok().and_then(|r| r.ok());
                            if custom_name.is_empty() { println!("Custom Module Output:\n{:?}", output); } else { println!("'{custom_name}' Command Was Running!!!, The Output Was:\n{:?}", output); }
                            output.map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default()
                        },
                        move |text| Message::CommandFinished(output_index.unwrap_or(0), text)
                    );
                }

                tokio::spawn(async move
                {
                    let output = tokio::process::Command::new(program).args(args).output().await;
                    if custom_name.is_empty() { println!("Custom Module Output:\n{:?}", output); } else { println!("'{custom_name}' Command executed (no output capture):\n{:?}", output); }
                });

                return Task::none();
            }
            else { println!("Empty command vector, no argument was parsed"); }
        }

        Message::TrayEvent(event) =>
        {
            match event
            {
                TrayEvent::ItemRegistered(service) =>
                {
                    if !app.modules_data.tray_icons.iter().any(|(_, s)| s == &service)
                    {
                        app.modules_data.tray_icons.push((None, service));
                    }
                }

                TrayEvent::ItemUnregistered(service) =>
                {
                    println!("\n=== Tray item Unregistered ===\n{service}");
                    app.modules_data.tray_icons.retain(|(_, s)| s != &service);
                }

                TrayEvent::Icon { combined, data, width, height } =>
                {
                    if let Some((handle, _)) = app.modules_data.tray_icons.iter_mut().find(|(_, s)| s == &combined)
                    {
                        *handle = Some(image::Handle::from_rgba(width, height, data));
                    }
                }

                TrayEvent::AttentionIcon { combined, data, width, height } =>
                {
                    if let Some((handle, _)) = app.modules_data.tray_icons.iter_mut().find(|(_, s)| s == &combined)
                    {
                        *handle = Some(image::Handle::from_rgba(width, height, data));
                    }
                }

                TrayEvent::IconRestored(combined) =>
                {
                    return Task::perform(
                        async move
                        {
                            let conn = zbus::Connection::session().await.ok()?;
                            crate::helpers::icons::fetch_icon(&conn, &combined).await.ok()
                        },
                        |maybe_event|
                        {
                            match maybe_event
                            {
                                Some(event) => Message::TrayEvent(event),
                                None        => Message::Nothing
                            }
                        }
                    );
                }
            }
        }

        Message::TrayIconClicked(combined) =>
        {
            println!("TrayIcon Clicked");
            let parts: Vec<&str> = combined.split('|').collect();
            if parts.len() != 2 { return Task::none(); }
            let service = parts[0].to_string();
            let path    = parts[1].to_string();
            return Task::perform(async move { load_tray_menu(service, path).await }, |result| match result
            {
                Ok((s, p, i)) => Message::MenuLoaded(s, p, i),
                Err(e) =>
                {
                    eprintln!("Failed to load tray menu: {e}");
                    Message::Nothing
                }
            });
        }

        Message::MenuLoaded(service, path, items) =>
        {
            println!("\n===# Menu Loaded!!! #===");
            println!("Service: {service}");
            println!("Menu Path: {path}");
            println!("Id: {:?}\n", items);
            let context_menu_data = crate::context_menu::ContextMenuData
            {
                mouse_position:        app.context_menu_data.mouse_position,
                default_font:          app.default_font,
                cursor_is_inside_menu: false,
                context_menu_is_open:  true,
                service,
                items,
                path
            };
            app.context_menu_data = context_menu_data;

            return create_context_menu(app);
        }

        Message::ShowCalendar =>
        {
            let already_open = app.ids.values().any(|v| *v == WindowInfo::Calendar);
            if already_open
            {
                app.modules_data.calendar_data.is_open = false;
                let ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Calendar).map(|(id, _)| *id).collect();
                for id in &ids_to_close { app.ids.remove(id); }
                return Task::batch(ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }
            app.modules_data.calendar_data.is_open       = true;
            app.modules_data.calendar_data.cursor_inside = false;
            return create_calendar_window(app);
        }

        Message::ShowVolumeOutputMixer =>
        {
            let already_open = app.ids.values().any(|v| *v == WindowInfo::VolumeOutputMixer);
            if already_open
            {
                app.modules_data.volume_mixer_data.output_mixer_open = false;
                let ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeOutputMixer).map(|(id, _)| *id).collect();
                for id in &ids { app.ids.remove(id); }
                return Task::batch(ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }
            let vmd = crate::volume_mixer::VolumeMixerData::from_config(
                &app.ron_config.volume_output_mixer,
                &app.ron_config.volume_input_mixer
            );
            app.modules_data.volume_mixer_data.output_device_cat_open = vmd.output_device_cat_open;
            app.modules_data.volume_mixer_data.output_app_cat_open    = vmd.output_app_cat_open;
            app.modules_data.volume_mixer_data.output_mixer_open      = true;
            app.modules_data.volume_mixer_data.output_cursor_inside   = false;
            return create_output_mixer_window(app);
        }

        Message::ShowVolumeInputMixer =>
        {
            let already_open = app.ids.values().any(|v| *v == WindowInfo::VolumeInputMixer);
            if already_open
            {
                app.modules_data.volume_mixer_data.input_mixer_open = false;
                let ids: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::VolumeInputMixer).map(|(id, _)| *id).collect();
                for id in &ids { app.ids.remove(id); }
                return Task::batch(ids.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }
            let vmd = crate::volume_mixer::VolumeMixerData::from_config(
                &app.ron_config.volume_output_mixer,
                &app.ron_config.volume_input_mixer
            );
            app.modules_data.volume_mixer_data.input_device_cat_open = vmd.input_device_cat_open;
            app.modules_data.volume_mixer_data.input_app_cat_open    = vmd.input_app_cat_open;
            app.modules_data.volume_mixer_data.input_mixer_open      = true;
            app.modules_data.volume_mixer_data.input_cursor_inside   = false;
            return create_input_mixer_window(app);
        }

        Message::MixerStateUpdated(new_state) =>
        {
            app.modules_data.mixer_state = new_state;
        }

        Message::SetDeviceVolume(kind, index, pct)  => { return task_set_device_volume(kind, index, pct); }
        Message::ToggleDeviceMute(kind, index)       => { return task_toggle_device_mute(kind, index); }
        Message::SetDefaultDevice(kind, name)        => { return task_set_default_device(kind, name); }
        Message::SetAppVolume(kind, index, pct)      => { return task_set_app_volume(kind, index, pct); }
        Message::ToggleAppMute(kind, index)          => { return task_toggle_app_mute(kind, index); }

        Message::ToggleOutputDeviceCategory =>
        {
            let current = app.modules_data.volume_mixer_data.output_device_cat_open;
            app.modules_data.volume_mixer_data.output_device_cat_open = !current;
        }

        Message::ToggleInputDeviceCategory =>
        {
            let current = app.modules_data.volume_mixer_data.input_device_cat_open;
            app.modules_data.volume_mixer_data.input_device_cat_open = !current;
        }

        Message::ToggleOutputAppCategory =>
        {
            let current = app.modules_data.volume_mixer_data.output_app_cat_open;
            app.modules_data.volume_mixer_data.output_app_cat_open = !current;
        }

        Message::ToggleInputAppCategory =>
        {
            let current = app.modules_data.volume_mixer_data.input_app_cat_open;
            app.modules_data.volume_mixer_data.input_app_cat_open = !current;
        }

        Message::CalendarPrev =>
        {
            match app.modules_data.calendar_data.current_view
            {
                CalendarView::Month =>
                {
                    let d = app.modules_data.calendar_data.viewing_month;
                    let (y, m) = if d.month() == 1 { (d.year() - 1, 12u32) } else { (d.year(), d.month() - 1) };
                    if let Some(nd) = chrono::NaiveDate::from_ymd_opt(y, m, 1)
                    {
                        app.modules_data.calendar_data.viewing_month = nd;
                    }
                }
                CalendarView::Year =>
                {
                    app.modules_data.calendar_data.viewing_year -= 1;
                }
                CalendarView::Decade =>
                {
                    app.modules_data.calendar_data.viewing_decade -= 10;
                }
            }
        }

        Message::CalendarNext =>
        {
            match app.modules_data.calendar_data.current_view
            {
                CalendarView::Month =>
                {
                    let d = app.modules_data.calendar_data.viewing_month;
                    let (y, m) = if d.month() == 12 { (d.year() + 1, 1u32) } else { (d.year(), d.month() + 1) };
                    if let Some(nd) = chrono::NaiveDate::from_ymd_opt(y, m, 1)
                    {
                        app.modules_data.calendar_data.viewing_month = nd;
                    }
                }
                CalendarView::Year =>
                {
                    app.modules_data.calendar_data.viewing_year += 1;
                }
                CalendarView::Decade =>
                {
                    app.modules_data.calendar_data.viewing_decade += 10;
                }
            }
        }

        Message::CalendarSetView(view) =>
        {
            match view
            {
                CalendarView::Year =>
                {
                    app.modules_data.calendar_data.viewing_year =
                        app.modules_data.calendar_data.viewing_month.year();
                }
                CalendarView::Decade =>
                {
                    let y = app.modules_data.calendar_data.viewing_year;
                    app.modules_data.calendar_data.viewing_decade = (y / 10) * 10;
                }
                _ => {}
            }
            app.modules_data.calendar_data.current_view = view;
        }

        Message::CalendarMonthSelected(month) =>
        {
            let year = app.modules_data.calendar_data.viewing_year;
            if let Some(nd) = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            {
                app.modules_data.calendar_data.viewing_month = nd;
            }
            app.modules_data.calendar_data.current_view = CalendarView::Month;
        }

        Message::CalendarYearSelected(year) =>
        {
            app.modules_data.calendar_data.viewing_year  = year;
            app.modules_data.calendar_data.current_view  = CalendarView::Year;
        }

        Message::CalendarDaySelected(day) =>
        {
            let base  = app.modules_data.calendar_data.viewing_month;
            let dated = chrono::NaiveDate::from_ymd_opt(base.year(), base.month(), day);

            match &app.ron_config.calendar_window.calendar_day_click_action.clone()
            {
                DayClickAction::HighlightOnly =>
                {
                    app.modules_data.calendar_data.selected_day = dated;
                }
                DayClickAction::CustomAction(cmd) =>
                {
                    app.modules_data.calendar_data.selected_day = dated;
                    if !cmd.is_empty()
                    {
                        let date_str = dated.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default();
                        let expanded: Vec<String> = cmd.iter().map(|part| part.replace("{date}", &date_str)).collect();
                        return Task::done(Message::CreateCustomModuleCommand(
                            (None, expanded, "Calendar Day Action".to_string(), true, false)
                        ));
                    }
                }
            }
        }

        Message::BarEnter =>
        {
            app.cursor_on_bar = true;
            app.hide_timer    = None;

            if let Some(cfg) = app.ron_config.auto_hide.as_ref() && !app.bar_visible
            {
                if cfg.show_delay_ms == 0
                {
                    app.bar_visible = true;
                    app.show_timer  = None;
                    return Task::batch(show_bar(app));
                }
                else if app.show_timer.is_none()
                {
                    app.show_timer = Some(Instant::now());
                }
            }
        }

        Message::BarLeave =>
        {
            app.cursor_on_bar = false;
            app.show_timer    = None;

            let popup_open = app.context_menu_data.context_menu_is_open
                || app.modules_data.calendar_data.is_open
                || app.modules_data.volume_mixer_data.output_mixer_open
                || app.modules_data.volume_mixer_data.input_mixer_open;

            if app.ron_config.auto_hide.is_some() && app.bar_visible && !popup_open
            {
                app.hide_timer = Some(Instant::now());
            }
        }

        Message::BarVisibilityTick =>
        {
            let (hide_delay_ms, show_delay_ms, peek_size) = match app.ron_config.auto_hide.as_ref()
            {
                Some(cfg) => (cfg.hide_delay_ms, cfg.show_delay_ms, cfg.peek_size),
                None      => return Task::none()
            };
            let mut tasks: Vec<Task<Message>> = Vec::new();

            if let Some(t) = app.hide_timer && t.elapsed() >= Duration::from_millis(hide_delay_ms)
            {
                app.hide_timer  = None;
                app.bar_visible = false;
                tasks.extend(hide_bar(app, peek_size));
            }

            if let Some(t) = app.show_timer && t.elapsed() >= Duration::from_millis(show_delay_ms)
            {
                app.show_timer  = None;
                app.bar_visible = true;
                tasks.extend(show_bar(app));
            }

            if !tasks.is_empty() { return Task::batch(tasks); }
        }

        _ => {}
    }

    Task::none()
}



fn maybe_restart_hide_timer(app: &mut AppData)
{
    if app.cursor_on_bar { return; }
    if app.ron_config.auto_hide.is_none() { return; }
    if !app.bar_visible { return; }

    let any_popup_still_open = app.context_menu_data.context_menu_is_open
        || app.modules_data.calendar_data.is_open
        || app.modules_data.volume_mixer_data.output_mixer_open
        || app.modules_data.volume_mixer_data.input_mixer_open;

    if !any_popup_still_open && app.hide_timer.is_none()
    {
        app.hide_timer = Some(Instant::now());
    }
}



fn hidden_exclusive_zone(peek: i32) -> i32 { peek.max(0) }



fn shown_exclusive_zone(app: &AppData) -> i32
{
    let cfg = &app.ron_config.general;
    use crate::ron::BarPosition;
    let base = match cfg.bar_position
    {
        BarPosition::Up | BarPosition::Down    => cfg.bar_size[1] as i32,
        BarPosition::Left | BarPosition::Right => cfg.bar_size[0] as i32
    };
    (base + cfg.increased_exclusive_bar_zone).max(0)
}



fn hide_bar(app: &AppData, peek: i32) -> Vec<Task<Message>>
{
    let Some(&id) = MAIN_ID.get() else { return vec![]; };
    use crate::ron::BarPosition;

    let bar_thick = match app.ron_config.general.bar_position
    {
        BarPosition::Up | BarPosition::Down    => app.ron_config.general.bar_size[1] as i32,
        BarPosition::Left | BarPosition::Right => app.ron_config.general.bar_size[0] as i32
    };
    let slide  = -(bar_thick - peek);
    let base   = app.ron_config.general.floating_space;
    let margin = match app.ron_config.general.bar_position
    {
        BarPosition::Up    => (base + slide, 0, 0, 0),
        BarPosition::Down  => (0, 0, base + slide, 0),
        BarPosition::Left  => (0, 0, 0, base + slide),
        BarPosition::Right => (0, base + slide, 0, 0)
    };
    vec![
        Task::done(Message::ExclusiveZoneChange { id, zone_size: hidden_exclusive_zone(peek) }),
        Task::done(Message::MarginChange        { id, margin })
    ]
}



fn show_bar(app: &AppData) -> Vec<Task<Message>>
{
    let Some(&id) = MAIN_ID.get() else { return vec![]; };
    use crate::ron::BarPosition;

    let base   = app.ron_config.general.floating_space;
    let margin = match app.ron_config.general.bar_position
    {
        BarPosition::Up    => (base, 0, 0, 0),
        BarPosition::Down  => (0, 0, base, 0),
        BarPosition::Left  => (0, 0, 0, base),
        BarPosition::Right => (0, base, 0, 0)
    };
    vec![
        Task::done(Message::ExclusiveZoneChange { id, zone_size: shown_exclusive_zone(app) }),
        Task::done(Message::MarginChange        { id, margin })
    ]
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::modules::network::NetworkData;
    use crate::modules::tray::TrayEvent;

    fn make_app() -> AppData { AppData { ..Default::default() } }


    #[test]
    fn message_is_hovering_volume_output_sets_flag_true()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringVolumeOutput(true));
        assert!(app.modules_data.volume_data.is_hovering_volume_output);
    }

    #[test]
    fn message_is_hovering_volume_output_sets_flag_false()
    {
        let mut app = make_app();
        app.modules_data.volume_data.is_hovering_volume_output = true;
        let _ = update(&mut app, Message::IsHoveringVolumeOutput(false));
        assert!(!app.modules_data.volume_data.is_hovering_volume_output);
    }

    #[test]
    fn message_is_hovering_volume_input_sets_flag()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringVolumeInput(true));
        assert!(app.modules_data.volume_data.is_hovering_volume_input);
    }

    #[test]
    fn message_is_hovering_workspace_sets_flag()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringWorkspace(true));
        assert!(app.modules_data.workspace_data.is_hovering_workspace);
    }

    #[test]
    fn message_is_hovering_media_player_sets_flag()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringMediaPlayerMetaData(true));
        assert!(app.modules_data.media_player_data.is_hovering_media_player_meta_data);
    }


    #[test]
    fn toggle_alt_clock_flips_from_false_to_true()
    {
        let mut app = make_app();
        assert!(!app.modules_data.clock_data.is_showing_alt_clock);
        let _ = update(&mut app, Message::ToggleAltClock);
        assert!(app.modules_data.clock_data.is_showing_alt_clock);
    }

    #[test]
    fn toggle_alt_clock_flips_back_on_second_call()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::ToggleAltClock);
        let _ = update(&mut app, Message::ToggleAltClock);
        assert!(!app.modules_data.clock_data.is_showing_alt_clock);
    }


    #[test]
    fn toggle_alt_network_flips_flag()
    {
        let mut app = make_app();
        assert!(!app.modules_data.network_data.is_showing_alt_network_module);
        let _ = update(&mut app, Message::ToggleAltNetwork);
        assert!(app.modules_data.network_data.is_showing_alt_network_module);
    }

    #[test]
    fn toggle_alt_network_swaps_to_alt_icons()
    {
        let mut app = make_app();
        app.ron_config.alt_network.alt_network_level_format           = ["A".into(), "B".into(), "C".into(), "D".into()];
        app.ron_config.alt_network.alt_network_connection_type_icons  = ["X".into(), "Y".into(), "Z".into()];

        let _ = update(&mut app, Message::ToggleAltNetwork);

        assert_eq!(app.modules_data.network_data.network_icons,         ["A", "B", "C", "D"]);
        assert_eq!(app.modules_data.network_data.connection_type_icons, ["X", "Y", "Z"]);
    }

    #[test]
    fn toggle_alt_network_swaps_back_to_normal_icons()
    {
        let mut app = make_app();
        app.ron_config.network.network_level_format                   = ["N1".into(), "N2".into(), "N3".into(), "N4".into()];
        app.ron_config.network.network_connection_type_icons          = ["E".into(), "W".into(), "?".into()];
        app.ron_config.alt_network.alt_network_level_format           = ["A".into(), "B".into(), "C".into(), "D".into()];
        app.ron_config.alt_network.alt_network_connection_type_icons  = ["X".into(), "Y".into(), "Z".into()];

        let _ = update(&mut app, Message::ToggleAltNetwork);
        let _ = update(&mut app, Message::ToggleAltNetwork);

        assert_eq!(app.modules_data.network_data.network_icons,         ["N1", "N2", "N3", "N4"]);
        assert_eq!(app.modules_data.network_data.connection_type_icons, ["E", "W", "?"]);
    }


    #[test]
    fn cursor_moved_updates_mouse_position()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 123.7, y: 456.2 }));
        assert_eq!(app.context_menu_data.mouse_position, (123, 456));
    }

    #[test]
    fn cursor_moved_truncates_not_rounds()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 99.9, y: 99.9 }));
        assert_eq!(app.context_menu_data.mouse_position, (99, 99));
    }


    #[test]
    fn command_finished_stores_output_at_index()
    {
        let mut app = make_app();
        app.modules_data.custom_module_data.cached_command_outputs = vec!["old".into(), "old".into()];
        let _ = update(&mut app, Message::CommandFinished(1, "new_output".into()));
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[1], "new_output");
    }

    #[test]
    fn command_finished_resizes_vec_if_index_out_of_bounds()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::CommandFinished(3, "hello".into()));
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs.len(), 4);
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[3], "hello");
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[0], "");
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[2], "");
    }

    #[test]
    fn command_finished_index_zero_works()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::CommandFinished(0, "result".into()));
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[0], "result");
    }


    #[test]
    fn network_updated_stores_data()
    {
        let mut app = make_app();
        let data = NetworkData { network_level: 4, connection_type: 2, network_speed: 100, id: "HomeWifi".into(), rx_bytes_per_sec: 0, tx_bytes_per_sec: 0, iface: String::new(), ..Default::default() };
        let _ = update(&mut app, Message::NetworkUpdated(data));
        assert_eq!(app.modules_data.network_data.id,            "HomeWifi");
        assert_eq!(app.modules_data.network_data.network_level, 4);
        assert_eq!(app.modules_data.network_data.network_speed, 100);
    }


    #[test]
    fn tray_item_registered_adds_to_list()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemRegistered("service|/path".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 1);
        assert_eq!(app.modules_data.tray_icons[0].1, "service|/path");
        assert!(app.modules_data.tray_icons[0].0.is_none());
    }

    #[test]
    fn tray_item_registered_does_not_duplicate()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemRegistered("svc|/path".into())));
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemRegistered("svc|/path".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 1);
    }

    #[test]
    fn tray_item_registered_allows_different_services()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemRegistered("svc1|/path".into())));
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemRegistered("svc2|/path".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 2);
    }


    #[test]
    fn tray_item_unregistered_removes_from_list()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![(None, "svc1|/p".into()), (None, "svc2|/p".into())];
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemUnregistered("svc1|/p".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 1);
        assert_eq!(app.modules_data.tray_icons[0].1, "svc2|/p");
    }

    #[test]
    fn tray_item_unregistered_nonexistent_service_does_nothing()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![(None, "svc1|/p".into())];
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemUnregistered("ghost|/p".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 1);
    }

    #[test]
    fn tray_item_unregistered_empties_list()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![(None, "only|/p".into())];
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemUnregistered("only|/p".into())));
        assert!(app.modules_data.tray_icons.is_empty());
    }


    #[test]
    fn message_nothing_does_not_change_state()
    {
        let mut app = make_app();
        let before = app.modules_data.clock_data.is_showing_alt_clock;
        let _ = update(&mut app, Message::Nothing);
        assert_eq!(app.modules_data.clock_data.is_showing_alt_clock, before);
    }

    #[test]
    fn toggle_alt_clock_and_alt_network_are_independent()
    {
        let mut app = AppData { ..Default::default() };
        let _ = update(&mut app, Message::ToggleAltClock);
        assert!(app.modules_data.clock_data.is_showing_alt_clock);
        assert!(!app.modules_data.network_data.is_showing_alt_network_module);

        let _ = update(&mut app, Message::ToggleAltNetwork);
        assert!(app.modules_data.clock_data.is_showing_alt_clock);
        assert!(app.modules_data.network_data.is_showing_alt_network_module);
    }

    #[test]
    fn multiple_command_finished_messages_stored_independently()
    {
        let mut app = AppData { ..Default::default() };
        let _ = update(&mut app, Message::CommandFinished(0, "out0".into()));
        let _ = update(&mut app, Message::CommandFinished(1, "out1".into()));
        let _ = update(&mut app, Message::CommandFinished(2, "out2".into()));

        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[0], "out0");
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[1], "out1");
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[2], "out2");
    }

    #[test]
    fn overwriting_command_output_replaces_not_appends()
    {
        let mut app = AppData { ..Default::default() };
        let _ = update(&mut app, Message::CommandFinished(0, "first".into()));
        let _ = update(&mut app, Message::CommandFinished(0, "second".into()));
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[0], "second");
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs.len(), 1);
    }

    #[test]
    fn cursor_moved_multiple_times_keeps_last_position()
    {
        let mut app = AppData { ..Default::default() };
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 10.0, y: 20.0 }));
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 300.0, y: 400.0 }));
        assert_eq!(app.context_menu_data.mouse_position, (300, 400));
    }

    #[test]
    fn cycle_clock_timezones_message_advances_timezone()
    {
        let mut app = make_app();
        app.ron_config.clock.clock_timezones             = Some(vec!["UTC".into(), "America/New_York".into()]);
        app.modules_data.clock_data.current_clock_timezone = Some(("UTC".into(), 0));

        let _ = update(&mut app, Message::CycleClockTimeZones);

        let (tz, idx) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "America/New_York");
        assert_eq!(idx, 1);
    }

    #[test]
    fn cycle_clock_timezones_message_wraps_at_end()
    {
        let mut app = make_app();
        app.ron_config.clock.clock_timezones             = Some(vec!["UTC".into(), "Europe/London".into()]);
        app.modules_data.clock_data.current_clock_timezone = Some(("Europe/London".into(), 1));

        let _ = update(&mut app, Message::CycleClockTimeZones);

        let (tz, idx) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }

    #[test]
    fn cycle_clock_timezones_message_with_no_timezones_configured_does_nothing()
    {
        let mut app = make_app();
        app.ron_config.clock.clock_timezones             = None;
        app.modules_data.clock_data.current_clock_timezone = Some(("UTC".into(), 0));

        let _ = update(&mut app, Message::CycleClockTimeZones);

        let (tz, idx) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }


    #[test]
    fn toggle_alt_clock_and_cycle_flips_alt_clock_flag()
    {
        let mut app = make_app();
        assert!(!app.modules_data.clock_data.is_showing_alt_clock);

        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);

        assert!(app.modules_data.clock_data.is_showing_alt_clock);
    }

    #[test]
    fn toggle_alt_clock_and_cycle_also_cycles_timezone()
    {
        let mut app = make_app();
        app.ron_config.clock.clock_timezones             = Some(vec!["UTC".into(), "Asia/Tokyo".into()]);
        app.modules_data.clock_data.current_clock_timezone = Some(("UTC".into(), 0));

        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);

        assert!(app.modules_data.clock_data.is_showing_alt_clock);
        let (tz, _) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "Asia/Tokyo");
    }

    #[test]
    fn toggle_alt_clock_and_cycle_called_twice_restores_flag_and_wraps_timezone()
    {
        let mut app = make_app();
        app.ron_config.clock.clock_timezones             = Some(vec!["UTC".into(), "Asia/Tokyo".into()]);
        app.modules_data.clock_data.current_clock_timezone = Some(("UTC".into(), 0));

        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);

        assert!(!app.modules_data.clock_data.is_showing_alt_clock);
        let (tz, idx) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }

    #[test]
    fn toggle_alt_clock_and_cycle_with_no_timezones_still_toggles_flag()
    {
        let mut app = make_app();
        app.ron_config.clock.clock_timezones             = None;
        app.modules_data.clock_data.current_clock_timezone = None;

        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);

        assert!(app.modules_data.clock_data.is_showing_alt_clock);
    }

    #[test]
    fn tray_icon_assigned_to_correct_service()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![
            (None, "svc1|/path".into()),
            (None, "svc2|/path".into())
        ];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon
        {
            combined: "svc2|/path".into(),
            data:     vec![0u8; 4],
            width:    1,
            height:   1
        }));

        assert!(app.modules_data.tray_icons[0].0.is_none());
        assert!(app.modules_data.tray_icons[1].0.is_some());
    }

    #[test]
    fn tray_icon_unknown_combined_does_nothing()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![(None, "svc1|/path".into())];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon
        {
            combined: "ghost|/path".into(),
            data:     vec![0u8; 4],
            width:    1,
            height:   1
        }));

        assert!(app.modules_data.tray_icons[0].0.is_none());
    }

    #[test]
    fn tray_icon_updates_existing_handle()
    {
        let mut app = make_app();
        let old_handle = Some(image::Handle::from_rgba(1, 1, vec![255u8; 4]));
        app.modules_data.tray_icons = vec![(old_handle, "svc1|/path".into())];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon
        {
            combined: "svc1|/path".into(),
            data:     vec![0u8; 4],
            width:    1,
            height:   1
        }));

        assert!(app.modules_data.tray_icons[0].0.is_some());
    }

    #[test]
    fn tray_icon_only_affects_matched_service()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![
            (None, "svc1|/path".into()),
            (None, "svc2|/path".into()),
            (None, "svc3|/path".into())
        ];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon
        {
            combined: "svc2|/path".into(),
            data:     vec![0u8; 4],
            width:    1,
            height:   1
        }));

        assert!(app.modules_data.tray_icons[0].0.is_none());
        assert!(app.modules_data.tray_icons[1].0.is_some());
        assert!(app.modules_data.tray_icons[2].0.is_none());
    }


    #[test]
    fn attention_icon_overwrites_existing_handle()
    {
        let mut app = AppData::default();
        app.modules_data.tray_icons = vec![(None, "svc|/path".into())];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::AttentionIcon
        {
            combined: "svc|/path".into(),
            data:     vec![255u8; 4],
            width:    1,
            height:   1
        }));

        assert!(app.modules_data.tray_icons[0].0.is_some());
    }

    #[test]
    fn attention_icon_unknown_combined_does_not_panic()
    {
        let mut app = AppData::default();
        app.modules_data.tray_icons = vec![(None, "svc|/path".into())];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::AttentionIcon
        {
            combined: "other|/path".into(),
            data:     vec![0u8; 4],
            width:    1,
            height:   1
        }));

        assert!(app.modules_data.tray_icons[0].0.is_none());
    }


    #[test]
    fn icon_restored_returns_task_for_known_item()
    {
        let mut app = AppData::default();
        app.modules_data.tray_icons = vec![(None, "svc|/path".into())];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::IconRestored("svc|/path".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 1);
    }

    #[test]
    fn icon_restored_unknown_combined_does_not_panic()
    {
        let mut app = AppData::default();
        app.modules_data.tray_icons = vec![(None, "svc|/path".into())];

        let _ = update(&mut app, Message::TrayEvent(TrayEvent::IconRestored("ghost|/path".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 1);
    }
}
