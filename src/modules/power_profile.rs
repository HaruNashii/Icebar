// ============ IMPORTS ============
use iced::widget::button;
use serde::{Deserialize, Serialize};
use std::process::Command;





// ============ CRATES ============
use crate::helpers::color::{ColorType, Gradient};
use crate::helpers::style::{SideOption, TextOrientation, UserStyle, orient_text, set_style};
use crate::ron::ActionOnClick;
use crate::AppData;





// ============ ENUM/STRUCT ============
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PowerProfileData
{
    pub current_profile: PowerProfile
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PowerProfile
{
    PowerSaver,
    #[default]
    Balanced,
    Performance
}

impl PowerProfile
{
    pub fn as_str(&self) -> &'static str
    {
        match self
        {
            PowerProfile::PowerSaver   => "power-saver",
            PowerProfile::Balanced     => "balanced",
            PowerProfile::Performance  => "performance"
        }
    }

    pub fn from_str(s: &str) -> Self
    {
        match s.trim()
        {
            "power-saver" => PowerProfile::PowerSaver,
            "performance" => PowerProfile::Performance,
            _             => PowerProfile::Balanced
        }
    }

    pub fn next(&self) -> Self
    {
        match self
        {
            PowerProfile::PowerSaver  => PowerProfile::Balanced,
            PowerProfile::Balanced    => PowerProfile::Performance,
            PowerProfile::Performance => PowerProfile::PowerSaver
        }
    }
}





// ============ CONFIG ============
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PowerProfileConfig
{
    pub power_profile_update_interval:               u64,
    pub power_profile_format_power_saver:            String,
    pub power_profile_format_balanced:               String,
    pub power_profile_format_performance:            String,
    pub action_on_left_click_power_profile:          ActionOnClick,
    pub action_on_right_click_power_profile:         ActionOnClick,
    pub power_profile_padding:                       u16,
    pub power_profile_text_size:                     u32,
    pub power_profile_text_color:                    ColorType,
    pub power_profile_text_orientation:              TextOrientation,
    pub power_profile_button_color:                  ColorType,
    pub power_profile_button_hovered_color:          ColorType,
    pub power_profile_button_hovered_text_color:     ColorType,
    pub power_profile_button_pressed_text_color:     ColorType,
    pub power_profile_button_pressed_color:          ColorType,
    pub power_profile_border_color:                  ColorType,
    pub power_profile_border_size:                   f32,
    pub power_profile_border_radius:                 [f32; 4],
    pub power_profile_side_separator:                Option<SideOption>,
    pub power_profile_side_separator_color:          ColorType,
    pub power_profile_side_separator_width:          f32,
    pub power_profile_side_separator_height:         f32,
    pub power_profile_button_gradient_color:         Option<Gradient>,
    pub power_profile_button_hovered_gradient_color: Option<Gradient>,
    pub power_profile_button_pressed_gradient_color: Option<Gradient>,
    pub power_profile_button_shadow_color:           Option<ColorType>,
    pub power_profile_button_shadow_x:               f32,
    pub power_profile_button_shadow_y:               f32,
    pub power_profile_button_shadow_blur:            f32
}

impl Default for PowerProfileConfig
{
    fn default() -> Self
    {
        Self
        {
            power_profile_update_interval:               5000,
            power_profile_format_power_saver:            " 󰌪 Saver".into(),
            power_profile_format_balanced:               " 󰈐 Balanced".into(),
            power_profile_format_performance:            " 󱐋 Performance".into(),
            action_on_left_click_power_profile:          ActionOnClick::DefaultAction,
            action_on_right_click_power_profile:         ActionOnClick::DefaultAction,
            power_profile_padding:                       0,
            power_profile_text_size:                     12,
            power_profile_text_color:                    ColorType::RGB([220, 220, 220]),
            power_profile_text_orientation:              TextOrientation::Horizontal,
            power_profile_button_color:                  ColorType::RGB([40, 40, 50]),
            power_profile_button_hovered_color:          ColorType::RGB([60, 60, 75]),
            power_profile_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            power_profile_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            power_profile_button_pressed_color:          ColorType::RGB([30, 30, 40]),
            power_profile_border_color:                  ColorType::RGB([80, 80, 100]),
            power_profile_border_size:                   1.0,
            power_profile_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            power_profile_side_separator:                None,
            power_profile_side_separator_color:          ColorType::RGB([75, 75, 75]),
            power_profile_side_separator_width:          1.,
            power_profile_side_separator_height:         16.,
            power_profile_button_gradient_color:         None,
            power_profile_button_hovered_gradient_color: None,
            power_profile_button_pressed_gradient_color: None,
            power_profile_button_shadow_color:           None,
            power_profile_button_shadow_x:               0.0,
            power_profile_button_shadow_y:               0.0,
            power_profile_button_shadow_blur:            0.0
        }
    }
}





// ============ FUNCTIONS ============
pub fn read_power_profile() -> Option<PowerProfile>
{
    let output = Command::new("powerprofilesctl")
        .arg("get")
        .output()
        .ok()?;

    if !output.status.success() { return None; }
    let s = String::from_utf8_lossy(&output.stdout);
    Some(PowerProfile::from_str(&s))
}

pub fn cycle_power_profile(current: &PowerProfile) -> Option<PowerProfile>
{
    let next = current.next();
    let status = Command::new("powerprofilesctl")
        .args(["set", next.as_str()])
        .status()
        .ok()?;

    if status.success() { Some(next) } else { None }
}



pub fn define_power_profile_text(app: &AppData) -> String
{
    match app.modules_data.power_profile_data.current_profile
    {
        PowerProfile::PowerSaver  => app.ron_config.power_profile.power_profile_format_power_saver.clone(),
        PowerProfile::Balanced    => app.ron_config.power_profile.power_profile_format_balanced.clone(),
        PowerProfile::Performance => app.ron_config.power_profile.power_profile_format_performance.clone()
    }
}



pub fn define_power_profile_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let cfg = &app.ron_config.power_profile;
    set_style(UserStyle
    {
        status,
        hovered:          cfg.power_profile_button_hovered_color,
        hovered_text:     cfg.power_profile_button_hovered_text_color,
        pressed_text:     cfg.power_profile_button_pressed_text_color,
        pressed:          cfg.power_profile_button_pressed_color,
        normal:           cfg.power_profile_button_color,
        normal_text:      cfg.power_profile_text_color,
        border_color:     cfg.power_profile_border_color,
        border_size:      cfg.power_profile_border_size,
        border_radius:    cfg.power_profile_border_radius,
        hovered_gradient: cfg.power_profile_button_hovered_gradient_color.clone(),
        normal_gradient:  cfg.power_profile_button_gradient_color.clone(),
        pressed_gradient: cfg.power_profile_button_pressed_gradient_color.clone(),
        shadow_color:     cfg.power_profile_button_shadow_color,
        shadow_x:         cfg.power_profile_button_shadow_x,
        shadow_y:         cfg.power_profile_button_shadow_y,
        shadow_blur:      cfg.power_profile_button_shadow_blur
    })
}



