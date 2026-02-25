// ============ IMPORTS ============
use iced::{Task as Command, mouse::ScrollDelta, widget::image};





// ============ CRATES ============
use crate::{modules::{clock::get_current_time, hypr::{self, UserHyprAction, change_workspace_hypr}, sway::{self, UserSwayAction, change_workspace_sway}, tray::TrayEvent, volume::{self, VolumeAction} }};
use crate::helpers::{misc::is_active_module, workspaces::build_workspace_list };
use crate::context_menu::run_context_menu;
use crate::{AppData, Message};





// ============ FUNCTIONS ============
pub fn update(app: &mut AppData, message: Message) -> Command<Message>
{
    match message
    {
        Message::IsHoveringVolumeOutput(bool) => { app.is_hovering_volume_output = bool; }
        Message::IsHoveringVolumeInput(bool) => { app.is_hovering_volume_input = bool; }
        Message::IsHoveringWorkspace(bool) => { app.is_hovering_workspace = bool; }
        Message::MuteAudioPressedOutput => { volume::volume( volume::VolumeAction::MuteOutput); }
        Message::MuteAudioPressedInput => { volume::volume( volume::VolumeAction::MuteInput); }
        Message::ToggleAltClock => { app.is_showing_alt_clock = !app.is_showing_alt_clock; }
        Message::CursorMoved(point) => { app.mouse_position = (point.x as i32, point.y as i32); }
        Message::CommandFinished(index, text) => { if app.cached_command_outputs.len() <= index { app.cached_command_outputs.resize(index + 1, String::new()); } app.cached_command_outputs[index] = text; }
        Message::WorkspaceButtonPressed(id) => { if is_active_module(&app.modules.active_modules, "hypr/workspaces".to_string()) { change_workspace_hypr(UserHyprAction::ChangeWithIndex(id)); } else if is_active_module(&app.modules.active_modules, "sway/workspaces".to_string()) { change_workspace_sway(UserSwayAction::ChangeWithIndex(id)); } }



        Message::MouseWheelScrolled(ScrollDelta::Pixels { x: _, y }) =>
        {
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
                let hypr_active = is_active_module(&app.modules.active_modules, "hypr/workspaces".to_string());
                let sway_active = is_active_module(&app.modules.active_modules, "sway/workspaces".to_string());

                // === SCROLL UP ===
                if y > 2. 
                { 
                    if app.ron_config.reverse_scroll_on_workspace
                    {
                        if hypr_active
                        {
                            change_workspace_hypr(UserHyprAction::MoveNext);
                        } 
                        else if sway_active
                        {
                            change_workspace_sway(UserSwayAction::MoveNext);
                        };
                    }
                    else if hypr_active
                    {
                        change_workspace_hypr(UserHyprAction::MovePrev);
                    } 
                    else if sway_active
                    {
                        change_workspace_sway(UserSwayAction::MovePrev);
                    }
                }

                // === SCROLL DOWN ===
                if y < 2. 
                { 
                    if app.ron_config.reverse_scroll_on_workspace
                    {
                        if hypr_active
                        {
                            change_workspace_hypr(UserHyprAction::MovePrev);
                        } 
                        else if sway_active
                        {
                            change_workspace_sway(UserSwayAction::MovePrev);
                        };
                    }
                    else if hypr_active
                    {
                        change_workspace_hypr(UserHyprAction::MoveNext);
                    } 
                    else if sway_active
                    {
                        change_workspace_sway(UserSwayAction::MoveNext);
                    }
                }
            }
        }



        Message::Tick =>
        {
            for module_name in &app.modules.active_modules
            {
                match module_name as &str
                {
                    "clock" => {let format_to_send = if app.is_showing_alt_clock { &app.ron_config.clock_alt_format } else { &app.ron_config.clock_format }; app.modules_data.clock_data.current_time = get_current_time(format_to_send)},
                    "volume/output" => app.modules_data.volume_data.output_volume_level = volume::volume(VolumeAction::GetOutput((&app.ron_config.output_volume_format, &app.ron_config.output_volume_muted_format))),
                    "volume/input" => app.modules_data.volume_data.input_volume_level = volume::volume(VolumeAction::GetInput((&app.ron_config.input_volume_format, &app.ron_config.input_volume_muted_format))),
                    "hypr/workspaces" => { app.modules_data.workspace_data.current_workspace = hypr::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&hypr::workspace_count(), app.ron_config.persistent_workspaces); }
                    "sway/workspaces" => { app.modules_data.workspace_data.current_workspace = sway::current_workspace(); app.modules_data.workspace_data.visible_workspaces = build_workspace_list(&sway::workspace_count(), app.ron_config.persistent_workspaces); }
                    received_str =>
                    {
                        if !received_str.contains("custom_module[") { continue; }
                        let index = received_str.replace("custom_module[", "").replace(']', "").replace([' ', '\n'], "").parse::<usize>();
                        let Ok(index) = index else { continue };
                        let module = &app.ron_config.custom_modules[index];
                        let command_vec = &module.continous_command;

                        if !command_vec.is_empty() && let Some((program, args)) = command_vec.split_first() 
                        {
                            let mut command = std::process::Command::new(program);
                            command.args(args);
                            let output = command.output();
                            if module.use_continous_output_as_text && let Ok(ref output_result) = output
                            {
                                if !app.cached_continuous_outputs.is_empty() && (app.cached_continuous_outputs.len() - 1) >= index
                                {
                                    app.cached_continuous_outputs[index] = String::from_utf8_lossy(&output_result.stdout).to_string();
                                }
                                else
                                {
                                    app.cached_continuous_outputs.push(String::from_utf8_lossy(&output_result.stdout).to_string());
                                }
                            }
                        };
                    }
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
                    return Command::perform
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
        
                return Command::none();
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
                            return Command::none();
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
                if parts.len() != 2 { return Command::none(); }
                let service = parts[0].to_string();
                let path = parts[1].to_string();
                return Command::perform(async move 
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

    Command::none()
}
