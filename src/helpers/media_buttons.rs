// ============ IMPORTS ============
use iced::{Element, widget::container, Alignment, Theme, widget::{button, text}};





// ============ CRATES ============
use crate::modules::media_player::define_media_player_buttons_style;
use crate::update::Message;
use crate::AppData;





// ============ FUNCTIONS ============
pub fn create_media_button<'a>(app: &'a AppData, label: String, message: Message, color: iced::Color) -> Element<'a, Message> 
{
    container
    (
        button
        (
            text(label)
            .color(color)
            .wrapping(iced::widget::text::Wrapping::Word)
            .font(app.default_font)
            .size(app.ron_config.media_player_button_text_size)
            .center()
        )
        .style(|_: &Theme, status: button::Status| 
        {
            define_media_player_buttons_style(app, status)
        }).on_press(message)).align_y(Alignment::Center).into()
}
