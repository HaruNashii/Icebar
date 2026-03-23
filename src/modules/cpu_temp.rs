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
pub struct CpuTempConfig
{
    pub cpu_temp_format:                        String,
    pub cpu_temp_update_interval:               u64,
    pub action_on_left_click_cpu_temp:          ActionOnClick,
    pub action_on_right_click_cpu_temp:         ActionOnClick,
    pub cpu_temp_padding:                       u16,
    pub cpu_temp_text_size:                     u32,
    pub cpu_temp_text_color:                    ColorType,
    pub cpu_temp_text_orientation:              TextOrientation,
    pub cpu_temp_button_color:                  ColorType,
    pub cpu_temp_button_hovered_color:          ColorType,
    pub cpu_temp_button_hovered_text_color:     ColorType,
    pub cpu_temp_button_pressed_text_color:     ColorType,
    pub cpu_temp_button_pressed_color:          ColorType,
    pub cpu_temp_border_color:                  ColorType,
    pub cpu_temp_border_size:                   f32,
    pub cpu_temp_border_radius:                 [f32; 4],
    pub cpu_temp_side_separator:                Option<SideOption>,
    pub cpu_temp_side_separator_color:          ColorType,
    pub cpu_temp_side_separator_width:          f32,
    pub cpu_temp_side_separator_height:         f32,
    pub cpu_temp_button_gradient_color:         Option<Gradient>,
    pub cpu_temp_button_hovered_gradient_color: Option<Gradient>,
    pub cpu_temp_button_pressed_gradient_color: Option<Gradient>,
    pub cpu_temp_button_shadow_color:           Option<ColorType>,
    pub cpu_temp_button_shadow_x:               f32,
    pub cpu_temp_button_shadow_y:               f32,
    pub cpu_temp_button_shadow_blur:            f32,
}

impl Default for CpuTempConfig
{
    fn default() -> Self
    {
        Self
        {
            cpu_temp_format:                        " {temp}°C".into(),
            cpu_temp_update_interval:               1050,
            action_on_left_click_cpu_temp:          ActionOnClick::DefaultAction,
            action_on_right_click_cpu_temp:         ActionOnClick::DefaultAction,
            cpu_temp_padding:                       0,
            cpu_temp_text_size:                     12,
            cpu_temp_text_color:                    ColorType::RGB([220, 220, 220]),
            cpu_temp_text_orientation:              TextOrientation::Horizontal,
            cpu_temp_button_color:                  ColorType::RGB([40, 40, 50]),
            cpu_temp_button_hovered_color:          ColorType::RGB([60, 60, 75]),
            cpu_temp_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            cpu_temp_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            cpu_temp_button_pressed_color:          ColorType::RGB([30, 30, 40]),
            cpu_temp_border_color:                  ColorType::RGB([80, 80, 100]),
            cpu_temp_border_size:                   1.0,
            cpu_temp_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            cpu_temp_side_separator:                None,
            cpu_temp_side_separator_color:          ColorType::RGB([75, 75, 75]),
            cpu_temp_side_separator_width:          1.,
            cpu_temp_side_separator_height:         16.,
            cpu_temp_button_gradient_color:         None,
            cpu_temp_button_hovered_gradient_color: None,
            cpu_temp_button_pressed_gradient_color: None,
            cpu_temp_button_shadow_color:           None,
            cpu_temp_button_shadow_x:               0.0,
            cpu_temp_button_shadow_y:               0.0,
            cpu_temp_button_shadow_blur:            0.0,
        }
    }
}



// ============ STRUCTS ============
#[derive(Default, Clone)]
pub struct CpuTempData
{
    pub temp_celsius: f32,
}



// ============ FUNCTIONS ============
pub fn read_cpu_temp() -> Option<f32>
{
    let base = "/sys/class/hwmon";
    let Ok(entries) = std::fs::read_dir(base) else { return None };

    let mut hwmons: Vec<_> = entries.flatten().collect();
    hwmons.sort_by_key(|e| e.file_name());

    for entry in hwmons
    {
        let path = entry.path();
        let name = std::fs::read_to_string(path.join("name")).unwrap_or_default();
        let name = name.trim();

        // These are the common CPU temp sources — add yours if different
        if matches!(name, "coretemp" | "k10temp" | "zenpower" | "acpitz" | "cpu_thermal")
        {
            // Try temp1_input, temp2_input, ... until one works
            for i in 1..=10
            {
                let temp_path = path.join(format!("temp{}_input", i));
                if let Some(t) = read_temp_file(temp_path.to_str()?)
                {
                    return Some(t);
                }
            }
        }
    }

    // Fallback: return the first temp*_input we can find anywhere
    let Ok(entries) = std::fs::read_dir(base) else { return None };
    for entry in entries.flatten()
    {
        for i in 1..=10
        {
            let temp_path = entry.path().join(format!("temp{}_input", i));
            if let Some(t) = read_temp_file(temp_path.to_str()?)
            {
                return Some(t);
            }
        }
    }

    None
}



