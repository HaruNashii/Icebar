// ============ IMPORTS ============
use iced::widget::button;
use chrono::{Local, Utc};
use chrono_tz::Tz;




// ============ CRATES ============
use crate::helpers::style::{UserStyle, set_style};
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct ClockData
{
    pub current_time: String
}





// ============ FUNCTIONS ============
pub fn get_current_time(time_format: &str, option_time_zone: &Option<(String, u32)>) -> String 
{
    if let Some((time_zone, _)) = option_time_zone
    {
        let result_timezone: Result<Tz, _> = time_zone.parse();
        match result_timezone
        {
            Ok(tz) => return Utc::now().with_timezone(&tz).format(time_format).to_string(),
            Err(err) => println!("Warning!!!: Failed to parse timezone. Err: {err}")
        }
    }
    
    Local::now().format(time_format).to_string() 
}



pub fn define_clock_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    if app.is_showing_alt_clock
    {
        let hovered =           app.ron_config.alt_clock_button_hovered_color;
        let hovered_text =      app.ron_config.alt_clock_button_hovered_text_color;
        let pressed =           app.ron_config.alt_clock_button_pressed_color;
        let normal =            app.ron_config.alt_clock_button_color;
        let normal_text =       app.ron_config.alt_clock_text_color;
        let border_size =       app.ron_config.alt_clock_border_size;
        let border_color =      app.ron_config.alt_clock_border_color;
        let border_radius =     app.ron_config.alt_clock_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.alt_clock_button_gradient_color.clone(), hovered_gradient: app.ron_config.alt_clock_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.alt_clock_button_pressed_gradient_color.clone() })
    }
    else
    {
        let hovered =           app.ron_config.clock_button_hovered_color;
        let hovered_text =      app.ron_config.clock_button_hovered_text_color;
        let pressed =           app.ron_config.clock_button_pressed_color;
        let normal =            app.ron_config.clock_button_color;
        let normal_text =       app.ron_config.clock_text_color;
        let border_size =       app.ron_config.clock_border_size;
        let border_color =      app.ron_config.clock_border_color;
        let border_radius =     app.ron_config.clock_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.clock_button_gradient_color.clone(), hovered_gradient: app.ron_config.clock_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.clock_button_pressed_gradient_color.clone() })
    }

}



