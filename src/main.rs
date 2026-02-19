use iced::{futures::StreamExt, theme::Style, time, widget::{button, container, row, text, image}, Alignment, Color, Element, Length, Task as Command};
use iced_layershell::{application, to_layer_message, reexport::Anchor, settings::{LayerShellSettings, StartMode, Settings}};
use std::{sync::Mutex, time::Duration};
use tokio::sync::mpsc;
use once_cell::sync::Lazy;





use crate::clock::{ClockData, get_current_time};
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





static TRAY_RECEIVER: Lazy<Mutex<Option<mpsc::Receiver<TrayEvent>>>> = Lazy::new(|| Mutex::new(None));





#[to_layer_message]
#[derive(Debug, Clone)]
enum Message
{
    IncrementPressed,
    DecrementPressed,
    Tick,

    TrayEvent(TrayEvent),
    TrayIconClicked(usize),

    MenuLoaded(String, String, Vec<tray::MenuItem>),

    CloseMenu,
}

#[derive(Default, Clone)]
struct AppData
{
    modules: Modules
}

#[derive(Default, Clone)]
struct Modules
{
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
    check_if_config_file_exists();
    let ron_config = read_ron_config();

    let anchor_position = match ron_config.bar_position.as_str()
    {
        "Up" => Anchor::Top | Anchor::Left | Anchor::Right,
        "Down" => Anchor::Bottom | Anchor::Left | Anchor::Right,
        "Left" => Anchor::Left | Anchor::Top | Anchor::Bottom,
        "Right" => Anchor::Right | Anchor::Top | Anchor::Bottom,
        _ => Anchor::Top | Anchor::Left | Anchor::Right,
    };

    // ---- tray watcher ----
    let (tx, rx) = mpsc::channel(32);
    *TRAY_RECEIVER.lock().unwrap() = Some(rx);

    tokio::spawn(async move 
    {
        let _ = tray::start_watcher(tx).await;
    });

    let binded_output_name = std::env::args().nth(1);

    let start_mode = match binded_output_name
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    application(AppData::default, namespace, update, view).style(style).subscription(subscription)
        .settings(Settings
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
        })
        .run()
}



fn namespace() -> String { String::from("icebar") }



fn tray_stream(_: &TraySubscription) -> impl iced::futures::Stream<Item = Message>
{
    let receiver = TRAY_RECEIVER.lock().unwrap().take().expect("tray receiver already taken");
    tokio_stream::wrappers::ReceiverStream::new(receiver).map(Message::TrayEvent)
}



fn subscription(_: &AppData) -> iced::Subscription<Message>
{
    iced::Subscription::batch
    ([
        iced::Subscription::run_with(TraySubscription, tray_stream),
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    ])
}



fn update(app: &mut AppData, message: Message) -> Command<Message>
{
    match message
    {
        Message::IncrementPressed => { volume::volume(volume::VolumeAction::Increase); }
        Message::DecrementPressed => { volume::volume(volume::VolumeAction::Decrease); }

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
    // ---------- tray icons ----------
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

    // ---------- bar ----------
    let bar = row!
    [
            // RIGHT
            container
            (
                row!
                [
                    text(&app.modules.volume_data.volume_level),
                    button("Increment").on_press(Message::IncrementPressed),
                    button("Decrement").on_press(Message::DecrementPressed)
                ].spacing(10)
            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Left).align_y(iced::alignment::Vertical::Top),
            


            // CENTER
            container
            (
                row!
                [
                    text(&app.modules.clock_data.current_time)
                ].spacing(10)
            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Top),



            // RIGHT
            container
            (
                row!
                [
                    tray
                ]
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

