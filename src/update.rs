// ============ IMPORTS ============
use iced::{Task, mouse::ScrollDelta, widget::image};
use std::{sync::Once, time::{Duration, Instant}};
use iced_layershell::to_layer_message;





// ============ STATICS ============
static WARNING_ONCE: Once = Once::new();





// ============ CRATES ============
use crate::modules::focused_window::{read_focused_window_hypr, read_focused_window_sway, read_focused_window_niri, };
use crate::helpers::string::{format_input_volume, format_output_volume};
use crate::modules::cpu_temp::read_cpu_temp;
use crate::modules::ram::read_ram_data;
use crate::modules::{image::preload_image, network::{read_rx_tx, PREV_NET}, disk::read_disk_data, clock::cycle_clock_timezones, cpu::{compute_cpu_usage, read_cpu_snapshot}};
use crate::{helpers::{misc::define_bar_anchor_position, font::build_font, fs::check_if_config_file_exists, monitor::get_monitor_res}, modules::{clock::get_current_time, data::Modules, hypr::{self, change_workspace_hypr}, media_player::{MediaPlayerAction, get_player_data_with_format, media_player_action}, network::NetworkData, niri::{self, change_workspace_niri}, sway::{self, change_workspace_sway}, tray::{load_tray_menu, MenuItem, TrayEvent}, volume, workspaces::UserWorkspaceAction }};
use crate::helpers::{misc::{is_active_module, validate_bar_data}, workspaces::build_workspace_list };
use crate::context_menu::{create_context_menu, get_context_menu_size};
use crate::ron::read_ron_config;
use crate::{warning::create_warning, MAIN_ID, AppData, WindowInfo};





