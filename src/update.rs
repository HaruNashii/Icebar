// ============ IMPORTS ============
use iced::{Task, mouse::ScrollDelta, widget::image};
use iced_layershell::to_layer_message;
use std::time::{Duration, Instant};
use std::sync::Once;




// ============ STATICS ============
static WARNING_ONCE: Once = Once::new();





// ============ CRATES ============
use crate::modules::focused_window::{read_focused_window_hypr, read_focused_window_sway, read_focused_window_niri, };
use crate::helpers::string::{format_input_volume, format_output_volume};
use crate::modules::cpu_temp::read_cpu_temp;
use crate::modules::ram::read_ram_data;
use crate::modules::{clock::cycle_clock_timezones, cpu::{compute_cpu_usage, read_cpu_snapshot}};
use crate::{helpers::{font::build_font, fs::check_if_config_file_exists, monitor::get_monitor_res}, modules::{clock::get_current_time, data::{Modules, ModulesData}, hypr::{self, change_workspace_hypr}, media_player::{MediaPlayerAction, get_player_data_with_format, media_player_action}, network::NetworkData, niri::{self, change_workspace_niri}, sway::{self, change_workspace_sway}, tray::{MenuItem, TrayEvent}, volume, workspaces::UserWorkspaceAction }};
use crate::helpers::{misc::{is_active_module, validade_bar_data}, workspaces::build_workspace_list };
use crate::context_menu::run_context_menu;
use crate::ron::read_ron_config;
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message
{
    CreateCustomModuleCommand((Option<usize>, Vec<String>, String, bool, bool)),
    MenuLoaded(String, String, Vec<MenuItem>),
    ToggleAltClockAndCycleClockTimeZones,
    IsHoveringMediaPlayerMetaData(bool),
    TrayIconClicked(usize),
    MouseWheelScrolled(ScrollDelta),
    CommandFinished(usize, String),
    WorkspaceButtonPressed(usize),
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

    Tick,
    VolumeUpdated(f32, bool, f32, bool),

    UpdateRam,
    UpdateCpu,
    UpdateCpuTemp,
    UpdateFocusedWindowNiri,
    UpdateFocusedWindowSway,
    UpdateFocusedWindowHypr,
    UpdateMediaPlayerMetadata,
    UpdateNiriWorkspaces,
    UpdateSwayWorkspaces,
    UpdateHyprWorkspaces,
    UpdateClock
}





