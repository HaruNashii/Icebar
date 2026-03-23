// ============ IMPORTS ============
use iced::widget::button;
use serde::{Deserialize, Serialize};




// ============ CRATES ============
use crate::helpers::style::{UserStyle, orient_text, set_style, TextOrientation, SideOption};
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
    pub cpu_button_shadow_blur:            f32,
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
            cpu_button_shadow_blur:            0.0,
        }
    }
}

// ============ STRUCTS ============
#[derive(Default, Copy, Clone, Debug)]
pub struct CpuData
{
    pub cpu_snapshot: Option<crate::modules::cpu::CpuSnapshot>,
    pub usage_percent: f32,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct CpuSnapshot
{
    pub total: u64,
    pub idle:  u64,
}





// ============ FUNCTIONS ============
pub fn read_cpu_snapshot() -> Option<CpuSnapshot>
{
    let content = std::fs::read_to_string("/proc/stat").ok()?;
    let cpu_line = content.lines().find(|l| l.starts_with("cpu "))?;

    let mut fields = cpu_line.split_whitespace().skip(1); // skip "cpu"
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

pub fn define_cpu_text(app: &AppData) -> String
{
    let usage = app.modules_data.cpu_data.usage_percent;
    let text  = app.ron_config.cpu.cpu_format.replace("{usage}", &format!("{:.0}", usage));
    orient_text(&text, &app.ron_config.cpu.cpu_text_orientation)
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
        shadow_blur: app.ron_config.cpu.cpu_button_shadow_blur,
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
        // delta_total=100, delta_idle=80 → 20% active
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
        // Pathological case: idle went negative (kernel counter wrap)
        let prev = CpuSnapshot { total: 1000, idle: 900 };
        let curr = CpuSnapshot { total: 1100, idle: 800 }; // idle decreased
        assert!(compute_cpu_usage(&prev, &curr) <= 100.0);
    }

    #[test]
    fn read_cpu_snapshot_returns_some_on_linux()
    {
        // /proc/stat is always present on Linux
        assert!(read_cpu_snapshot().is_some());
    }

        // ---- compute_cpu_usage: more cases --------------------------------------
 
    #[test]
    fn compute_cpu_usage_50_percent()
    {
        let prev = CpuSnapshot { total: 1000, idle: 1000 };
        let curr = CpuSnapshot { total: 1200, idle: 1100 };
        // delta_total=200, delta_idle=100 → 50%
        assert!((compute_cpu_usage(&prev, &curr) - 50.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_75_percent()
    {
        let prev = CpuSnapshot { total: 0,   idle: 0   };
        let curr = CpuSnapshot { total: 400, idle: 100 };
        // delta_total=400, delta_idle=100 → 75%
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
        // curr idle > prev idle by more than total growth — pathological wrap
        let prev = CpuSnapshot { total: 500, idle: 100 };
        let curr = CpuSnapshot { total: 600, idle: 600 };
        assert!(compute_cpu_usage(&prev, &curr) >= 0.0);
    }
 
    #[test]
    fn compute_cpu_usage_large_counter_values()
    {
        // Counters grow for weeks on long-running machines
        let base: u64 = 1_000_000_000;
        let prev = CpuSnapshot { total: base,          idle: base - 200 };
        let curr = CpuSnapshot { total: base + 1000,   idle: base - 200 + 500 };
        // delta_total=1000, delta_idle=500 → 50%
        let usage = compute_cpu_usage(&prev, &curr);
        assert!((usage - 50.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_single_tick_difference()
    {
        // Minimal possible delta: 1 tick active
        let prev = CpuSnapshot { total: 100, idle: 100 };
        let curr = CpuSnapshot { total: 101, idle: 100 };
        assert!((compute_cpu_usage(&prev, &curr) - 100.0).abs() < 0.01);
    }
 
    #[test]
    fn compute_cpu_usage_single_tick_idle()
    {
        let prev = CpuSnapshot { total: 100, idle: 99 };
        let curr = CpuSnapshot { total: 101, idle: 100 };
        // delta_total=1, delta_idle=1 → 0%
        assert_eq!(compute_cpu_usage(&prev, &curr), 0.0);
    }
 
    // ---- read_cpu_snapshot: content checks ----------------------------------
 
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
        // CPU counters are monotonically increasing
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
        // A real delta over two snapshots must be 0–100%
        let prev = read_cpu_snapshot().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        let curr = read_cpu_snapshot().unwrap();
        let usage = compute_cpu_usage(&prev, &curr);
        assert!(usage >= 0.0 && usage <= 100.0);
    }
 
    // ---- CpuSnapshot default ------------------------------------------------
 
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
        // Default snapshot vs itself — must not divide by zero
        let snap = CpuSnapshot::default();
        assert_eq!(compute_cpu_usage(&snap, &snap), 0.0);
    }
 
    // ---- CpuData default ----------------------------------------------------
 
    #[test]
    fn cpu_data_default_usage_is_zero()
    {
        assert_eq!(CpuData::default().usage_percent, 0.0);
    }
}
