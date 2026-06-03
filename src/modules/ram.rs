// ============ IMPORTS ============
use iced::widget::button;





// ============ CRATES ============
use crate::helpers::style::{UserStyle, set_style};
use crate::helpers::style::match_color_or_gradient;
use crate::ron::ActionOnClick;
use crate::AppData;





// ============ CONFIG ============
use serde::{Deserialize, Serialize};
use crate::helpers::style::{TextOrientation, SideOption};
use crate::helpers::color::{ColorType, Gradient};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct RamConfig
{
    pub ram_format:                        String,
    pub ram_update_interval:               u64,
    pub action_on_left_click_ram:          ActionOnClick,
    pub action_on_right_click_ram:         ActionOnClick,
    pub ram_padding:                       u16,
    pub ram_text_size:                     u32,
    pub ram_text_color:                    ColorType,
    pub ram_text_orientation:              TextOrientation,
    pub ram_button_color:                  ColorType,
    pub ram_button_hovered_color:          ColorType,
    pub ram_button_hovered_text_color:     ColorType,
    pub ram_button_pressed_text_color:     ColorType,
    pub ram_button_pressed_color:          ColorType,
    pub ram_border_color:                  ColorType,
    pub ram_border_size:                   f32,
    pub ram_border_radius:                 [f32; 4],
    pub ram_side_separator:                Option<SideOption>,
    pub ram_side_separator_color:          ColorType,
    pub ram_side_separator_width:          f32,
    pub ram_side_separator_height:         f32,
    pub ram_button_gradient_color:         Option<Gradient>,
    pub ram_button_hovered_gradient_color: Option<Gradient>,
    pub ram_button_pressed_gradient_color: Option<Gradient>,
    pub ram_button_shadow_color:           Option<ColorType>,
    pub ram_button_shadow_x:               f32,
    pub ram_button_shadow_y:               f32,
    pub ram_button_shadow_blur:            f32
}

impl Default for RamConfig
{
    fn default() -> Self
    {
        Self
        {
            ram_format:                        " {used}MB / {total}MB {percent}%".into(),
            ram_update_interval:               1050,
            action_on_left_click_ram:          ActionOnClick::DefaultAction,
            action_on_right_click_ram:         ActionOnClick::DefaultAction,
            ram_padding:                       0,
            ram_text_size:                     12,
            ram_text_color:                    ColorType::RGB([220, 220, 220]),
            ram_text_orientation:              TextOrientation::Horizontal,
            ram_button_color:                  ColorType::RGB([40, 40, 50]),
            ram_button_hovered_color:          ColorType::RGB([60, 60, 75]),
            ram_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            ram_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            ram_button_pressed_color:          ColorType::RGB([30, 30, 40]),
            ram_border_color:                  ColorType::RGB([80, 80, 100]),
            ram_border_size:                   1.0,
            ram_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            ram_side_separator:                None,
            ram_side_separator_color:          ColorType::RGB([75, 75, 75]),
            ram_side_separator_width:          1.,
            ram_side_separator_height:         16.,
            ram_button_gradient_color:         None,
            ram_button_hovered_gradient_color: None,
            ram_button_pressed_gradient_color: None,
            ram_button_shadow_color:           None,
            ram_button_shadow_x:               0.0,
            ram_button_shadow_y:               0.0,
            ram_button_shadow_blur:            0.0
        }
    }
}





// ============ STRUCTS ============
#[derive(Default, Debug, Clone)]
pub struct RamData
{
    pub used_mb:    u64,
    pub total_mb:   u64,
    pub percent:    f32
}





// ============ FUNCTIONS ============
fn compute_ram_data(total_kb: u64, available_kb: u64) -> RamData
{
    let used_kb  = total_kb.saturating_sub(available_kb);
    let total_mb = total_kb / 1024;
    let used_mb  = used_kb  / 1024;
    let percent  = if total_kb == 0 { 0.0 } else { used_kb as f32 / total_kb as f32 * 100.0 };
    RamData { used_mb, total_mb, percent }
}



pub fn read_ram_data() -> Option<RamData>
{
    use std::io::{BufRead, BufReader};
    let file = std::fs::File::open("/proc/meminfo").ok()?;
    let reader = BufReader::new(file);

    let mut total:     Option<u64> = None;
    let mut available: Option<u64> = None;

    for line in reader.lines().map_while(Result::ok)
    {
        if line.starts_with("MemTotal:")     { total     = parse_kb(&line); }
        if line.starts_with("MemAvailable:") { available = parse_kb(&line); }
        if total.is_some() && available.is_some() { break; }
    }

    let total_kb     = total?;
    let available_kb = available?;

    Some(compute_ram_data(total_kb, available_kb))
}



fn parse_kb(line: &str) -> Option<u64>
{
    line.split_whitespace().nth(1)?.parse().ok()
}



