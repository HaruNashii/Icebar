// ============ IMPORTS ============
use iced::{Alignment, Border, Color, Element, Theme, Vector, border::Radius, theme::Style, widget::{Space, button, column, container, row}};
use iced_layershell::reexport::core::{Degrees, gradient::Linear};
use serde::{Serialize, Deserialize};





// ============ CRATES ============
use crate::helpers::color::{ColorType, Gradient};
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum TextOrientation
{
    #[default] Horizontal,
    Vertical
}

pub struct UserStyle
{
    pub normal_background:  Option<iced::Background>,
    pub hovered_background: Option<iced::Background>,
    pub pressed_background: Option<iced::Background>,
    pub status:             iced::widget::button::Status,
    pub border_color:       ColorType,
    pub hovered_text:       ColorType,
    pub border_radius:      [f32; 4],
    pub normal_text:        ColorType,
    pub pressed_text:       ColorType,
    pub border_size:        f32,
    pub shadow_color:       Option<ColorType>,
    pub shadow_x:           f32,
    pub shadow_y:           f32,
    pub shadow_blur:        f32
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SideOption 
{
    #[default] Left,
    Right,
    Up,
    Down,
    LeftAndRight,
    UpAndDown
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


pub fn match_color_or_gradient(gradient: Option<&Gradient>, color: ColorType) -> Option<iced::Background>
{
    match gradient
    {
        Some(Gradient::Gradient(received_gradient)) =>
        {
            let mut gradient = Linear::new(Degrees(received_gradient.0));
            for entry in &received_gradient.1
            {
                gradient = gradient.add_stop(entry.0, entry.1.to_iced_color());
            }
            Some(iced::Background::Gradient(gradient.into()))
        },
        None => Some(iced::Background::Color(color.to_iced_color()))
    }
}


pub fn set_style(user_style: UserStyle) -> iced::widget::button::Style
{
    let mut style = button::Style
    {
        border: Border
        {
            color:  user_style.border_color.to_iced_color(),
            width:  user_style.border_size,
            radius: Radius { top_left: user_style.border_radius[0], top_right: user_style.border_radius[1], bottom_left: user_style.border_radius[2], bottom_right: user_style.border_radius[3]}
        },
        ..Default::default()
    };

    if let Some(shadow_color) = user_style.shadow_color 
    {
        style.shadow.color       = shadow_color.to_iced_color();
        style.shadow.offset      = Vector::new(user_style.shadow_x, user_style.shadow_y);
        style.shadow.blur_radius = user_style.shadow_blur;
    };

    match user_style.status 
    {
        button::Status::Hovered =>
        {
            style.background = user_style.hovered_background;
            style.text_color = user_style.hovered_text.to_iced_color();
        }
        button::Status::Pressed =>
        {
            style.background = user_style.pressed_background;
            style.text_color = user_style.pressed_text.to_iced_color();
        }
        _ =>
        {
            style.background = user_style.normal_background;
            style.text_color = user_style.normal_text.to_iced_color();
        }
    }

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
    let color = app.ron_config.general.bar_background_color.to_iced_color();
    let bar_style: container::Style = 
    {
        container::Style 
        {
            border: Border 
            {
                radius: Radius { top_left: app.ron_config.general.bar_border_radius[0], top_right: app.ron_config.general.bar_border_radius[1], bottom_left: app.ron_config.general.bar_border_radius[2], bottom_right: app.ron_config.general.bar_border_radius[3]},
                width: app.ron_config.general.bar_border_size, 
                color: app.ron_config.general.bar_border_color.to_iced_color()
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
        SideOption::LeftAndRight => 
        {
            let new_strip: Element<'a, Message> = container(Space::new()).width(width).height(height).align_x(Alignment::Center).align_y(Alignment::Center).style(move |_theme| container::Style
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

        SideOption::Up => column![strip, content.into()].align_x(Alignment::Center).into(),
        SideOption::Down => column![content.into(), strip].align_x(Alignment::Center).into(),
        SideOption::UpAndDown => 
        {
            let new_strip: Element<'a, Message> = container(Space::new()).width(width).height(height).align_x(Alignment::Center).align_y(Alignment::Center).style(move |_theme| container::Style
            {
                background: Some(color.into()),
                ..Default::default()
            }).into();

            column!
            [
                strip, 
                content.into(), 
                new_strip
            ].align_x(Alignment::Center).into()
        }
    }
}


 
pub fn apply_separator<'a, Message: 'a>(element: Element<'a, Message>, flags: Option<SideOption>, color: Color, width: f32, height: f32) -> Element<'a, Message>
{
    match flags
    {
        Some(flags) => with_unique_border(element, color, width, height, flags),
        None => element
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
            normal_text:         ColorType::RGB([200, 210, 220]),
            hovered_text:        ColorType::RGB([255, 255, 255]),
            pressed_text:        ColorType::RGB([255, 255, 255]),
            border_color:        ColorType::RGB([1, 2, 3]),
            border_size:         2.5,
            border_radius:       [1.0, 2.0, 3.0, 4.0],
            normal_background:   match_color_or_gradient(None, ColorType::RGB([10, 20, 30])),
            hovered_background:  match_color_or_gradient(None, ColorType::RGB([50, 60, 70])),
            pressed_background:  match_color_or_gradient(None, ColorType::RGB([80, 90, 100])),
            shadow_color:        None,
            shadow_x:            0.0,
            shadow_y:            0.0,
            shadow_blur:         0.0
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
            normal_text:        ColorType::RGB([200, 210, 220]),
            hovered_text:       ColorType::RGB([255, 255, 255]),
            pressed_text:       ColorType::RGB([255, 255, 255]),
            border_color:       ColorType::RGB([1, 2, 3]),
            border_size:        2.0,
            border_radius:      [1.0, 2.0, 3.0, 4.0],
            normal_background:  match_color_or_gradient(None, ColorType::RGB([10, 20, 30])),
            hovered_background: match_color_or_gradient(None, ColorType::RGB([50, 60, 70])),
            pressed_background: match_color_or_gradient(None, ColorType::RGB([80, 90, 100])),
            shadow_color:       None,
            shadow_x:           0.0,
            shadow_y:           0.0,
            shadow_blur:        0.0
        })
    }
 
 
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
 
 
    #[test]
    fn set_style_pressed_uses_pressed_background()
    {
        use iced::{Background, Color};
        let style = make_style(button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(80, 90, 100))));
    }
 
 
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
        assert_eq!(style.border.color, Color::from_rgb8(1, 2, 3));
    }
 
 
    #[test]
    fn set_style_all_statuses_produce_background()
    {
        for status in [button::Status::Active, button::Status::Hovered, button::Status::Pressed, button::Status::Disabled]
        {
            let style = make_style(status);
            assert!(style.background.is_some(), "Expected background for status {:?}", status);
        }
    }

 
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


