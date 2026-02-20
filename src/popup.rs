use std::time::Duration;
use iced::{Color, Element, Length, Task, Theme, event, mouse, theme::Style, time, widget::{button, column, container, text}};
use iced_layershell::{application, reexport::{Anchor, Layer, core::keyboard}, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use crate::{ron::BarConfig, tray::MenuItem};




#[to_layer_message]
#[derive(Debug, Clone)]
pub enum PopupMessage 
{
    Tick,
    Close,
    Action(String, String, i32, String),
    CursorMoved(iced::Point),
    MouseButtonClicked,
}

#[derive(Default, Clone)]
pub struct PopupData 
{
    pub service: String,
    pub path: String,
    pub items: Vec<MenuItem>,
    pub cursor_is_inside_menu: bool,
    pub ron_config: BarConfig,
    pub popup_position: (i32, i32),
    pub monitor_size: (u32, u32)
}



pub fn smart_popup_position(cursor_x: i32, cursor_y: i32, screen_w: i32, screen_h: i32, popup_w: i32, popup_h: i32) -> (i32, i32) 
{
    let mut x = cursor_x - popup_w / 2;
    let mut y = cursor_y - popup_h / 2;

    x = x.clamp(0, screen_w - popup_w);
    y = y.clamp(0, screen_h - popup_h);

    (x, y)
}

pub async fn run_popup(data: PopupData) -> Result<(), iced_layershell::Error> 
{
    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name
    {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };


    let popup_size: (u32, u32) = (data.ron_config.context_menu_width, (data.items.len() * 37) as u32);
    let (x, y) = smart_popup_position(data.popup_position.0, data.popup_position.1, data.monitor_size.0 as i32, data.monitor_size.1 as i32, popup_size.0 as i32, popup_size.1 as i32);

    application( move || data.clone(), namespace, update, view).style(style).subscription(subscription).settings(Settings 
    {
        layer_settings: LayerShellSettings 
        {
            layer: Layer::Overlay,
            size: Some((popup_size.0, popup_size.1)),
            exclusive_zone: 0,
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::Exclusive,
            anchor: Anchor::Top | Anchor::Left,
            start_mode,
            margin: (y, 0, 0, x),
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
            iced::Event::Keyboard(keyboard::Event::KeyPressed {key: keyboard::Key::Named(keyboard::key::Named::Escape), .. }) => 
            {
                println!("Escape Pressed!!!");
                Some(PopupMessage::Close)
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
            data.cursor_is_inside_menu = false;
            if (position.y >= 0. && position.y <= menu_size) && (position.x >= 0. && position.x <= 200.)
            {
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
                let hovered = data.ron_config.context_menu_button_hovered;
                let hovered_text = data.ron_config.context_menu_button_hovered_text;
                let pressed = data.ron_config.context_menu_button_pressed;
                let normal = data.ron_config.context_menu_button;
                let normal_text = data.ron_config.context_menu_button_text;
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
                let border_color = data.ron_config.context_menu_border_color;
                style.border.width = data.ron_config.context_menu_border_size;
                style.border.color = Color::from_rgb8(border_color[0], border_color[0],  border_color[0]);
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


fn style(app: &PopupData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba8(app.ron_config.context_menu_background_color_rgba[0], app.ron_config.context_menu_background_color_rgba[1], app.ron_config.context_menu_background_color_rgba[2], app.ron_config.context_menu_background_color_rgba[3] as f32 / 100.),
        text_color: Color::from_rgb(1.0, 1., 1.)
    }
}

