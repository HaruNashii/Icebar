// ============ IMPORTS ============
use iced::{Font, font::Family, Task, mouse::ScrollDelta, widget::image};
use iced_layershell::to_layer_message;
use std::time::{Duration, Instant};
use std::sync::Once;




// ============ STATICS ============
static WARNING_ONCE: Once = Once::new();





use crate::helpers::string::weight_from_str;
use crate::modules::clock::cycle_clock_timezones;
// ============ CRATES ============
use crate::{helpers::{fs::check_if_config_file_exists, monitor::get_monitor_res}, modules::{clock::get_current_time, data::{Modules, ModulesData}, hypr::{self, change_workspace_hypr}, media_player::{MediaPlayerAction, get_player_data_with_format, media_player_action}, network::NetworkData, niri::{self, change_workspace_niri}, sway::{self, change_workspace_sway}, tray::{MenuItem, TrayEvent}, volume::{self, VolumeAction}, workspaces::UserWorkspaceAction }};
use crate::helpers::{misc::{is_active_module, validade_bar_size_and_margin}, workspaces::build_workspace_list };
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
    MouseWheelScrolled(ScrollDelta),
    CommandFinished(usize, String),
    WorkspaceButtonPressed(usize),
    IsHoveringVolumeOutput(bool),
    IsHoveringVolumeInput(bool),
    NetworkUpdated(NetworkData),
    IsHoveringWorkspace(bool),
    MediaPlayerClickPlayPause,
    CursorMoved(iced::Point),
    TrayIconClicked(usize),
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
    Tick
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
        Message::ToggleAltClock => { app.is_showing_alt_clock = !app.is_showing_alt_clock; }
        Message::CursorMoved(point) => { app.mouse_position = (point.x as i32, point.y as i32); }
        Message::CommandFinished(index, text) => { if app.cached_command_outputs.len() <= index { app.cached_command_outputs.resize(index + 1, String::new()); } app.cached_command_outputs[index] = text; }
        Message::WorkspaceButtonPressed(id) => { if is_active_module(&app.modules_data.active_modules,  Modules::HyprWorkspaces) { change_workspace_hypr(UserWorkspaceAction::ChangeWithIndex(id)); } else if is_active_module(&app.modules_data.active_modules, Modules::SwayWorkspaces) { change_workspace_sway(UserWorkspaceAction::ChangeWithIndex(id)); } else if is_active_module(&app.modules_data.active_modules, Modules::NiriWorkspaces) { change_workspace_niri(UserWorkspaceAction::ChangeWithIndex(id)); } }
        Message::NetworkUpdated(data) => { app.modules_data.network_data = data }
        Message::MediaPlayerClickNext => media_player_action(&app.ron_config.player, MediaPlayerAction::Next),
        Message::MediaPlayerClickPlayPause => media_player_action(&app.ron_config.player, MediaPlayerAction::PlayPause),
        Message::MediaPlayerClickPrev => media_player_action(&app.ron_config.player, MediaPlayerAction::Prev),
        Message::CycleClockTimeZones => cycle_clock_timezones(app),
        Message::ToggleAltClockAndCycleClockTimeZones => { app.is_showing_alt_clock = !app.is_showing_alt_clock; cycle_clock_timezones(app); },

        Message::ConfigChanged =>
        {
            println!("\n=== CONFIG RELOAD ===");
            println!("[icebar] config.ron changed — reloading in place...");
            check_if_config_file_exists();
            let (new_config, new_anchor, current_clock_timezone, active_modules) = read_ron_config();
            let (mut bar_size, exclusive_zone, floating_space) = validade_bar_size_and_margin(&new_config);
            let monitor_res = get_monitor_res(new_config.display.clone());
            if bar_size.0 == 0 { bar_size.0 = monitor_res.0; };
            if bar_size.1 == 0 { bar_size.1 = monitor_res.1; };
            let font_name = new_config.font_family.clone();
            let modules_data = ModulesData 
            {
                network_data: app.modules_data.network_data.clone(),
                volume_data: app.modules_data.volume_data.clone(),
                tray_icons: app.modules_data.tray_icons.clone(),
                clock_data: app.modules_data.clock_data.clone(),
                active_modules: active_modules.clone(),
                ..Default::default() 
            };
            *app = AppData
            {
                default_font: Font { family: Family::Name(Box::leak(font_name.into_boxed_str())), weight: weight_from_str(&new_config.font_style), ..iced::Font::DEFAULT}, 
                monitor_size: monitor_res,
                custom_module_last_run: vec![Instant::now(); new_config.custom_modules.len()],
                current_clock_timezone,
                network_icons: new_config.network_level_format.clone(),
                connection_type_icons: new_config.network_connection_type_icons.clone(),
                ron_config: new_config, 
                modules_data,
                ..Default::default()
            };

            //if is_active_module(&active_modules, Modules::Tray) { start_tray(); }

            println!("\n=== CONFIG RELOAD ===");
            println!("Reloaded Successfully");
            return Task::batch(vec!
            [
                Task::done(Message::SizeChange(bar_size)),
                Task::done(Message::AnchorChange(new_anchor)),
                Task::done(Message::MarginChange(floating_space)),
                Task::done(Message::ExclusiveZoneChange(exclusive_zone as i32)),
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
            for module_name in &app.modules_data.active_modules
            {
                match module_name
                {
                    Modules::MediaPlayerMetaData => { app.modules_data.media_player_data = get_player_data_with_format(&app.ron_config); }
                    Modules::HyprWorkspaces => { app.modules_data.workspace_data.current_workspace = hypr::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&hypr::workspace_count(), app.ron_config.persistent_workspaces); }
                    Modules::SwayWorkspaces => { app.modules_data.workspace_data.current_workspace = sway::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&sway::workspace_count(), app.ron_config.persistent_workspaces); }
                    Modules::NiriWorkspaces => 
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

                    Modules::Clock => 
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


                    Modules::VolumeOutput => 
                    {
                        let (volume_output, is_muted) = volume::volume(VolumeAction::GetOutput((&app.ron_config.output_volume_format, &app.ron_config.output_volume_muted_format)));
                        app.modules_data.volume_data.output_volume_level = volume_output;
                        app.volume_output_is_muted = is_muted;
                    }

                    Modules::VolumeInput => 
                    {
                        let (volume_input, is_muted) = volume::volume(VolumeAction::GetInput((&app.ron_config.input_volume_format, &app.ron_config.input_volume_muted_format)));
                        app.modules_data.volume_data.input_volume_level = volume_input;
                        app.volume_input_is_muted = is_muted;
                    }

                    Modules::CustomModule(borrowed_index) =>
                    {
                        let index = *borrowed_index;
                        let module = &app.ron_config.custom_modules[index];
                        if module.continous_command.is_empty() { continue; }
                        if app.custom_module_last_run[index].elapsed() < Duration::from_millis(module.continous_command_interval) { continue; }
                        app.custom_module_last_run[index] = Instant::now();
                        let command_vec = &module.continous_command;
                        if !command_vec.is_empty() && let Some((program, args)) = command_vec.split_first() 
                        {
                            let mut command = std::process::Command::new(program);
                            command.args(args);
                            let output = command.output();
                            if module.use_continous_output_as_text 
                            {
                                match output
                                {
                                    Ok(result_output) =>
                                    {
                                        let output_string = if result_output.stdout.is_empty() && module.display_err_output_if_failed
                                        {
                                            &result_output.stderr
                                        }
                                        else
                                        {
                                            &result_output.stdout
                                        };

                                        if !app.cached_continuous_outputs.is_empty() && (app.cached_continuous_outputs.len() - 1) >= index
                                        {
                                            app.cached_continuous_outputs[index] = String::from_utf8_lossy(output_string).to_string();
                                        }
                                        else
                                        {
                                            app.cached_continuous_outputs.push(String::from_utf8_lossy(output_string).to_string());
                                        }
                                    }
                                    Err(err) =>
                                    {
                                        let error_msg = "WARING!!!: Failed To Execute Continous Command!!!".to_string();
                                        println!("\n\n\n{error_msg}\nErr: {err}\n\n\n");
                                        if !app.cached_continuous_outputs.is_empty() && (app.cached_continuous_outputs.len() - 1) >= index
                                        {
                                            app.cached_continuous_outputs[index] = error_msg;
                                        }
                                        else
                                        {
                                            app.cached_continuous_outputs.push(error_msg);
                                        }
                                    }
                                }
                            };
                        };
                    }
                    _=> {},
                }
            }
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
                                let output = std::process::Command::new(program).args(args).output().ok();
                                if custom_name.is_empty() { println!("Custom Module Output:\n{:?}", output); } else { println!( "'{custom_name}' Command Was Running!!!, The Output Was:\n{:?}", output); }
                                output.map(|o| String::from_utf8_lossy(&o.stdout).to_string()).unwrap_or_default()
                        },
                        move |text| { Message::CommandFinished(output_index.unwrap_or(0), text) },
                    );
                }
        

                // ==============================
                // FIRE & FORGET → no message
                // ==============================
                std::thread::spawn(move || 
                {
                    let output = std::process::Command::new(program).args(args).output();
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