    #[test]
    fn orient_text_horizontal_preserves_spaces()
    {
        assert_eq!(orient_text("a b c", &TextOrientation::Horizontal), "a b c");
    }

    #[test]
    fn orient_text_vertical_single_char()
    {
        assert_eq!(orient_text("X", &TextOrientation::Vertical), "X");
    }

    #[test]
    fn orient_text_vertical_two_chars_joined_by_newline()
    {
        assert_eq!(orient_text("AB", &TextOrientation::Vertical), "A\nB");
    }

    #[test]
    fn orient_text_vertical_space_becomes_narrow_space_on_new_line()
    {
        let result = orient_text("a b", &TextOrientation::Vertical);
        assert!(result.contains('\n'));
    }

    #[test]
    fn orient_text_vertical_trims_trailing_newline()
    {
        let result = orient_text("A ", &TextOrientation::Vertical);
        assert!(!result.ends_with('\n'));
    }

    #[test]
    fn orient_text_horizontal_unicode_string_unchanged()
    {
        let input = "héllo wörld";
        assert_eq!(orient_text(input, &TextOrientation::Horizontal), input);
    }


    #[test]
    fn match_color_or_gradient_none_returns_solid_color_background()
    {
        use crate::helpers::color::ColorType;
        use iced::Background;
        let bg = match_color_or_gradient(None, ColorType::RGB([255, 0, 0]));
        assert!(matches!(bg, Some(Background::Color(_))));
    }

    #[test]
    fn match_color_or_gradient_none_color_matches_input()
    {
        use crate::helpers::color::ColorType;
        use iced::{Background, Color};
        let bg = match_color_or_gradient(None, ColorType::RGB([100, 150, 200])).unwrap();
        if let Background::Color(c) = bg
        {
            let expected = Color::from_rgb8(100, 150, 200);
            assert!((c.r - expected.r).abs() < 0.01);
            assert!((c.g - expected.g).abs() < 0.01);
            assert!((c.b - expected.b).abs() < 0.01);
        }
        else { panic!("expected Color background"); }
    }

    #[test]
    fn match_color_or_gradient_with_gradient_returns_gradient_background()
    {
        use crate::helpers::color::{ColorType, Gradient};
        use iced::Background;
        let g = Gradient::Gradient((45.0, vec![
            (0.0, ColorType::RGB([0,   0,   0  ])),
            (1.0, ColorType::RGB([255, 255, 255])),
        ]));
        let bg = match_color_or_gradient(Some(&g), ColorType::RGB([0, 0, 0]));
        assert!(matches!(bg, Some(Background::Gradient(_))));
    }

    #[test]
    fn match_color_or_gradient_with_empty_gradient_stops_still_returns_gradient()
    {
        use crate::helpers::color::{ColorType, Gradient};
        use iced::Background;
        let g = Gradient::Gradient((0.0, vec![]));
        let bg = match_color_or_gradient(Some(&g), ColorType::RGB([0, 0, 0]));
        assert!(matches!(bg, Some(Background::Gradient(_))));
    }
}