pub fn define_power_profile_rich_text(app: &AppData) -> String
{
    orient_text(&define_power_profile_text(app), &app.ron_config.power_profile.power_profile_text_orientation)
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;


    #[test]
    fn from_str_power_saver()
    {
        assert_eq!(PowerProfile::from_str("power-saver"), PowerProfile::PowerSaver);
    }

    #[test]
    fn from_str_balanced()
    {
        assert_eq!(PowerProfile::from_str("balanced"), PowerProfile::Balanced);
    }

    #[test]
    fn from_str_performance()
    {
        assert_eq!(PowerProfile::from_str("performance"), PowerProfile::Performance);
    }

    #[test]
    fn from_str_unknown_defaults_to_balanced()
    {
        assert_eq!(PowerProfile::from_str("unknown"), PowerProfile::Balanced);
        assert_eq!(PowerProfile::from_str(""),         PowerProfile::Balanced);
    }

    #[test]
    fn from_str_trims_whitespace()
    {
        assert_eq!(PowerProfile::from_str("  performance\n"), PowerProfile::Performance);
        assert_eq!(PowerProfile::from_str("  power-saver "), PowerProfile::PowerSaver);
    }


    #[test]
    fn as_str_round_trips()
    {
        for p in [PowerProfile::PowerSaver, PowerProfile::Balanced, PowerProfile::Performance]
        {
            assert_eq!(PowerProfile::from_str(p.as_str()), p);
        }
    }


    #[test]
    fn next_cycles_power_saver_to_balanced()
    {
        assert_eq!(PowerProfile::PowerSaver.next(), PowerProfile::Balanced);
    }

    #[test]
    fn next_cycles_balanced_to_performance()
    {
        assert_eq!(PowerProfile::Balanced.next(), PowerProfile::Performance);
    }

    #[test]
    fn next_cycles_performance_back_to_power_saver()
    {
        assert_eq!(PowerProfile::Performance.next(), PowerProfile::PowerSaver);
    }

    #[test]
    fn next_full_cycle_returns_to_start()
    {
        let start = PowerProfile::Balanced;
        let cycled = start.next().next().next();
        assert_eq!(cycled, start);
    }


    #[test]
    fn config_default_update_interval_is_positive()
    {
        assert!(PowerProfileConfig::default().power_profile_update_interval > 0);
    }

    #[test]
    fn config_default_formats_are_non_empty()
    {
        let cfg = PowerProfileConfig::default();
        assert!(!cfg.power_profile_format_power_saver.is_empty());
        assert!(!cfg.power_profile_format_balanced.is_empty());
        assert!(!cfg.power_profile_format_performance.is_empty());
    }


    #[test]
    fn data_default_profile_is_balanced()
    {
        assert_eq!(PowerProfileData::default().current_profile, PowerProfile::Balanced);
    }

    // ---- Additional PowerProfile tests ----

    #[test]
    fn power_profile_all_variants_have_nonempty_as_str()
    {
        assert!(!PowerProfile::PowerSaver.as_str().is_empty());
        assert!(!PowerProfile::Balanced.as_str().is_empty());
        assert!(!PowerProfile::Performance.as_str().is_empty());
    }

    #[test]
    fn power_profile_as_str_is_lowercase()
    {
        let s = PowerProfile::PowerSaver.as_str();
        assert_eq!(s, s.to_lowercase());
        let s2 = PowerProfile::Performance.as_str();
        assert_eq!(s2, s2.to_lowercase());
    }

    #[test]
    fn power_profile_from_str_empty_defaults_to_balanced()
    {
        assert_eq!(PowerProfile::from_str(""), PowerProfile::Balanced);
    }

    #[test]
    fn power_profile_from_str_garbage_defaults_to_balanced()
    {
        assert_eq!(PowerProfile::from_str("xyz_not_a_profile"), PowerProfile::Balanced);
    }

    #[test]
    fn power_profile_from_str_case_matters()
    {
        // "POWER-SAVER" is not matched (from_str is exact match after trim)
        // if unknown → Balanced
        let result = PowerProfile::from_str("POWER-SAVER");
        assert_eq!(result, PowerProfile::Balanced);
    }

    #[test]
    fn power_profile_next_forms_complete_cycle()
    {
        let start = PowerProfile::PowerSaver;
        let a = start.next();
        let b = a.next();
        let c = b.next();
        assert_eq!(c, PowerProfile::PowerSaver);
    }

    #[test]
    fn power_profile_variants_are_distinct()
    {
        assert_ne!(PowerProfile::PowerSaver, PowerProfile::Balanced);
        assert_ne!(PowerProfile::Balanced, PowerProfile::Performance);
        assert_ne!(PowerProfile::PowerSaver, PowerProfile::Performance);
    }

    #[test]
    fn power_profile_round_trip_as_str_then_from_str()
    {
        for profile in [PowerProfile::PowerSaver, PowerProfile::Balanced, PowerProfile::Performance]
        {
            let s = profile.as_str();
            assert_eq!(PowerProfile::from_str(s), profile);
        }
    }

    #[test]
    fn from_str_trims_and_parses_power_saver_with_surrounding_whitespace()
    {
        // from_str already trims (based on the existing trim test), verify again
        assert_eq!(PowerProfile::from_str("  power-saver  "), PowerProfile::PowerSaver);
    }

    #[test]
    fn power_profile_config_default_text_size_is_positive()
    {
        assert!(PowerProfileConfig::default().power_profile_text_size > 0);
    }

    #[test]
    fn power_profile_config_default_update_interval_is_positive()
    {
        assert!(PowerProfileConfig::default().power_profile_update_interval > 0);
    }

    #[test]
    fn power_profile_config_default_gradient_is_none()
    {
        let cfg = PowerProfileConfig::default();
        assert!(cfg.power_profile_button_gradient_color.is_none());
    }

    #[test]
    fn power_profile_config_default_shadow_color_is_none()
    {
        assert!(PowerProfileConfig::default().power_profile_button_shadow_color.is_none());
    }

    #[test]
    fn power_profile_config_default_border_radius_is_uniform()
    {
        let r = PowerProfileConfig::default().power_profile_border_radius;
        assert_eq!(r[0], r[1]);
        assert_eq!(r[1], r[2]);
    }
}





