// ============ IMPORTS ============
use serde::{Deserialize, Serialize};





// ============ ENUM/STRUCT, ETC ============
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Gradient
{
    Gradient((f32, Vec<(f32, ColorType)>))
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum ColorType
{
    RGB([u32; 3]),
    RGBA([u32; 4]),
    HEX([u8; 9])
}

impl Default for ColorType { fn default() -> Self { ColorType::RGB([255, 255, 255]) } }

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
            HEX(String)
        }
        match Helper::deserialize(deserializer)?
        {
            Helper::RGB(v)  => Ok(ColorType::RGB(v)),
            Helper::RGBA(v) => Ok(ColorType::RGBA(v)),
            Helper::HEX(s)  => Ok(hex_color(&s))
        }
    }
}

impl ColorType
{
    pub fn to_iced_color(self) -> iced::Color
    {
        match self
        {
            ColorType::RGB([r, g, b])     => iced::Color::from_rgb8(r as u8, g as u8, b as u8),
            ColorType::RGBA([r, g, b, a]) => iced::Color::from_rgba8(r as u8, g as u8, b as u8, (a as f32).clamp(0., 100.) / 100.),
            ColorType::HEX(bytes)         => hex_to_iced_color(&bytes).unwrap_or(iced::Color::WHITE)
        }
    }
}





// ============ FUNCTIONS ============
pub fn hex_color(s: &str) -> ColorType
{
    let src = s.as_bytes();
    if src.len() > 9
    {
        eprintln!("Warning!!!: HEX color string '{s}' is longer than 9 bytes and cannot be stored — falling back to white.");
        return ColorType::RGB([255, 255, 255]);
    }
    let mut bytes = [0u8; 9];
    let len = src.len();
    bytes[..len].copy_from_slice(&src[..len]);
    ColorType::HEX(bytes)
}



