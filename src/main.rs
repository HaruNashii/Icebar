// ============ IMPORTS ============
use iced::{Alignment, Color, Element, Length, Task as Command, Theme, border::Radius, event, mouse::{self, ScrollDelta}, theme::Style, time, widget::{button, container, image, mouse_area, row, text}};
use iced_layershell::{application, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use hyprland::dispatch::*;
use std::time::Duration;






// ============ CRATES ============
use crate::{clock::{ClockData, get_current_time}, hypr::{current_workspace, workspace_count}, monitor::get_monitor_res, ron::BarConfig, tray::start_tray};
use crate::tray::{TrayEvent, tray_stream, TraySubscription};
use crate::fs::check_if_config_file_exists;
use crate::ron::read_ron_config;
use crate::volume::VolumeData;





// ============ MOD'S ============
mod monitor;
mod volume;
mod clock;
mod popup;
mod tray;
mod hypr;
mod ron;
mod fs;





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
    modules: Modules
}

#[derive(Default, Clone)]
struct Modules
{
    tray_icons: Vec<(Option<image::Handle>, String)>,
    volume_data: VolumeData,
    clock_data: ClockData
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





// ============ FUNCTIONS ============
#[tokio::main]
pub async fn main() -> Result<(), iced_layershell::Error>
{
    check_if_config_file_exists();
    let (ron_config, anchor_position) = read_ron_config();
    let monitor_res = get_monitor_res(ron_config.display.clone());
    let ron_config_clone = ron_config.clone();

    let modules = Modules
    {
        volume_data: VolumeData::default(), 
        clock_data: ClockData::default(), 
        tray_icons: Vec::new()
    };
    let app_data = AppData
    {
        monitor_size: (monitor_res.0, monitor_res.1),
        is_hovering_volume_output: false, 
        is_hovering_volume_input: false, 
        is_hovering_workspace: false, 
        ron_config: ron_config_clone, 
        is_showing_alt_clock: false,
        mouse_position: (0, 0),
        modules
    };

    start_tray();

    let start_mode = match ron_config.display
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active
    };

