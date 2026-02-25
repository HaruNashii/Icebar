// ============ IMPORTS ============
use iced::font::Weight;





// ============ FUNCTIONS ============
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



pub fn ellipsize(text: &str, limit: usize) -> String 
{
    if text.chars().count() <= limit 
    {
        text.to_owned()
    } 
    else 
    {
        format!("{}...", text.chars().take(limit).collect::<String>())
    }
}
