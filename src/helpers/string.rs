// ============ IMPORTS ============
use iced::{Alignment, Color, font::Weight, widget::{rich_text, span, text::{Rich, Span}}};
use std::collections::HashSet;
use std::sync::Mutex;





// ============ STATIC'S ============
static INTERNED_STRINGS: Mutex<Option<HashSet<&'static str>>> = Mutex::new(None);





// ============ ENUM/STRUCT, ETC ============
struct Segment 
{
    text:  String,
    color: Option<Color>,
}

enum Tag<'a>
{
    Color(Color, &'a str),
    Tuning(u32),
}





// ============ FUNCTIONS ============
pub fn format_output_volume(vol: f32, muted: bool, formats: &[String; 6], muted_format: &str) -> (String, bool)
{
    if muted { return (muted_format.to_string(), true); }
    (apply_format(vol, formats), false)
}
 


pub fn format_input_volume(vol: f32, muted: bool, formats: &[String; 6], muted_format: &str) -> (String, bool)
{
    if muted { return (muted_format.to_string(), true); }
    (apply_format(vol, formats), false)
}
 

pub fn intern_string(s: String) -> &'static str
{
    let mut guard = INTERNED_STRINGS.lock().unwrap_or_else(|p| p.into_inner());
    let set = guard.get_or_insert_with(HashSet::new);

    // If this exact string was already interned, return the existing pointer
    if let Some(&existing) = set.iter().find(|&&interned| interned == s.as_str())
    {
        return existing;
    }

    // New string — leak it once and store the pointer
    let leaked: &'static str = Box::leak(s.into_boxed_str());
    set.insert(leaked);
    leaked
}


pub fn apply_format(vol: f32, formats: &[String; 6]) -> String
{
    let thresholds = 
    [
        (0.000, &formats[0]),
        (0.240, &formats[1]),
        (0.490, &formats[2]),
        (0.900, &formats[3]),
        (1.009, &formats[4]),
        (9999.9, &formats[5]),
    ];
    let fmt = thresholds.iter().find(|&&(max, _)| vol <= max).map(|&(_, f)| f).unwrap_or(&formats[0]);
    let percent = ((vol * 100.0).round() as u32).to_string();
    fmt.replace("{}", &percent)
}



pub fn convert_text_to_rich_text<'a, Message: 'a>(text: &str) -> Rich<'a, (), Message> 
{
    let spans = segments_to_spans(parse_to_segments(text));
    rich_text(spans).align_y(Alignment::Center).align_x(Alignment::Center).center()
}



pub fn convert_text_to_rich_text_ellipsized<'a, Message: 'a>(text: &str, ellipsis: &str, limit: usize) -> Rich<'a, (), Message> 
{
    let segments = parse_to_segments(text);
    let ellipsized = ellipsize_segments(segments, ellipsis, limit);
    rich_text(segments_to_spans(ellipsized))
}



fn parse_to_segments(text: &str) -> Vec<Segment>
{
    let mut segments = Vec::new();
    match try_parse_tag(text)
    {
        Some((before, tag, rest)) =>
        {
            if !before.is_empty()
            {
                segments.push(Segment { text: before.to_string(), color: None });
            }
            match tag
            {
                Tag::Color(color, colored_text) =>
                {
                    segments.push(Segment { text: colored_text.to_string(), color: Some(color) });
                }
                Tag::Tuning(n) =>
                {
                    let hair_spaces = "\u{200A}".repeat(n as usize);
                    segments.push(Segment { text: hair_spaces, color: None });
                }
            }
            if !rest.is_empty()
            {
                segments.extend(parse_to_segments(rest));
            }
        }
        None =>
        {
            if !text.is_empty()
            {
                segments.push(Segment { text: text.to_string(), color: None });
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


fn try_parse_tag<'a>(text: &'a str) -> Option<(&'a str, Tag<'a>, &'a str)>
{
    let bracket_start = text.find('[')?;
    let before        = &text[..bracket_start];
    let inside        = text[bracket_start + 1..].trim_start();

    // try Tuning tag first
    if let Some(inside) = inside.strip_prefix("Tuning")
    {
        let inside = inside.trim_start().strip_prefix('=')?.trim_start();
        let (num_str, rest) = inside.split_once(']')?;
        let n = num_str.trim().parse::<u32>().ok()?;
        return Some((before, Tag::Tuning(n), rest));
    }

    // existing Color tag
    let inside        = inside.strip_prefix("Color")?.trim_start();
    let inside        = inside.strip_prefix('=')?.trim_start();
    let inside        = inside.strip_prefix('(')?.trim_start();
    let (rgb_str, inside) = inside.split_once(')')?;
    let color         = parse_color(rgb_str)?;
    let inside        = inside.trim_start();
    let inside        = inside.strip_prefix(',')?.trim_start();
    let inside        = inside.strip_prefix("String")?.trim_start();
    let inside        = inside.strip_prefix('=')?.trim_start();
    let (colored_text, rest) = inside.split_once(']')?;
    let colored_text  = colored_text.trim_end();
    Some((before, Tag::Color(color, colored_text), rest))
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
 
    // ---- try_parse_tag — Color ----------------------------------------------
 
    #[test]
    fn try_parse_tag_basic_unquoted()
    {
        let (before, tag, rest) = try_parse_tag("[Color=(0, 255, 0),String=world] rest").unwrap();
        assert_eq!(before, "");
        assert_eq!(rest, " rest");
        match tag
        {
            Tag::Color(color, text) =>
            {
                assert_eq!(color, Color::from_rgb8(0, 255, 0));
                assert_eq!(text, "world");
            }
            _ => panic!("expected Color tag"),
        }
    }
 
    #[test]
    fn try_parse_tag_with_text_before_tag()
    {
        let (before, tag, rest) = try_parse_tag("prefix [Color=(0, 0, 255),String=blue] after").unwrap();
        assert_eq!(before, "prefix ");
        assert_eq!(rest, " after");
        match tag
        {
            Tag::Color(_, text) => assert_eq!(text, "blue"),
            _ => panic!("expected Color tag"),
        }
    }
 
    #[test]
    fn try_parse_tag_with_spaces_inside_brackets()
    {
        let (_, tag, _) = try_parse_tag("[ Color = ( 255 , 128 , 0 ) , String =spaced ] after").unwrap();
        match tag
        {
            Tag::Color(color, text) =>
            {
                assert_eq!(color, Color::from_rgb8(255, 128, 0));
                assert_eq!(text, "spaced");
            }
            _ => panic!("expected Color tag"),
        }
    }
 
    #[test]
    fn try_parse_tag_preserves_leading_spaces_in_rest()
    {
        let (_, _, rest) = try_parse_tag("[Color=(255,255,255),String=abc]   three spaces").unwrap();
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
        assert!(try_parse_tag("[Color=(255,0,0),String=abc").is_none());
    }
 
    #[test]
    fn try_parse_tag_bad_color_returns_none()
    {
        assert!(try_parse_tag("[Color=(red,green,blue),String=abc]").is_none());
    }

    #[test]
    fn try_parse_tag_only_colored_text_no_rest()
    {
        let (_, tag, rest) = try_parse_tag("[Color=(10,20,30),String=only]").unwrap();
        assert_eq!(rest, "");
        match tag
        {
            Tag::Color(color, text) =>
            {
                assert_eq!(color, Color::from_rgb8(10, 20, 30));
                assert_eq!(text, "only");
            }
            _ => panic!("expected Color tag"),
        }
    }
 
    #[test]
    fn try_parse_tag_empty_colored_string()
    {
        let (_, tag, rest) = try_parse_tag("[Color=(1,2,3),String=] tail").unwrap();
        assert_eq!(rest, " tail");
        match tag
        {
            Tag::Color(_, text) => assert_eq!(text, ""),
            _ => panic!("expected Color tag"),
        }
    }
 
    #[test]
    fn try_parse_tag_wrong_keyword_returns_none()
    {
        assert!(try_parse_tag("[colour=(255,0,0),String=abc]").is_none());
    }
 
    #[test]
    fn try_parse_tag_missing_comma_between_rgb_and_string_returns_none()
    {
        assert!(try_parse_tag("[Color=(255,0,0) String=abc]").is_none());
    }

    // ---- try_parse_tag — Tuning ---------------------------------------------

    #[test]
    fn try_parse_tag_tuning_basic()
    {
        let (before, tag, rest) = try_parse_tag("text[Tuning=3]after").unwrap();
        assert_eq!(before, "text");
        assert_eq!(rest, "after");
        match tag
        {
            Tag::Tuning(n) => assert_eq!(n, 3),
            _ => panic!("expected Tuning tag"),
        }
    }

    #[test]
    fn try_parse_tag_tuning_zero()
    {
        let (_, tag, _) = try_parse_tag("[Tuning=0]").unwrap();
        match tag
        {
            Tag::Tuning(n) => assert_eq!(n, 0),
            _ => panic!("expected Tuning tag"),
        }
    }

    #[test]
    fn try_parse_tag_tuning_no_closing_bracket_returns_none()
    {
        assert!(try_parse_tag("[Tuning=3").is_none());
    }

    #[test]
    fn try_parse_tag_tuning_non_numeric_returns_none()
    {
        assert!(try_parse_tag("[Tuning=abc]").is_none());
    }

    #[test]
    fn tuning_produces_correct_number_of_hair_spaces()
    {
        let segments = parse_to_segments("[Tuning=4]");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text.chars().count(), 4);
        assert!(segments[0].text.chars().all(|c| c == '\u{200A}'));
    }

    #[test]
    fn tuning_zero_produces_empty_segment()
    {
        let segments = parse_to_segments("[Tuning=0]");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "");
    }

    // ---- two_consecutive_color_tags -----------------------------------------
 
    #[test]
    fn two_consecutive_color_tags_both_parsed()
    {
        let input = "[Color=(255,0,0),String=red][Color=(0,255,0),String=green]";

        let (before1, tag1, rest1) = try_parse_tag(input).unwrap();
        assert_eq!(before1, "");
        match tag1
        {
            Tag::Color(c, t) => { assert_eq!(c, Color::from_rgb8(255, 0, 0)); assert_eq!(t, "red"); }
            _ => panic!("expected Color tag"),
        }

        let (before2, tag2, rest2) = try_parse_tag(rest1).unwrap();
        assert_eq!(before2, "");
        assert_eq!(rest2, "");
        match tag2
        {
            Tag::Color(c, t) => { assert_eq!(c, Color::from_rgb8(0, 255, 0)); assert_eq!(t, "green"); }
            _ => panic!("expected Color tag"),
        }
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
        let result = ellipsize(&"...".to_string(), "héllo", 5);
        assert_eq!(result, "héllo");
    }

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

    // ---- full pipeline ------------------------------------------------------
 
    #[test]
    fn full_pipeline_plain_text_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("plain text no tags");
    }
     
    #[test]
    fn full_pipeline_single_color_tag_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("[Color=(255,0,0),String=red] rest");
    }

    #[test]
    fn full_pipeline_tuning_tag_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("text[Tuning=3]more text");
    }
     
    #[test]
    fn full_pipeline_multiple_tags_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("[Color=(255,0,0),String=red][Tuning=2][Color=(0,255,0),String=green]");
    }
     
    #[test]
    fn full_pipeline_malformed_tag_falls_back_to_plain()
    {
        let _ = convert_text_to_rich_text::<()>("[Color=(bad),String=abc] rest");
    }
     
    #[test]
    fn full_pipeline_empty_string_does_not_panic()
    {
        let _ = convert_text_to_rich_text::<()>("");
    }
     
    #[test]
    fn full_pipeline_deeply_nested_tags_does_not_panic()
    {
        let mut input = String::new();
        for i in 0..20
        {
            input.push_str(&format!("[Color=({i},{i},{i}),String=tag{i}][Tuning=1]"));
        }
        let _ = convert_text_to_rich_text::<()>(&input);
    }
}