pub fn cycle_clock_timezones(app: &mut AppData)
{
    if let Some((current_time_zone, index)) = &app.current_clock_timezone && let Some(timezones) = &app.ron_config.clock_timezones && !timezones.is_empty()
    {
        if (*index as usize + 1) <= (timezones.len().saturating_sub(1))
        {
            println!("\n=== CLOCK ACTION ===");
            println!("Cycling Timezone!: {} -> {}", current_time_zone, timezones[*index as usize + 1]);
            app.current_clock_timezone = Some((timezones[*index as usize + 1].clone(), (index + 1)));
        }
        else
        {
            println!("\n=== CLOCK ACTION ===");
            println!("Cycling Timezone!: {} -> {}", current_time_zone, timezones[0]);
            app.current_clock_timezone = Some((timezones[0].clone(), 0));
        };
    };
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use iced::{widget::button, Background, Color};
    use crate::helpers::color::ColorType;
 
    fn make_clock_app(is_alt: bool) -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.is_showing_alt_clock = is_alt;
        app.ron_config.clock_button_color = ColorType::RGB([10, 20, 30]);
        app.ron_config.clock_button_hovered_color = ColorType::RGB([15, 25, 35]);
        app.ron_config.clock_button_pressed_color = ColorType::RGB([5, 10, 15]);
        app.ron_config.alt_clock_button_color = ColorType::RGB([200, 100, 50]);
        app.ron_config.alt_clock_button_hovered_color = ColorType::RGB([210, 110, 60]);
        app.ron_config.alt_clock_button_pressed_color = ColorType::RGB([190, 90, 40]);
        app
    }
 
    #[test]
    fn clock_style_active_normal_uses_clock_color()
    {
        let style = define_clock_style(&make_clock_app(false), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }
 
    #[test]
    fn clock_style_active_alt_uses_alt_clock_color()
    {
        let style = define_clock_style(&make_clock_app(true), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(200, 100, 50))));
    }
 
    #[test]
    fn clock_style_normal_and_alt_backgrounds_differ()
    {
        let normal = define_clock_style(&make_clock_app(false), button::Status::Active);
        let alt    = define_clock_style(&make_clock_app(true),  button::Status::Active);
        assert_ne!(normal.background, alt.background);
    }
 
    #[test]
    fn clock_style_hovered_normal_uses_hovered_color()
    {
        let style = define_clock_style(&make_clock_app(false), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(15, 25, 35))));
    }
 
    #[test]
    fn clock_style_hovered_alt_uses_alt_hovered_color()
    {
        let style = define_clock_style(&make_clock_app(true), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(210, 110, 60))));
    }
 
    #[test]
    fn clock_style_pressed_normal_uses_pressed_color()
    {
        let style = define_clock_style(&make_clock_app(false), button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(5, 10, 15))));
    }

    #[test]
    fn get_current_time_returns_non_empty_string()
    {
        let result = get_current_time("%H:%M", &None);
        assert!(!result.is_empty());
    }
 
    #[test]
    fn get_current_time_format_hhmm_has_correct_length()
    {
        // "%H:%M" always produces exactly 5 chars: "HH:MM"
        let result = get_current_time("%H:%M", &None);
        assert_eq!(result.len(), 5);
        assert!(result.contains(':'));
    }
 
    #[test]
    fn get_current_time_with_invalid_timezone_falls_back_to_local()
    {
        let tz = Some(("NotARealTimezone".to_string(), 0));
        // Should not panic — falls back to local time silently
        let result = get_current_time("%H:%M", &tz);
        assert_eq!(result.len(), 5);
    }
 
    #[test]
    fn get_current_time_with_valid_timezone_returns_time()
    {
        let tz = Some(("America/New_York".to_string(), 0));
        let result = get_current_time("%H:%M", &tz);
        assert_eq!(result.len(), 5);
        assert!(result.contains(':'));
    }
 
    #[test]
    fn get_current_time_static_literal_format()
    {
        // Literal text in strftime is passed through unchanged
        let result = get_current_time("TIME", &None);
        assert_eq!(result, "TIME");
    }

    // ---- cycle_clock_timezones ----------------------------------------------
 
    #[test]
    fn cycle_clock_advances_to_next_timezone()
    {
        let mut app = AppData { ..Default::default() };
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "America/New_York".into(), "Asia/Tokyo".into()]);
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        cycle_clock_timezones(&mut app);
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "America/New_York");
        assert_eq!(idx, 1);
    }
 
    #[test]
    fn cycle_clock_wraps_around_to_first()
    {
        let mut app = AppData { ..Default::default() };
        app.ron_config.clock_timezones = Some(vec!["UTC".into(), "America/New_York".into()]);
        app.current_clock_timezone = Some(("America/New_York".into(), 1));
 
        cycle_clock_timezones(&mut app);
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }
 
    #[test]
    fn cycle_clock_no_timezones_configured_does_nothing()
    {
        let mut app = AppData { ..Default::default() };
        app.ron_config.clock_timezones = None;
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        cycle_clock_timezones(&mut app);
        // Should remain unchanged
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }
 
    #[test]
    fn cycle_clock_empty_timezones_list_does_nothing()
    {
        let mut app = AppData { ..Default::default() };
        app.ron_config.clock_timezones = Some(vec![]);
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        cycle_clock_timezones(&mut app);
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }
 
    #[test]
    fn cycle_clock_single_timezone_wraps_to_itself()
    {
        let mut app = AppData { ..Default::default() };
        app.ron_config.clock_timezones = Some(vec!["UTC".into()]);
        app.current_clock_timezone = Some(("UTC".into(), 0));
 
        cycle_clock_timezones(&mut app);
        let (tz, idx) = app.current_clock_timezone.unwrap();
        assert_eq!(tz, "UTC");
        assert_eq!(idx, 0);
    }
}