// ============ FUNCTIONS ============
pub fn update(app: &mut AppData, message: Message) -> Task<Message>
{
    match message
    {
        Message::IsHoveringVolumeOutput(bool) => { app.is_hovering_volume_output = bool; }
        Message::IsHoveringVolumeInput(bool) => { app.is_hovering_volume_input = bool; }
        Message::IsHoveringWorkspace(bool) => { app.is_hovering_workspace = bool; }
        Message::IsHoveringMediaPlayerMetaData(bool) => { app.is_hovering_media_player_meta_data = bool; }
        Message::MuteAudioPressedOutput => { volume::volume( volume::VolumeAction::MuteOutput); }
        Message::MuteAudioPressedInput => { volume::volume( volume::VolumeAction::MuteInput); }
        Message::ToggleAltClock => { app.is_showing_alt_clock = !app.is_showing_alt_clock; }
        Message::CursorMoved(point) => 
        { 
            let new_pos = (point.x as i32, point.y as i32);
            if new_pos != app.mouse_position
            {
                app.mouse_position = new_pos;
            }
        }
        Message::CommandFinished(index, text) => { if app.cached_command_outputs.len() <= index { app.cached_command_outputs.resize(index + 1, String::new()); } app.cached_command_outputs[index] = text; }
        Message::WorkspaceButtonPressed(id) => { if is_active_module(&app.modules_data.active_modules,  Modules::HyprWorkspaces) { change_workspace_hypr(UserWorkspaceAction::ChangeWithIndex(id)); } else if is_active_module(&app.modules_data.active_modules, Modules::SwayWorkspaces) { change_workspace_sway(UserWorkspaceAction::ChangeWithIndex(id)); } else if is_active_module(&app.modules_data.active_modules, Modules::NiriWorkspaces) { change_workspace_niri(UserWorkspaceAction::ChangeWithIndex(id)); } }
        Message::NetworkUpdated(data) => { app.modules_data.network_data = data }
        Message::MediaPlayerClickNext => media_player_action(&app.ron_config.player, MediaPlayerAction::Next),
        Message::MediaPlayerClickPlayPause => media_player_action(&app.ron_config.player, MediaPlayerAction::PlayPause),
        Message::MediaPlayerClickPrev => media_player_action(&app.ron_config.player, MediaPlayerAction::Prev),
        Message::CycleClockTimeZones => cycle_clock_timezones(app),
        Message::ToggleAltClockAndCycleClockTimeZones => { app.is_showing_alt_clock = !app.is_showing_alt_clock; cycle_clock_timezones(app); },
        Message::UpdateCpuTemp => if let Some(temp) = read_cpu_temp() { app.modules_data.cpu_temp_data.temp_celsius = temp; }
        Message::UpdateRam => { if let Some(data) = read_ram_data() { app.modules_data.ram_data = data; }},
        Message::UpdateFocusedWindowNiri => { app.modules_data.focused_window_data.title = read_focused_window_niri().unwrap_or_default(); }, 
        Message::UpdateFocusedWindowSway => { app.modules_data.focused_window_data.title = read_focused_window_sway().unwrap_or_default(); },
        Message::UpdateFocusedWindowHypr => { app.modules_data.focused_window_data.title = read_focused_window_hypr().unwrap_or_default(); },
        Message::UpdateHyprWorkspaces => { app.modules_data.workspace_data.current_workspace = hypr::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&hypr::workspace_count(), app.ron_config.persistent_workspaces); },
        Message::UpdateSwayWorkspaces => { app.modules_data.workspace_data.current_workspace = sway::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&sway::workspace_count(), app.ron_config.persistent_workspaces); },
        Message::UpdateMediaPlayerMetadata => { app.modules_data.media_player_data = get_player_data_with_format(&app.ron_config); },

        Message::VolumeUpdated(out_vol, out_muted, in_vol, in_muted) =>
        {
            // Format output
            let (output_str, _) = format_output_volume(out_vol, out_muted, &app.ron_config.output_volume_format, &app.ron_config.output_volume_muted_format);
            app.modules_data.volume_data.output_volume_level = output_str;
            app.volume_output_is_muted = out_muted;
 
            // Format input
            let (input_str, _) = format_input_volume(in_vol, in_muted, &app.ron_config.input_volume_format, &app.ron_config.input_volume_muted_format);
            app.modules_data.volume_data.input_volume_level = input_str;
            app.volume_input_is_muted = in_muted;
        }
 
        Message::UpdateCpu =>
        {
            if let Some(curr) = read_cpu_snapshot()
            {
                if let Some(prev) = &app.cpu_snapshot
                {
                    app.modules_data.cpu_data.usage_percent = compute_cpu_usage(prev, &curr);
                }
                app.cpu_snapshot = Some(curr);
            }
        }

        Message::UpdateNiriWorkspaces =>
        { 
            WARNING_ONCE.call_once(|| 
            {
                if app.ron_config.persistent_workspaces.is_some()
                {
                    println!("\n=== Niri Workspaces Warning ===");
                    for _ in 0..3
                    {
                        println!("Warning!!!: Persistent Elements Defined But Niri Doesn't Support Persistent Workspaces.");
                    }
                    println!("\n");
                }
            });
            app.modules_data.workspace_data.current_workspace = niri::current_workspace(); 
            app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&niri::workspace_count(), None); 
        }

        Message::UpdateClock =>
        {
            let format_to_send = if app.is_showing_alt_clock 
            { 
                &app.ron_config.clock_alt_format 
            } 
            else 
            {
                &app.ron_config.clock_format 
            }; 
            app.modules_data.clock_data.current_time = get_current_time(format_to_send, &app.current_clock_timezone)
        },

        Message::ToggleAltNetwork => 
        { 
            app.is_showing_alt_network_module = !app.is_showing_alt_network_module; 
            if app.is_showing_alt_network_module 
            { 
                app.connection_type_icons = app.ron_config.alt_network_connection_type_icons.clone();
                app.network_icons = app.ron_config.alt_network_level_format.clone();
            }
            else 
            {
                app.connection_type_icons = app.ron_config.network_connection_type_icons.clone();
                app.network_icons = app.ron_config.network_level_format.clone();
            };
        }

        Message::ConfigChanged =>
        {
            println!("\n=== CONFIG RELOAD ===");
            println!("[icebar] config.ron changed — reloading in place...");
            check_if_config_file_exists();
            let (new_config, new_anchor, current_clock_timezone, active_modules) = read_ron_config();
            let bar_data_validated = validade_bar_data(&new_config);
            let mut bar_size = bar_data_validated.bar_size;
            let monitor_res = get_monitor_res(new_config.display.clone());
            if bar_size.0 == 0 { bar_size.0 = monitor_res.0; };
            if bar_size.1 == 0 { bar_size.1 = monitor_res.1; };
            let font_name = new_config.font_family.clone();
            let modules_data = ModulesData 
            {
                focused_window_data: app.modules_data.focused_window_data.clone(),
                cpu_data: app.modules_data.cpu_data.clone(),
                ram_data: app.modules_data.ram_data.clone(),
                media_player_data: app.modules_data.media_player_data.clone(),
                workspace_data: app.modules_data.workspace_data.clone(),
                cpu_temp_data: app.modules_data.cpu_temp_data.clone(),
                network_data: app.modules_data.network_data.clone(),
                volume_data: app.modules_data.volume_data.clone(),
                tray_icons: app.modules_data.tray_icons.clone(),
                clock_data: app.modules_data.clock_data.clone(),
                active_modules: active_modules.clone(),
            };
            *app = AppData
            {
                default_font: build_font(&font_name, &new_config.font_style),
                monitor_size: monitor_res,
                custom_module_last_run: vec![Instant::now(); new_config.custom_modules.len()],
                current_clock_timezone,
                network_icons: new_config.network_level_format.clone(),
                connection_type_icons: new_config.network_connection_type_icons.clone(),
                ron_config: new_config, 
                mouse_position: app.mouse_position,
                modules_data,
                ..Default::default()
            };

            println!("\n=== CONFIG RELOAD ===");
            println!("Reloaded Successfully");
            return Task::batch(vec!
            [
                Task::done(Message::SizeChange(bar_size)),
                Task::done(Message::AnchorChange(new_anchor)),
                Task::done(Message::MarginChange(bar_data_validated.floating_space)),
                Task::done(Message::ExclusiveZoneChange(bar_data_validated.exclusive_zone)),
            ]);
        }

        Message::MouseWheelScrolled(ScrollDelta::Pixels { x: _, y }) =>
        {
            if app.is_hovering_media_player_meta_data
            {
                if y > 2. { media_player_action(&app.ron_config.player, MediaPlayerAction::VolumeUp); }
                if y < 2. { media_player_action(&app.ron_config.player, MediaPlayerAction::VolumeDown); }
            }

            if app.is_hovering_volume_output
            {
                if y > 2. { volume::volume(volume::VolumeAction::IncreaseOutput(app.ron_config.incremental_steps_output)); }
                if y < 2. { volume::volume(volume::VolumeAction::DecreaseOutput(app.ron_config.incremental_steps_output)); }
            }

            if app.is_hovering_volume_input
            {
                if y > 2. { volume::volume(volume::VolumeAction::IncreaseInput(app.ron_config.incremental_steps_input)); }
                if y < 2. { volume::volume(volume::VolumeAction::DecreaseInput(app.ron_config.incremental_steps_input)); }
            }

            if app.is_hovering_workspace
            {
                let hypr_active = is_active_module(&app.modules_data.active_modules, Modules::HyprWorkspaces);
                let sway_active = is_active_module(&app.modules_data.active_modules, Modules::SwayWorkspaces);
                let niri_active = is_active_module(&app.modules_data.active_modules, Modules::NiriWorkspaces);

                // === SCROLL UP ===
                if y > 2. 
                { 
                    if app.ron_config.reverse_scroll_on_workspace
                    {
                        if hypr_active
                        {
                            change_workspace_hypr(UserWorkspaceAction::MoveNext);
                        } 
                        else if sway_active
                        {
                            change_workspace_sway(UserWorkspaceAction::MoveNext);
                        }
                        else if niri_active
                        {
                            change_workspace_niri(UserWorkspaceAction::MoveNext);
                        };

                    }
                    else if hypr_active
                    {
                        change_workspace_hypr(UserWorkspaceAction::MovePrev);
                    } 
                    else if sway_active
                    {
                        change_workspace_sway(UserWorkspaceAction::MovePrev);
                    }
                    else if niri_active
                    {
                        change_workspace_niri(UserWorkspaceAction::MovePrev);
                    };
                }

                // === SCROLL DOWN ===
                if y < 2. 
                { 
                    if app.ron_config.reverse_scroll_on_workspace
                    {
                        if hypr_active
                        {
                            change_workspace_hypr(UserWorkspaceAction::MovePrev);
                        } 
                        else if sway_active
                        {
                            change_workspace_sway(UserWorkspaceAction::MovePrev);
                        }
                        else if niri_active
                        {
                            change_workspace_niri(UserWorkspaceAction::MovePrev);
                        };
                    }
                    else if hypr_active
                    {
                        change_workspace_hypr(UserWorkspaceAction::MoveNext);
                    } 
                    else if sway_active
                    {
                        change_workspace_sway(UserWorkspaceAction::MoveNext);
                    }
                    else if niri_active
                    {
                        change_workspace_niri(UserWorkspaceAction::MoveNext);
                    };
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
                    let index  = *index;
                    let module = &app.ron_config.custom_modules[index];
                    if module.continous_command.is_empty() { continue; }
                    if app.custom_module_last_run[index].elapsed() < Duration::from_millis(module.continous_command_interval) { continue; }
                    app.custom_module_last_run[index] = Instant::now();
        
                    if let Some((program, args)) = module.continous_command.split_first()
                    {
                        let program     = program.clone();
                        let args        = args.to_vec();
                        let display_err = module.display_err_output_if_failed;
        
                        tasks.push(Task::perform(
                            async move {
                                let out = tokio::process::Command::new(program).args(args).output().await.ok();
                                out.map(|o| {
                                    if o.stdout.is_empty() && display_err { String::from_utf8_lossy(&o.stderr).into() }
                                    else { String::from_utf8_lossy(&o.stdout).into() }
                                }).unwrap_or_default()
                            },
                            move |text| Message::CommandFinished(index, text),
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
                let args = args.to_vec();
        
                println!("\n=== Custom Module ===");
                if custom_name.is_empty() {if is_left_click { println!("Custom Module Button Was *Left* Clicked!!"); } else { println!("Custom Module Button Was *Right* Clicked!!"); } } else if is_left_click { println!("Your '{custom_name}' Button Was *Left* Clicked!!"); } else { println!("Your '{custom_name}' Button Was *Right* Clicked!!"); }
        

                // ==============================
                // OUTPUT USED → async + message
                // ==============================
                if output_as_text 
                {
                    return Task::perform
                    (async move 
                        {
                                let output = tokio::process::Command::new(program).args(args).output().await.ok();
                                if custom_name.is_empty() { println!("Custom Module Output:\n{:?}", output); } else { println!( "'{custom_name}' Command Was Running!!!, The Output Was:\n{:?}", output); }
                                output.map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default()
                        },
                        move |text| { Message::CommandFinished(output_index.unwrap_or(0), text) },
                    );
                }
        

                // ==============================
                // FIRE & FORGET → no message
                // ==============================
                tokio::spawn(async move 
                {
                    let output = tokio::process::Command::new(program).args(args).output().await;
                    if custom_name.is_empty() { println!("Custom Module Output:\n{:?}", output); } else { println!( "'{custom_name}' Command executed (no output capture):\n{:?}", output); }
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

                TrayEvent::Icon { data, width, height } =>
                {
                    for (handle, _) in &mut app.modules_data.tray_icons
                    {
                        if handle.is_none()
                        {
                            *handle = Some(image::Handle::from_rgba(width, height, data.clone()));
                            return Task::none();
                        }
                    }
                }

            }
        }



        Message::TrayIconClicked(idx) =>
        {
            println!("TrayIcon Clicked");
            if let Some((_, combined)) = app.modules_data.tray_icons.get(idx)
            {
                let parts: Vec<&str> = combined.split('|').collect();
                if parts.len() != 2 { return Task::none(); }
                let service = parts[0].to_string();
                let path = parts[1].to_string();
                return Task::perform(async move 
                {
                    let conn = zbus::Connection::session().await.unwrap();
                    let proxy: zbus::Proxy<'_> = zbus::Proxy::new(&conn, service.as_str(), path.as_str(), "org.kde.StatusNotifierItem").await.unwrap();
                    let menu_path: zbus::zvariant::OwnedObjectPath = proxy.get_property("Menu").await.unwrap();
                    let items = crate::tray::load_menu(&service, menu_path.as_str()).await.unwrap_or_default();
                    (service, menu_path.to_string(), items)
                },
                |(s,p,i)| Message::MenuLoaded(s,p,i));
            }
        }



        Message::MenuLoaded(service, path, items) =>
        {
            println!("\n===# Menu Loaded!!! #===");
            println!("Service: {service}");
            println!("Menu Path: {path}");
            println!("Id: {:?}\n", items);
            let context_menu_data = crate::context_menu::ContextMenuData 
            {
                mouse_position: app.mouse_position,
                ron_config: app.ron_config.clone(),
                default_font: app.default_font,
                monitor_size: app.monitor_size,
                cursor_is_inside_menu: false, 
                service,
                items,
                path,
            };
            
            std::thread::spawn(move || 
            {
                run_context_menu(context_menu_data);
            });

        }
        _=> {},
    }

    Task::none()
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::modules::network::NetworkData;
    use crate::modules::tray::TrayEvent;
 
    fn make_app() -> AppData { AppData::default() }
 
    // ---- IsHovering* flags --------------------------------------------------
 
    #[test]
    fn message_is_hovering_volume_output_sets_flag_true()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringVolumeOutput(true));
        assert!(app.is_hovering_volume_output);
    }
 
    #[test]
    fn message_is_hovering_volume_output_sets_flag_false()
    {
        let mut app = make_app();
        app.is_hovering_volume_output = true;
        let _ = update(&mut app, Message::IsHoveringVolumeOutput(false));
        assert!(!app.is_hovering_volume_output);
    }
 
    #[test]
    fn message_is_hovering_volume_input_sets_flag()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringVolumeInput(true));
        assert!(app.is_hovering_volume_input);
    }
 
    #[test]
    fn message_is_hovering_workspace_sets_flag()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringWorkspace(true));
        assert!(app.is_hovering_workspace);
    }
 
    #[test]
    fn message_is_hovering_media_player_sets_flag()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::IsHoveringMediaPlayerMetaData(true));
        assert!(app.is_hovering_media_player_meta_data);
    }
 
    // ---- ToggleAltClock -----------------------------------------------------
 
    #[test]
    fn toggle_alt_clock_flips_from_false_to_true()
    {
        let mut app = make_app();
        assert!(!app.is_showing_alt_clock);
        let _ = update(&mut app, Message::ToggleAltClock);
        assert!(app.is_showing_alt_clock);
    }
 
    #[test]
    fn toggle_alt_clock_flips_back_on_second_call()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::ToggleAltClock);
        let _ = update(&mut app, Message::ToggleAltClock);
        assert!(!app.is_showing_alt_clock);
    }
 
    // ---- ToggleAltNetwork ---------------------------------------------------
 
    #[test]
    fn toggle_alt_network_flips_flag()
    {
        let mut app = make_app();
        assert!(!app.is_showing_alt_network_module);
        let _ = update(&mut app, Message::ToggleAltNetwork);
        assert!(app.is_showing_alt_network_module);
    }
 
    #[test]
    fn toggle_alt_network_swaps_to_alt_icons()
    {
        let mut app = make_app();
        app.ron_config.alt_network_level_format = ["A".into(), "B".into(), "C".into(), "D".into()];
        app.ron_config.alt_network_connection_type_icons = ["X".into(), "Y".into(), "Z".into()];
 
        let _ = update(&mut app, Message::ToggleAltNetwork);
 
        assert_eq!(app.network_icons, ["A", "B", "C", "D"]);
        assert_eq!(app.connection_type_icons, ["X", "Y", "Z"]);
    }
 
    #[test]
    fn toggle_alt_network_swaps_back_to_normal_icons()
    {
        let mut app = make_app();
        app.ron_config.network_level_format = ["N1".into(), "N2".into(), "N3".into(), "N4".into()];
        app.ron_config.network_connection_type_icons = ["E".into(), "W".into(), "?".into()];
        app.ron_config.alt_network_level_format = ["A".into(), "B".into(), "C".into(), "D".into()];
        app.ron_config.alt_network_connection_type_icons = ["X".into(), "Y".into(), "Z".into()];
 
        let _ = update(&mut app, Message::ToggleAltNetwork); // → alt
        let _ = update(&mut app, Message::ToggleAltNetwork); // → normal
 
        assert_eq!(app.network_icons, ["N1", "N2", "N3", "N4"]);
        assert_eq!(app.connection_type_icons, ["E", "W", "?"]);
    }
 
    // ---- CursorMoved --------------------------------------------------------
 
    #[test]
    fn cursor_moved_updates_mouse_position()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 123.7, y: 456.2 }));
        assert_eq!(app.mouse_position, (123, 456));
    }

    #[test]
    fn cursor_moved_truncates_not_rounds()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 99.9, y: 99.9 }));
        assert_eq!(app.mouse_position, (99, 99));
    }
 
    // ---- CommandFinished ----------------------------------------------------
 
    #[test]
    fn command_finished_stores_output_at_index()
    {
        let mut app = make_app();
        app.cached_command_outputs = vec!["old".into(), "old".into()];
        let _ = update(&mut app, Message::CommandFinished(1, "new_output".into()));
        assert_eq!(app.cached_command_outputs[1], "new_output");
    }
 
    #[test]
    fn command_finished_resizes_vec_if_index_out_of_bounds()
    {
        let mut app = make_app();
        // vec is empty, index 3 requires resize to length 4
        let _ = update(&mut app, Message::CommandFinished(3, "hello".into()));
        assert_eq!(app.cached_command_outputs.len(), 4);
        assert_eq!(app.cached_command_outputs[3], "hello");
        // Slots 0..2 should be empty strings
        assert_eq!(app.cached_command_outputs[0], "");
        assert_eq!(app.cached_command_outputs[2], "");
    }
 
    #[test]
    fn command_finished_index_zero_works()
    {
        let mut app = make_app();
        let _ = update(&mut app, Message::CommandFinished(0, "result".into()));
        assert_eq!(app.cached_command_outputs[0], "result");
    }
 
    // ---- NetworkUpdated -----------------------------------------------------
 
    #[test]
    fn network_updated_stores_data()
    {
        let mut app = make_app();
        let data = NetworkData { network_level: 4, connection_type: 2, network_speed: 100, id: "HomeWifi".into() };
        let _ = update(&mut app, Message::NetworkUpdated(data));
        assert_eq!(app.modules_data.network_data.id, "HomeWifi");
        assert_eq!(app.modules_data.network_data.network_level, 4);
        assert_eq!(app.modules_data.network_data.network_speed, 100);
    }
 
    // ---- TrayEvent: ItemRegistered ------------------------------------------
 
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
 
    // ---- TrayEvent: ItemUnregistered ----------------------------------------
 
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
 
    // ---- Nothing -----------------------------------------------------------
 
    #[test]
    fn message_nothing_does_not_change_state()
    {
        let mut app = make_app();
        let before = app.is_showing_alt_clock;
        let _ = update(&mut app, Message::Nothing);
        assert_eq!(app.is_showing_alt_clock, before);
    }

    #[test]
    fn toggle_alt_clock_and_alt_network_are_independent()
    {
        let mut app = AppData::default();
        let _ = update(&mut app, Message::ToggleAltClock);
        assert!(app.is_showing_alt_clock);
        assert!(!app.is_showing_alt_network_module); // network untouched
     
        let _ = update(&mut app, Message::ToggleAltNetwork);
        assert!(app.is_showing_alt_clock);            // clock untouched
        assert!(app.is_showing_alt_network_module);
    }
     
    #[test]
    fn multiple_command_finished_messages_stored_independently()
    {
        let mut app = AppData::default();
        let _ = update(&mut app, Message::CommandFinished(0, "out0".into()));
        let _ = update(&mut app, Message::CommandFinished(1, "out1".into()));
        let _ = update(&mut app, Message::CommandFinished(2, "out2".into()));
     
        assert_eq!(app.cached_command_outputs[0], "out0");
        assert_eq!(app.cached_command_outputs[1], "out1");
        assert_eq!(app.cached_command_outputs[2], "out2");
    }
     
    #[test]
    fn overwriting_command_output_replaces_not_appends()
    {
        let mut app = AppData::default();
        let _ = update(&mut app, Message::CommandFinished(0, "first".into()));
        let _ = update(&mut app, Message::CommandFinished(0, "second".into()));
        assert_eq!(app.cached_command_outputs[0], "second");
        assert_eq!(app.cached_command_outputs.len(), 1);
    }
     
    #[test]
    fn cursor_moved_multiple_times_keeps_last_position()
    {
        let mut app = AppData::default();
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 10.0, y: 20.0 }));
        let _ = update(&mut app, Message::CursorMoved(iced::Point { x: 300.0, y: 400.0 }));
        assert_eq!(app.mouse_position, (300, 400));
    }
     
    #[test]
    fn tray_register_then_unregister_leaves_empty_list()
    {
        let mut app = AppData::default();
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemRegistered("s|/p".into())));
        assert_eq!(app.modules_data.tray_icons.len(), 1);
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::ItemUnregistered("s|/p".into())));
        assert!(app.modules_data.tray_icons.is_empty());
    }

    #[test]
    fn cycle_clock_timezones_message_advances_timezone()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "America/New_York".into()]);
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        let _ = update(&mut app, Message::CycleClockTimeZones);
 
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "America/New_York");
        assert_eq!(idx, 1);
    }
 
    #[test]
    fn cycle_clock_timezones_message_wraps_at_end()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "Europe/London".into()]);
        app.current_clock_timezone = Some(("Europe/London".into(), 1));
 
        let _ = update(&mut app, Message::CycleClockTimeZones);
 
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }
 
    #[test]
    fn cycle_clock_timezones_message_with_no_timezones_configured_does_nothing()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = None;
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        let _ = update(&mut app, Message::CycleClockTimeZones);
 
        // State must be unchanged
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }
 
    // ---- ToggleAltClockAndCycleClockTimeZones --------------------------------
 
    #[test]
    fn toggle_alt_clock_and_cycle_flips_alt_clock_flag()
    {
        let mut app = make_app();
        assert!(!app.is_showing_alt_clock);
 
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);
 
        assert!(app.is_showing_alt_clock);
    }
 
    #[test]
    fn toggle_alt_clock_and_cycle_also_cycles_timezone()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "Asia/Tokyo".into()]);
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);
 
        // Both effects must have applied
        assert!(app.is_showing_alt_clock);
        let (tz, _) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "Asia/Tokyo");
    }
 
    #[test]
    fn toggle_alt_clock_and_cycle_called_twice_restores_flag_and_wraps_timezone()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "Asia/Tokyo".into()]);
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones); // on + advance
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones); // off + wrap
 
        assert!(!app.is_showing_alt_clock);
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC"); // wrapped back
        assert_eq!(idx, 0);
    }
 
    #[test]
    fn toggle_alt_clock_and_cycle_with_no_timezones_still_toggles_flag()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = None;
        app.current_clock_timezone = None;
 
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);
 
        // Flag must flip even when timezone cycling is a no-op
        assert!(app.is_showing_alt_clock);
    }
}