pub fn define_ram_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let cfg = &app.ron_config.ram;
    let normal_background  = match_color_or_gradient(cfg.ram_button_gradient_color.as_ref(),         cfg.ram_button_color);
    let hovered_background = match_color_or_gradient(cfg.ram_button_hovered_gradient_color.as_ref(), cfg.ram_button_hovered_color);
    let pressed_background = match_color_or_gradient(cfg.ram_button_pressed_gradient_color.as_ref(), cfg.ram_button_pressed_color);
    set_style(UserStyle
    {
        status,
        normal_text:        cfg.ram_text_color,
        hovered_text:       cfg.ram_button_hovered_text_color,
        pressed_text:       cfg.ram_button_pressed_text_color,
        border_color:       cfg.ram_border_color,
        border_size:        cfg.ram_border_size,
        border_radius:      cfg.ram_border_radius,
        normal_background,
        hovered_background,
        pressed_background,
        shadow_color:       cfg.ram_button_shadow_color,
        shadow_x:           cfg.ram_button_shadow_x,
        shadow_y:           cfg.ram_button_shadow_y,
        shadow_blur:        cfg.ram_button_shadow_blur
    })
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;


    #[test]
    fn parse_kb_valid_line()
    {
        assert_eq!(parse_kb("MemTotal:       16384000 kB"), Some(16384000));
    }

    #[test]
    fn parse_kb_missing_value_returns_none()
    {
        assert_eq!(parse_kb("MemTotal:"), None);
    }

    #[test]
    fn parse_kb_non_numeric_returns_none()
    {
        assert_eq!(parse_kb("MemTotal: abc kB"), None);
    }


    #[test]
    fn read_ram_data_returns_some_on_linux()
    {
        assert!(read_ram_data().is_some());
    }

    #[test]
    fn read_ram_data_used_does_not_exceed_total()
    {
        let d = read_ram_data().unwrap();
        assert!(d.used_mb <= d.total_mb);
    }

    #[test]
    fn read_ram_data_percent_within_0_to_100()
    {
        let d = read_ram_data().unwrap();
        assert!(d.percent >= 0.0 && d.percent <= 100.0);
    }

    #[test]
    fn read_ram_data_total_is_nonzero()
    {
        let d = read_ram_data().unwrap();
        assert!(d.total_mb > 0);
    }

 
    #[test]
    fn parse_kb_zero_value()
    {
        assert_eq!(parse_kb("MemAvailable:  0 kB"), Some(0));
    }
 
    #[test]
    fn parse_kb_no_unit_still_parses_number()
    {
        assert_eq!(parse_kb("MemTotal: 8192"), Some(8192));
    }
 
    #[test]
    fn parse_kb_empty_string_returns_none()
    {
        assert_eq!(parse_kb(""), None);
    }
 
    #[test]
    fn parse_kb_whitespace_only_returns_none()
    {
        assert_eq!(parse_kb("   "), None);
    }
 
    #[test]
    fn parse_kb_only_key_no_value_returns_none()
    {
        assert_eq!(parse_kb("MemTotal:"), None);
    }
 
    #[test]
    fn parse_kb_large_value()
    {
        assert_eq!(parse_kb("MemTotal: 67108864 kB"), Some(67108864));
    }
 
    #[test]
    fn parse_kb_negative_string_returns_none()
    {
        assert_eq!(parse_kb("MemTotal: -1024 kB"), None);
    }
 
    #[test]
    fn parse_kb_float_string_returns_none()
    {
        assert_eq!(parse_kb("MemTotal: 1024.5 kB"), None);
    }
 
 
    #[test]
    fn compute_ram_data_used_is_total_minus_available()
    {
        let d = compute_ram_data(8_000_000, 6_000_000);
        assert_eq!(d.used_mb, 2_000_000 / 1024);
    }
 
    #[test]
    fn compute_ram_data_total_mb_is_total_kb_divided_by_1024()
    {
        let d = compute_ram_data(1024 * 1024, 0);
        assert_eq!(d.total_mb, 1024);
    }
 
    #[test]
    fn compute_ram_data_used_mb_is_used_kb_divided_by_1024()
    {
        let d = compute_ram_data(2048, 1024);
        assert_eq!(d.used_mb, 1);
    }
 
    #[test]
    fn compute_ram_data_percent_50()
    {
        let d = compute_ram_data(1000, 500);
        assert!((d.percent - 50.0).abs() < 0.1);
    }
 
    #[test]
    fn compute_ram_data_percent_100_when_available_is_zero()
    {
        let d = compute_ram_data(1000, 0);
        assert!((d.percent - 100.0).abs() < 0.1);
    }
 
    #[test]
    fn compute_ram_data_percent_0_when_nothing_used()
    {
        let d = compute_ram_data(1000, 1000);
        assert_eq!(d.percent, 0.0);
    }
 
    #[test]
    fn compute_ram_data_zero_total_returns_zero_percent()
    {
        let d = compute_ram_data(0, 0);
        assert_eq!(d.percent, 0.0);
    }
 
    #[test]
    fn compute_ram_data_available_greater_than_total_saturates_to_zero_used()
    {
        let d = compute_ram_data(1000, 2000);
        assert_eq!(d.used_mb, 0);
        assert_eq!(d.percent, 0.0);
    }
 
    #[test]
    fn compute_ram_data_percent_within_0_to_100()
    {
        for (total, avail) in [(1000, 0), (1000, 500), (1000, 1000), (1000, 2000)]
        {
            let d = compute_ram_data(total, avail);
            assert!(d.percent >= 0.0 && d.percent <= 100.0,
                "percent out of range for total={total} avail={avail}: {}", d.percent);
        }
    }
 
    #[test]
    fn compute_ram_data_used_never_exceeds_total()
    {
        for (total, avail) in [(0, 0), (1024, 512), (1024, 2048)]
        {
            let d = compute_ram_data(total, avail);
            assert!(d.used_mb <= d.total_mb,
                "used_mb {} > total_mb {} for total={total} avail={avail}", d.used_mb, d.total_mb);
        }
    }
 
 
    #[test]
    fn read_ram_data_used_is_nonzero_on_running_system()
    {
        let d = read_ram_data().unwrap();
        assert!(d.used_mb > 0);
    }
 
    #[test]
    fn read_ram_data_percent_is_nonzero_on_running_system()
    {
        let d = read_ram_data().unwrap();
        assert!(d.percent > 0.0);
    }
 
    #[test]
    fn read_ram_data_used_plus_available_approximately_equals_total()
    {
        let content = std::fs::read_to_string("/proc/meminfo").unwrap();
        let mut available_kb: u64 = 0;
        for line in content.lines()
        {
            if line.starts_with("MemAvailable:") { available_kb = parse_kb(line).unwrap_or(0); }
        }
        let d = read_ram_data().unwrap();
        let available_mb = available_kb / 1024;
        assert!(d.used_mb + available_mb <= d.total_mb + 1);
    }
 
 
    #[test]
    fn ram_data_default_all_zero()
    {
        let d = RamData::default();
        assert_eq!(d.used_mb,  0);
        assert_eq!(d.total_mb, 0);
        assert_eq!(d.percent,  0.0);
    }



    #[test]
    fn compute_ram_data_used_mb_correct()
    {
        let d = compute_ram_data(2048, 1024);
        assert_eq!(d.used_mb, 1);
    }

    #[test]
    fn compute_ram_data_total_mb_correct()
    {
        let d = compute_ram_data(4096, 0);
        assert_eq!(d.total_mb, 4);
    }

    #[test]
    fn compute_ram_data_100_percent_when_all_used()
    {
        let d = compute_ram_data(1000, 0);
        assert!((d.percent - 100.0).abs() < 0.1);
    }

    #[test]
    fn compute_ram_data_0_percent_when_nothing_used()
    {
        let d = compute_ram_data(1000, 1000);
        assert_eq!(d.percent, 0.0);
    }

    #[test]
    fn compute_ram_data_percent_is_finite()
    {
        let d = compute_ram_data(10_000_000, 5_000_000);
        assert!(d.percent.is_finite());
    }

    #[test]
    fn compute_ram_data_percent_in_0_100_range_for_various_inputs()
    {
        for (t, a) in [(0u64, 0u64), (1024, 0), (1024, 512), (1024, 1024), (1024, 2048)]
        {
            let d = compute_ram_data(t, a);
            assert!(d.percent >= 0.0 && d.percent <= 100.0,
                "percent={} for total={} avail={}", d.percent, t, a);
        }
    }


    #[test]
    fn ram_config_default_format_contains_placeholders()
    {
        let fmt = RamConfig::default().ram_format;
        assert!(fmt.contains("{used}") || fmt.contains("{total}") || fmt.contains("{percent}"));
    }

    #[test]
    fn ram_config_default_update_interval_is_positive()
    {
        assert!(RamConfig::default().ram_update_interval > 0);
    }

    #[test]
    fn ram_config_default_text_size_is_positive()
    {
        assert!(RamConfig::default().ram_text_size > 0);
    }

    #[test]
    fn ram_config_default_gradient_is_none()
    {
        assert!(RamConfig::default().ram_button_gradient_color.is_none());
    }

    #[test]
    fn ram_config_default_shadow_color_is_none()
    {
        assert!(RamConfig::default().ram_button_shadow_color.is_none());
    }

    #[test]
    fn ram_config_default_side_separator_is_none()
    {
        assert!(RamConfig::default().ram_side_separator.is_none());
    }

    #[test]
    fn ram_config_default_border_radius_all_equal()
    {
        let r = RamConfig::default().ram_border_radius;
        assert_eq!(r[0], r[1]);
        assert_eq!(r[1], r[2]);
        assert_eq!(r[2], r[3]);
    }


    #[test]
    fn read_ram_data_total_mb_is_nonzero()
    {
        let d = read_ram_data().unwrap();
        assert!(d.total_mb > 0);
    }

    #[test]
    fn read_ram_data_used_does_not_exceed_total_mb()
    {
        let d = read_ram_data().unwrap();
        assert!(d.used_mb <= d.total_mb);
    }

    #[test]
    fn read_ram_data_called_twice_gives_consistent_total()
    {
        let a = read_ram_data().unwrap();
        let b = read_ram_data().unwrap();
        assert_eq!(a.total_mb, b.total_mb);
    }
}
