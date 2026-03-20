// ============ IMPORTS ============
use iced::widget::button;





// ============ CRATES ============
use crate::helpers::style::{UserStyle, orient_text, set_style};
use crate::AppData;





// ============ STRUCTS ============
#[derive(Default, Clone)]
pub struct RamData
{
    pub used_mb:    u64,
    pub total_mb:   u64,
    pub percent:    f32,
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
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;

    let mut total:     Option<u64> = None;
    let mut available: Option<u64> = None;

    for line in content.lines()
    {
        if line.starts_with("MemTotal:")     { total     = parse_kb(line); }
        if line.starts_with("MemAvailable:") { available = parse_kb(line); }
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



pub fn define_ram_text(app: &AppData) -> String
{
    let d    = &app.modules_data.ram_data;
    let text = app.ron_config.ram_format
        .replace("{used}",    &d.used_mb.to_string())
        .replace("{total}",   &d.total_mb.to_string())
        .replace("{percent}", &format!("{:.0}", d.percent));
    orient_text(&text, &app.ron_config.ram_text_orientation)
}



pub fn define_ram_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    set_style(UserStyle
    {
        status,
        normal:            app.ron_config.ram_button_color,
        normal_text:       app.ron_config.ram_text_color,
        hovered:           app.ron_config.ram_button_hovered_color,
        hovered_text:      app.ron_config.ram_button_hovered_text_color,
        pressed:           app.ron_config.ram_button_pressed_color,
        border_color: app.ron_config.ram_border_color,
        border_size:       app.ron_config.ram_border_size,
        border_radius:     app.ron_config.ram_border_radius,
        hovered_gradient: app.ron_config.ram_button_hovered_gradient_color.clone(),
        normal_gradient: app.ron_config.ram_button_gradient_color.clone(),
        pressed_gradient: app.ron_config.ram_button_pressed_gradient_color.clone()
    })
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;

    // ---- parse_kb -----------------------------------------------------------

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

    // ---- read_ram_data ------------------------------------------------------

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

        // ---- parse_kb: edge cases -----------------------------------------------
 
    #[test]
    fn parse_kb_zero_value()
    {
        assert_eq!(parse_kb("MemAvailable:  0 kB"), Some(0));
    }
 
    #[test]
    fn parse_kb_no_unit_still_parses_number()
    {
        // unit column is ignored — only the number matters
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
        // u64 cannot be negative
        assert_eq!(parse_kb("MemTotal: -1024 kB"), None);
    }
 
    #[test]
    fn parse_kb_float_string_returns_none()
    {
        assert_eq!(parse_kb("MemTotal: 1024.5 kB"), None);
    }
 
    // ---- compute_ram_data: math ---------------------------------------------
 
    #[test]
    fn compute_ram_data_used_is_total_minus_available()
    {
        let d = compute_ram_data(8_000_000, 6_000_000);
        assert_eq!(d.used_mb, 2_000_000 / 1024);
    }
 
    #[test]
    fn compute_ram_data_total_mb_is_total_kb_divided_by_1024()
    {
        let d = compute_ram_data(1024 * 1024, 0); // 1 GB
        assert_eq!(d.total_mb, 1024);
    }
 
    #[test]
    fn compute_ram_data_used_mb_is_used_kb_divided_by_1024()
    {
        let d = compute_ram_data(2048, 1024); // used_kb = 1024
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
        // Guard against divide-by-zero
        let d = compute_ram_data(0, 0);
        assert_eq!(d.percent, 0.0);
    }
 
    #[test]
    fn compute_ram_data_available_greater_than_total_saturates_to_zero_used()
    {
        // saturating_sub prevents underflow
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
 
    // ---- read_ram_data: live /proc/meminfo ----------------------------------
 
    #[test]
    fn read_ram_data_used_is_nonzero_on_running_system()
    {
        // A running system always has some RAM in use
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
        // used_mb + available_mb should be very close to total_mb
        // We re-parse available directly to check the invariant
        let content = std::fs::read_to_string("/proc/meminfo").unwrap();
        let mut available_kb: u64 = 0;
        for line in content.lines()
        {
            if line.starts_with("MemAvailable:") { available_kb = parse_kb(line).unwrap_or(0); }
        }
        let d = read_ram_data().unwrap();
        let available_mb = available_kb / 1024;
        // Allow 1 MB rounding slack from the /1024 integer division
        assert!(d.used_mb + available_mb <= d.total_mb + 1);
    }
 
    // ---- RamData default ----------------------------------------------------
 
    #[test]
    fn ram_data_default_all_zero()
    {
        let d = RamData::default();
        assert_eq!(d.used_mb,  0);
        assert_eq!(d.total_mb, 0);
        assert_eq!(d.percent,  0.0);
    }

}
