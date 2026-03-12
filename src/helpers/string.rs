// ============ IMPORTS ============
use iced::{Color, font::Weight, widget::{rich_text, span, text::{Span, Rich}}};





// ============ FUNCTIONS ============
pub fn convert_text_to_rich_text<'a, Message: 'a>(text: &str, default_color: Option<Color>) -> Rich<'a, (), Message> 
{
    rich_text(parse_colored_spans(text, default_color))
}



fn parse_colored_spans<'a>(text: &str, default_color: Option<Color>) -> Vec<Span<'a>> 
{
    let mut spans = Vec::new();

    match try_parse_tag(text) 
    {
        Some((before, color, colored_text, rest)) => 
        {
            if !before.is_empty() { spans.push(make_span(before, default_color)); }
            spans.push(span(colored_text.to_string()).color(color));
            if !rest.is_empty() { spans.extend(parse_colored_spans(rest, default_color)); }
        }
        None => 
        {
            if !text.is_empty() { spans.push(make_span(text, default_color)); }
        }
    }

    spans
}



fn make_span<'a>(text: &str, color: Option<Color>) -> Span<'a> 
{
    match color 
    {
        Some(c) => span(text.to_string()).color(c),
        None => span(text.to_string()),
    }
}


fn try_parse_tag(text: &str) -> Option<(&str, Color, &str, &str)> 
{
    let bracket_start = text.find('[')?;
    let before = &text[..bracket_start];

    let inside = text[bracket_start + 1..].trim_start();
    let inside = inside.strip_prefix("Color")?.trim_start();
    let inside = inside.strip_prefix('=')?.trim_start();
    let inside = inside.strip_prefix('(')?.trim_start();

    let (rgb_str, inside) = inside.split_once(')')?;
    let color = parse_color(rgb_str)?;

    let inside = inside.trim_start();
    let inside = inside.strip_prefix(',')?.trim_start();
    let inside = inside.strip_prefix("String")?.trim_start();
    let inside = inside.strip_prefix('=')?.trim_start();

    // Support both String="text" and String=text
    let (colored_text, rest) = if let Some(inside) = inside.strip_prefix('"') 
    {
        let (text, after_quote) = inside.split_once('"')?;
        let bracket_end = after_quote.trim_start().find(']')?;
        let rest = after_quote.trim_start()[bracket_end + 1..].trim_start();
        (text, rest)
    }
    else 
    {
        // No quotes — read until ]
        let (text, rest) = inside.split_once(']')?;
        (text.trim_end(), rest)
    };

    Some((before, color, colored_text, rest))
}



fn parse_color(rgb_str: &str) -> Option<Color> 
{
    let values: Vec<f32> = rgb_str.split(',').map(|v| v.trim().parse::<f32>()).collect::<Result<_, _>>().ok()?;
    match values.as_slice() 
    {
        [r, g, b] => Some(Color::from_rgb8(*r as u8, *g as u8, *b as u8)),
        _ => None,
    }
}



pub fn weight_from_str(s: &str) -> Weight 
{
    match s.to_lowercase().as_str() 
    {
        "thin" => Weight::Thin,
        "extra_light" | "extralight" | "ultralight" => Weight::ExtraLight,
        "light" => Weight::Light,
        "normal" | "regular" => Weight::Normal,
        "medium" => Weight::Medium,
        "semibold" | "semi_bold" => Weight::Semibold,
        "bold" => Weight::Bold,
        "extra_bold" | "extrabold" | "ultrabold" => Weight::ExtraBold,
        "black" | "heavy" => Weight::Black,
        _ => Weight::Normal, 
    }
}



