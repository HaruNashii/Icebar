// ============ IMPORTS ============
use iced_layershell::reexport::{Anchor, Layer};
use iced::{Alignment, Color, Element, Length, Task, Theme, border::Radius, widget::{button, container, row, text}};
use iced_layershell::reexport::NewLayerShellSettings;



// ============ CRATES ============
use crate::{set_style, AppData, WindowInfo, update::Message};
use crate::helpers::color::ColorType;




// ============ FUNCTIONS ============
pub fn create_warning(app: &mut AppData) -> Task<Message>
{
    println!("\n=== WARNING ===");
    println!("Creating Warning Window!!!");
    let id = iced::window::Id::unique();
    app.ids.insert(id, WindowInfo::Warning);
    Task::done(Message::NewLayerShell 
    {
        settings: NewLayerShellSettings 
        {
            layer: Layer::Overlay,
            size: Some((0, 60)),
            exclusive_zone: Some(60),
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::None,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            margin: Some((0, 0, 0, 0)),
            ..Default::default()
        },
        id,
    })
}


pub fn warning_view<'a>(err: &'a String) -> Element<'a, Message> 
{
    let row: Element<'a, Message> = row!
    (
        text(err).color(Color::from_rgb8(255, 255, 255)).size(20).width(Length::Fill).height(Length::Fill),

        button
        (
            text("X")
            .color(Color::from_rgb8(255, 255, 255))
            .align_y(Alignment::Center)
            .align_y(Alignment::Center)
            .size(15)
            .width(Length::Fill)
            .height(Length::Fill)
            .center()
        )
        .width(50)
        .height(50)
        .on_press(Message::CloseWarning)
        .style(|_: &Theme, status: button::Status| 
        {
            let hovered =           ColorType::RGB([225, 255, 5]);
            let hovered_text =      ColorType::RGB([255, 255, 255]);
            let pressed =           ColorType::RGB([125, 155, 5]);
            let normal =            ColorType::RGB([225, 50, 50]);
            let normal_text =       ColorType::RGB([255, 255, 255]); 
            let border_color =      ColorType::RGB([225, 255, 5]);
            let border_size =       1.0;
            let border_radius =     [0., 0., 0., 0.];
            set_style(crate::UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color, border_size, border_radius, hovered_gradient: None, normal_gradient: None, pressed_gradient: None })
        })
    ).into();
    
    container
    (
        row
    )
    .padding(1)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_: &Theme| warning_background_button_style())
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}



fn warning_background_button_style() -> iced::widget::container::Style
{
    let mut background_style = iced::widget::container::Style 
    { 
        background: Some(iced::Background::Color(Color::from_rgb8(225, 50, 50))), 
        ..Default::default()
    };
    background_style.background =       Some(iced::Background::Color(Color::from_rgb8(225, 50, 50)));
    background_style.border.color =     Color::from_rgb8(225, 255, 5);
    background_style.border.width =     1.;
    background_style.border.radius =    Radius { top_left: 0., top_right: 0., bottom_left: 0., bottom_right: 0.};

    background_style
}
