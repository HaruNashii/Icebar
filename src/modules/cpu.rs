// ============ IMPORTS ============
use iced::widget::button;
use serde::{Deserialize, Serialize};





// ============ CRATES ============
use crate::helpers::style::{UserStyle, set_style, TextOrientation, SideOption};
use crate::helpers::color::{ColorType, Gradient};
use crate::ron::ActionOnClick;
use crate::AppData;





// ============ CONFIG ============
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct CpuConfig
{
    pub cpu_format:                        String,
    pub cpu_update_interval:               u64,
    pub action_on_left_click_cpu:          ActionOnClick,
    pub action_on_right_click_cpu:         ActionOnClick,
    pub cpu_padding:                       u16,
    pub cpu_text_size:                     u32,
    pub cpu_text_color:                    ColorType,
    pub cpu_text_orientation:              TextOrientation,
    pub cpu_button_color:                  ColorType,
    pub cpu_button_hovered_color:          ColorType,
    pub cpu_button_hovered_text_color:     ColorType,
    pub cpu_button_pressed_text_color:     ColorType,
    pub cpu_button_pressed_color:          ColorType,
    pub cpu_border_color:                  ColorType,
    pub cpu_border_size:                   f32,
    pub cpu_border_radius:                 [f32; 4],
    pub cpu_side_separator:                Option<SideOption>,
    pub cpu_side_separator_color:          ColorType,
    pub cpu_side_separator_width:          f32,
    pub cpu_side_separator_height:         f32,
    pub cpu_button_gradient_color:         Option<Gradient>,
    pub cpu_button_hovered_gradient_color: Option<Gradient>,
    pub cpu_button_pressed_gradient_color: Option<Gradient>,
    pub cpu_button_shadow_color:           Option<ColorType>,
    pub cpu_button_shadow_x:               f32,
    pub cpu_button_shadow_y:               f32,
    pub cpu_button_shadow_blur:            f32
}

impl Default for CpuConfig
{
    fn default() -> Self
    {
        Self
        {
            cpu_format:                        "CPU: {usage}%".into(),
            cpu_update_interval:               1050,
            action_on_left_click_cpu:          ActionOnClick::DefaultAction,
            action_on_right_click_cpu:         ActionOnClick::DefaultAction,
            cpu_padding:                       0,
            cpu_text_size:                     12,
            cpu_text_color:                    ColorType::RGB([220, 220, 220]),
            cpu_text_orientation:              TextOrientation::Horizontal,
            cpu_button_color:                  ColorType::RGB([40, 40, 50]),
            cpu_button_hovered_color:          ColorType::RGB([60, 60, 75]),
            cpu_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            cpu_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            cpu_button_pressed_color:          ColorType::RGB([30, 30, 40]),
            cpu_border_color:                  ColorType::RGB([80, 80, 100]),
            cpu_border_size:                   1.0,
            cpu_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            cpu_side_separator:                None,
            cpu_side_separator_color:          ColorType::RGB([75, 75, 75]),
            cpu_side_separator_width:          1.,
            cpu_side_separator_height:         16.,
            cpu_button_gradient_color:         None,
            cpu_button_hovered_gradient_color: None,
            cpu_button_pressed_gradient_color: None,
            cpu_button_shadow_color:           None,
            cpu_button_shadow_x:               0.0,
            cpu_button_shadow_y:               0.0,
            cpu_button_shadow_blur:            0.0
        }
    }
}





// ============ STRUCTS ============
#[derive(Default, Copy, Clone, Debug)]
pub struct CpuSnapshot
{
    pub total: u64,
    pub idle:  u64
}





