use std::time::Duration;
use iced::{Color, Element, Length, Task, theme::Style, time, widget::{button, column, container, text}};
use iced_layershell::{application, reexport::{Anchor, Layer}, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use crate::tray::MenuItem;





#[to_layer_message]
#[derive(Debug, Clone)]
pub enum PopupMessage 
{
    Tick,
    Close,
    Action(String, String, i32, String),
}

#[derive(Default, Clone)]
pub struct PopupData 
{
    pub service: String,
    pub path: String,
    pub items: Vec<MenuItem>,
}






pub async fn run_popup(data: PopupData) -> Result<(), iced_layershell::Error> 
{
    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    application( move || data.clone(), namespace, update, view).style(style).subscription(subscription)
    .settings(Settings 
    {
        layer_settings: LayerShellSettings 
        {
            layer: Layer::Overlay,
            size: Some((200, 600)),
            exclusive_zone: 0,
            anchor: Anchor::Top | Anchor::Right,
            start_mode,
            margin: (30, 10, 0, 0),
            ..Default::default()
        },
        ..Default::default()
    }).run()
}



fn namespace() -> String { "icebar-popup".into() }
fn subscription(_: &PopupData) -> iced::Subscription<PopupMessage>
{
    iced::Subscription::batch
    ([
        time::every(Duration::from_secs(1)).map(|_| PopupMessage::Tick)
    ])
}



fn update(_: &mut PopupData, popup_message: PopupMessage) -> Task<PopupMessage> 
{ 
    match popup_message
    {
        PopupMessage::Action(service, path, id, label) =>
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
            return iced::exit();
        }
        PopupMessage::Close =>
        {
            return iced::exit();
        }
        _=> {},
    }
    Task::none() 
}



fn view(data: &PopupData) -> Element<'_, PopupMessage> 
{
    container
    (
        column
        (
            data.items.iter().map(|item| 
            {
                button(text(&item.label)).on_press(PopupMessage::Action(data.service.to_string(), data.path.to_string(), item.id, item.label.to_string())).into()
            }).collect::<Vec<_>>()
        ).spacing(4)
    )
    .padding(6).width(Length::Shrink).height(Length::Shrink).into()
}



fn style(_: &PopupData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba(0.134,0.206,0.203,0.255),
        text_color: Color::WHITE
    }
}

