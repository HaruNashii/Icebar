// ============ IMPORTS ============
use iced::{Alignment, Border, Color, Element, Theme, border::Radius, theme::Style, widget::{Space, button, container, row}};
use serde::{Serialize, Deserialize};



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

pub enum SideOption 
{
    Left,
    Right,
    All
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
                color: Color::from_rgb8(app.ron_config.bar_border_color_rgb[0], app.ron_config.bar_border_color_rgb[1], app.ron_config.bar_border_color_rgb[2])
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



pub fn with_unique_border<'a, Message: 'a>(content: impl Into<Element<'a, Message>>, color: Color, width: f32, height: f32, side_option: SideOption) -> Element<'a, Message>
{
    let strip: Element<'a, Message> = container(Space::new()).width(width).height(height).align_y(Alignment::Center).style(move |_theme| container::Style
    {
        background: Some(color.into()),
        ..Default::default()
    }).into();

    match side_option
    {
        SideOption::Left => row![strip, content.into()].align_y(Alignment::Center).into(),
        SideOption::Right => row![content.into(), strip].align_y(Alignment::Center).into(),
        SideOption::All => 
        {
            let new_strip: Element<'a, Message> = container(Space::new()).width(width).height(height).align_y(Alignment::Center).style(move |_theme| container::Style
            {
                background: Some(color.into()),
                ..Default::default()
            }).into();

            row!
            [
                strip, 
                content.into(), 
                new_strip
            ].align_y(Alignment::Center).into()
        }
    }
}


 
pub fn apply_separator<'a, Message: 'a>(element: Element<'a, Message>, flags: [bool; 2], color: Color, width: f32, height: f32) -> Element<'a, Message>
{
    match flags
    {
        [true,  false] => with_unique_border(element, color, width, height, SideOption::Left),
        [false, true]  => with_unique_border(element, color, width, height, SideOption::Right),
        [true,  true]  => with_unique_border(element, color, width, height, SideOption::All),
        [false, false] => element,
    }
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use iced::widget::button;
    use iced::{Background, Color};
    use iced::border::Radius;
    use super::*;
 
    fn base_user_style(status: button::Status) -> UserStyle
    {
        UserStyle
        {
            status,
            normal:            [10, 20, 30],
            normal_text:       [200, 210, 220],
            hovered:           [50, 60, 70],
            hovered_text:      [255, 255, 255],
            pressed:           [80, 90, 100],
            border_color_rgba: [1, 2, 3, 128],
            border_size:       2.5,
            border_radius:     [1.0, 2.0, 3.0, 4.0],
        }
    }
 
    #[test]
    fn set_style_active_background_is_normal_color()
    {
        let style = set_style(base_user_style(button::Status::Active));
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }
 
    #[test]
    fn set_style_active_text_is_normal_text_color()
    {
        let style = set_style(base_user_style(button::Status::Active));
        assert_eq!(style.text_color, Color::from_rgb8(200, 210, 220));
    }
 
    #[test]
    fn set_style_hovered_background_is_hovered_color()
    {
        let style = set_style(base_user_style(button::Status::Hovered));
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(50, 60, 70))));
    }
 
    #[test]
    fn set_style_hovered_text_is_hovered_text_color()
    {
        let style = set_style(base_user_style(button::Status::Hovered));
        assert_eq!(style.text_color, Color::from_rgb8(255, 255, 255));
    }
 
    #[test]
    fn set_style_pressed_background_is_pressed_color()
    {
        let style = set_style(base_user_style(button::Status::Pressed));
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(80, 90, 100))));
    }
 
    #[test]
    fn set_style_disabled_background_is_normal_color()
    {
        // Disabled falls through to the `_` arm which uses normal colors
        let style = set_style(base_user_style(button::Status::Disabled));
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }
 
    #[test]
    fn set_style_border_width_applied_correctly()
    {
        let style = set_style(base_user_style(button::Status::Active));
        assert_eq!(style.border.width, 2.5);
    }
 
    #[test]
    fn set_style_border_radius_applied_correctly()
    {
        let style = set_style(base_user_style(button::Status::Active));
        assert_eq!(style.border.radius, Radius { top_left: 1.0, top_right: 2.0, bottom_left: 3.0, bottom_right: 4.0 });
    }
 
    #[test]
    fn set_style_zero_border_size()
    {
        let style = set_style(UserStyle { border_size: 0.0, ..base_user_style(button::Status::Active) });
        assert_eq!(style.border.width, 0.0);
    }
 
    #[test]
    fn set_style_all_statuses_produce_some_background()
    {
        for status in [button::Status::Active, button::Status::Hovered, button::Status::Pressed, button::Status::Disabled]
        {
            let style = set_style(base_user_style(status));
            assert!(style.background.is_some(), "Expected background for status {:?}", status);
        }
    }
 
    #[test]
    fn set_style_active_and_hovered_backgrounds_differ()
    {
        let active  = set_style(base_user_style(button::Status::Active));
        let hovered = set_style(base_user_style(button::Status::Hovered));
        assert_ne!(active.background, hovered.background);
    }
 
    #[test]
    fn set_style_hovered_and_pressed_backgrounds_differ()
    {
        let hovered = set_style(base_user_style(button::Status::Hovered));
        let pressed = set_style(base_user_style(button::Status::Pressed));
        assert_ne!(hovered.background, pressed.background);
    }
 
    fn make_style(status: button::Status) -> iced::widget::button::Style
    {
        set_style(UserStyle
        {
            status,
            normal:            [10, 20, 30],
            normal_text:       [200, 210, 220],
            hovered:           [50, 60, 70],
            hovered_text:      [255, 255, 255],
            pressed:           [80, 90, 100],
            border_color_rgba: [1, 2, 3, 50],
            border_size:       2.0,
            border_radius:     [1.0, 2.0, 3.0, 4.0],
        })
    }
 
    // ---- set_style: Active/Normal ------------------------------------------
 
    #[test]
    fn set_style_active_uses_normal_background()
    {
        use iced::{Background, Color};
        let style = make_style(button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }
 
    #[test]
    fn set_style_active_uses_normal_text_color()
    {
        use iced::Color;
        let style = make_style(button::Status::Active);
        assert_eq!(style.text_color, Color::from_rgb8(200, 210, 220));
    }
 
    // ---- set_style: Hovered ------------------------------------------------
 
    #[test]
    fn set_style_hovered_uses_hovered_background()
    {
        use iced::{Background, Color};
        let style = make_style(button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(50, 60, 70))));
    }
 
    #[test]
    fn set_style_hovered_uses_hovered_text_color()
    {
        use iced::Color;
        let style = make_style(button::Status::Hovered);
        assert_eq!(style.text_color, Color::from_rgb8(255, 255, 255));
    }
 
    // ---- set_style: Pressed ------------------------------------------------
 
    #[test]
    fn set_style_pressed_uses_pressed_background()
    {
        use iced::{Background, Color};
        let style = make_style(button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(80, 90, 100))));
    }
 
    // ---- set_style: Border -------------------------------------------------
 
    #[test]
    fn set_style_border_width_applied()
    {
        let style = make_style(button::Status::Active);
        assert_eq!(style.border.width, 2.0);
    }
 
    #[test]
    fn set_style_border_radius_applied()
    {
        use iced::border::Radius;
        let style = make_style(button::Status::Active);
        assert_eq!(style.border.radius, Radius { top_left: 1.0, top_right: 2.0, bottom_left: 3.0, bottom_right: 4.0 });
    }
 
    #[test]
    fn set_style_border_color_applied()
    {
        use iced::Color;
        let style = make_style(button::Status::Active);
        assert_eq!(style.border.color, Color::from_rgba8(1, 2, 3, 50.0));
    }
 
    // ---- set_style: all statuses produce non-None background ---------------
 
    #[test]
    fn set_style_all_statuses_produce_background()
    {
        for status in [button::Status::Active, button::Status::Hovered, button::Status::Pressed, button::Status::Disabled]
        {
            let style = make_style(status);
            assert!(style.background.is_some(), "Expected background for status {:?}", status);
        }
    }

    // ---- orient_text --------------------------------------------------------
 
    #[test]
    fn orient_text_horizontal_unchanged()
    {
        let result = orient_text("hello world", &TextOrientation::Horizontal);
        assert_eq!(result, "hello world");
    }
 
    #[test]
    fn orient_text_vertical_joins_with_newlines()
    {
        let result = orient_text("abc", &TextOrientation::Vertical);
        assert_eq!(result, "a\nb\nc");
    }
 
    #[test]
    fn orient_text_vertical_space_becomes_blank_line()
    {
        let result = orient_text("a b", &TextOrientation::Vertical);
        assert_eq!(result, "a\n \nb");
    }
 
    #[test]
    fn orient_text_vertical_trims_trailing_whitespace()
    {
        let result = orient_text("ab ", &TextOrientation::Vertical);
        assert_eq!(result, "a\nb");
    }
 
    #[test]
    fn orient_text_empty_string()
    {
        assert_eq!(orient_text("", &TextOrientation::Horizontal), "");
        assert_eq!(orient_text("", &TextOrientation::Vertical),   "");
    }
}