fn hex_to_iced_color(bytes: &[u8; 9]) -> Option<iced::Color>
{
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





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;


    #[test]
    fn color_type_default_is_white_rgb()
    {
        assert_eq!(ColorType::default(), ColorType::RGB([255, 255, 255]));
    }


    #[test]
    fn hex_color_6_digit_with_hash_produces_hex_variant()
    {
        let c = hex_color("#ff0000");
        assert!(matches!(c, ColorType::HEX(_)));
    }

    #[test]
    fn hex_color_6_digit_without_hash_produces_hex_variant()
    {
        let c = hex_color("ff0000");
        assert!(matches!(c, ColorType::HEX(_)));
    }

    #[test]
    fn hex_color_8_digit_with_hash_produces_hex_variant()
    {
        let c = hex_color("#ff0000ff");
        assert!(matches!(c, ColorType::HEX(_)));
    }

    #[test]
    fn hex_color_empty_string_produces_hex_variant_all_zeros()
    {
        let c = hex_color("");
        if let ColorType::HEX(bytes) = c
        {
            assert!(bytes.iter().all(|&b| b == 0));
        }
        else { panic!("expected HEX variant"); }
    }

    #[test]
    fn hex_color_exactly_9_bytes_is_accepted()
    {
        let c = hex_color("#ff0000f");
        assert!(matches!(c, ColorType::HEX(_)));
    }

    #[test]
    fn hex_color_string_longer_than_9_bytes_falls_back_to_white()
    {
        let c = hex_color("#ff0000ffX");
        assert_eq!(c, ColorType::RGB([255, 255, 255]));
    }

    #[test]
    fn hex_color_10_byte_string_falls_back_to_white()
    {
        let c = hex_color("0123456789");
        assert_eq!(c, ColorType::RGB([255, 255, 255]));
    }

    #[test]
    fn hex_color_preserves_bytes_correctly()
    {
        let input = "#abc";
        let c = hex_color(input);
        if let ColorType::HEX(bytes) = c
        {
            assert_eq!(&bytes[..4], b"#abc");
            assert_eq!(bytes[4], 0);
        }
        else { panic!("expected HEX variant"); }
    }


    #[test]
    fn rgb_black_converts_correctly()
    {
        let c = ColorType::RGB([0, 0, 0]).to_iced_color();
        assert_eq!(c, iced::Color::from_rgb8(0, 0, 0));
    }

    #[test]
    fn rgb_white_converts_correctly()
    {
        let c = ColorType::RGB([255, 255, 255]).to_iced_color();
        assert_eq!(c, iced::Color::from_rgb8(255, 255, 255));
    }

    #[test]
    fn rgb_red_converts_correctly()
    {
        let c = ColorType::RGB([255, 0, 0]).to_iced_color();
        assert_eq!(c, iced::Color::from_rgb8(255, 0, 0));
    }

    #[test]
    fn rgb_green_converts_correctly()
    {
        let c = ColorType::RGB([0, 255, 0]).to_iced_color();
        assert_eq!(c, iced::Color::from_rgb8(0, 255, 0));
    }

    #[test]
    fn rgb_blue_converts_correctly()
    {
        let c = ColorType::RGB([0, 0, 255]).to_iced_color();
        assert_eq!(c, iced::Color::from_rgb8(0, 0, 255));
    }

    #[test]
    fn rgb_arbitrary_color_converts_correctly()
    {
        let c = ColorType::RGB([100, 150, 200]).to_iced_color();
        assert_eq!(c, iced::Color::from_rgb8(100, 150, 200));
    }

    #[test]
    fn rgb_truncates_values_over_255_via_as_cast()
    {
        let c = ColorType::RGB([256, 256, 256]).to_iced_color();
        assert_eq!(c, iced::Color::from_rgb8(0, 0, 0));
    }


    #[test]
    fn rgba_full_opacity_converts_correctly()
    {
        let c = ColorType::RGBA([255, 0, 0, 100]).to_iced_color();
        let expected = iced::Color::from_rgba8(255, 0, 0, 1.0);
        assert!((c.a - expected.a).abs() < 0.01);
        assert_eq!(c.r, expected.r);
    }

    #[test]
    fn rgba_zero_opacity_converts_correctly()
    {
        let c = ColorType::RGBA([255, 0, 0, 0]).to_iced_color();
        assert!((c.a - 0.0).abs() < 0.01);
    }

    #[test]
    fn rgba_50_percent_opacity()
    {
        let c = ColorType::RGBA([0, 0, 0, 50]).to_iced_color();
        assert!((c.a - 0.5).abs() < 0.01);
    }

    #[test]
    fn rgba_alpha_above_100_is_clamped_to_1()
    {
        let c = ColorType::RGBA([0, 0, 0, 200]).to_iced_color();
        assert!((c.a - 1.0).abs() < 0.01);
    }


    #[test]
    fn hex_6_digit_red_converts_to_red()
    {
        let c = hex_color("#ff0000").to_iced_color();
        let expected = iced::Color::from_rgb8(255, 0, 0);
        assert!((c.r - expected.r).abs() < 0.01);
        assert!((c.g - expected.g).abs() < 0.01);
        assert!((c.b - expected.b).abs() < 0.01);
    }

    #[test]
    fn hex_6_digit_without_hash_converts()
    {
        let c = hex_color("00ff00").to_iced_color();
        let expected = iced::Color::from_rgb8(0, 255, 0);
        assert!((c.r - expected.r).abs() < 0.01);
        assert!((c.g - expected.g).abs() < 0.01);
        assert!((c.b - expected.b).abs() < 0.01);
    }

    #[test]
    fn hex_8_digit_with_full_alpha_converts()
    {
        let c = hex_color("#000000ff").to_iced_color();
        assert!((c.a - 1.0).abs() < 0.01);
    }

    #[test]
    fn hex_8_digit_with_zero_alpha_converts()
    {
        let c = hex_color("#00000000").to_iced_color();
        assert!((c.a - 0.0).abs() < 0.01);
    }

    #[test]
    fn hex_invalid_string_falls_back_to_white()
    {
        let c = hex_color("ZZZZZZ").to_iced_color();
        assert_eq!(c, iced::Color::WHITE);
    }

    #[test]
    fn hex_5_digit_string_falls_back_to_white()
    {
        let c = hex_color("#abcd").to_iced_color();
        assert_eq!(c, iced::Color::WHITE);
    }

    #[test]
    fn hex_empty_string_falls_back_to_white()
    {
        let c = hex_color("").to_iced_color();
        assert_eq!(c, iced::Color::WHITE);
    }

    #[test]
    fn hex_only_hash_falls_back_to_white()
    {
        let c = hex_color("#").to_iced_color();
        assert_eq!(c, iced::Color::WHITE);
    }


    #[test]
    fn same_rgb_colors_are_equal()
    {
        assert_eq!(ColorType::RGB([10, 20, 30]), ColorType::RGB([10, 20, 30]));
    }

    #[test]
    fn different_rgb_colors_are_not_equal()
    {
        assert_ne!(ColorType::RGB([10, 20, 30]), ColorType::RGB([10, 20, 31]));
    }

    #[test]
    fn rgb_and_rgba_variants_are_not_equal()
    {
        assert_ne!(ColorType::RGB([255, 255, 255]), ColorType::RGBA([255, 255, 255, 100]));
    }

    #[test]
    fn same_rgba_colors_are_equal()
    {
        assert_eq!(ColorType::RGBA([1, 2, 3, 100]), ColorType::RGBA([1, 2, 3, 100]));
    }

    #[test]
    fn different_rgba_alpha_values_are_not_equal()
    {
        assert_ne!(ColorType::RGBA([0, 0, 0, 50]), ColorType::RGBA([0, 0, 0, 100]));
    }


    #[test]
    fn color_type_is_copy()
    {
        let original = ColorType::RGB([1, 2, 3]);
        let copy = original;
        assert_eq!(original, copy);
    }

    #[test]
    fn color_type_clone_equals_original()
    {
        let original = ColorType::RGBA([10, 20, 30, 50]);
        assert_eq!(original.clone(), original);
    }


    #[test]
    fn gradient_wraps_f32_and_vec()
    {
        let Gradient::Gradient((angle, stops)) = Gradient::Gradient((45.0, vec![(0.0, ColorType::RGB([0, 0, 0])), (1.0, ColorType::RGB([255, 255, 255]))]));
        assert!((angle - 45.0).abs() < 0.001);
        assert_eq!(stops.len(), 2);
    }
    
    #[test]
    fn gradient_with_empty_stops_is_valid()
    {
        let Gradient::Gradient((_, stops)) = Gradient::Gradient((0.0, vec![]));
        assert!(stops.is_empty());
    }

    #[test]
    fn gradient_clone_equals_original()
    {
        let g = Gradient::Gradient((90.0, vec![(0.5, ColorType::RGB([128, 0, 128]))]));
        assert_eq!(g.clone(), g);
    }


    #[test]
    fn color_type_rgb_serializes_and_deserializes()
    {
        let original = ColorType::RGB([100, 150, 200]);
        let serialized = ron::to_string(&original).unwrap();
        let back: ColorType = ron::from_str(&serialized).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn color_type_rgba_serializes_and_deserializes()
    {
        let original = ColorType::RGBA([10, 20, 30, 80]);
        let serialized = ron::to_string(&original).unwrap();
        let back: ColorType = ron::from_str(&serialized).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn color_type_hex_deserializes_from_ron_hex_string()
    {
        let ron_str = r#"HEX("ff0000")"#;
        let c: ColorType = ron::from_str(ron_str).unwrap();
        assert!(matches!(c, ColorType::HEX(_)));
    }


    #[test]
    fn rgb_to_iced_color_called_twice_is_same()
    {
        let ct = ColorType::RGB([42, 43, 44]);
        assert_eq!(ct.to_iced_color(), ct.to_iced_color());
    }

    #[test]
    fn hex_color_all_zeros_without_hash_converts_to_black()
    {
        let c = hex_color("000000").to_iced_color();
        let expected = iced::Color::from_rgb8(0, 0, 0);
        assert!((c.r - expected.r).abs() < 0.01);
        assert!((c.g - expected.g).abs() < 0.01);
        assert!((c.b - expected.b).abs() < 0.01);
    }

    #[test]
    fn hex_uppercase_digits_parsed_correctly()
    {
        let c = hex_color("#FF0000").to_iced_color();
        let expected = iced::Color::from_rgb8(255, 0, 0);
        assert!((c.r - expected.r).abs() < 0.01);
    }

    #[test]
    fn hex_mixed_case_digits_parsed_correctly()
    {
        let c = hex_color("#Ff0000").to_iced_color();
        let expected = iced::Color::from_rgb8(255, 0, 0);
        assert!((c.r - expected.r).abs() < 0.01);
    }

    #[test]
    fn rgba_25_percent_alpha()
    {
        let c = ColorType::RGBA([0, 0, 0, 25]).to_iced_color();
        assert!((c.a - 0.25).abs() < 0.01);
    }

    #[test]
    fn rgba_75_percent_alpha()
    {
        let c = ColorType::RGBA([0, 0, 0, 75]).to_iced_color();
        assert!((c.a - 0.75).abs() < 0.01);
    }

    #[test]
    fn hex_color_string_length_9_no_hash_is_accepted()
    {
        let s = "123456789";
        let c = hex_color(s);
        assert!(matches!(c, ColorType::HEX(_)));
    }

    #[test]
    fn color_type_debug_contains_variant_name()
    {
        let debug_str = format!("{:?}", ColorType::RGB([0, 0, 0]));
        assert!(debug_str.contains("RGB"));
    }

    #[test]
    fn color_type_rgba_debug_contains_variant_name()
    {
        let debug_str = format!("{:?}", ColorType::RGBA([0, 0, 0, 100]));
        assert!(debug_str.contains("RGBA"));
    }

    #[test]
    fn color_type_hex_debug_contains_variant_name()
    {
        let c = hex_color("#aabbcc");
        let debug_str = format!("{:?}", c);
        assert!(debug_str.contains("HEX"));
    }
}
