// ============ IMPORTS ============
use iced::{Color, border::Radius, theme::Style, widget::button};





// ============ CRATES ============
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
pub struct UserStyle
{
    pub status: iced::widget::button::Status, 
    pub border_color_rgba: [u8;4], 
    pub hovered_text: [u8;3], 
    pub border_radius: [f32;4],
    pub normal_text: [u8;3], 
    pub hovered: [u8; 3], 
    pub border_size: f32, 
    pub pressed: [u8;3], 
    pub normal: [u8;3]
}





// ============ FUNCTIONS ============
pub fn style(app: &AppData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba8(app.ron_config.bar_background_color_rgba[0],app.ron_config.bar_background_color_rgba[1],app.ron_config.bar_background_color_rgba[2],app.ron_config.bar_background_color_rgba[3] as f32 / 100.),
        text_color: Color::WHITE
    }
}



pub fn set_style(user_style: UserStyle) -> iced::widget::button::Style
{
    let mut style = button::Style::default();
    match user_style.status 
    {
        button::Status::Hovered => 
        {
            style.background = Some(iced::Background::Color(Color::from_rgb8(user_style.hovered[0], user_style.hovered[1], user_style.hovered[2])));
            style.text_color = Color::from_rgb8(user_style.hovered_text[0], user_style.hovered_text[1], user_style.hovered_text[2]);
        }
        button::Status::Pressed => 
        {
            style.background = Some(iced::Background::Color(Color::from_rgb8(user_style.pressed[0], user_style.pressed[1], user_style.pressed[2])));
        }
        _ => 
        {
            style.background = Some(iced::Background::Color(Color::from_rgb8(user_style.normal[0], user_style.normal[1], user_style.normal[2])));
            style.text_color = Color::from_rgb8(user_style.normal_text[0], user_style.normal_text[1], user_style.normal_text[2]);
        }
    }
    style.border.color = Color::from_rgba8(user_style.border_color_rgba[0], user_style.border_color_rgba[1],  user_style.border_color_rgba[2], user_style.border_color_rgba[3] as f32);
    style.border.width = user_style.border_size;
    style.border.radius = Radius { top_left: user_style.border_radius[0], top_right: user_style.border_radius[1], bottom_left: user_style.border_radius[2], bottom_right: user_style.border_radius[3]};
    style
}
