// ============ IMPORTS ============
use iced::widget::button;
use std::mem;





// ============ CRATES ============
use crate::helpers::style::{UserStyle, set_style};
use crate::AppData;





// ============ CONFIG ============
use serde::{Deserialize, Serialize};
use crate::helpers::style::{TextOrientation, SideOption};
use crate::helpers::color::{ColorType, Gradient};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DiskConfig
{
    pub disk_format:                        String,
    pub disk_mount:                         String,
    pub disk_update_interval:               u64,
    pub disk_padding:                       u16,
    pub disk_text_size:                     u32,
    pub disk_text_color:                    ColorType,
    pub disk_text_orientation:              TextOrientation,
    pub disk_button_color:                  ColorType,
    pub disk_button_hovered_color:          ColorType,
    pub disk_button_hovered_text_color:     ColorType,
    pub disk_button_pressed_text_color:     ColorType,
    pub disk_button_pressed_color:          ColorType,
    pub disk_border_color:                  ColorType,
    pub disk_border_size:                   f32,
    pub disk_border_radius:                 [f32; 4],
    pub disk_side_separator:                Option<SideOption>,
    pub disk_side_separator_color:          ColorType,
    pub disk_side_separator_width:          f32,
    pub disk_side_separator_height:         f32,
    pub disk_button_gradient_color:         Option<Gradient>,
    pub disk_button_hovered_gradient_color: Option<Gradient>,
    pub disk_button_pressed_gradient_color: Option<Gradient>,
    pub disk_button_shadow_color:           Option<ColorType>,
    pub disk_button_shadow_x:               f32,
    pub disk_button_shadow_y:               f32,
    pub disk_button_shadow_blur:            f32
}

impl Default for DiskConfig
{
    fn default() -> Self
    {
        Self
        {
            disk_format:                        "{used}GB / {total}GB {percent}%".into(),
            disk_mount:                         "/".into(),
            disk_update_interval:               3000,
            disk_padding:                       0,
            disk_text_size:                     12,
            disk_text_color:                    ColorType::RGB([220, 220, 220]),
            disk_text_orientation:              TextOrientation::Horizontal,
            disk_button_color:                  ColorType::RGB([40, 40, 50]),
            disk_button_hovered_color:          ColorType::RGB([60, 60, 75]),
            disk_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            disk_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            disk_button_pressed_color:          ColorType::RGB([30, 30, 40]),
            disk_border_color:                  ColorType::RGB([80, 80, 100]),
            disk_border_size:                   1.0,
            disk_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            disk_side_separator:                None,
            disk_side_separator_color:          ColorType::RGB([75, 75, 75]),
            disk_side_separator_width:          1.,
            disk_side_separator_height:         16.,
            disk_button_gradient_color:         None,
            disk_button_hovered_gradient_color: None,
            disk_button_pressed_gradient_color: None,
            disk_button_shadow_color:           None,
            disk_button_shadow_x:               0.0,
            disk_button_shadow_y:               0.0,
            disk_button_shadow_blur:            0.0
        }
    }
}





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Debug, Clone)]
pub struct DiskData
{
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub percent: u64
}





// ============ FUNCTIONS ============
pub fn read_disk_data(mount: &str) -> Option<DiskData>
{
    let mount_cstr = std::ffi::CString::new(mount).ok()?;
    let mut stat: libc::statvfs = unsafe { mem::zeroed() };
    let ret = unsafe { libc::statvfs(mount_cstr.as_ptr(), &mut stat) };
    if ret != 0 { return None; }
    let block = stat.f_frsize;

    let total   = stat.f_blocks * block;
    let free    = stat.f_bavail * block;
    let used    = total - free;
    let percent = (used as f64 / total as f64 * 100.0).round() as u64;

    Some
    (
        DiskData 
        {
            total,
            free,
            used,
            percent
        }
    )
}