// ============ POWER PROFILE DBUS SUBSCRIPTION ============
use std::pin::Pin;

pub fn power_profile_subscription() -> Pin<Box<dyn futures::Stream<Item = crate::update::Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        yield crate::update::Message::UpdatePowerProfile;

        loop
        {
            let conn = match zbus::Connection::system().await
            {
                Ok(c) => c,
                Err(e) =>
                {
                    eprintln!("[icebar] power_profile_subscription: DBus connect error: {e}");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            let proxy = match zbus::Proxy::new(
                &conn,
                "net.hadess.PowerProfiles",
                "/net/hadess/PowerProfiles",
                "org.freedesktop.DBus.Properties"
            ).await
            {
                Ok(p) => p,
                Err(e) =>
                {
                    eprintln!("[icebar] power_profile_subscription: proxy error: {e}");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            let mut signals = match proxy.receive_signal("PropertiesChanged").await
            {
                Ok(s) => s,
                Err(e) =>
                {
                    eprintln!("[icebar] power_profile_subscription: signal error: {e}");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            use futures::StreamExt;
            while signals.next().await.is_some()
            {
                yield crate::update::Message::UpdatePowerProfile;
            }

            eprintln!("[icebar] power_profile_subscription: signal stream ended — reconnecting in 5s");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    })
}
