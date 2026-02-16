use iced::Alignment;
use iced::{Color, Element, Event, Length, Task as Command, event};
use iced_layershell::settings::{LayerShellSettings, StartMode, Settings};
use crate::clock::ClockData;
use crate::clock::get_current_time;
use crate::fs::check_if_config_file_exists;
use iced::widget::{button, container, row, text};
use iced_layershell::reexport::Anchor;
use iced_layershell::to_layer_message;
use iced_layershell::application;
use crate::ron::read_ron_config;
use crate::volume::VolumeData;
use crate::volume::volume;
use iced::theme::Style;





mod volume;
mod clock;
mod fs;
mod ron;





#[to_layer_message]
#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Message 
{
    IncrementPressed,
    DecrementPressed,
    IcedEvent(Event),
}





#[derive(Default)]
struct Modules
{
    volume_data: VolumeData,
    clock_data: ClockData
}





pub fn main() -> Result<(), iced_layershell::Error> 
{
    check_if_config_file_exists();
    let ron_config = read_ron_config();
    let anchor_position = match ron_config.bar_position.as_str() 
    {
        "Up" => Anchor::Top | Anchor::Left | Anchor::Right,
        "Down" => Anchor::Bottom | Anchor::Left | Anchor::Right,
        "Left" => Anchor::Left | Anchor::Top | Anchor::Bottom,
        "Right" => Anchor::Right | Anchor::Top | Anchor::Bottom,
        _ => Anchor::Top | Anchor::Left | Anchor::Right, // default
    };

    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name 
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    application(Modules::default, namespace, update, view)
        .style(style)
        .subscription(subscription)
        .settings
        (Settings 
            {
                layer_settings: LayerShellSettings 
                {
                    size: Some((0, ron_config.bar_size)),
                    exclusive_zone: (ron_config.bar_size as i32),
                    anchor: anchor_position,
                    start_mode,
                    ..Default::default()
                },
                ..Default::default()
            }
        )
        .run()
}





fn namespace() -> String { String::from("icebar") }
fn subscription(_: &Modules) -> iced::Subscription<Message> { event::listen().map(Message::IcedEvent) }
fn update(modules: &mut Modules, message: Message) -> Command<Message> 
{
    modules.clock_data.current_time = get_current_time();
    modules.volume_data.volume_level = volume(volume::VolumeAction::Get);
    match message 
    {
        Message::IcedEvent(_) => Command::none(),
        Message::IncrementPressed => 
        {
            volume(volume::VolumeAction::Increase);
            Command::none()
        }
        Message::DecrementPressed => 
        {
            volume(volume::VolumeAction::Decrease);
            Command::none()
        }
        _ => unreachable!(),
    }
}



fn view(modules: &Modules) -> Element<'_, Message> 
{
    row!
    [
        // LEFT
        container
        (
            text(&modules.volume_data.volume_level).size(15)
        )
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Left),


        // CENTER
        container
        (
            row!
            [
                button("Increment").on_press(Message::IncrementPressed),
                button("Decrement").on_press(Message::DecrementPressed),
            ]
            .spacing(10)
        )
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center),


        // RIGHT
        container
        (
            text(&modules.clock_data.current_time).size(15)
        )
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right),
    ]
    .padding(20)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn style(_: &Modules, theme: &iced::Theme) -> iced::theme::Style 
{
    Style 
    {
        background_color: Color::from_rgba(0.134, 0.206, 0.203, 0.255),
        text_color: theme.palette().text,
    }
}

