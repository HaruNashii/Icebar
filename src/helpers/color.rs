// ============ IMPORTS ============
use serde::{Deserialize, Serialize};





// ============ STRUCTS/ENUM'S ============
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum ColorType
{
    RGB([u32;3]),
    RGBA([u32;4]),
    HEX([u8; 9]),
}





// ============ IMPL ============
impl Default for ColorType { fn default() -> Self { ColorType::RGB([255, 255, 255]) } }



// Custom deserializer so HEX can be written as HEX("#FF0000") in config files
// rather than as a raw byte array.
impl<'de> serde::Deserialize<'de> for ColorType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
    {
        #[derive(Deserialize)]
        #[allow(clippy::upper_case_acronyms)]
        enum Helper
        {
            RGB([u32; 3]),
            RGBA([u32; 4]),
            HEX(String),
        }
        match Helper::deserialize(deserializer)?
        {
            Helper::RGB(v)  => Ok(ColorType::RGB(v)),
            Helper::RGBA(v) => Ok(ColorType::RGBA(v)),
            Helper::HEX(s)  => Ok(hex_color(&s)),
        }
    }
}



impl ColorType
{
    pub fn to_iced_color(self) -> iced::Color
    {
        match self
        {
            ColorType::RGB([r, g, b])    => iced::Color::from_rgb8(r as u8, g as u8, b as u8),
            ColorType::RGBA([r, g, b, a]) => iced::Color::from_rgba8(r as u8, g as u8, b as u8, (a as f32).clamp(0., 100.) / 100.),
            ColorType::HEX(bytes)        => hex_to_iced_color(&bytes).unwrap_or(iced::Color::WHITE),
        }
    }
}





// ============ FUNCTIONS ============
pub fn hex_color(s: &str) -> ColorType
{
    let mut bytes = [0u8; 9];
    let src = s.as_bytes();
    let len = src.len().min(9);
    bytes[..len].copy_from_slice(&src[..len]);
    ColorType::HEX(bytes)
}



fn hex_to_iced_color(bytes: &[u8; 9]) -> Option<iced::Color>
{
    // Find the null terminator to get the actual string length
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(9);
    let s = std::str::from_utf8(&bytes[..end]).ok()?;
    let hex = s.trim_start_matches('#');
    if hex.len() == 6
    {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(iced::Color::from_rgb8(r, g, b))
    }
    else if hex.len() == 8
    {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some(iced::Color::from_rgba8(r, g, b, a as f32 / 255.))
    }
    else { None }
}


