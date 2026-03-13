// ============ IMPORTS ============
use iced::{Alignment, Color, font::Weight, widget::{rich_text, span, text::{Rich, Span}}};





// ============ ENUM/STRUCT, ETC ============
struct Segment 
{
    text:  String,
    color: Option<Color>,
}





pub fn convert_text_to_rich_text<'a, Message: 'a>(text: &str, default_color: Option<Color>) -> Rich<'a, (), Message> 
{
    let spans = segments_to_spans(parse_to_segments(text, default_color));
    rich_text(spans).align_y(Alignment::Center).align_x(Alignment::Center).center()
}



pub fn convert_text_to_rich_text_ellipsized<'a, Message: 'a>(text: &str, default_color: Option<Color>, ellipsis: &str, limit: usize) -> Rich<'a, (), Message> 
{
    let segments = parse_to_segments(text, default_color);
    let ellipsized = ellipsize_segments(segments, ellipsis, limit);
    rich_text(segments_to_spans(ellipsized))
}



fn parse_to_segments(text: &str, default_color: Option<Color>) -> Vec<Segment> 
{
    let mut segments = Vec::new();
    match try_parse_tag(text) 
    {
        Some((before, color, colored_text, rest)) => 
        {
            if !before.is_empty() 
            { 
                segments.push(Segment { text: before.to_string(), color: default_color }); 
            }
            segments.push(Segment { text: colored_text.to_string(), color: Some(color) });
            if !rest.is_empty() 
            { 
                segments.extend(parse_to_segments(rest, default_color)); 
            }
        }
        None => 
        {
            if !text.is_empty() 
            { 
                segments.push(Segment { text: text.to_string(), color: default_color }); 
            }
        }
    }
    segments
}



fn ellipsize_segments(segments: Vec<Segment>, ellipsis: &str, limit: usize) -> Vec<Segment> 
{
    let total_visible: usize = segments.iter().map(|s| s.text.chars().count()).sum();
    if total_visible <= limit 
    {
        return segments;
    }

    let ellipsis_len = ellipsis.chars().count();
    let mut budget    = limit.saturating_sub(ellipsis_len);
    let mut result    = Vec::new();
    let mut truncated = false;

    for seg in segments 
    {
        if budget == 0 
        { 
            break; 
        }

        let char_count = seg.text.chars().count();

        if char_count <= budget 
        {
            budget -= char_count;
            result.push(seg);
        } 
        else 
        {
            let cut: String = seg.text.chars().take(budget).collect();
            result.push(Segment 
            {
                text:  format!("{}{}", cut, ellipsis),
                color: seg.color,
            });
            budget    = 0;
            truncated = true;
        }
    }

    if !truncated && !result.is_empty() 
    {
        let last = result.last_mut().unwrap();
        last.text.push_str(ellipsis);
    }

    result
}



fn segments_to_spans(segments: Vec<Segment>) -> Vec<Span<'static>> 
{
    segments.into_iter().map(|seg| make_span_owned(seg.text, seg.color)).collect()
}



fn make_span_owned(text: String, color: Option<Color>) -> Span<'static> 
{
    match color 
    {
        Some(c) => span(text).color(c),
        None    => span(text),
    }
}



fn try_parse_tag(text: &str) -> Option<(&str, Color, &str, &str)> 
{
    let bracket_start = text.find('[')?;
    let before        = &text[..bracket_start];
    let inside        = text[bracket_start + 1..].trim_start();
    let inside        = inside.strip_prefix("Color")?.trim_start();
    let inside        = inside.strip_prefix('=')?.trim_start();
    let inside        = inside.strip_prefix('(')?.trim_start();
    let (rgb_str, inside) = inside.split_once(')')?;
    let color         = parse_color(rgb_str)?;
    let inside        = inside.trim_start();
    let inside        = inside.strip_prefix(',')?.trim_start();
    let inside        = inside.strip_prefix("String")?.trim_start();
    let inside        = inside.strip_prefix('=')?.trim_start();
    let (unformated_colored_text, rest) = inside.split_once(']')?;
    let colored_text  = unformated_colored_text.trim_end();
    Some((before, color, colored_text, rest))
}



fn parse_color(rgb_str: &str) -> Option<Color> 
{
    let values: Vec<f32> = rgb_str.split(',').map(|v| v.trim().parse::<f32>()).collect::<Result<_, _>>().ok()?;
    match values.as_slice() 
    {
        [r, g, b] => Some(Color::from_rgb8(*r as u8, *g as u8, *b as u8)),
        _=> None,
    }
}