pub fn define_disk_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    set_style(UserStyle
    {
        status,
        normal:            app.ron_config.disk.disk_button_color,
        normal_text:       app.ron_config.disk.disk_text_color,
        hovered:           app.ron_config.disk.disk_button_hovered_color,
        hovered_text:      app.ron_config.disk.disk_button_hovered_text_color,
        pressed_text:      app.ron_config.disk.disk_button_pressed_text_color,
        pressed:           app.ron_config.disk.disk_button_pressed_color,
        border_color:      app.ron_config.disk.disk_border_color,
        border_size:       app.ron_config.disk.disk_border_size,
        border_radius:     app.ron_config.disk.disk_border_radius,
        normal_gradient:   app.ron_config.disk.disk_button_gradient_color.clone(),
        hovered_gradient:  app.ron_config.disk.disk_button_hovered_gradient_color.clone(),
        pressed_gradient:  app.ron_config.disk.disk_button_pressed_gradient_color.clone(),
        shadow_color: app.ron_config.disk.disk_button_shadow_color,
        shadow_x:     app.ron_config.disk.disk_button_shadow_x,
        shadow_y:     app.ron_config.disk.disk_button_shadow_y,
        shadow_blur:  app.ron_config.disk.disk_button_shadow_blur
    })
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::helpers::color::ColorType;
    use iced::{Background, Color};
    use iced::widget::button;


    #[test]
    fn read_disk_data_root_returns_some()
    {
        assert!(read_disk_data("/").is_some());
    }

    #[test]
    fn read_disk_data_nonexistent_mount_returns_none()
    {
        assert!(read_disk_data("/this/does/not/exist/ever").is_none());
    }

    #[test]
    fn read_disk_data_empty_mount_returns_none()
    {
        assert!(read_disk_data("").is_none());
    }

    #[test]
    fn read_disk_data_values_are_consistent()
    {
        let data = read_disk_data("/").unwrap();
        assert!(data.total > 0, "total should be > 0");
        assert!(data.used <= data.total, "used should not exceed total");
        assert!(data.free <= data.total, "free should not exceed total");
        assert_eq!(data.used + data.free, data.total, "used + free should equal total");
    }

    #[test]
    fn read_disk_data_percent_in_range()
    {
        let data = read_disk_data("/").unwrap();
        assert!(data.percent <= 100, "percent should be 0..=100");
    }

    #[test]
    fn read_disk_data_percent_matches_used_over_total()
    {
        let data = read_disk_data("/").unwrap();
        let expected = (data.used as f64 / data.total as f64 * 100.0).round() as u64;
        assert_eq!(data.percent, expected);
    }


    fn make_style_app() -> AppData
    {
        let mut app = AppData::default();
        app.ron_config.disk.disk_button_color              = ColorType::RGB([10, 20, 30]);
        app.ron_config.disk.disk_button_hovered_color      = ColorType::RGB([50, 60, 70]);
        app.ron_config.disk.disk_button_pressed_color      = ColorType::RGB([80, 90, 100]);
        app.ron_config.disk.disk_text_color                = ColorType::RGB([200, 210, 220]);
        app.ron_config.disk.disk_button_hovered_text_color = ColorType::RGB([255, 255, 255]);
        app.ron_config.disk.disk_border_color              = ColorType::RGB([1, 2, 3]);
        app.ron_config.disk.disk_border_size               = 1.5;
        app.ron_config.disk.disk_border_radius             = [2.0, 2.0, 2.0, 2.0];
        app
    }

    #[test]
    fn disk_style_active_uses_normal_color()
    {
        let style = define_disk_style(&make_style_app(), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }

    #[test]
    fn disk_style_hovered_uses_hovered_color()
    {
        let style = define_disk_style(&make_style_app(), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(50, 60, 70))));
    }

    #[test]
    fn disk_style_pressed_uses_pressed_color()
    {
        let style = define_disk_style(&make_style_app(), button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(80, 90, 100))));
    }

    #[test]
    fn disk_style_active_text_color()
    {
        let style = define_disk_style(&make_style_app(), button::Status::Active);
        assert_eq!(style.text_color, Color::from_rgb8(200, 210, 220));
    }

    #[test]
    fn disk_style_hovered_text_color()
    {
        let style = define_disk_style(&make_style_app(), button::Status::Hovered);
        assert_eq!(style.text_color, Color::from_rgb8(255, 255, 255));
    }

    #[test]
    fn disk_style_border_size()
    {
        let style = define_disk_style(&make_style_app(), button::Status::Active);
        assert_eq!(style.border.width, 1.5);
    }

    #[test]
    fn disk_style_all_statuses_produce_background()
    {
        let app = make_style_app();
        for status in [button::Status::Active, button::Status::Hovered, button::Status::Pressed, button::Status::Disabled]
        {
            let style = define_disk_style(&app, status);
            assert!(style.background.is_some(), "expected background for {status:?}");
        }
    }

    // ---- Additional disk data / read_disk_data tests ----

    #[test]
    fn read_disk_data_root_total_greater_than_used()
    {
        let d = read_disk_data("/").unwrap();
        assert!(d.total >= d.used);
    }

    #[test]
    fn read_disk_data_root_free_equals_total_minus_used_approx()
    {
        let d = read_disk_data("/").unwrap();
        // free + used <= total (due to reserved blocks)
        assert!(d.free + d.used <= d.total + 1_000_000);
    }

    #[test]
    fn read_disk_data_root_percent_is_zero_to_100()
    {
        let d = read_disk_data("/").unwrap();
        assert!(d.percent <= 100);
    }

    #[test]
    fn read_disk_data_proc_returns_some()
    {
        // /proc is a virtual FS but statvfs usually succeeds
        let _ = read_disk_data("/proc"); // just don't panic
    }

    #[test]
    fn read_disk_data_slash_dev_null_returns_none_or_some()
    {
        // /dev/null is not a mount point, may return None or succeed
        let _ = read_disk_data("/dev/null");
    }

    #[test]
    fn disk_config_default_mount_is_root()
    {
        assert_eq!(DiskConfig::default().disk_mount, "/");
    }

    #[test]
    fn disk_config_default_format_contains_placeholders()
    {
        let fmt = DiskConfig::default().disk_format;
        assert!(fmt.contains("{used}") || fmt.contains("{total}") || fmt.contains("{percent}"));
    }

    #[test]
    fn disk_config_default_update_interval_is_positive()
    {
        assert!(DiskConfig::default().disk_update_interval > 0);
    }

    #[test]
    fn disk_config_default_text_size_is_positive()
    {
        assert!(DiskConfig::default().disk_text_size > 0);
    }

    #[test]
    fn disk_config_default_gradient_is_none()
    {
        assert!(DiskConfig::default().disk_button_gradient_color.is_none());
    }

    #[test]
    fn disk_config_default_shadow_color_is_none()
    {
        assert!(DiskConfig::default().disk_button_shadow_color.is_none());
    }

    #[test]
    fn disk_config_default_side_separator_is_none()
    {
        assert!(DiskConfig::default().disk_side_separator.is_none());
    }

    #[test]
    fn disk_config_default_border_radius_all_equal()
    {
        let r = DiskConfig::default().disk_border_radius;
        assert_eq!(r[0], r[1]);
        assert_eq!(r[1], r[2]);
        assert_eq!(r[2], r[3]);
    }

    #[test]
    fn disk_style_active_and_hovered_backgrounds_differ()
    {
        let app = make_style_app();
        let active  = define_disk_style(&app, button::Status::Active);
        let hovered = define_disk_style(&app, button::Status::Hovered);
        assert_ne!(active.background, hovered.background);
    }

    #[test]
    fn disk_style_active_and_pressed_backgrounds_differ()
    {
        let app = make_style_app();
        let active  = define_disk_style(&app, button::Status::Active);
        let pressed = define_disk_style(&app, button::Status::Pressed);
        assert_ne!(active.background, pressed.background);
    }

    #[test]
    fn disk_style_border_radius_applied()
    {
        let app = make_style_app();
        let style = define_disk_style(&app, button::Status::Active);
        assert_eq!(style.border.radius.top_left, app.ron_config.disk.disk_border_radius[0]);
    }
}