// ============ ENUM/STRUCT, ETC ============
#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message
{
    //CONTEXT MENU
    TrayAction(String, String, i32, String),
    MouseButtonClicked,
    CloseContextMenu,
    CloseWarning,

    MediaPlayerDataFetched(crate::modules::media_player::MediaPlayerData),
    CreateCustomModuleCommand((Option<usize>, Vec<String>, String, bool, bool)),
    MenuLoaded(String, String, Vec<MenuItem>),
    ContinuousCommandFinished(usize, String),
    ToggleAltClockAndCycleClockTimeZones,
    IsHoveringMediaPlayerMetaData(bool),
    TrayIconClicked(usize),
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

    Tick,
    VolumeUpdated(f32, bool, f32, bool),

    UpdateNetworkSpeed,
    UpdateDisk,
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
        //CONTEXT MENU
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
                |_| Message::Nothing,
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
        }

        Message::MouseButtonClicked =>
        {
            let has_context_menu = app.ids.values().any(|v| *v == WindowInfo::ContextMenu);
            if !has_context_menu { return Task::none(); }  // add this guard
            app.context_menu_data.context_menu_is_open = false;
            if !app.context_menu_data.cursor_is_inside_menu
            {
                let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::ContextMenu).map(|(id, _)| *id).collect();
                for id in &window_ids_to_close { app.ids.remove(id);  }
                return Task::batch(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            }
        }

        Message::CloseContextMenu =>
        {
            app.context_menu_data.context_menu_is_open = false;
            let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::ContextMenu).map(|(id, _)| *id).collect();
            for id in &window_ids_to_close { app.ids.remove(id);  }
            return Task::batch(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
        }




        // MAIN APP
        Message::CloseWarning =>
        {
            app.config_parsed_failed = false;
            let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Warning).map(|(id, _)| *id).collect();
            for id in &window_ids_to_close { app.ids.remove(id);  }
            return Task::batch(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
        }
        Message::IsHoveringVolumeOutput(bool) => { app.modules_data.volume_data.is_hovering_volume_output = bool; }
        Message::IsHoveringVolumeInput(bool) => { app.modules_data.volume_data.is_hovering_volume_input = bool; }
        Message::IsHoveringWorkspace(bool) => { app.modules_data.workspace_data.is_hovering_workspace = bool; }
        Message::IsHoveringMediaPlayerMetaData(bool) => { app.modules_data.media_player_data.is_hovering_media_player_meta_data = bool; }
        Message::MuteAudioPressedOutput => { volume::volume( volume::VolumeAction::MuteOutput); }
        Message::MuteAudioPressedInput => { volume::volume( volume::VolumeAction::MuteInput); }
        Message::ToggleAltClock => { app.modules_data.clock_data.is_showing_alt_clock = !app.modules_data.clock_data.is_showing_alt_clock; }
        Message::CommandFinished(index, text) => { if app.modules_data.custom_module_data.cached_command_outputs.len() <= index { app.modules_data.custom_module_data.cached_command_outputs.resize(index + 1, String::new()); } app.modules_data.custom_module_data.cached_command_outputs[index] = text; }
        Message::ContinuousCommandFinished(index, text) => { if app.modules_data.custom_module_data.cached_continuous_outputs.len() <= index { app.modules_data.custom_module_data.cached_continuous_outputs.resize(index + 1, String::new()); } app.modules_data.custom_module_data.cached_continuous_outputs[index] = text; }
        Message::WorkspaceButtonPressed(id) => { if is_active_module(&app.modules_data.active_modules,  Modules::HyprWorkspaces) { change_workspace_hypr(UserWorkspaceAction::ChangeWithIndex(id)); } else if is_active_module(&app.modules_data.active_modules, Modules::SwayWorkspaces) { change_workspace_sway(UserWorkspaceAction::ChangeWithIndex(id)); } else if is_active_module(&app.modules_data.active_modules, Modules::NiriWorkspaces) { change_workspace_niri(UserWorkspaceAction::ChangeWithIndex(id)); } }
        Message::MediaPlayerClickNext => media_player_action(&app.ron_config.player, MediaPlayerAction::Next),
        Message::MediaPlayerClickPlayPause => media_player_action(&app.ron_config.player, MediaPlayerAction::PlayPause),
        Message::MediaPlayerClickPrev => media_player_action(&app.ron_config.player, MediaPlayerAction::Prev),
        Message::CycleClockTimeZones => cycle_clock_timezones(app),
        Message::ToggleAltClockAndCycleClockTimeZones => { app.modules_data.clock_data.is_showing_alt_clock = !app.modules_data.clock_data.is_showing_alt_clock; cycle_clock_timezones(app); },
        Message::UpdateCpuTemp => if let Some(temp) = read_cpu_temp() { app.modules_data.cpu_temp_data.temp_celsius = temp; }
        Message::UpdateRam => { if let Some(data) = read_ram_data() { app.modules_data.ram_data = data; }},
        Message::UpdateFocusedWindowNiri => { app.modules_data.focused_window_data.title = read_focused_window_niri().unwrap_or_default(); }, 
        Message::UpdateFocusedWindowSway => { app.modules_data.focused_window_data.title = read_focused_window_sway().unwrap_or_default(); },
        Message::UpdateFocusedWindowHypr => { app.modules_data.focused_window_data.title = read_focused_window_hypr().unwrap_or_default(); },
        Message::UpdateHyprWorkspaces => { app.modules_data.workspace_data.current_workspace = hypr::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&hypr::workspace_count(), app.ron_config.persistent_workspaces); },
        Message::UpdateSwayWorkspaces => { app.modules_data.workspace_data.current_workspace = sway::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&sway::workspace_count(), app.ron_config.persistent_workspaces); },
        Message::MediaPlayerDataFetched(data) => { app.modules_data.media_player_data = data; }

        Message::NetworkUpdated(data) => 
        { 
            app.modules_data.network_data.connection_type = data.connection_type;
            app.modules_data.network_data.network_level = data.network_level;
            app.modules_data.network_data.network_speed = data.network_speed;
            app.modules_data.network_data.id = data.id;
            app.modules_data.network_data.iface = data.iface;
            app.modules_data.network_data.rx_bytes_per_sec = data.rx_bytes_per_sec;
            app.modules_data.network_data.tx_bytes_per_sec = data.tx_bytes_per_sec;
        }

        Message::UpdateNetworkSpeed =>
        {
            let interface = &app.modules_data.network_data.iface;
            if interface.is_empty() { return Task::none(); }
        
            if let Some((rx, tx)) = read_rx_tx(interface)
            {
                let now = Instant::now();
                let mut prev = PREV_NET.lock().unwrap();
        
                if let Some((prev_rx, prev_tx, prev_time)) = *prev
                {
                    let elapsed = prev_time.elapsed().as_secs_f64();
                    if elapsed > 0.0
                    {
                        app.modules_data.network_data.rx_bytes_per_sec = ((rx.saturating_sub(prev_rx)) as f64 / elapsed) as u64;
                        app.modules_data.network_data.tx_bytes_per_sec = ((tx.saturating_sub(prev_tx)) as f64 / elapsed) as u64;
                    }
                }
                *prev = Some((rx, tx, now));
            }
        }

        Message::UpdateDisk => 
        { 
            if let Some(data) = read_disk_data(&app.ron_config.disk_mount) 
            {
                app.modules_data.disk_data = data; 
            } 
        }

        Message::UpdateMediaPlayerMetadata => 
        { 
            let player = app.ron_config.player.clone();
            let format = app.ron_config.media_player_metadata_format.clone();
            return Task::perform
            (
                async move { get_player_data_with_format(&player, &format).await },
                Message::MediaPlayerDataFetched,
            );
        },

        Message::VolumeUpdated(out_vol, out_muted, in_vol, in_muted) =>
        {
            app.modules_data.volume_data.volume_output_raw = out_vol;
            app.modules_data.volume_data.volume_input_raw = in_vol;

            // Format output
            let (output_str, _) = format_output_volume(out_vol, out_muted, &app.ron_config.output_volume_format, &app.ron_config.output_volume_muted_format);
            app.modules_data.volume_data.output_volume_level = output_str;
            app.modules_data.volume_data.volume_output_is_muted = out_muted;
 
            // Format input
            let (input_str, _) = format_input_volume(in_vol, in_muted, &app.ron_config.input_volume_format, &app.ron_config.input_volume_muted_format);
            app.modules_data.volume_data.input_volume_level = input_str;
            app.modules_data.volume_data.volume_input_is_muted = in_muted;
        }
 
        Message::UpdateCpu =>
        {
            if let Some(curr) = read_cpu_snapshot()
            {
                if let Some(prev) = &app.modules_data.cpu_data.cpu_snapshot
                {
                    app.modules_data.cpu_data.usage_percent = compute_cpu_usage(prev, &curr);
                }
                app.modules_data.cpu_data.cpu_snapshot = Some(curr);
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
            let format_to_send = if app.modules_data.clock_data.is_showing_alt_clock 
            { 
                &app.ron_config.clock_alt_format 
            } 
            else 
            {
                &app.ron_config.clock_format 
            }; 
            app.modules_data.clock_data.current_time = get_current_time(format_to_send, &app.modules_data.clock_data.current_clock_timezone)
        },

        Message::ToggleAltNetwork => 
        { 
            app.modules_data.network_data.is_showing_alt_network_module = !app.modules_data.network_data.is_showing_alt_network_module; 
            if app.modules_data.network_data.is_showing_alt_network_module 
            { 
                app.modules_data.network_data.connection_type_icons = app.ron_config.alt_network_connection_type_icons.clone();
                app.modules_data.network_data.network_icons = app.ron_config.alt_network_level_format.clone();
            }
            else 
            {
                app.modules_data.network_data.connection_type_icons = app.ron_config.network_connection_type_icons.clone();
                app.modules_data.network_data.network_icons = app.ron_config.network_level_format.clone();
            };
        }

        Message::ConfigChanged =>
        {
            let Some(&id) = MAIN_ID.get() else { return Task::none(); };
            println!("\n=== CONFIG RELOAD ===");
            println!("[icebar] config.ron changed — reloading in place...");
            check_if_config_file_exists();
            let (new_config, current_clock_timezone, active_modules, (mut config_parsed_failed, mut warning_err)) = read_ron_config();
            let preloaded_images = preload_image(&mut warning_err, &mut config_parsed_failed, &new_config.images);
            let new_anchor = define_bar_anchor_position(&new_config.bar_position);
            let monitor_res = get_monitor_res(new_config.display.clone());
            let font_name = new_config.font_family.clone();
            let mut modules_data = app.modules_data.clone();

            modules_data.active_modules = active_modules.clone();
            modules_data.clock_data.current_clock_timezone = current_clock_timezone;
            modules_data.network_data.network_icons = new_config.network_level_format.clone();
            modules_data.network_data.connection_type_icons = new_config.network_connection_type_icons.clone();
            modules_data.custom_module_data.custom_module_last_run = vec![Instant::now(); new_config.custom_modules.len()];
            modules_data.image_data.preloaded_images_handle = preloaded_images;

            let old_config_parse_status = app.config_parsed_failed;


            *app = AppData
            {
                warning_err,
                config_parsed_failed,
                ids: app.ids.clone(),
                default_font: build_font(&font_name, &new_config.font_style),
                monitor_size: monitor_res,
                ron_config: new_config, 
                modules_data,
                ..Default::default()
            };

            let bar_data_validated = validate_bar_data(app);

            let mut bar_size = bar_data_validated.bar_size;
            if bar_size.0 == 0 { bar_size.0 = monitor_res.0; };
            if bar_size.1 == 0 { bar_size.1 = monitor_res.1; };

            let mut task_vec = vec!
            [
                Task::done(Message::SizeChange{id, size: bar_size}),
                Task::done(Message::AnchorChange{id, anchor: new_anchor}),
                Task::done(Message::MarginChange{id, margin: bar_data_validated.floating_space}),
                Task::done(Message::ExclusiveZoneChange{id, zone_size: bar_data_validated.exclusive_zone}),
            ];
            if !config_parsed_failed && old_config_parse_status 
            {
                let window_ids_to_close: Vec<iced::window::Id> = app.ids.iter().filter(|(_, info)| **info == WindowInfo::Warning).map(|(id, _)| *id).collect();
                for id in &window_ids_to_close { app.ids.remove(id); }
                task_vec.extend(window_ids_to_close.into_iter().map(|id| Task::done(Message::RemoveWindow(id))));
            };


            let (output_str, _) = format_output_volume(app.modules_data.volume_data.volume_output_raw, app.modules_data.volume_data.volume_output_is_muted, &app.ron_config.output_volume_format, &app.ron_config.output_volume_muted_format);
            app.modules_data.volume_data.output_volume_level = output_str;
 
            let (input_str, _) = format_input_volume(app.modules_data.volume_data.volume_input_raw, app.modules_data.volume_data.volume_input_is_muted, &app.ron_config.input_volume_format, &app.ron_config.input_volume_muted_format);
            app.modules_data.volume_data.input_volume_level = input_str;


            println!("\n=== CONFIG RELOAD ===");
            println!("Reloaded Successfully");
            return Task::batch(task_vec);
        }

        Message::MouseWheelScrolled(ScrollDelta::Pixels { x: _, y }) =>
        {
            if app.modules_data.media_player_data.is_hovering_media_player_meta_data
            {
                if y > 0. { media_player_action(&app.ron_config.player, MediaPlayerAction::VolumeUp); }
                if y < 0. { media_player_action(&app.ron_config.player, MediaPlayerAction::VolumeDown); }
            }

            if app.modules_data.volume_data.is_hovering_volume_output
            {
                if y > 0. { volume::volume(volume::VolumeAction::IncreaseOutput(app.ron_config.incremental_steps_output)); }
                if y < 0. { volume::volume(volume::VolumeAction::DecreaseOutput(app.ron_config.incremental_steps_output)); }
            }

            if app.modules_data.volume_data.is_hovering_volume_input
            {
                if y > 0. { volume::volume(volume::VolumeAction::IncreaseInput(app.ron_config.incremental_steps_input)); }
                if y < 0. { volume::volume(volume::VolumeAction::DecreaseInput(app.ron_config.incremental_steps_input)); }
            }

            if app.modules_data.workspace_data.is_hovering_workspace
            {
                let hypr_active = is_active_module(&app.modules_data.active_modules, Modules::HyprWorkspaces);
                let sway_active = is_active_module(&app.modules_data.active_modules, Modules::SwayWorkspaces);
                let niri_active = is_active_module(&app.modules_data.active_modules, Modules::NiriWorkspaces);

                // === SCROLL UP ===
                if y > 0. 
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
                if y < 0. 
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
                    let Some(module) = app.ron_config.custom_modules.get(index) else { continue; };
                    if module.continous_command.is_empty() { continue; }
                    if app.modules_data.custom_module_data.custom_module_last_run[index].elapsed() < Duration::from_millis(module.continous_command_interval) { continue; }
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
                                let out = tokio::process::Command::new(program).args(args).output().await.ok();
                                out.map
                                (|o| { 
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
                            move |text| Message::ContinuousCommandFinished(index, text),
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

                TrayEvent::Icon { combined, data, width, height } =>
                {
                    if let Some((handle, _)) = app.modules_data.tray_icons.iter_mut().find(|(_, s)| s == &combined)
                    {
                        *handle = Some(image::Handle::from_rgba(width, height, data));
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
                return Task::perform(async move { load_tray_menu(service, path).await }, |result| match result 
                {
                        Ok((s, p, i)) => Message::MenuLoaded(s, p, i),
                        Err(e) => 
                        {
                            eprintln!("Failed to load tray menu: {e}");
                            Message::Nothing 
                        }
                    }
                );
            }
            return Task::none();
        }


        Message::MenuLoaded(service, path, items) =>
        {
            println!("\n===# Menu Loaded!!! #===");
            println!("Service: {service}");
            println!("Menu Path: {path}");
            println!("Id: {:?}\n", items);
            let context_menu_data = crate::context_menu::ContextMenuData 
            {
                mouse_position: app.context_menu_data.mouse_position,
                default_font: app.default_font,
                cursor_is_inside_menu: false, 
                context_menu_is_open: true,
                service,
                items,
                path,
            };
            app.context_menu_data = context_menu_data;
            
            return create_context_menu(app);
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
 
    fn make_app() -> AppData 
    { 
        AppData 
        { 
            ..Default::default() 
        }
    }
 
    // ---- IsHovering* flags --------------------------------------------------
 
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
 
    // ---- ToggleAltClock -----------------------------------------------------
 
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
 
    // ---- ToggleAltNetwork ---------------------------------------------------
 
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
        app.ron_config.alt_network_level_format = ["A".into(), "B".into(), "C".into(), "D".into()];
        app.ron_config.alt_network_connection_type_icons = ["X".into(), "Y".into(), "Z".into()];
 
        let _ = update(&mut app, Message::ToggleAltNetwork);
 
        assert_eq!(app.modules_data.network_data.network_icons, ["A", "B", "C", "D"]);
        assert_eq!(app.modules_data.network_data.connection_type_icons, ["X", "Y", "Z"]);
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
 
        assert_eq!(app.modules_data.network_data.network_icons, ["N1", "N2", "N3", "N4"]);
        assert_eq!(app.modules_data.network_data.connection_type_icons, ["E", "W", "?"]);
    }
 
    // ---- CursorMoved --------------------------------------------------------
 
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
 
    // ---- CommandFinished ----------------------------------------------------
 
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
        // vec is empty, index 3 requires resize to length 4
        let _ = update(&mut app, Message::CommandFinished(3, "hello".into()));
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs.len(), 4);
        assert_eq!(app.modules_data.custom_module_data.cached_command_outputs[3], "hello");
        // Slots 0..2 should be empty strings
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
 
    // ---- NetworkUpdated -----------------------------------------------------
 
    #[test]
    fn network_updated_stores_data()
    {
        let mut app = make_app();
        let data = NetworkData { network_level: 4, connection_type: 2, network_speed: 100, id: "HomeWifi".into(), rx_bytes_per_sec: 0, tx_bytes_per_sec: 0, iface: String::new(), ..Default::default() };
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
        let before = app.modules_data.clock_data.is_showing_alt_clock;
        let _ = update(&mut app, Message::Nothing);
        assert_eq!(app.modules_data.clock_data.is_showing_alt_clock, before);
    }

    #[test]
    fn toggle_alt_clock_and_alt_network_are_independent()
    {
        let mut app = AppData 
        { 
            ..Default::default() 
        };
        let _ = update(&mut app, Message::ToggleAltClock);
        assert!(app.modules_data.clock_data.is_showing_alt_clock);
        assert!(!app.modules_data.network_data.is_showing_alt_network_module); // network untouched
     
        let _ = update(&mut app, Message::ToggleAltNetwork);
        assert!(app.modules_data.clock_data.is_showing_alt_clock);            // clock untouched
        assert!(app.modules_data.network_data.is_showing_alt_network_module);
    }
     
    #[test]
    fn multiple_command_finished_messages_stored_independently()
    {
        let mut app = AppData 
        { 
            ..Default::default() 
        };
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
    fn tray_register_then_unregister_leaves_empty_list()
    {
        let mut app = AppData { ..Default::default() };
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
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "Europe/London".into()]);
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
        app.ron_config.clock_timezones = None;
        app.modules_data.clock_data.current_clock_timezone = Some(("UTC".into(), 0));
 
        let _ = update(&mut app, Message::CycleClockTimeZones);
 
        // State must be unchanged
        let (tz, idx) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }
 
    // ---- ToggleAltClockAndCycleClockTimeZones --------------------------------
 
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
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "Asia/Tokyo".into()]);
        app.modules_data.clock_data.current_clock_timezone = Some(("UTC".into(), 0));
 
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);
 
        // Both effects must have applied
        assert!(app.modules_data.clock_data.is_showing_alt_clock);
        let (tz, _) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "Asia/Tokyo");
    }
 
    #[test]
    fn toggle_alt_clock_and_cycle_called_twice_restores_flag_and_wraps_timezone()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "Asia/Tokyo".into()]);
        app.modules_data.clock_data.current_clock_timezone = Some(("UTC".into(), 0));
 
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones); // on + advance
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones); // off + wrap
 
        assert!(!app.modules_data.clock_data.is_showing_alt_clock);
        let (tz, idx) = app.modules_data.clock_data.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC"); // wrapped back
        assert_eq!(idx, 0);
    }
 
    #[test]
    fn toggle_alt_clock_and_cycle_with_no_timezones_still_toggles_flag()
    {
        let mut app = make_app();
        app.ron_config.clock_timezones = None;
        app.modules_data.clock_data.current_clock_timezone = None;
 
        let _ = update(&mut app, Message::ToggleAltClockAndCycleClockTimeZones);
 
        // Flag must flip even when timezone cycling is a no-op
        assert!(app.modules_data.clock_data.is_showing_alt_clock);
    }

    #[test]
    fn tray_icon_assigned_to_correct_service()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![
            (None, "svc1|/path".into()),
            (None, "svc2|/path".into()),
        ];
    
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon {
            combined: "svc2|/path".into(),
            data: vec![0u8; 4],
            width: 1,
            height: 1,
        }));
    
        // svc1 must still be None — icon must NOT go to the first empty slot
        assert!(app.modules_data.tray_icons[0].0.is_none());
        // svc2 must have the icon
        assert!(app.modules_data.tray_icons[1].0.is_some());
    }
    
    #[test]
    fn tray_icon_unknown_combined_does_nothing()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![(None, "svc1|/path".into())];
    
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon {
            combined: "ghost|/path".into(),
            data: vec![0u8; 4],
            width: 1,
            height: 1,
        }));
    
        // nothing should have changed
        assert!(app.modules_data.tray_icons[0].0.is_none());
    }
    
    #[test]
    fn tray_icon_updates_existing_handle()
    {
        let mut app = make_app();
        let old_handle = Some(image::Handle::from_rgba(1, 1, vec![255u8; 4]));
        app.modules_data.tray_icons = vec![(old_handle, "svc1|/path".into())];
    
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon {
            combined: "svc1|/path".into(),
            data: vec![0u8; 4],
            width: 1,
            height: 1,
        }));
    
        // handle must have been replaced, not left as old value
        assert!(app.modules_data.tray_icons[0].0.is_some());
    }
    
    #[test]
    fn tray_icon_only_affects_matched_service()
    {
        let mut app = make_app();
        app.modules_data.tray_icons = vec![
            (None, "svc1|/path".into()),
            (None, "svc2|/path".into()),
            (None, "svc3|/path".into()),
        ];
    
        let _ = update(&mut app, Message::TrayEvent(TrayEvent::Icon {
            combined: "svc2|/path".into(),
            data: vec![0u8; 4],
            width: 1,
            height: 1,
        }));
    
        assert!(app.modules_data.tray_icons[0].0.is_none()); // untouched
        assert!(app.modules_data.tray_icons[1].0.is_some()); // assigned
        assert!(app.modules_data.tray_icons[2].0.is_none()); // untouched
    }
}