pub fn weight_from_str(s: &str) -> Weight 
{
    match s.to_lowercase().as_str() 
    {
        "thin"                              => Weight::Thin,
        "extra_light" | "extralight" | "ultralight" => Weight::ExtraLight,
        "light"                             => Weight::Light,
        "normal" | "regular"               => Weight::Normal,
        "medium"                            => Weight::Medium,
        "semibold" | "semi_bold"           => Weight::Semibold,
        "bold"                              => Weight::Bold,
        "extra_bold" | "extrabold" | "ultrabold" => Weight::ExtraBold,
        "black" | "heavy"                  => Weight::Black,
        _                                  => Weight::Normal,
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
        let result = try_parse_tag(r#"prefix [Color=(0, 0, 255),String=blue] after"#);
        let (before, _color, colored, rest) = result.unwrap();
        assert_eq!(before, "prefix ");
        assert_eq!(colored, "blue");
        assert_eq!(rest, " after");
    }
 
    #[test]
    fn try_parse_tag_with_spaces_inside_brackets()
    {
        let result = try_parse_tag(r#"[ Color = ( 255 , 128 , 0 ) , String =spaced ] after"#);
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
        assert!(try_parse_tag(r#"[Color=(255,0,0),String=abc"#).is_none());
    }
 
    #[test]
    fn try_parse_tag_bad_color_returns_none()
    {
        assert!(try_parse_tag(r#"[Color=(red,green,blue),String=abc]"#).is_none());
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
    
    // ---- parse_colored_spans (via multiple-tag round-trip) ------------------
 
    #[test]
    fn two_consecutive_color_tags_both_parsed()
    {
        // The first tag's rest feeds recursively into parse_colored_spans, so
        // we verify that two back-to-back tags both produce colored spans.
        // We do this indirectly through try_parse_tag twice.
        let input = r#"[Color=(255,0,0),String=red][Color=(0,255,0),String=green]"#;
 
        let (before1, c1, text1, rest1) = try_parse_tag(input).unwrap();
        assert_eq!(before1, "");
        assert_eq!(c1, Color::from_rgb8(255, 0, 0));
        assert_eq!(text1, "red");
 
        let (before2, c2, text2, rest2) = try_parse_tag(rest1).unwrap();
        assert_eq!(before2, "");
        assert_eq!(c2, Color::from_rgb8(0, 255, 0));
        assert_eq!(text2, "green");
        assert_eq!(rest2, "");
    }
 
    #[test]
    fn try_parse_tag_only_colored_text_no_rest()
    {
        let result = try_parse_tag(r#"[Color=(10,20,30),String=only]"#);
        let (_before, color, colored, rest) = result.unwrap();
        assert_eq!(color, Color::from_rgb8(10, 20, 30));
        assert_eq!(colored, "only");
        assert_eq!(rest, "");
    }
 
    #[test]
    fn try_parse_tag_empty_colored_string_quoted()
    {
        // String="" is valid — empty colored span
        let result = try_parse_tag(r#"[Color=(1,2,3),String=] tail"#);
        let (_before, _color, colored, rest) = result.unwrap();
        assert_eq!(colored, "");
        assert_eq!(rest, " tail");
    }
 
    #[test]
    fn try_parse_tag_wrong_keyword_returns_none()
    {
        // "colour" instead of "Color" must not match
        assert!(try_parse_tag(r#"[colour=(255,0,0),String=abc]"#).is_none());
    }
 
    #[test]
    fn try_parse_tag_missing_comma_between_rgb_and_string_returns_none()
    {
        assert!(try_parse_tag(r#"[Color=(255,0,0) String=abc]"#).is_none());
    }
 
    // ---- ellipsize edge cases -----------------------------------------------
 
    #[test]
    fn ellipsize_limit_zero_appends_ellipsis_immediately()
    {
        let result = ellipsize(&"...".to_string(), "hello", 0);
        assert_eq!(result, "...");
    }
 
    #[test]
    fn ellipsize_empty_ellipsis_string()
    {
        let result = ellipsize(&String::new(), "abcdef", 3);
        assert_eq!(result, "abc");
    }

    #[test]
    fn full_pipeline_plain_text_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("plain text no tags", None);
    }
     
    #[test]
    fn full_pipeline_single_tag_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("[Color=(255,0,0),String=red] rest", None);
    }
     
    #[test]
    fn full_pipeline_multiple_tags_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>(
            "[Color=(255,0,0),String=red][Color=(0,255,0),String=green] plain",
            None,
        );
    }
     
    #[test]
    fn full_pipeline_with_default_color_does_not_panic()
    {
        use iced::Color;
        let _ = convert_text_to_rich_text::<()>(
            "[Color=(255,0,0),String=hello] world",
            Some(Color::from_rgb8(200, 200, 200)),
        );
    }
     
    #[test]
    fn full_pipeline_malformed_tag_falls_back_to_plain()
    {
        // Malformed tag — should not panic, should render as plain text
        let _ = convert_text_to_rich_text::<()>("[Color=(bad),String=abc] rest", None);
    }
     
    #[test]
    fn full_pipeline_empty_string_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("", None);
    }
     
    #[test]
    fn full_pipeline_deeply_nested_tags_does_not_panic()
    {
        // Many chained tags to stress-test the recursion
        let mut input = String::new();
        for i in 0..20
        {
            input.push_str(&format!("[Color=({i},{i},{i}),String=tag{i}] "));
        }
        let _ = convert_text_to_rich_text::<()>(&input, None);
    }
}
