use iced::{Alignment, Color, Element, Length, Task as Command, Theme, event, futures::StreamExt, mouse::{self, ScrollDelta}, theme::Style, time, widget::{button, container, image, mouse_area, row, text}};
use iced_layershell::{application, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use std::{sync::Mutex, time::Duration};
use tokio::sync::mpsc;
use once_cell::sync::Lazy;
use hyprland::dispatch::*;





use crate::{clock::{ClockData, get_current_time}, hypr::{HyprlandData, get_hyprland_data}, monitor::get_monitor_res, ron::BarConfig};
use crate::fs::check_if_config_file_exists;
use crate::ron::read_ron_config;
use crate::tray::{TrayEvent};
use crate::volume::VolumeData;





mod volume;
mod clock;
mod tray;
mod popup;
mod monitor;
mod fs;
mod ron;
mod hypr;




static TRAY_RECEIVER: Lazy<Mutex<Option<mpsc::Receiver<TrayEvent>>>> = Lazy::new(|| Mutex::new(None));





#[to_layer_message]
#[derive(Debug, Clone)]
enum Message
{
    MuteAudioPressed,
    Tick,

    IsHoveringVolume(bool),
    IsHoveringWorkspace(bool),
    MouseWheelScrolled(ScrollDelta),

    WorkspaceButtonPressed(usize),
    ToggleAltClock,

    TrayEvent(TrayEvent),
    TrayIconClicked(usize),

    MenuLoaded(String, String, Vec<tray::MenuItem>),

    CursorMoved(iced::Point),
}

#[derive(Default, Clone)]
struct AppData
{
    modules: Modules,
    is_showing_alt_clock: bool,
    is_hovering_volume: bool,
    is_hovering_workspace: bool,
    ron_config: BarConfig,
    mouse_position: (i32, i32),
    monitor_size: (u32, u32)
}

#[derive(Default, Clone)]
struct Modules
{
    hypr_data: HyprlandData,
    volume_data: VolumeData,
    clock_data: ClockData,
    // (icon, service|path)
    tray_icons: Vec<(Option<image::Handle>, String)>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct TraySubscription;





#[tokio::main]
pub async fn main() -> Result<(), iced_layershell::Error>
{
    let monitor_res = get_monitor_res();
    let hypr_data = get_hyprland_data();
    check_if_config_file_exists();
    let (ron_config, anchor_position) = read_ron_config();
    let ron_config_clone = ron_config.clone();

    let modules = Modules
    {
        hypr_data, volume_data: VolumeData::default(), clock_data: ClockData::default(), tray_icons: Vec::new()
    };
    let app_data = AppData
    {
        modules,
        is_showing_alt_clock: false,
        is_hovering_volume: false, 
        is_hovering_workspace: false, 
        ron_config: ron_config_clone, 
        mouse_position: (0, 0),
        monitor_size: (monitor_res.0, monitor_res.1),
    };

    // ---- tray watcher ----
    let (tx, rx) = mpsc::channel(32);
    *TRAY_RECEIVER.lock().unwrap() = Some(rx);
    tokio::spawn(async move 
    {
        let _ = tray::start_watcher(tx).await;
    });

    let start_mode = match std::env::args().nth(1)
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
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



fn tray_stream(_: &TraySubscription) -> impl iced::futures::Stream<Item = Message>
{
    let receiver = TRAY_RECEIVER.lock().unwrap().take().expect("tray receiver already taken");
    tokio_stream::wrappers::ReceiverStream::new(receiver).map(Message::TrayEvent)
}



fn subscription(_: &AppData) -> iced::Subscription<Message>
{
    let sub = event::listen_with(|event, _status, _id| 
    {
        match event 
        {
            iced::Event::Mouse(mouse::Event::WheelScrolled {delta, ..} ) => 
            {
                Some(Message::MouseWheelScrolled(delta))
            },
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => 
            {
                Some(Message::CursorMoved(position))
            }
            _=> None
        }
    });

    iced::Subscription::batch
    ([
        iced::Subscription::run_with(TraySubscription, tray_stream),
        time::every(Duration::from_secs(1)).map(|_| Message::Tick),
        sub
    ])
}



fn update(app: &mut AppData, message: Message) -> Command<Message>
{
    match message
    {
        Message::IsHoveringVolume(bool) => { app.is_hovering_volume = bool; }
        Message::IsHoveringWorkspace(bool) => { app.is_hovering_workspace = bool; }
        Message::MuteAudioPressed => { volume::volume( volume::VolumeAction::Mute); }
        Message::ToggleAltClock => { app.is_showing_alt_clock = !app.is_showing_alt_clock; }
        Message::WorkspaceButtonPressed(id) => { let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id as i32))); }
        Message::CursorMoved(point) => { app.mouse_position = (point.x as i32, point.y as i32); }

        Message::MouseWheelScrolled(scrolldelta) =>
        {
            if app.is_hovering_volume
            {
                if let ScrollDelta::Pixels{x: _, y} = scrolldelta 
                {
                    if y > 2.
                    {
                        volume::volume(volume::VolumeAction::Increase);
                    }
                    if y < 2.
                    {
                        volume::volume(volume::VolumeAction::Decrease);
                    }
                }
            }
            if app.is_hovering_workspace
            {
                if let ScrollDelta::Pixels{x: _, y} = scrolldelta 
                {
                    if y > 2.
                    {
                        let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(1)));
                    }
                    if y < 2.
                    {
                        let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Relative(-1)));
                    }
                }
            }
        }

        Message::Tick =>
        {
            let format_to_send = if app.is_showing_alt_clock
            {
                &app.ron_config.clock_alt_format
            }
            else
            {
                &app.ron_config.clock_format
            };
            app.modules.clock_data.current_time = get_current_time(format_to_send);
            app.modules.volume_data.volume_level = volume::volume(volume::VolumeAction::Get([&app.ron_config.volume_format, &app.ron_config.volume_muted_format]));
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
                    let menu_path:zbus::zvariant::OwnedObjectPath = proxy.get_property("Menu").await.unwrap();
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
                popup_position: app.mouse_position.clone(),
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



fn build_modules<'a>(list: &'a Vec<String>, app: &'a AppData) -> Element<'a, Message> 
{
    let mut children = Vec::new();
    for item in list 
    {
        let element: Element<_> = match item.as_str() 
        {
            "tray" => 
                row ( app.modules.tray_icons.iter().enumerate().map(|(i,(icon,_))| { let content: Element<_> = if let Some(icon) = icon { image(icon.clone()).width(18).height(18).into() } else { text("?").into() }; 
                button(content)
                    .style(|_: &Theme, status: button::Status| 
                    {
                        let mut style = button::Style::default();
                        let hovered = app.ron_config.tray_button_hovered;
                        let hovered_text = app.ron_config.tray_button_hovered_text;
                        let pressed = app.ron_config.tray_button_pressed;
                        let normal = app.ron_config.tray_button;
                        let normal_text = app.ron_config.tray_button_text;
                        match status 
                        {
                            button::Status::Hovered => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(hovered[0], hovered[1], hovered[2])));
                                style.text_color = Color::from_rgb8(hovered_text[0], hovered_text[1], hovered_text[2]);
                            }
                            button::Status::Pressed => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(pressed[0], pressed[1], pressed[2])));
                            }
                            _ => 
                            {
                                // Default active state
                                style.background = Some(iced::Background::Color(Color::from_rgb8(normal[0], normal[1], normal[2])));
                                style.text_color = Color::from_rgb8(normal_text[0], normal_text[1], normal_text[2]);
                            }
                        }
                        let border_color = app.ron_config.tray_border_color;
                        style.border.width = app.ron_config.tray_border_size;
                        style.border.color = Color::from_rgb8(border_color[0], border_color[0],  border_color[0]);
                        style
                    })
                    .padding(2).on_press(Message::TrayIconClicked(i)

                ).into() })).spacing(8).align_y(Alignment::Start).into(),
            "hypr/workspaces" => 
            mouse_area 
            ( 
                row 
                ( 
                    (1..app.modules.hypr_data.workspace_count + 1)
                    .map(|i| { 

                    button(text(format!("{i}")))
                    .on_press(Message::WorkspaceButtonPressed(i))
                    .style(|_: &Theme, status: button::Status| 
                    {
                        let mut style = button::Style::default();
                        let hovered = app.ron_config.hypr_workspace_button_hovered;
                        let hovered_text = app.ron_config.hypr_workspace_button_hovered_text;
                        let pressed = app.ron_config.hypr_workspace_button_pressed;
                        let normal = app.ron_config.hypr_workspace_button;
                        let normal_text = app.ron_config.hypr_workspace_button_text;
                        match status 
                        {
                            button::Status::Hovered => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(hovered[0], hovered[1], hovered[2])));
                                style.text_color = Color::from_rgb8(hovered_text[0], hovered_text[1], hovered_text[2]);
                            }
                            button::Status::Pressed => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(pressed[0], pressed[1], pressed[2])));
                            }
                            _ => 
                            {
                                // Default active state
                                style.background = Some(iced::Background::Color(Color::from_rgb8(normal[0], normal[1], normal[2])));
                                style.text_color = Color::from_rgb8(normal_text[0], normal_text[1], normal_text[2]);
                            }
                        }
                        let border_color = app.ron_config.hypr_workspace_border_color;
                        style.border.width = app.ron_config.hypr_workspace_border_size;
                        style.border.color = Color::from_rgb8(border_color[0], border_color[0],  border_color[0]);
                        style
                    })
                    .into() })).spacing(8).align_y(Alignment::Start)).on_enter(Message::IsHoveringWorkspace(true)).on_exit(Message::IsHoveringWorkspace(false)).into(),
            "clock" => container(

                button(&*app.modules.clock_data.current_time).on_press(Message::ToggleAltClock)
                    .style(|_: &Theme, status: button::Status| 
                    {
                        let mut style = button::Style::default();
                        let hovered = app.ron_config.clock_button_hovered;
                        let hovered_text = app.ron_config.clock_button_hovered_text;
                        let pressed = app.ron_config.clock_button_pressed;
                        let normal = app.ron_config.clock_button;
                        let normal_text = app.ron_config.clock_button_text;
                        match status 
                        {
                            button::Status::Hovered => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(hovered[0], hovered[1], hovered[2])));
                                style.text_color = Color::from_rgb8(hovered_text[0], hovered_text[1], hovered_text[2]);
                            }
                            button::Status::Pressed => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(pressed[0], pressed[1], pressed[2])));
                            }
                            _ => 
                            {
                                // Default active state
                                style.background = Some(iced::Background::Color(Color::from_rgb8(normal[0], normal[1], normal[2])));
                                style.text_color = Color::from_rgb8(normal_text[0], normal_text[1], normal_text[2]);
                            }
                        }
                        let border_color = app.ron_config.clock_border_color;
                        style.border.width = app.ron_config.clock_border_size;
                        style.border.color = Color::from_rgb8(border_color[0], border_color[0],  border_color[0]);
                        style
                    })




                ).align_y(Alignment::Start).into(),
            "volume/output" => 
            container
            (
                mouse_area ( 

                    button (&*app.modules.volume_data.volume_level).on_press(Message::MuteAudioPressed)
                    .style(|_: &Theme, status: button::Status| 
                    {
                        let mut style = button::Style::default();
                        let hovered = app.ron_config.volume_output_button_hovered;
                        let hovered_text = app.ron_config.volume_output_button_hovered_text;
                        let pressed = app.ron_config.volume_output_button_pressed;
                        let normal = app.ron_config.volume_output_button;
                        let normal_text = app.ron_config.volume_output_button_text;
                        match status 
                        {
                            button::Status::Hovered => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(hovered[0], hovered[1], hovered[2])));
                                style.text_color = Color::from_rgb8(hovered_text[0], hovered_text[1], hovered_text[2]);
                            }
                            button::Status::Pressed => 
                            {
                                style.background = Some(iced::Background::Color(Color::from_rgb8(pressed[0], pressed[1], pressed[2])));
                            }
                            _ => 
                            {
                                // Default active state
                                style.background = Some(iced::Background::Color(Color::from_rgb8(normal[0], normal[1], normal[2])));
                                style.text_color = Color::from_rgb8(normal_text[0], normal_text[1], normal_text[2]);
                            }
                        }
                        let border_color = app.ron_config.volume_output_border_color;
                        style.border.width = app.ron_config.volume_output_border_size;
                        style.border.color = Color::from_rgb8(border_color[0], border_color[0],  border_color[0]);
                        style
                    })



            ).on_enter(Message::IsHoveringVolume(true)).on_exit(Message::IsHoveringVolume(false))).align_y(Alignment::Start).into(),
            _ => continue,
        };

        children.push(element);
    }

    row(children).spacing(8).align_y(Alignment::Start).into()
}



fn view(app: &AppData) -> Element<'_,Message>
{
    //
    // ---------- MODULES ----------
    //
    let left = build_modules(&app.ron_config.left_modules, app);
    let center = build_modules(&app.ron_config.center_modules, app);
    let right = build_modules(&app.ron_config.right_modules, app);



    //
    // ---------- bar ----------
    //
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

