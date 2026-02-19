use std::time::Duration;
use iced::{Color, Element, Length, Task, Theme, event, mouse, theme::Style, time, widget::{button, column, container, text}};
use iced_layershell::{application, reexport::{Anchor, Layer, core::keyboard}, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use crate::tray::MenuItem;





#[to_layer_message]
#[derive(Debug, Clone)]
pub enum PopupMessage 
{
    Tick,
    Close,
    Action(String, String, i32, String),
    CursorMoved(iced::Point),
    MouseButtonClicked
}

#[derive(Default, Clone)]
pub struct PopupData 
{
    pub service: String,
    pub path: String,
    pub items: Vec<MenuItem>,
    pub cursor_is_inside_menu: bool
}





pub async fn run_popup(data: PopupData) -> Result<(), iced_layershell::Error> 
{
    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    println!("\nAmount Of Itens in The MenuItem Vector: {}\n", data.items.len());
    let popup_size_y = (data.items.len() * 37) as u32;
    application( move || data.clone(), namespace, update, view).style(style).subscription(subscription).settings(Settings 
    {
        layer_settings: LayerShellSettings 
        {
            layer: Layer::Overlay,
            size: Some((200, popup_size_y)),
            exclusive_zone: 0,
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::Exclusive,
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
    let sub = event::listen_with(|event, _status, _id| 
    {
        match event 
        {
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => 
            {
                match key 
                {
                    keyboard::Key::Named(keyboard::key::Named::Escape) => 
                    {
                        println!("Escape Pressed!!!");
                        Some(PopupMessage::Close)
                    }
                    _ => None,
                }
            }
            iced::Event::Mouse(mouse::Event::ButtonPressed(_)) => 
            {
                Some(PopupMessage::MouseButtonClicked)
            }
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => 
            {
                Some(PopupMessage::CursorMoved(position))
            }
            _ => None,
        }
    });



    iced::Subscription::batch
    ([
        time::every(Duration::from_secs(1)).map(|_| PopupMessage::Tick),
        sub
    ])
}



fn update(data: &mut PopupData, popup_message: PopupMessage) -> Task<PopupMessage> 
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
        PopupMessage::CursorMoved(position) =>
        {
            let menu_size = (data.items.len() * 37) as f32;
            println!("menu_size: {menu_size}");
            println!("mouse position: {:?}", position);
            data.cursor_is_inside_menu = false;
            if (position.y >= 0. && position.y <= menu_size) && (position.x >= 0. && position.x <= 200.)
            {
                println!("x and y is inside");
                data.cursor_is_inside_menu = true;
            };
        }
        PopupMessage::MouseButtonClicked =>
        {
            if !data.cursor_is_inside_menu
            {
                println!("Mouse Button Clicked Outside Menu");
                return iced::exit();
            }
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
    let button_vec = data.items.iter().map(|item| 
    {
        button(text(&item.label).width(Length::Fill).height(Length::Fill).center() )
            .on_press(PopupMessage::Action(data.service.to_string(), data.path.to_string(), item.id, item.label.to_string()))
            .style(|_: &Theme, status: button::Status| 
            {
                let mut style = button::Style::default();
                match status 
                {
                    button::Status::Hovered => 
                    {
                        style.background = Some(iced::Background::Color(Color::from_rgb(0.0, 0.5, 1.0))); // Blue background on hover
                        style.text_color = Color::WHITE;
                    }
                    button::Status::Pressed => 
                    {
                        style.background = Some(iced::Background::Color(Color::from_rgb(0.0, 0.3, 0.7))); // Darker blue when pressed
                    }
                    _ => 
                    {
                        // Default active state
                        style.background = Some(iced::Background::Color(Color::from_rgb(0.0, 0.7, 1.0)));
                        style.text_color = Color::BLACK;
                    }
                }
                style.border.width = 1.0;
                style.border.color = Color::BLACK;
                style
             })
            .width(Length::Fill).into()
    }).collect::<Vec<_>>();

    container
    (
        column
        (
            button_vec
        ).spacing(4).width(Length::Fill).height(Length::Fill)
    ).padding(6).width(Length::Fill).height(Length::Fill).into()
}


fn style(_: &PopupData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba(0.061,0.056,0.070,0.255),
        text_color: Color::WHITE
    }
}

