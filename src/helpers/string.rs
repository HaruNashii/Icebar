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