// ============ FUNCTIONS ============
pub fn read_cpu_snapshot() -> Option<CpuSnapshot>
{
    use std::io::{BufRead, BufReader};
    let file = std::fs::File::open("/proc/stat").ok()?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).ok()?;
    if !line.starts_with("cpu ") { return None; }

    let mut fields = line.split_whitespace().skip(1);
    let user:    u64 = fields.next()?.parse().ok()?;
    let nice:    u64 = fields.next()?.parse().ok()?;
    let system:  u64 = fields.next()?.parse().ok()?;
    let idle:    u64 = fields.next()?.parse().ok()?;
    let iowait:  u64 = fields.next()?.parse().ok()?;
    let irq:     u64 = fields.next()?.parse().ok()?;
    let softirq: u64 = fields.next()?.parse().ok()?;
    let steal:   u64 = fields.next().and_then(|v| v.parse().ok()).unwrap_or(0);

    let idle  = idle + iowait;
    let total = user + nice + system + idle + irq + softirq + steal;

    Some(CpuSnapshot { total, idle })
}

pub fn compute_cpu_usage(prev: &CpuSnapshot, curr: &CpuSnapshot) -> f32
{
    let total_delta = curr.total.saturating_sub(prev.total) as f32;
    let idle_delta  = curr.idle.saturating_sub(prev.idle)  as f32;
    if total_delta == 0.0 { return 0.0; }
    ((total_delta - idle_delta) / total_delta * 100.0).clamp(0.0, 100.0)
}

