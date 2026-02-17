use iced::Alignment;
use iced::{Color, Element, Length, Task as Command};
use iced_layershell::settings::{LayerShellSettings, StartMode, Settings};
use iced_layershell::reexport::Anchor;
use iced_layershell::{application, to_layer_message};
use iced::widget::{button, container, row, text, image};
use iced::theme::Style;
use iced::time;
use std::time::Duration;
use tokio::sync::mpsc;
use iced::futures::StreamExt;
use once_cell::sync::Lazy;
use std::sync::Mutex;





use crate::clock::{ClockData, get_current_time};
use crate::fs::check_if_config_file_exists;
use crate::ron::read_ron_config;
use crate::tray::{TrayEvent, call_app_context_menu};
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
    TrayIconClicked(usize)
}





#[derive(Default)]
struct Modules
{
    volume_data: VolumeData,
    clock_data: ClockData,
    tray_icons: Vec<(Option<image::Handle>, String)>
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


    // tray watcher
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

    application(Modules::default, namespace, update, view)
        .style(style)
        .subscription(subscription)
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
        }).run()
}

fn namespace() -> String { String::from("icebar") }
fn tray_stream(_: &TraySubscription) -> impl iced::futures::Stream<Item = Message>
{
    let receiver = TRAY_RECEIVER.lock().unwrap().take().expect("tray receiver already taken");
    tokio_stream::wrappers::ReceiverStream::new(receiver).map(Message::TrayEvent)
}
fn subscription(_: &Modules) -> iced::Subscription<Message>
{
    let tray_subscription = iced::Subscription::run_with(TraySubscription, tray_stream);
    iced::Subscription::batch([tray_subscription, time::every(Duration::from_secs(1)).map(|_| Message::Tick)])
}

fn update(modules: &mut Modules, message: Message) -> Command<Message>
{
    match message
    {
        Message::Tick =>
        {
            modules.clock_data.current_time = get_current_time();
            modules.volume_data.volume_level = volume::volume(volume::VolumeAction::Get);
        }
        Message::IncrementPressed =>
        {
            volume::volume(volume::VolumeAction::Increase);
            modules.volume_data.volume_level = volume::volume(volume::VolumeAction::Get);
        }
        Message::DecrementPressed =>
        {
            volume::volume(volume::VolumeAction::Decrease);
            modules.volume_data.volume_level = volume::volume(volume::VolumeAction::Get);
        }
        Message::TrayEvent(event) =>
        {
            match event
            {
                TrayEvent::ItemRegistered(service) =>
                {
                    println!("Item Registered: {service}");
                    if !modules.tray_icons.iter().any(|(_, s)| s == &service)
                    {
                        modules.tray_icons.push((None, service));
                    }
                }
                TrayEvent::Icon { data, width, height } =>
                {
                    // Find existing placeholder and update
                    let mut found = false;
                    for (handle, _service) in modules.tray_icons.iter_mut()
                    {
                        if handle.is_none()
                        {
                            *handle = Some(image::Handle::from_rgba(width, height, data.clone()));
                            found = true;
                            break;
                        }
                    }
                    // If no placeholder, push new icon
                    if !found
                    {
                        modules.tray_icons.push((Some(image::Handle::from_rgba(width, height, data)), "unknown|unknown".into()));
                    }
                }
            }
        }
        Message::TrayIconClicked(idx) =>
        {
            if let Some((_, service_path)) = modules.tray_icons.get(idx)
            {
                println!("Opening context menu for icon {idx}: {service_path}");
                let parts: Vec<&str> = service_path.split('|').collect();
                if parts.len() == 2
                {
                    let service = parts[0].to_string();
                    let path = parts[1].to_string();
                    tokio::spawn(async move
                    {
                        let conn = zbus::Connection::session().await.unwrap();
                        let _ = call_app_context_menu(&conn, &service, &path, 0, 0).await;
                    });
                }
            }
        }
        _ => {}
    }

    Command::none()
}

fn view(modules: &Modules) -> Element<'_, Message>
{
    let tray_elements: Vec<Element<Message>> = modules.tray_icons.iter().enumerate().map(|(idx, (icon_opt, _service_path))|
    {
        let icon_widget: Element<Message> = if let Some(icon) = icon_opt
        {
            image(icon)
                .width(Length::Fixed(18.0))
                .height(Length::Fixed(18.0))
                .into() // convert Image<_> to Element<Message>
        }
        else
        {
            text("?")
                .size(12)
                .into() // convert Text<_> to Element<Message>
        };
        button(icon_widget)
            .padding(2)
            .on_press(Message::TrayIconClicked(idx))
            .into()
    }).collect();

    let tray_row = row(tray_elements).spacing(8);

    row!
    [
        // LEFT
        container(
            text(&modules.volume_data.volume_level).size(15)
        ).width(Length::Fill).align_x(iced::alignment::Horizontal::Left),

        // CENTER
        container(
            row!
            [
                button("Increment").on_press(Message::IncrementPressed),
                button("Decrement").on_press(Message::DecrementPressed),
            ].spacing(10)
        ).width(Length::Fill).align_x(iced::alignment::Horizontal::Center),

        // RIGHT
        container(
            row!
            [
                tray_row,
                text(&modules.clock_data.current_time).size(15)
            ]
        ).width(Length::Fill).align_x(iced::alignment::Horizontal::Right),
    ].padding(20).align_y(Alignment::Center).width(Length::Fill).into()
}

fn style(_: &Modules, _theme: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba(0.134, 0.206, 0.203, 0.255),
        text_color: Color::WHITE
    }
}