fn read_temp_file(path: &str) -> Option<f32>
{
    // /sys/.../temp is in millidegrees Celsius
    let raw: i64 = std::fs::read_to_string(path).ok()?.trim().parse().ok()?;
    Some(raw as f32 / 1000.0)
}



pub fn define_cpu_temp_text(app: &AppData) -> String
{
    let temp = app.modules_data.cpu_temp_data.temp_celsius;
    let text = app.ron_config.cpu_temp.cpu_temp_format.replace("{temp}", &format!("{:.0}", temp));
    orient_text(&text, &app.ron_config.cpu_temp.cpu_temp_text_orientation)
}



pub fn define_cpu_temp_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    set_style(UserStyle
    {
        status,
        normal:            app.ron_config.cpu_temp.cpu_temp_button_color,
        normal_text:       app.ron_config.cpu_temp.cpu_temp_text_color,
        hovered:           app.ron_config.cpu_temp.cpu_temp_button_hovered_color,
        hovered_text:      app.ron_config.cpu_temp.cpu_temp_button_hovered_text_color,
        pressed_text:      app.ron_config.cpu_temp.cpu_temp_button_pressed_text_color,
        pressed:           app.ron_config.cpu_temp.cpu_temp_button_pressed_color,
        border_color:  app.ron_config.cpu_temp.cpu_temp_border_color,
        border_size:       app.ron_config.cpu_temp.cpu_temp_border_size,
        border_radius:     app.ron_config.cpu_temp.cpu_temp_border_radius,
        normal_gradient:   app.ron_config.cpu_temp.cpu_temp_button_gradient_color.clone(),
        hovered_gradient:  app.ron_config.cpu_temp.cpu_temp_button_hovered_gradient_color.clone(),
        pressed_gradient:  app.ron_config.cpu_temp.cpu_temp_button_pressed_gradient_color.clone(),
        shadow_color: app.ron_config.cpu_temp.cpu_temp_button_shadow_color,
        shadow_x: app.ron_config.cpu_temp.cpu_temp_button_shadow_x,
        shadow_y: app.ron_config.cpu_temp.cpu_temp_button_shadow_y,
        shadow_blur: app.ron_config.cpu_temp.cpu_temp_button_shadow_blur,
    })
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;

    // ---- read_temp_file -----------------------------------------------------

    #[test]
    fn read_temp_file_parses_millidegrees()
    {
        // Write a fake temp file
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "45000\n").unwrap();
        let result = read_temp_file(path.to_str().unwrap());
        assert!((result.unwrap() - 45.0).abs() < 0.01);
    }

    #[test]
    fn read_temp_file_negative_temp_parsed_correctly()
    {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "-5000\n").unwrap();
        let result = read_temp_file(path.to_str().unwrap());
        assert!((result.unwrap() - (-5.0)).abs() < 0.01);
    }

    #[test]
    fn read_temp_file_missing_file_returns_none()
    {
        assert!(read_temp_file("/tmp/this_does_not_exist_12345").is_none());
    }

    #[test]
    fn read_temp_file_non_numeric_returns_none()
    {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "hot\n").unwrap();
        assert!(read_temp_file(path.to_str().unwrap()).is_none());
    }

    // ---- read_cpu_temp ------------------------------------------------------

    #[test]
    fn read_cpu_temp_returns_plausible_value()
    {
        if let Some(temp) = read_cpu_temp()
        {
            // Sane range for any running system: -10°C to 110°C
            assert!(temp > -10.0 && temp < 110.0, "implausible temp: {temp}");
        }
        // If None, machine has no thermal zones — that's fine, don't fail
    }

    
    // ---- read_temp_file: edge cases -----------------------------------------
 
    #[test]
    fn read_temp_file_zero_millidegrees_returns_zero()
    {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "0\n").unwrap();
        assert_eq!(read_temp_file(path.to_str().unwrap()).unwrap(), 0.0);
    }
 
    #[test]
    fn read_temp_file_large_value_100_degrees()
    {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "100000\n").unwrap();
        assert!((read_temp_file(path.to_str().unwrap()).unwrap() - 100.0).abs() < 0.01);
    }
 
    #[test]
    fn read_temp_file_whitespace_only_returns_none()
    {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "   \n").unwrap();
        assert!(read_temp_file(path.to_str().unwrap()).is_none());
    }
 
    #[test]
    fn read_temp_file_empty_file_returns_none()
    {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "").unwrap();
        assert!(read_temp_file(path.to_str().unwrap()).is_none());
    }
 
    #[test]
    fn read_temp_file_float_string_returns_none()
    {
        // The kernel always writes integers — a float must not parse
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        std::fs::write(&path, "45.5\n").unwrap();
        assert!(read_temp_file(path.to_str().unwrap()).is_none());
    }
 
    #[test]
    fn read_temp_file_divides_by_1000_correctly()
    {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("temp");
        // 1 millidegree → 0.001 °C
        std::fs::write(&path, "1\n").unwrap();
        assert!((read_temp_file(path.to_str().unwrap()).unwrap() - 0.001).abs() < 0.0001);
    }
 
    // ---- read_cpu_temp via fake hwmon tree ----------------------------------
 
    fn write_hwmon(dir: &tempfile::TempDir, name: &str, temps: &[i64])
    {
        let hwmon = dir.path().join("hwmon0");
        std::fs::create_dir_all(&hwmon).unwrap();
        std::fs::write(hwmon.join("name"), format!("{}\n", name)).unwrap();
        for (i, t) in temps.iter().enumerate()
        {
            std::fs::write(hwmon.join(format!("temp{}_input", i + 1)), format!("{}\n", t)).unwrap();
        }
    }
 
    // Helper that runs read_cpu_temp against a custom base path
    fn read_cpu_temp_from(base: &str) -> Option<f32>
    {
        let Ok(entries) = std::fs::read_dir(base) else { return None };
        let mut hwmons: Vec<_> = entries.flatten().collect();
        hwmons.sort_by_key(|e| e.file_name());
 
        for entry in &hwmons
        {
            let path = entry.path();
            let name = std::fs::read_to_string(path.join("name")).unwrap_or_default();
            let name = name.trim().to_string();
            if matches!(name.as_str(), "coretemp" | "k10temp" | "zenpower" | "acpitz" | "cpu_thermal")
            {
                for i in 1..=10
                {
                    let temp_path = path.join(format!("temp{}_input", i));
                    if let Some(t) = read_temp_file(temp_path.to_str()?) { return Some(t); }
                }
            }
        }
 
        // Fallback
        let Ok(entries) = std::fs::read_dir(base) else { return None };
        for entry in entries.flatten()
        {
            for i in 1..=10
            {
                let temp_path = entry.path().join(format!("temp{}_input", i));
                if let Some(t) = read_temp_file(temp_path.to_str()?) { return Some(t); }
            }
        }
        None
    }
 
    #[test]
    fn fake_hwmon_coretemp_returns_correct_value()
    {
        let dir = tempfile::tempdir().unwrap();
        write_hwmon(&dir, "coretemp", &[55000]);
        let result = read_cpu_temp_from(dir.path().to_str().unwrap());
        assert!((result.unwrap() - 55.0).abs() < 0.01);
    }
 
    #[test]
    fn fake_hwmon_k10temp_returns_correct_value()
    {
        let dir = tempfile::tempdir().unwrap();
        write_hwmon(&dir, "k10temp", &[72000]);
        let result = read_cpu_temp_from(dir.path().to_str().unwrap());
        assert!((result.unwrap() - 72.0).abs() < 0.01);
    }
 
    #[test]
    fn fake_hwmon_unknown_name_uses_fallback()
    {
        // Name is not in the known list — should still work via fallback loop
        let dir = tempfile::tempdir().unwrap();
        write_hwmon(&dir, "some_unknown_sensor", &[40000]);
        let result = read_cpu_temp_from(dir.path().to_str().unwrap());
        assert!((result.unwrap() - 40.0).abs() < 0.01);
    }
 
    #[test]
    fn fake_hwmon_multiple_temp_inputs_returns_first()
    {
        let dir = tempfile::tempdir().unwrap();
        write_hwmon(&dir, "coretemp", &[30000, 60000, 90000]);
        let result = read_cpu_temp_from(dir.path().to_str().unwrap());
        assert!((result.unwrap() - 30.0).abs() < 0.01);
    }
 
    #[test]
    fn fake_hwmon_empty_dir_returns_none()
    {
        let dir = tempfile::tempdir().unwrap();
        // No hwmon subdirectories at all
        assert!(read_cpu_temp_from(dir.path().to_str().unwrap()).is_none());
    }
 
    #[test]
    fn fake_hwmon_name_file_missing_uses_fallback()
    {
        // No name file → name defaults to "" → not in known list → fallback picks it up
        let dir    = tempfile::tempdir().unwrap();
        let hwmon  = dir.path().join("hwmon0");
        std::fs::create_dir_all(&hwmon).unwrap();
        std::fs::write(hwmon.join("temp1_input"), "38000\n").unwrap();
        let result = read_cpu_temp_from(dir.path().to_str().unwrap());
        assert!((result.unwrap() - 38.0).abs() < 0.01);
    }
 
    // ---- CpuTempData default ------------------------------------------------
 
    #[test]
    fn cpu_temp_data_default_is_zero()
    {
        assert_eq!(CpuTempData::default().temp_celsius, 0.0);
    }
}
