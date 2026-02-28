// ============ IMPORTS ============
use serde::{Serialize, Deserialize};
use iced::{Color, border::Radius, Border, Theme, theme::Style, widget::{button, container}};



// ============ CRATES ============
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum TextOrientation
{
    #[default] Horizontal,
    Vertical
}

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
pub fn style(_: &AppData, _: &iced::Theme) -> Style
{
    Style
    {
        background_color: Color::from_rgba8(0, 0, 0, 0.),
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



pub fn orient_text(input: &str, orientation: &TextOrientation) -> String 
{
    match orientation 
    {
        TextOrientation::Horizontal => input.to_string(),
        TextOrientation::Vertical => input.chars().map(|c| { if c == ' ' { " ".to_string() } else { c.to_string() } }).collect::<Vec<_>>().join("\n").trim_end().to_string()
    }
}


pub fn bar_style(app: &AppData) -> impl Fn(&Theme) -> container::Style
{
    let color = Color::from_rgba8(app.ron_config.bar_background_color_rgba[0], app.ron_config.bar_background_color_rgba[1],app.ron_config.bar_background_color_rgba[2], app.ron_config.bar_background_color_rgba[3] as f32 / 100.);
    let bar_style: container::Style = 
    {
        container::Style 
        {
            border: Border 
            {
                radius: Radius { top_left: app.ron_config.bar_border_radius[0], top_right: app.ron_config.bar_border_radius[1], bottom_left: app.ron_config.bar_border_radius[2], bottom_right: app.ron_config.bar_border_radius[3]},
                width: app.ron_config.bar_border_size, 
                color: Color::from_rgba8(app.ron_config.bar_border_color_rgba[0], app.ron_config.bar_border_color_rgba[1], app.ron_config.bar_border_color_rgba[2], app.ron_config.bar_border_color_rgba[3] as f32)
            },
            background: { Some(iced::Background::Color(color)) },
            text_color: None,
            ..Default::default()
        }
    };

    move |_theme: &Theme| 
    {
        bar_style
    }
}
