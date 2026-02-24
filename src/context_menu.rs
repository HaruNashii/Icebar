// ============ IMPORTS ============
use iced_layershell::{application, reexport::{Anchor, Layer, core::keyboard}, settings::{LayerShellSettings, Settings, StartMode}, to_layer_message};
use iced::{Color, Element, Font, Length, Task, Theme, border::Radius, event, mouse, theme::Style, time, widget::{button, column, container, text}};
use std::time::Duration;





// ============ CRATES ============
use crate::{ron::{BarConfig, BarPosition}, set_style, tray::MenuItem};





// ============ ENUM/STRUCT, ETC ============
#[to_layer_message]
#[derive(Debug, Clone)]
pub enum ContextMenuMessage 
{
    Action(String, String, i32, String),
    CursorMoved(iced::Point),
    MouseButtonClicked,
    Close,
    Tick
}

#[derive(Default, Clone)]
pub struct ContextMenuData 
{
    pub cursor_is_inside_menu: bool,
    pub mouse_position: (i32, i32),
    pub monitor_size: (u32, u32),
    pub ron_config: BarConfig,
    pub items: Vec<MenuItem>,
    pub default_font: Font,
    pub service: String,
    pub path: String
}



// ============ FUNCTIONS ============
pub fn smart_popup_position(cursor_x: i32, cursor_y: i32, screen_w: i32, screen_h: i32, popup_w: i32, popup_h: i32) -> (i32, i32) 
{
    let mut x = cursor_x - popup_w / 2;
    let mut y = cursor_y - popup_h / 2;
    x = x.clamp(0, screen_w - popup_w);
    y = y.clamp(0, screen_h - popup_h);
    (x, y)
}



pub fn run_context_menu(data: ContextMenuData)
{
    let start_mode = match data.ron_config.display
    {
        Some(ref output) => StartMode::TargetScreen(output.to_string()),
        None => StartMode::Active,
    };

    let context_menu_size: (u32, u32) = (data.ron_config.context_menu_width, (data.items.len() * 37) as u32);
    let (x, y) = if let Some(forced_values) = data.ron_config.force_static_position_context_menu
    {
        forced_values
    }
    else 
    {
        smart_popup_position(data.mouse_position.0, data.mouse_position.1, data.monitor_size.0 as i32, data.monitor_size.1 as i32, context_menu_size.0 as i32, context_menu_size.1 as i32)
    };


    let anchor_position = match data.ron_config.bar_position
    {
        BarPosition::Down => Anchor::Bottom | Anchor::Left,
        BarPosition::Up => Anchor::Top | Anchor::Left
    };

    application( move || data.clone(), namespace, update, view).style(user_style).subscription(subscription).settings(Settings 
    {
        layer_settings: LayerShellSettings 
        {
            layer: Layer::Overlay,
            size: Some((context_menu_size.0, context_menu_size.1)),
            exclusive_zone: 0,
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::Exclusive,
            anchor: anchor_position,
            start_mode,
            margin: (y, 0, 0, x),
            ..Default::default()
        },
        ..Default::default()
    }).run().expect("Failed To Create ContextMenu Window");
}



fn namespace() -> String { "IceBar_Tray_ContextMenu".into() }



fn subscription(_: &ContextMenuData) -> iced::Subscription<ContextMenuMessage>
{
    let sub = event::listen_with(|event, _status, _id| 
    {
        match event 
        {
            iced::Event::Keyboard(keyboard::Event::KeyPressed {key: keyboard::Key::Named(keyboard::key::Named::Escape), .. }) => { Some(ContextMenuMessage::Close) }
            iced::Event::Mouse(mouse::Event::ButtonPressed(_)) => { Some(ContextMenuMessage::MouseButtonClicked) }
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => { Some(ContextMenuMessage::CursorMoved(position)) }
            _ => None,
        }
    });

    iced::Subscription::batch
    ([
        time::every(Duration::from_secs(1)).map(|_| ContextMenuMessage::Tick),
        sub
    ])
}



fn update(data: &mut ContextMenuData, popup_message: ContextMenuMessage) -> Task<ContextMenuMessage> 
{ 
    match popup_message
    {
        ContextMenuMessage::Action(service, path, id, label) =>
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
        ContextMenuMessage::CursorMoved(position) =>
        {
            let menu_size = (data.items.len() * 37) as f32;
            data.cursor_is_inside_menu = false;
            if (position.y >= 0. && position.y <= menu_size) && (position.x >= 0. && position.x <= 200.)
            {
                data.cursor_is_inside_menu = true;
            };
        }
        ContextMenuMessage::MouseButtonClicked =>
        {
            if !data.cursor_is_inside_menu
            {
                return iced::exit();
            }
        }
        ContextMenuMessage::Close =>
        {
            return iced::exit();
        }
        _=> {},
    }
    Task::none() 
}



fn view(data: &ContextMenuData) -> Element<'_, ContextMenuMessage> 
{
    let button_vec: Vec<Element<'_, ContextMenuMessage>> = data.items.iter().map(|item| { button(text(&item.label).font(data.default_font).size(data.ron_config.context_menu_text_size).width(Length::Fill).height(Length::Fill).center()).on_press(ContextMenuMessage::Action(data.service.to_string(), data.path.to_string(), item.id, item.label.to_string())).style(|_: &Theme, status: button::Status| 
    {
        let hovered = data.ron_config.context_menu_button_hovered_color_rgb;
        let hovered_text = data.ron_config.context_menu_button_hovered_text_color_rgb;
        let pressed = data.ron_config.context_menu_button_pressed_color_rgb;
        let normal = data.ron_config.context_menu_button_color_rgb;
        let normal_text = data.ron_config.context_menu_button_text_color_rgb;
        let border_color_rgba = data.ron_config.context_menu_border_color_rgba;
        let border_size = data.ron_config.context_menu_border_size;
        let border_radius = data.ron_config.context_menu_border_radius;
        set_style(crate::UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius })
    }).into()}).collect();


    container
    (
        column
        (
            button_vec
        ).spacing(0).width(Length::Fill).height(Length::Fill)
    ).padding(data.ron_config.context_menu_background_size).width(Length::Fill).height(Length::Fill).style(|_: &Theme| context_menu_background_button_style(&data.ron_config)).into()
}


fn context_menu_background_button_style(ron_config: &BarConfig) -> iced::widget::container::Style
{
    let mut background_style = container::Style::default();
    let bgc = ron_config.context_menu_background_color_rgba;
    let bgr = ron_config.context_menu_background_border_radius;
    background_style.border.color = Color::from_rgba8(bgc[0], bgc[1], bgc[2], bgc[3] as f32); 
    background_style.border.width = ron_config.context_menu_background_border_size;
    background_style.border.radius = Radius { top_left: bgr[0] as f32, top_right: bgr[1] as f32, bottom_left: bgr[2] as f32, bottom_right: bgr[3] as f32};
    background_style
}

fn user_style(app: &ContextMenuData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba8(app.ron_config.context_menu_background_color_rgba[0], app.ron_config.context_menu_background_color_rgba[1], app.ron_config.context_menu_background_color_rgba[2], app.ron_config.context_menu_background_color_rgba[3] as f32 / 100.),
        text_color: Color::from_rgb8(255, 255, 255)
    }
}

