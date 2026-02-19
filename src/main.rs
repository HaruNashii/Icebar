use iced::{Alignment, Color, Element, Length, Renderer, Task as Command, Theme, event, futures::StreamExt, mouse::{self, ScrollDelta}, theme::Style, time, widget::{MouseArea, button, container, image, mouse_area, row, text}};
use iced_layershell::{application, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use std::{sync::Mutex, time::Duration};
use tokio::sync::mpsc;
use once_cell::sync::Lazy;
use hyprland::dispatch::*;





use crate::{clock::{ClockData, get_current_time}, hypr::{HyprlandData, get_hyprland_data}, ron::BarConfig};
use crate::fs::check_if_config_file_exists;
use crate::ron::read_ron_config;
use crate::tray::{TrayEvent};
use crate::volume::VolumeData;





mod volume;
mod clock;
mod tray;
mod popup;
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
    Nothing,

    IsHoveringVolume(bool),
    IsHoveringWorkspace(bool),
    MouseWheelScrolled(ScrollDelta),

    WorkspaceButtonPressed(usize),

    TrayEvent(TrayEvent),
    TrayIconClicked(usize),

    MenuLoaded(String, String, Vec<tray::MenuItem>),

    CloseMenu,
}

#[derive(Default, Clone)]
struct AppData
{
    modules: Modules,
    is_hovering_volume: bool,
    is_hovering_workspace: bool,
    ron_config: BarConfig
}

#[derive(Default, Clone)]
struct Modules
{
    hyprland_data: HyprlandData,
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
    let hyprland_data = get_hyprland_data();
    check_if_config_file_exists();
    let (ron_config, anchor_position) = read_ron_config();
    let ron_config_clone = ron_config.clone();

    let modules = Modules
    {
        hyprland_data, volume_data: VolumeData::default(), clock_data: ClockData::default(), tray_icons: Vec::new()
    };
    let app_data = AppData
    {
        modules,
        is_hovering_volume: false, 
        is_hovering_workspace: false, 
        ron_config: ron_config_clone
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
            size: Some((0, ron_config.bar_size)),
            exclusive_zone: ron_config.bar_size as i32,
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
        Message::WorkspaceButtonPressed(id) =>
        {
            let _ = Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(id as i32)));
        }

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
            app.modules.clock_data.current_time = get_current_time();
            app.modules.volume_data.volume_level = volume::volume(volume::VolumeAction::Get);
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
                cursor_is_inside_menu: false
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
    //
    // ---------- MODULES ----------
    //
    let tray = row
    (
        app.modules.tray_icons.iter().enumerate().map(|(i,(icon,_))|
        {
            let content: Element<_> = if let Some(icon) = icon
            {
                image(icon.clone()).width(18).height(18).into()
            }
            else 
            { 
                text("?").into() 
            };

            button(content).padding(2).on_press(Message::TrayIconClicked(i)).into()
        })
    ).spacing(8);

    let workspace_buttons: MouseArea<'_, Message> = mouse_area
    (
        row
        (
            (1..app.modules.hyprland_data.workspace_count + 1).map(|i| 
            {
                    button(text(format!("{i}"))).on_press(Message::WorkspaceButtonPressed(i)).into()
            })
        ).spacing(8)
    ).on_enter(Message::IsHoveringWorkspace(true)).on_exit(Message::IsHoveringWorkspace(false));

    let volume_button: MouseArea<'_, Message> = mouse_area(button(&*app.modules.volume_data.volume_level).on_press(Message::MuteAudioPressed)).on_enter(Message::IsHoveringVolume(true)).on_exit(Message::IsHoveringVolume(false));
    
    let clock: iced_layershell::reexport::core::widget::Text<'_, Theme, Renderer> = text(&app.modules.clock_data.current_time);


    //
    // ---------- bar ----------
    //
    let bar = row!
    [
            // RIGHT
            container
            (
                row!
                [
                    workspace_buttons,
                    volume_button
                ].spacing(10)
            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Left).align_y(iced::alignment::Vertical::Top),
            


            // CENTER
            container
            (
                row!
                [
                    clock
                ].spacing(10)
            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Top),



            // RIGHT
            container
            (
                row!
                [
                    tray
                ].spacing(10)
            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Right).align_y(iced::alignment::Vertical::Top),
        ].padding(20).align_y(Alignment::Center);


    bar.into()
}



fn style(_: &AppData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba(0.134,0.206,0.203,0.255),
        text_color: Color::WHITE
    }
}

