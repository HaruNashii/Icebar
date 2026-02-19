use iced::{futures::StreamExt, theme::Style, time, widget::{button, container, row, text, image, column, stack, Space}, Alignment, Color, Element, Length, Task as Command};
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
    MenuAction(String, String, i32, String),

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

    // active popup menu
    tray_menu: Option<(String, String, Vec<tray::MenuItem>)>,
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
                            *handle =
                                Some(image::Handle::from_rgba(width, height, data.clone()));
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
            app.modules.tray_menu = Some((service, path, items));
        }

        Message::MenuAction(service, path, id, label) =>
        {
            println!("\n===# Menu Action Activated!!! #===");
            println!("Label: {label}");
            println!("Service: {service}");
            println!("Menu Path: {path}");
            println!("Id: {id}");
            tokio::spawn(async move 
            {
                let _ = crate::tray::activate_menu_item(&service, &path, id).await;
            });
            app.modules.tray_menu = None;
        }

        Message::CloseMenu => app.modules.tray_menu = None,
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
            container(text(&app.modules.volume_data.volume_level)).width(Length::Fill),

            container
            (
                row!
                [
                    button("Increment").on_press(Message::IncrementPressed),
                    button("Decrement").on_press(Message::DecrementPressed),
                ].spacing(10)
            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Center),

            container
            (
                row!
                [
                    tray,
                    text(&app.modules.clock_data.current_time)
                ]
            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Right),
        ].padding(20).align_y(Alignment::Center);


    // ---------- overlay menu ----------
    let overlay: Element<_> = if let Some((service,path,items)) = &app.modules.tray_menu
    {
        container
        (
            column
            (
                items.iter().map(|item|


                    button(text(&item.label)).on_press(Message::MenuAction(service.clone(), path.clone(), item.id, item.label.clone())).into()

                ).collect::<Vec<_>>()
            ).spacing(4).padding(6)
        ).width(Length::Shrink).into()
    }
    else
    {
        Space::new().into()
    };

    stack![bar, overlay].into()
}

fn style(_: &AppData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba(0.134,0.206,0.203,0.255),
        text_color: Color::WHITE
    }
}