    application(move || app_data.clone(), namespace, update, view).style(style).subscription(subscription).settings(Settings
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
        Message::WorkspaceButtonPressed(id) => { let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id as i32))); }
        Message::CursorMoved(point) => { app.mouse_position = (point.x as i32, point.y as i32); }

        Message::MouseWheelScrolled(ScrollDelta::Pixels { x: _, y }) =>
        {
            if app.is_hovering_volume_output
            {
                    if y > 2. { volume::volume(volume::VolumeAction::IncreaseOutput); }
                    if y < 2. { volume::volume(volume::VolumeAction::DecreaseOutput); }
            }
            if app.is_hovering_volume_input
            {
                    if y > 2. { volume::volume(volume::VolumeAction::IncreaseInput); }
                    if y < 2. { volume::volume(volume::VolumeAction::DecreaseInput); }
            }
            if app.is_hovering_workspace
            {
                if y > 2. { let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(1))); }
                if y < 2. { let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(-1))); }
            }
        }

        Message::Tick =>
        {
            let format_to_send = if app.is_showing_alt_clock { &app.ron_config.clock_alt_format } else { &app.ron_config.clock_format };
            app.modules.clock_data.current_time = get_current_time(format_to_send);
            app.modules.volume_data.output_volume_level = volume::volume(volume::VolumeAction::GetOutput([&app.ron_config.output_volume_format, &app.ron_config.output_volume_muted_format]));
            app.modules.volume_data.input_volume_level = volume::volume(volume::VolumeAction::GetInput([&app.ron_config.input_volume_format, &app.ron_config.input_volume_muted_format]));
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
            let popup_data = crate::popup::PopupData 
            {
                service,
                path,
                items,
                cursor_is_inside_menu: false, 
                ron_config: app.ron_config.clone(),
                popup_position: app.mouse_position,
                monitor_size: app.monitor_size,
            };
            
            tokio::spawn(async move 
            {
                let _ = crate::popup::run_popup(popup_data).await;
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
            "tray" => row ( app.modules.tray_icons.iter().enumerate().map(|(i,(icon,_))| { let content: Element<_> = if let Some(icon) = icon { image(icon.clone()).width(18).height(18).into() } else { text("?").into() }; button(content).style(|_: &Theme, status: button::Status| 
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
            }).padding(1).on_press(Message::TrayIconClicked(i)).into() })).spacing(8).align_y(Alignment::Start).into(),



            "hypr/workspaces" => mouse_area ( row((1..workspace_count() + 1).map(|i| 
            { 
                #[allow(unused)]
                let mut workspace_text = &String::new();
                let index_string = i.to_string();
                match i
                {
                  1  => {workspace_text = &app.ron_config.hypr_workspace_text[0]},  
                  2  => {workspace_text = &app.ron_config.hypr_workspace_text[1]},  
                  3  => {workspace_text = &app.ron_config.hypr_workspace_text[2]},  
                  4  => {workspace_text = &app.ron_config.hypr_workspace_text[3]},  
                  5  => {workspace_text = &app.ron_config.hypr_workspace_text[4]},  
                  6  => {workspace_text = &app.ron_config.hypr_workspace_text[5]},  
                  7  => {workspace_text = &app.ron_config.hypr_workspace_text[6]},  
                  8  => {workspace_text = &app.ron_config.hypr_workspace_text[7]},  
                  9  => {workspace_text = &app.ron_config.hypr_workspace_text[8]},  
                  10 => {workspace_text = &app.ron_config.hypr_workspace_text[9]},  
                  _=> { workspace_text = &index_string }
                };
                button(text(workspace_text.clone())).on_press(Message::WorkspaceButtonPressed(i)).style(move|_: &Theme, status: button::Status| 
                {
                    let hovered = app.ron_config.hypr_workspace_button_hovered_color_rgb;
                    let hovered_text = app.ron_config.hypr_workspace_button_hovered_text_color_rgb;
                    let pressed = app.ron_config.hypr_workspace_button_pressed_color_rgb;
                    let normal = if current_workspace() == i as i32 { app.ron_config.hypr_workspace_button_selected_color_rgb } else { app.ron_config.hypr_workspace_button_color_rgb };
                    let normal_text = app.ron_config.hypr_workspace_button_text_color_rgb;
                    let border_size = app.ron_config.hypr_workspace_border_size;
                    let border_color_rgba = app.ron_config.hypr_workspace_border_color_rgba;
                    let border_radius = app.ron_config.hypr_workspace_border_radius;
                    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
                }).into() 
            })).spacing(8).align_y(Alignment::Start)).on_enter(Message::IsHoveringWorkspace(true)).on_exit(Message::IsHoveringWorkspace(false)).into(),



            "clock" => container(button(&*app.modules.clock_data.current_time).on_press(Message::ToggleAltClock).style(|_: &Theme, status: button::Status| 
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
            })).align_y(Alignment::Start).into(),



            "volume/output" => container(mouse_area ( button (&*app.modules.volume_data.output_volume_level).on_press(Message::MuteAudioPressedOutput).style(|_: &Theme, status: button::Status| 
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
            })).on_enter(Message::IsHoveringVolumeOutput(true)).on_exit(Message::IsHoveringVolumeOutput(false))).align_y(Alignment::Start).into(),



            "volume/input" => container(mouse_area ( button (&*app.modules.volume_data.input_volume_level).on_press(Message::MuteAudioPressedInput).style(|_: &Theme, status: button::Status| 
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
            })).on_enter(Message::IsHoveringVolumeInput(true)).on_exit(Message::IsHoveringVolumeInput(false))).align_y(Alignment::Start).into(),
            _ => continue,
        };



        children.push(element);
    }

    row(children).spacing(8).align_y(Alignment::Start).into()
}
