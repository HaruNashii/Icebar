// ============ IMPORTS ============
use iced::{Alignment, Color, Element, Font, font::Weight, Length, Task as Command, Theme, border::Radius, event, font::Family, mouse::{self, ScrollDelta}, theme::Style, time, widget::{button, container, image, mouse_area, row, text}};
use iced_layershell::{application, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use std::{sync::{OnceLock}, time::Duration};
use hyprland::dispatch::*;






// ============ CRATES ============
use crate::{helpers::workspaces::WorkspaceData, modules::{sway::{UserSwayAction, change_workspace}, tray::{self, TrayEvent, TraySubscription, start_tray, tray_stream}}};
use crate::modules::clock::{ClockData, get_current_time};
use crate::helpers::fs::check_if_config_file_exists;
use crate::modules::volume::{self, VolumeAction};
use crate::helpers::monitor::get_monitor_res;
use crate::context_menu::run_context_menu;
use crate::modules::volume::VolumeData;
use crate::ron::read_ron_config;
use crate::ron::BarConfig;
use crate::modules::hypr;
use crate::modules::sway;





// ============ MOD'S ============
mod context_menu;
mod modules;
mod helpers;
mod ron;





// ============ ENUM/STRUCT, ETC ============
#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message
{
    MenuLoaded(String, String, Vec<tray::MenuItem>),
    MouseWheelScrolled(ScrollDelta),
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
    Tick
}

#[derive(Default, Clone)]
struct AppData
{
    is_hovering_volume_output: bool,
    is_hovering_volume_input: bool,
    is_hovering_workspace: bool,
    is_showing_alt_clock: bool,
    mouse_position: (i32, i32),
    monitor_size: (u32, u32),
    ron_config: BarConfig,
    default_font: Font,
    modules: Modules
}

#[derive(Default, Clone)]
struct Modules
{
    tray_icons: Vec<(Option<image::Handle>, String)>,
    volume_data: VolumeData,
    clock_data: ClockData,
    workspace_data: WorkspaceData,
}

pub struct UserStyle
{
    status: iced::widget::button::Status, 
    border_color_rgba: [u8;4], 
    hovered_text: [u8;3], 
    border_radius: [u32;4],
    normal_text: [u8;3], 
    hovered: [u8; 3], 
    border_size: f32, 
    pressed: [u8;3], 
    normal: [u8;3]
}





// ============ STATICS ============
static DEFAULT_FONT: OnceLock<(String, Weight)> = OnceLock::new();





// ============ FUNCTIONS ============
#[tokio::main]
pub async fn main() -> Result<(), iced_layershell::Error>
{
    check_if_config_file_exists();
    let (ron_config, anchor_position) = read_ron_config();
    let monitor_res = get_monitor_res(ron_config.display.clone());
    let ron_config_clone = ron_config.clone();

    if module_is_active(&ron_config, "tray".to_string())
    {
        start_tray();
    }

    let font_name = ron_config.font_family;
    let font_style_string = ron_config.font_style;
    let font_style = weight_from_str(&font_style_string);
    DEFAULT_FONT.set((font_name, font_style)).expect("DEFAULT_FONT already initialized");

    let modules = Modules
    {
        workspace_data: WorkspaceData::default(),
        volume_data: VolumeData::default(), 
        clock_data: ClockData::default(), 
        tray_icons: Vec::new()
    };
    let app_data = AppData
    {
        default_font: Font { family: Family::Name(&DEFAULT_FONT.get().expect("DEFAULT_FONT not initialized").0), weight: DEFAULT_FONT.get().expect("DEFAULT_FONT not initialized").1, ..iced::Font::DEFAULT}, 
        monitor_size: (monitor_res.0, monitor_res.1),
        is_hovering_volume_output: false, 
        is_hovering_volume_input: false, 
        is_hovering_workspace: false, 
        ron_config: ron_config_clone, 
        is_showing_alt_clock: false,
        mouse_position: (0, 0),
        modules
    };


    let start_mode = match ron_config.display
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active
    };

    let default_font = app_data.default_font;
    application(move || app_data.clone(), namespace, update, view).default_font(default_font).style(style).subscription(subscription).settings(Settings
    {
        layer_settings: LayerShellSettings
        {
            size: Some((ron_config.bar_size[0], ron_config.bar_size[1])),
            exclusive_zone: ron_config.bar_size[1] as i32,
            anchor: anchor_position,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    }).run()
}



fn namespace() -> String { String::from("icebar") }



fn subscription(_: &AppData) -> iced::Subscription<Message>
{
    let event_reader = event::listen_with(|event, _status, _id| 
    {
        match event 
        {
            iced::Event::Mouse(mouse::Event::WheelScrolled {delta, ..} ) => { Some(Message::MouseWheelScrolled(delta)) }
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => { Some(Message::CursorMoved(position)) }
            _=> None
        }
    });

    iced::Subscription::batch
    ([
        iced::Subscription::run_with(TraySubscription, tray_stream),
        time::every(Duration::from_millis(150)).map(|_| Message::Tick),
        event_reader
    ])
}



fn update(app: &mut AppData, message: Message) -> Command<Message>
{
    match message
    {
        Message::IsHoveringVolumeOutput(bool) => { app.is_hovering_volume_output = bool; }
        Message::IsHoveringVolumeInput(bool) => { app.is_hovering_volume_input = bool; }
        Message::IsHoveringWorkspace(bool) => { app.is_hovering_workspace = bool; }
        Message::MuteAudioPressedOutput => { volume::volume( volume::VolumeAction::MuteOutput); }
        Message::MuteAudioPressedInput => { volume::volume( volume::VolumeAction::MuteInput); }
        Message::ToggleAltClock => { app.is_showing_alt_clock = !app.is_showing_alt_clock; }
        Message::WorkspaceButtonPressed(id) => 
        {
            if module_is_active(&app.ron_config, "hypr/workspaces".to_string())
            {
                let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id as i32))); 
            }
            else if module_is_active(&app.ron_config, "sway/workspaces".to_string())
            {
                change_workspace(UserSwayAction::ChangeWithIndex(id));
            }
        }
        Message::CursorMoved(point) => { app.mouse_position = (point.x as i32, point.y as i32); }

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
                if y > 2. 
                { 
                    if app.ron_config.reverse_scroll_on_workspace
                    {
                        if module_is_active(&app.ron_config, "hypr/workspaces".to_string())
                        {
                            let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(1))); 
                        } 
                        else if module_is_active(&app.ron_config, "sway/workspaces".to_string())
                        {
                            change_workspace(UserSwayAction::MoveNext);
                        };
                    }
                    else if module_is_active(&app.ron_config, "hypr/workspaces".to_string())
                    {
                        let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(-1))); 
                    } 
                    else if module_is_active(&app.ron_config, "sway/workspaces".to_string())
                    {
                        change_workspace(UserSwayAction::MovePrev);
                    }
                }
                if y < 2. 
                { 
                    if app.ron_config.reverse_scroll_on_workspace
                    {
                        if module_is_active(&app.ron_config, "hypr/workspaces".to_string())
                        {
                            let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(-1))); 
                        } 
                        else if module_is_active(&app.ron_config, "sway/workspaces".to_string())
                        {
                            change_workspace(UserSwayAction::MovePrev);
                        };
                    }
                    else if module_is_active(&app.ron_config, "hypr/workspaces".to_string())
                    {
                        let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(1))); 
                    } 
                    else if module_is_active(&app.ron_config, "sway/workspaces".to_string())
                    {
                        change_workspace(UserSwayAction::MoveNext);
                    }
                }
            }
        }

        Message::Tick =>
        {
            let format_to_send = if app.is_showing_alt_clock { &app.ron_config.clock_alt_format } else { &app.ron_config.clock_format };
            if module_is_active(&app.ron_config, "clock".to_string())
            {
                app.modules.clock_data.current_time = get_current_time(format_to_send);
            }
            if module_is_active(&app.ron_config, "volume/output".to_string())
            {
                app.modules.volume_data.output_volume_level = volume::volume(VolumeAction::GetOutput((&app.ron_config.output_volume_format, &app.ron_config.output_volume_muted_format)));
            }
            if module_is_active(&app.ron_config, "volume/input".to_string())
            {
                app.modules.volume_data.input_volume_level = volume::volume(VolumeAction::GetInput((&app.ron_config.input_volume_format, &app.ron_config.input_volume_muted_format)));
            }
            if module_is_active(&app.ron_config, "hypr/workspaces".to_string())
            {
                app.modules.workspace_data.current_workspace = hypr::current_workspace();
                app.modules.workspace_data.workspace_count = hypr::workspace_count();
            }
            if module_is_active(&app.ron_config, "sway/workspaces".to_string())
            {
                app.modules.workspace_data.current_workspace = sway::current_workspace();
                app.modules.workspace_data.workspace_count = sway::workspace_count();
            }
        }

        Message::TrayEvent(event) =>
        {
            match event
            {
                TrayEvent::ItemRegistered(service) =>
                {
                    if !app.modules.tray_icons.iter().any(|(_, s)| s == &service)
                    {
                        app.modules.tray_icons.push((None, service));
                    }
                }

                TrayEvent::ItemUnregistered(service) => 
                {
                    println!("\n=== Tray item Unregistered ===\n{service}");
                    app.modules.tray_icons.retain(|(_, s)| s != &service);
                }

                TrayEvent::Icon { data, width, height } =>
                {
                    for (handle, _) in &mut app.modules.tray_icons
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
            if let Some((_, combined)) = app.modules.tray_icons.get(idx)
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



fn view(app: &AppData) -> Element<'_,Message>
{
    // ---------- MODULES ----------
    let left = build_modules(&app.ron_config.left_modules, app);
    let center = build_modules(&app.ron_config.center_modules, app);
    let right = build_modules(&app.ron_config.right_modules, app);

    // ---------- bar ----------
    let bar = row!
    [
        // RIGHT
        container(left).width(Length::Fill).align_x(iced::alignment::Horizontal::Left).align_y(iced::alignment::Vertical::Top),
        
        // CENTER
        container(center).width(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Top),

        // RIGHT
        container(right).width(Length::Fill).align_x(iced::alignment::Horizontal::Right).align_y(iced::alignment::Vertical::Top),
    ].padding(app.ron_config.bar_general_padding).align_y(Alignment::Start);
    bar.into()
}



fn style(app: &AppData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba8(app.ron_config.bar_background_color_rgba[0],app.ron_config.bar_background_color_rgba[1],app.ron_config.bar_background_color_rgba[2],app.ron_config.bar_background_color_rgba[3] as f32 / 100.),
        text_color: Color::WHITE
    }
}



pub fn set_style(user_style: UserStyle) -> iced::widget::button::Style
{
    let mut style = button::Style::default();
    match user_style.status 
    {
        button::Status::Hovered => 
        {
            style.background = Some(iced::Background::Color(Color::from_rgb8(user_style.hovered[0], user_style.hovered[1], user_style.hovered[2])));
            style.text_color = Color::from_rgb8(user_style.hovered_text[0], user_style.hovered_text[1], user_style.hovered_text[2]);
        }
        button::Status::Pressed => 
        {
            style.background = Some(iced::Background::Color(Color::from_rgb8(user_style.pressed[0], user_style.pressed[1], user_style.pressed[2])));
        }
        _ => 
        {
            style.background = Some(iced::Background::Color(Color::from_rgb8(user_style.normal[0], user_style.normal[1], user_style.normal[2])));
            style.text_color = Color::from_rgb8(user_style.normal_text[0], user_style.normal_text[1], user_style.normal_text[2]);
        }
    }
    style.border.color = Color::from_rgba8(user_style.border_color_rgba[0], user_style.border_color_rgba[1],  user_style.border_color_rgba[2], user_style.border_color_rgba[3] as f32);
    style.border.width = user_style.border_size;
    style.border.radius = Radius { top_left: user_style.border_radius[0] as f32, top_right: user_style.border_radius[1] as f32, bottom_left: user_style.border_radius[2] as f32, bottom_right: user_style.border_radius[3] as f32};
    style
}



fn build_modules<'a>(list: &'a Vec<String>, app: &'a AppData) -> Element<'a, Message> 
{
    let mut children = Vec::new();
    for item in list 
    {
        let element: Element<_> = match item.as_str() 
        {
            "tray" => row ( app.modules.tray_icons.iter().enumerate().map(|(i,(icon,_))| { let content: Element<_> = if let Some(icon) = icon { image(icon.clone()).width(app.ron_config.tray_icon_size).height(app.ron_config.tray_icon_size).into() } else { text("?").into() }; button(content).style(|_: &Theme, status: button::Status| 
            {
                let hovered = app.ron_config.tray_button_hovered_color_rgb;
                let hovered_text = app.ron_config.tray_button_hovered_text_color_rgb;
                let pressed = app.ron_config.tray_button_pressed_color_rgb;
                let normal = app.ron_config.tray_button_color_rgb;
                let normal_text = app.ron_config.tray_button_text_color_rgb;
                let border_size = app.ron_config.tray_border_size;
                let border_color_rgba = app.ron_config.tray_border_color_rgba;
                let border_radius = app.ron_config.tray_border_radius;
                set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
            }).padding(app.ron_config.tray_button_size).on_press(Message::TrayIconClicked(i)).into() })).spacing(app.ron_config.tray_spacing).align_y(Alignment::Center).into(),



            "hypr/workspaces" | "sway/workspaces" => mouse_area ( row((1..app.modules.workspace_data.workspace_count + 1).map(|i| 
            { 
                let workspace_text = if let Some(selected_text) = &app.ron_config.workspace_selected_text && app.modules.workspace_data.current_workspace as usize == i
                {
                    if i > app.ron_config.workspace_text.len() 
                    {
                        &format!("{}", i).to_string()
                    } 
                    else 
                    {
                        &selected_text[i - 1]
                    }
                } 
                else if i > app.ron_config.workspace_text.len() 
                {
                    &format!("{}", i).to_string()
                } 
                else 
                {
                    &app.ron_config.workspace_text[i - 1]
                };
                button(text(workspace_text.clone()).font(app.default_font).size(app.ron_config.workspace_text_size)).on_press(Message::WorkspaceButtonPressed(i)).style(move|_: &Theme, status: button::Status| 
                {
                    let hovered = app.ron_config.workspace_button_hovered_color_rgb;
                    let hovered_text = app.ron_config.workspace_button_hovered_text_color_rgb;
                    let pressed = app.ron_config.workspace_button_pressed_color_rgb;
                    let normal = if app.modules.workspace_data.current_workspace == i as i32 { app.ron_config.workspace_button_selected_color_rgb } else { app.ron_config.workspace_button_color_rgb };
                    let normal_text = app.ron_config.workspace_button_text_color_rgb;
                    let border_size = app.ron_config.workspace_border_size;
                    let border_color_rgba = app.ron_config.workspace_border_color_rgba;
                    let border_radius = app.ron_config.workspace_border_radius;
                    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
                }).into() 
            })).spacing(app.ron_config.workspace_spacing).align_y(Alignment::Center)).on_enter(Message::IsHoveringWorkspace(true)).on_exit(Message::IsHoveringWorkspace(false)).into(),



            "clock" => container(button(text(&*app.modules.clock_data.current_time).font(app.default_font).size(app.ron_config.clock_text_size)).on_press(Message::ToggleAltClock).style(|_: &Theme, status: button::Status| 
            {
                let hovered = app.ron_config.clock_button_hovered_color_rgb;
                let hovered_text = app.ron_config.clock_button_hovered_text_color_rgb;
                let pressed = app.ron_config.clock_button_pressed_color_rgb;
                let normal = app.ron_config.clock_button_color_rgb;
                let normal_text = app.ron_config.clock_button_text_color_rgb;
                let border_size = app.ron_config.clock_border_size;
                let border_color_rgba = app.ron_config.clock_border_color_rgba;
                let border_radius = app.ron_config.clock_border_radius;
                set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
            })).align_y(Alignment::Center).into(),



            "volume/output" => container(mouse_area ( button (text(&*app.modules.volume_data.output_volume_level).font(app.default_font).size(app.ron_config.volume_output_text_size)).on_press(Message::MuteAudioPressedOutput).style(|_: &Theme, status: button::Status| 
            {
                let hovered = app.ron_config.volume_output_button_hovered_color_rgb;
                let hovered_text = app.ron_config.volume_output_button_hovered_text_color_rgb;
                let pressed = app.ron_config.volume_output_button_pressed_color_rgb;
                let normal = app.ron_config.volume_output_button_color_rgb;
                let normal_text = app.ron_config.volume_output_button_text_color_rgb;
                let border_size = app.ron_config.volume_output_border_size;
                let border_color_rgba = app.ron_config.volume_output_border_color_rgba;
                let border_radius = app.ron_config.volume_output_border_radius;
                set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
            })).on_enter(Message::IsHoveringVolumeOutput(true)).on_exit(Message::IsHoveringVolumeOutput(false))).align_y(Alignment::Center).into(),



            "volume/input" => container(mouse_area ( button (text(&*app.modules.volume_data.input_volume_level).font(app.default_font).size(app.ron_config.volume_input_text_size)).on_press(Message::MuteAudioPressedInput).style(|_: &Theme, status: button::Status| 
            {
                let hovered = app.ron_config.volume_input_button_hovered_color_rgb;
                let hovered_text = app.ron_config.volume_input_button_hovered_text_color_rgb;
                let pressed = app.ron_config.volume_input_button_pressed_color_rgb;
                let normal = app.ron_config.volume_input_button_color_rgb;
                let normal_text = app.ron_config.volume_input_button_text_color_rgb;
                let border_size = app.ron_config.volume_input_border_size;
                let border_color_rgba = app.ron_config.volume_input_border_color_rgba;
                let border_radius = app.ron_config.volume_input_border_radius;
                set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
            })).on_enter(Message::IsHoveringVolumeInput(true)).on_exit(Message::IsHoveringVolumeInput(false))).align_y(Alignment::Center).into(),
            _ => continue,
        };



        children.push(element);
    }

    row(children).spacing(8).align_y(Alignment::Center).into()
}



fn module_is_active(ron_config: &BarConfig, module: String) -> bool
{
    let all_possible_position = [&ron_config.left_modules, &ron_config.center_modules, &ron_config.right_modules];
    for position in all_possible_position
    {
        for item in position 
        {
            if *item == module
            {
                return true;
            }
        }
    }
    false
}



fn weight_from_str(s: &str) -> Weight 
{
    match s.to_lowercase().as_str() 
    {
        "thin" => Weight::Thin,
        "extra_light" | "extralight" | "ultralight" => Weight::ExtraLight,
        "light" => Weight::Light,
        "normal" | "regular" => Weight::Normal,
        "medium" => Weight::Medium,
        "semibold" | "semi_bold" => Weight::Semibold,
        "bold" => Weight::Bold,
        "extra_bold" | "extrabold" | "ultrabold" => Weight::ExtraBold,
        "black" | "heavy" => Weight::Black,
        _ => Weight::Normal, 
    }
}