pub fn define_cpu_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    set_style(UserStyle
    {
        status,
        normal:            app.ron_config.cpu.cpu_button_color,
        normal_text:       app.ron_config.cpu.cpu_text_color,
        hovered:           app.ron_config.cpu.cpu_button_hovered_color,
        hovered_text:      app.ron_config.cpu.cpu_button_hovered_text_color,
        pressed_text:      app.ron_config.cpu.cpu_button_pressed_text_color,
        pressed:           app.ron_config.cpu.cpu_button_pressed_color,
        border_color: app.ron_config.cpu.cpu_border_color,
        border_size:       app.ron_config.cpu.cpu_border_size,
        border_radius:     app.ron_config.cpu.cpu_border_radius,
        normal_gradient: app.ron_config.cpu.cpu_button_gradient_color.clone(),
        hovered_gradient: app.ron_config.cpu.cpu_button_hovered_gradient_color.clone(),
        pressed_gradient: app.ron_config.cpu.cpu_button_pressed_gradient_color.clone(),
        shadow_color: app.ron_config.cpu.cpu_button_shadow_color,
        shadow_x: app.ron_config.cpu.cpu_button_shadow_x,
        shadow_y: app.ron_config.cpu.cpu_button_shadow_y,
        shadow_blur: app.ron_config.cpu.cpu_button_shadow_blur
    })
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn compute_cpu_usage_normal_case()
    {
        let prev = CpuSnapshot { total: 1000, idle: 800 };
        let curr = CpuSnapshot { total: 1100, idle: 880 };
        let usage = compute_cpu_usage(&prev, &curr);
        assert!((usage - 20.0).abs() < 0.01);
    }

    #[test]
    fn compute_cpu_usage_zero_delta_returns_zero()
    {
        let snap  = CpuSnapshot { total: 1000, idle: 800 };
        assert_eq!(compute_cpu_usage(&snap, &snap), 0.0);
    }

    #[test]
    fn compute_cpu_usage_fully_active_returns_100()
    {
        let prev = CpuSnapshot { total: 1000, idle: 0 };
        let curr = CpuSnapshot { total: 1100, idle: 0 };
        assert!((compute_cpu_usage(&prev, &curr) - 100.0).abs() < 0.01);
    }

    #[test]
    fn compute_cpu_usage_fully_idle_returns_zero()
    {
        let prev = CpuSnapshot { total: 1000, idle: 1000 };
        let curr = CpuSnapshot { total: 1100, idle: 1100 };
        assert_eq!(compute_cpu_usage(&prev, &curr), 0.0);
    }

    #[test]
    fn compute_cpu_usage_clamped_to_100()
    {
        let prev = CpuSnapshot { total: 1000, idle: 900 };
        let curr = CpuSnapshot { total: 1100, idle: 800 }; // idle decreased
        assert!(compute_cpu_usage(&prev, &curr) <= 100.0);
    }

    #[test]
    fn read_cpu_snapshot_returns_some_on_linux()
    {
        assert!(read_cpu_snapshot().is_some());
    }

 
    #[test]
    fn compute_cpu_usage_50_percent()
    {
        let prev = CpuSnapshot { total: 1000, idle: 1000 };
        let curr = CpuSnapshot { total: 1200, idle: 1100 };
        assert!((compute_cpu_usage(&prev, &curr) - 50.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_75_percent()
    {
        let prev = CpuSnapshot { total: 0,   idle: 0   };
        let curr = CpuSnapshot { total: 400, idle: 100 };
        assert!((compute_cpu_usage(&prev, &curr) - 75.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_1_percent()
    {
        let prev = CpuSnapshot { total: 0,    idle: 0    };
        let curr = CpuSnapshot { total: 1000, idle: 990  };
        assert!((compute_cpu_usage(&prev, &curr) - 1.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_result_is_never_negative()
    {
        let prev = CpuSnapshot { total: 500, idle: 100 };
        let curr = CpuSnapshot { total: 600, idle: 600 };
        assert!(compute_cpu_usage(&prev, &curr) >= 0.0);
    }
 
    #[test]
    fn compute_cpu_usage_large_counter_values()
    {
        let base: u64 = 1_000_000_000;
        let prev = CpuSnapshot { total: base,          idle: base - 200 };
        let curr = CpuSnapshot { total: base + 1000,   idle: base - 200 + 500 };
        let usage = compute_cpu_usage(&prev, &curr);
        assert!((usage - 50.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_single_tick_difference()
    {
        let prev = CpuSnapshot { total: 100, idle: 100 };
        let curr = CpuSnapshot { total: 101, idle: 100 };
        assert!((compute_cpu_usage(&prev, &curr) - 100.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_single_tick_idle()
    {
        let prev = CpuSnapshot { total: 100, idle: 99 };
        let curr = CpuSnapshot { total: 101, idle: 100 };
        assert_eq!(compute_cpu_usage(&prev, &curr), 0.0);
    }
 
 
    #[test]
    fn read_cpu_snapshot_total_is_nonzero()
    {
        let snap = read_cpu_snapshot().unwrap();
        assert!(snap.total > 0);
    }
 
    #[test]
    fn read_cpu_snapshot_total_greater_than_or_equal_to_idle()
    {
        let snap = read_cpu_snapshot().unwrap();
        assert!(snap.total >= snap.idle);
    }
 
    #[test]
    fn read_cpu_snapshot_two_calls_total_is_nondecreasing()
    {
        let first  = read_cpu_snapshot().unwrap();
        let second = read_cpu_snapshot().unwrap();
        assert!(second.total >= first.total);
    }
 
    #[test]
    fn read_cpu_snapshot_two_calls_idle_is_nondecreasing()
    {
        let first  = read_cpu_snapshot().unwrap();
        let second = read_cpu_snapshot().unwrap();
        assert!(second.idle >= first.idle);
    }
 
    #[test]
    fn read_cpu_snapshot_usage_between_snapshots_is_valid()
    {
        let prev = read_cpu_snapshot().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        let curr = read_cpu_snapshot().unwrap();
        let usage = compute_cpu_usage(&prev, &curr);
        assert!(usage >= 0.0 && usage <= 100.0);
    }
 
 
    #[test]
    fn cpu_snapshot_default_is_all_zeros()
    {
        let snap = CpuSnapshot::default();
        assert_eq!(snap.total, 0);
        assert_eq!(snap.idle,  0);
    }
 
    #[test]
    fn compute_cpu_usage_both_zero_snapshots_returns_zero()
    {
        let snap = CpuSnapshot::default();
        assert_eq!(compute_cpu_usage(&snap, &snap), 0.0);
    }

    // ---- Additional compute_cpu_usage edge cases ----

    #[test]
    fn compute_cpu_usage_exactly_25_percent()
    {
        let prev = CpuSnapshot { total: 0,   idle: 0   };
        let curr = CpuSnapshot { total: 400, idle: 300 };
        assert!((compute_cpu_usage(&prev, &curr) - 25.0).abs() < 0.01);
    }

    #[test]
    fn compute_cpu_usage_exactly_99_percent()
    {
        let prev = CpuSnapshot { total: 0,    idle: 0   };
        let curr = CpuSnapshot { total: 100,  idle: 1   };
        assert!((compute_cpu_usage(&prev, &curr) - 99.0).abs() < 0.01);
    }

    #[test]
    fn compute_cpu_usage_prev_equals_curr_returns_zero()
    {
        let snap = CpuSnapshot { total: 5000, idle: 3000 };
        assert_eq!(compute_cpu_usage(&snap, &snap), 0.0);
    }

    #[test]
    fn compute_cpu_usage_result_at_most_100()
    {
        // Even if idle somehow decreases, result should not exceed 100.0
        for (pt, pi, ct, ci) in [(0u64, 0u64, 100u64, 0u64), (0, 0, 1, 0), (100, 50, 200, 50)]
        {
            let prev = CpuSnapshot { total: pt, idle: pi };
            let curr = CpuSnapshot { total: ct, idle: ci };
            assert!(compute_cpu_usage(&prev, &curr) <= 100.0);
        }
    }

    #[test]
    fn compute_cpu_usage_result_is_finite()
    {
        let prev = CpuSnapshot { total: 0, idle: 0 };
        let curr = CpuSnapshot { total: 1000, idle: 200 };
        let usage = compute_cpu_usage(&prev, &curr);
        assert!(usage.is_finite());
    }

    // ---- read_cpu_snapshot properties ----

    #[test]
    fn read_cpu_snapshot_idle_never_exceeds_total()
    {
        let snap = read_cpu_snapshot().unwrap();
        assert!(snap.idle <= snap.total);
    }

    #[test]
    fn read_cpu_snapshot_multiple_calls_are_monotonically_nondecreasing()
    {
        let a = read_cpu_snapshot().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let b = read_cpu_snapshot().unwrap();
        assert!(b.total >= a.total);
    }

    // ---- CpuSnapshot Debug ----

    #[test]
    fn cpu_snapshot_debug_shows_fields()
    {
        let s = CpuSnapshot { total: 999, idle: 123 };
        let d = format!("{:?}", s);
        assert!(d.contains("999"));
        assert!(d.contains("123"));
    }

    // ---- CpuConfig defaults ----

    #[test]
    fn cpu_config_default_format_contains_usage_placeholder()
    {
        assert!(CpuConfig::default().cpu_format.contains("{usage}"));
    }

    #[test]
    fn cpu_config_default_update_interval_is_positive()
    {
        assert!(CpuConfig::default().cpu_update_interval > 0);
    }

    #[test]
    fn cpu_config_default_text_size_is_positive()
    {
        assert!(CpuConfig::default().cpu_text_size > 0);
    }

    #[test]
    fn cpu_config_default_border_radius_all_equal()
    {
        let r = CpuConfig::default().cpu_border_radius;
        assert_eq!(r[0], r[1]);
        assert_eq!(r[1], r[2]);
        assert_eq!(r[2], r[3]);
    }

    #[test]
    fn cpu_config_default_shadow_color_is_none()
    {
        assert!(CpuConfig::default().cpu_button_shadow_color.is_none());
    }

    #[test]
    fn cpu_config_default_side_separator_is_none()
    {
        assert!(CpuConfig::default().cpu_side_separator.is_none());
    }

    #[test]
    fn cpu_config_default_gradient_is_none()
    {
        assert!(CpuConfig::default().cpu_button_gradient_color.is_none());
        assert!(CpuConfig::default().cpu_button_hovered_gradient_color.is_none());
        assert!(CpuConfig::default().cpu_button_pressed_gradient_color.is_none());
    }

}