pub fn ellipsize(ellipsis: &String, text: &str, limit: usize) -> String 
{
    if text.chars().count() <= limit 
    {
        text.to_owned()
    } 
    else 
    {
        format!("{}{}", text.chars().take(limit).collect::<String>(), ellipsis)
    }
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use iced::Color;
 
    // ---- parse_color --------------------------------------------------------
 
    #[test]
    fn parse_color_valid_rgb()
    {
        let color = parse_color("255, 0, 128").unwrap();
        assert_eq!(color, Color::from_rgb8(255, 0, 128));
    }
 
    #[test]
    fn parse_color_with_extra_spaces()
    {
        let color = parse_color("  10 ,  20 ,  30 ").unwrap();
        assert_eq!(color, Color::from_rgb8(10, 20, 30));
    }
 
    #[test]
    fn parse_color_too_few_components_returns_none()
    {
        assert!(parse_color("255, 0").is_none());
    }
 
    #[test]
    fn parse_color_too_many_components_returns_none()
    {
        assert!(parse_color("255, 0, 0, 255").is_none());
    }
 
    #[test]
    fn parse_color_non_numeric_returns_none()
    {
        assert!(parse_color("red, green, blue").is_none());
    }
 
    #[test]
    fn parse_color_empty_returns_none()
    {
        assert!(parse_color("").is_none());
    }
 
    // ---- try_parse_tag ------------------------------------------------------
 
    #[test]
    fn try_parse_tag_basic_quoted()
    {
        let result = try_parse_tag(r#"[Color=(255, 0, 0),String="hello"] world"#);
        let (before, color, colored, rest) = result.unwrap();
        assert_eq!(before, "");
        assert_eq!(color, Color::from_rgb8(255, 0, 0));
        assert_eq!(colored, "hello");
        assert_eq!(rest, "world");
    }
 
    #[test]
    fn try_parse_tag_basic_unquoted()
    {
        let result = try_parse_tag("[Color=(0, 255, 0),String=world] rest");
        let (before, color, colored, rest) = result.unwrap();
        assert_eq!(before, "");
        assert_eq!(color, Color::from_rgb8(0, 255, 0));
        assert_eq!(colored, "world");
        assert_eq!(rest, " rest");
    }
 
    #[test]
    fn try_parse_tag_with_text_before_tag()
    {
        let result = try_parse_tag(r#"prefix [Color=(0, 0, 255),String="blue"] after"#);
        let (before, _color, colored, rest) = result.unwrap();
        assert_eq!(before, "prefix ");
        assert_eq!(colored, "blue");
        assert_eq!(rest, "after");
    }
 
    #[test]
    fn try_parse_tag_with_spaces_inside_brackets()
    {
        let result = try_parse_tag(r#"[ Color = ( 255 , 128 , 0 ) , String = "spaced" ] after"#);
        let (_before, color, colored, _rest) = result.unwrap();
        assert_eq!(color, Color::from_rgb8(255, 128, 0));
        assert_eq!(colored, "spaced");
    }
 
    #[test]
    fn try_parse_tag_preserves_leading_spaces_in_rest()
    {
        let result = try_parse_tag(r#"[Color=(255,255,255),String=abc]   three spaces"#);
        let (_before, _color, _colored, rest) = result.unwrap();
        assert_eq!(rest, "   three spaces");
    }
 
    #[test]
    fn try_parse_tag_no_tag_returns_none()
    {
        assert!(try_parse_tag("just plain text").is_none());
    }
 
    #[test]
    fn try_parse_tag_malformed_no_closing_bracket_returns_none()
    {
        assert!(try_parse_tag(r#"[Color=(255,0,0),String="abc""#).is_none());
    }
 
    #[test]
    fn try_parse_tag_bad_color_returns_none()
    {
        assert!(try_parse_tag(r#"[Color=(red,green,blue),String="abc"]"#).is_none());
    }
 
    // ---- ellipsize ----------------------------------------------------------
 
    #[test]
    fn ellipsize_short_text_unchanged()
    {
        let result = ellipsize(&"...".to_string(), "hello", 10);
        assert_eq!(result, "hello");
    }
 
    #[test]
    fn ellipsize_exact_limit_unchanged()
    {
        let result = ellipsize(&"...".to_string(), "hello", 5);
        assert_eq!(result, "hello");
    }
 
    #[test]
    fn ellipsize_over_limit_truncates_and_appends()
    {
        let result = ellipsize(&"...".to_string(), "hello world", 5);
        assert_eq!(result, "hello...");
    }
 
    #[test]
    fn ellipsize_custom_ellipsis()
    {
        let result = ellipsize(&"~".to_string(), "abcdef", 3);
        assert_eq!(result, "abc~");
    }
 
    #[test]
    fn ellipsize_empty_text()
    {
        let result = ellipsize(&"...".to_string(), "", 5);
        assert_eq!(result, "");
    }
 
    #[test]
    fn ellipsize_unicode_counts_chars_not_bytes()
    {
        // "é" is 2 bytes but 1 char
        let result = ellipsize(&"...".to_string(), "héllo", 5);
        assert_eq!(result, "héllo");
    }
 
    // ---- weight_from_str ----------------------------------------------------
 
    #[test]
    fn weight_from_str_known_values()
    {
        use iced::font::Weight;
        assert_eq!(weight_from_str("bold"),       Weight::Bold);
        assert_eq!(weight_from_str("thin"),       Weight::Thin);
        assert_eq!(weight_from_str("light"),      Weight::Light);
        assert_eq!(weight_from_str("medium"),     Weight::Medium);
        assert_eq!(weight_from_str("semibold"),   Weight::Semibold);
        assert_eq!(weight_from_str("semi_bold"),  Weight::Semibold);
        assert_eq!(weight_from_str("extrabold"),  Weight::ExtraBold);
        assert_eq!(weight_from_str("black"),      Weight::Black);
        assert_eq!(weight_from_str("heavy"),      Weight::Black);
        assert_eq!(weight_from_str("normal"),     Weight::Normal);
        assert_eq!(weight_from_str("regular"),    Weight::Normal);
    }
 
    #[test]
    fn weight_from_str_case_insensitive()
    {
        use iced::font::Weight;
        assert_eq!(weight_from_str("BOLD"),   Weight::Bold);
        assert_eq!(weight_from_str("Bold"),   Weight::Bold);
        assert_eq!(weight_from_str("THIN"),   Weight::Thin);
    }
 
    #[test]
    fn weight_from_str_unknown_falls_back_to_normal()
    {
        use iced::font::Weight;
        assert_eq!(weight_from_str("garbage"), Weight::Normal);
        assert_eq!(weight_from_str(""),         Weight::Normal);
    }
}
