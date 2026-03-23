// ============ IMPORTS ============
use serde::{Deserialize, Serialize};
use std::{fs, collections::HashSet};





// ============ CRATES ============
use crate::modules::
{
    clock::ClockConfig,
    cpu::CpuConfig,
    cpu_temp::CpuTempConfig,
    ram::RamConfig,
    disk::DiskConfig,
    focused_window::FocusedWindowConfig,
    tray::TrayConfig,
    network::{NetworkConfig, AltNetworkConfig},
    volume::{VolumeOutputConfig, MutedVolumeOutputConfig, VolumeInputConfig, MutedVolumeInputConfig},
    media_player::{MediaPlayerMetadataConfig, MediaPlayerButtonConfig},
    workspaces::WorkspaceConfig,
    image::ImageConfig,
    custom_modules::CustomModuleConfig,
    data::Modules,
};
use crate::context_menu::ContextMenuConfig;
use crate::helpers::{string::find_field_colon, color::{ColorType, Gradient}, ron_general::apply_general_settings, style::{SideOption, TextOrientation}};





// ============ TYPE ============
type RonReturn = (BarConfig, Option<(String, u32)>, HashSet<Modules>, (bool, String));





// ============ STRUCTS/ENUM'S ============
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum BarPosition
{
    Up,
    Down,
    Left, 
    Right
}


#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub enum ActionOnClick 
{
    #[default] Nothing,
    DefaultAction,
    CustomAction(Vec<String>),
    CycleClockTimezones,
    ToggleAltClockAndCycleClockTimezones
}





// ============ GENERAL CONFIG ============
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GeneralConfig
{
    pub display:                          Option<String>,
    pub bar_position:                     BarPosition,
    pub floating_space:                   i32,
    pub increased_exclusive_bar_zone:     i32,
    pub bar_check_reload_interval_ms:     Option<u64>,
    pub bar_side_spaces_size:             u32,
    pub bar_size:                         [u32; 2],
    pub bar_border_radius:                [f32; 4],
    pub bar_border_size:                  f32,
    pub bar_border_color:                 ColorType,
    pub bar_background_color:             ColorType,
    pub font_family:                      String,
    pub font_style:                       String,
    pub spacing_between_all_modules:      u32,
    pub ellipsis_text:                    String,
    pub force_static_position_context_menu: Option<(i32, i32)>,
    pub left_modules:                     Vec<Modules>,
    pub center_modules:                   Vec<Modules>,
    pub right_modules:                    Vec<Modules>,
}
impl Default for GeneralConfig
{
    fn default() -> Self
    {
        Self
        {
            bar_position:                       BarPosition::Up,
            bar_check_reload_interval_ms:       Some(500),
            bar_size:                           [0, 40],
            bar_border_radius:                  [0., 0., 0., 0.],
            bar_border_size:                    1.0,
            bar_border_color:                   ColorType::RGB([26, 26, 26]),
            bar_background_color:               ColorType::RGB([36, 36, 36]),
            font_family:                        "JetBrains".into(),
            font_style:                         "Bold".into(),
            center_modules:                     vec![Modules::Clock],
            ellipsis_text:                      "...".to_string(),

            left_modules: Vec::new(),
            right_modules: Vec::new(),
            bar_side_spaces_size: 8,
            force_static_position_context_menu: None,
            increased_exclusive_bar_zone: 0,
            spacing_between_all_modules: 0,
            floating_space: 0,
            display: None,
        }
    }
}



#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GeneralStyleConfig
{
    pub general_padding:                        Option<u16>,
    pub general_text_size:                      Option<u32>,
    pub general_text_color:                     Option<ColorType>,
    pub general_text_orientation:               Option<TextOrientation>,
    pub general_button_color:                   Option<ColorType>,
    pub general_button_hovered_color:           Option<ColorType>,
    pub general_button_hovered_text_color:      Option<ColorType>,
    pub general_button_pressed_text_color:      Option<ColorType>,
    pub general_button_pressed_color:           Option<ColorType>,
    pub general_button_gradient_color:          Option<Gradient>,
    pub general_button_pressed_gradient_color:  Option<Gradient>,
    pub general_button_hovered_gradient_color:  Option<Gradient>,

    pub general_button_shadow_color:            Option<ColorType>,
    pub general_button_shadow_x:                Option<f32>,
    pub general_button_shadow_y:                Option<f32>,
    pub general_button_shadow_blur:             Option<f32>,

    pub general_border_color:                   Option<ColorType>,
    pub general_border_size:                    Option<f32>,
    pub general_border_radius:                  Option<[f32; 4]>,
    pub general_side_separator:                 Option<SideOption>,
    pub general_side_separator_color:           Option<ColorType>,
    pub general_side_separator_width:           Option<f32>,
    pub general_side_separator_height:          Option<f32>,
    pub general_alt_side_separator:             Option<SideOption>,
    pub general_alt_side_separator_color:       Option<ColorType>,
    pub general_alt_side_separator_width:       Option<f32>,
    pub general_alt_side_separator_height:      Option<f32>,
    pub general_alt_padding:                    Option<u16>,
    pub general_alt_text_size:                  Option<u32>,
    pub general_alt_text_color:                 Option<ColorType>,
    pub general_alt_text_orientation:           Option<TextOrientation>,
    pub general_alt_button_color:               Option<ColorType>,
    pub general_alt_button_hovered_color:       Option<ColorType>,
    pub general_alt_button_pressed_text_color:  Option<ColorType>,
    pub general_alt_button_hovered_text_color:  Option<ColorType>,
    pub general_alt_button_pressed_color:       Option<ColorType>,
    pub general_alt_border_color:               Option<ColorType>,
    pub general_alt_border_size:                Option<f32>,
    pub general_alt_border_radius:              Option<[f32; 4]>,
    pub general_alt_button_gradient_color:      Option<Gradient>,
    pub general_alt_button_pressed_gradient_color: Option<Gradient>,
    pub general_alt_button_hovered_gradient_color: Option<Gradient>,
    pub general_alt_button_shadow_color:            Option<ColorType>,
    pub general_alt_button_shadow_x:                Option<f32>,
    pub general_alt_button_shadow_y:                Option<f32>,
    pub general_alt_button_shadow_blur:             Option<f32>,
}



#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BarConfig
{
    pub general:                GeneralConfig,
    pub general_style:          GeneralStyleConfig,
    pub clock:                  ClockConfig,
    pub volume_output:          VolumeOutputConfig,
    pub muted_volume_output:    MutedVolumeOutputConfig,
    pub volume_input:           VolumeInputConfig,
    pub muted_volume_input:     MutedVolumeInputConfig,
    pub network:                NetworkConfig,
    pub alt_network:            AltNetworkConfig,
    pub workspace:              WorkspaceConfig,
    pub tray:                   TrayConfig,
    pub media_player_metadata:  MediaPlayerMetadataConfig,
    pub media_player_button:    MediaPlayerButtonConfig,
    pub cpu:                    CpuConfig,
    pub cpu_temp:               CpuTempConfig,
    pub ram:                    RamConfig,
    pub disk:                   DiskConfig,
    pub focused_window:         FocusedWindowConfig,
    pub context_menu:           ContextMenuConfig,
    pub image:                  ImageConfig,
    pub custom_module:          CustomModuleConfig,
}





// ============ FUNCTIONS ============
pub fn read_ron_config() -> RonReturn
{

    println!("\n=== READING CONFIG FILE ===");
    let home_path = match home::home_dir()
    {
        Some(home_dir) => home_dir,
        None => 
        {
            let mut modules_hashmap = HashSet::new();
            modules_hashmap.insert(Modules::Clock);
            return (BarConfig::default(), None, modules_hashmap, (true, "Warning!!!: Failed to get Home directory".to_string()))
        }
    };
    let path = home_path.join(".config/icebar/config.ron");
    let mut config_failed = false;
    let mut warning_logs = String::new();

    let mut bar_config: BarConfig = match fs::read_to_string(&path)
    {
        Err(err) =>
        {
            config_failed = true;
            warning_logs = format!("WARNING: Failed to read config file: {err}\nWARNING: Using default config.");
            eprintln!("{warning_logs}");
            BarConfig::default()
        }
        Ok(content) =>
        {
            match ron::from_str::<BarConfig>(&content)
            {
                Ok(cfg) =>
                {
                    println!("Config loaded successfully.");
                    cfg
                }
                Err(err) =>
                {
                    config_failed = true;
                    warning_logs = format!("WARNING: Config parse failed: {err}\nWARNING: Attempting field-by-field fallback...");
                    eprintln!("{warning_logs}");
                    parse_with_fallback(&content)
                }
            }
        }
    };

    apply_general_settings(&mut bar_config);

    let current_time_zone = if let Some(ref time_zone) = bar_config.clock.clock_timezones && !time_zone.is_empty()
    {
        Some((time_zone[0].clone(), 0))
    }
    else
    {
        None
    };

    let mut active_modules: HashSet<Modules> = HashSet::new();
    let all_possible_default_modules = [Modules::Disk, Modules::FocusedWindowSway, Modules::FocusedWindowHypr, Modules::FocusedWindowNiri, Modules::CpuTemp, Modules::Ram, Modules::Cpu, Modules::NiriWorkspaces, Modules::MediaPlayerMetaData, Modules::MediaPlayerButtons, Modules::Network, Modules::HyprWorkspaces, Modules::SwayWorkspaces, Modules::VolumeOutput, Modules::VolumeInput, Modules::Clock, Modules::Tray];
    let all_possible_position = [&bar_config.general.left_modules, &bar_config.general.center_modules, &bar_config.general.right_modules];
    for position in all_possible_position
    {
        for item in position
        {
            if let Modules::Image(_) = item
            {
                active_modules.insert(item.to_owned());
            }
            if let Modules::CustomModule(_) = item
            {
                active_modules.insert(item.to_owned());
            }
            for module in &all_possible_default_modules
            {
                if *item == *module
                {
                    active_modules.insert(module.to_owned());
                }
            }
        }
    }

    println!("Active modules: {:?}", active_modules);

    (bar_config, current_time_zone, active_modules, (config_failed, warning_logs))
}


fn parse_with_fallback(content: &str) -> BarConfig
{
    // Strip the outer BarConfig( ... ) wrapper to get a flat body we can iterate.
    let body = extract_outer_body(content);
    let top_level = collect_fields(&body);

    let mut good_fields: Vec<String> = Vec::new();

    for (field_name, raw_value) in top_level
    {
        // Fast path: the whole top-level field is valid as-is.
        let snippet = format!("({}: {})", field_name, raw_value);
        if ron::from_str::<BarConfig>(&snippet).is_ok()
        {
            good_fields.push(format!("{}: {}", field_name, raw_value));
            continue;
        }

        // The top-level field failed. If it's a nested struct, try dropping
        // inner fields one-by-one until the struct parses cleanly.
        // Because all inner structs use #[serde(default)], any subset of
        // valid fields will parse — we only need to exclude the bad one(s).
        if let Some(inner_body) = extract_struct_body(&raw_value)
        {
            let inner_fields = collect_fields(&inner_body);
            let recovered = recover_struct(&field_name, inner_fields);
            if let Some(recovered_field) = recovered
            {
                good_fields.push(recovered_field);
                continue;
            }
        }

        eprintln!("WARNING: Skipping field '{}': could not recover any valid value", field_name);
    }

    let clean_ron = format!("({})", good_fields.join(", "));
    match ron::from_str::<BarConfig>(&clean_ron)
    {
        Ok(cfg) =>
        {
            println!("Partial config loaded with fallback defaults for bad fields.");
            cfg
        }
        Err(e) =>
        {
            eprintln!("WARNING: Partial config still failed: {e}");
            eprintln!("WARNING: Using full defaults.");
            BarConfig::default()
        }
    }
}


// Try to build a valid "field_name: ( ...inner_fields... )" by dropping any
// inner fields that cause a parse failure, one at a time.
// Returns None only if no combination of the inner fields produces a valid result.
fn recover_struct(field_name: &str, inner_fields: Vec<(String, String)>) -> Option<String>
{
    // Start with all fields included, then drop bad ones.
    let mut candidates: Vec<String> = inner_fields
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect();

    loop
    {
        let attempt = format!("{}: ({})", field_name, candidates.join(", "));
        let snippet  = format!("({})", attempt);

        if ron::from_str::<BarConfig>(&snippet).is_ok()
        {
            if candidates.len() < inner_fields.len()
            {
                eprintln!(
                    "WARNING: Recovered '{}' after dropping {} bad inner field(s).",
                    field_name,
                    inner_fields.len() - candidates.len()
                );
            }
            return Some(attempt);
        }

        // Find and drop the first inner field that is individually bad.
        // We do this by trying each candidate removed in turn.
        let mut dropped = false;
        for i in 0..candidates.len()
        {
            let mut without = candidates.clone();
            without.remove(i);
            let try_snippet = format!("({}: ({}))", field_name, without.join(", "));
            if ron::from_str::<BarConfig>(&try_snippet).is_ok() || without.is_empty()
            {
                let bad_key = inner_fields
                    .iter()
                    .find(|(k, v)| candidates[i] == format!("{}: {}", k, v))
                    .map(|(k, _)| k.as_str())
                    .unwrap_or("?");
                eprintln!("WARNING: Skipping field '{}.{}': invalid value", field_name, bad_key);
                candidates.remove(i);
                dropped = true;
                break;
            }
        }

        if !dropped
        {
            // Could not isolate a single bad field — drop all and give up.
            eprintln!("WARNING: Could not recover any fields from '{}'", field_name);
            return None;
        }

        if candidates.is_empty()
        {
            // All inner fields were bad; return an empty struct so defaults apply.
            let attempt = format!("{}: ()", field_name);
            let snippet  = format!("({})", attempt);
            if ron::from_str::<BarConfig>(&snippet).is_ok()
            {
                return Some(attempt);
            }
            return None;
        }
    }
}


// Strip "BarConfig\n(\n...\n)" and return the inner body string.
// Skips comment lines when searching for the opening paren so that a '('
// inside a header comment doesn't get mistaken for the struct opener.
fn extract_outer_body(content: &str) -> String
{
    // Walk line-by-line, skipping comment lines, to find the first '(' that
    // actually opens the BarConfig struct body.
    let start = {
        let mut found = None;
        let mut byte_pos = 0usize;
        for line in content.lines()
        {
            let trimmed = line.trim();
            if !trimmed.starts_with("//") && let Some(rel) = line.find('(')
            {
                    found = Some(byte_pos + rel);
                    break;
            }
            byte_pos += line.len() + 1; // +1 for '\n'
        }
        found.unwrap_or(0)
    };
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    let mut end = content.len();

    for (i, c) in content[start..].char_indices()
    {
        if escaped               { escaped = false; continue; }
        if c == '\\'             { escaped = true;  continue; }
        if c == '"'              { in_str = !in_str; continue; }
        if in_str                { continue; }
        match c
        {
            '(' => depth += 1,
            ')' =>
            {
                depth -= 1;
                if depth == 0 { end = start + i; break; }
            }
            _ => {}
        }
    }

    content[start + 1..end].to_string()
}


// If `raw` is a RON struct literal "( ... )", return the inner body.
fn extract_struct_body(raw: &str) -> Option<String>
{
    let trimmed = raw.trim();
    if !trimmed.starts_with('(') { return None; }
    Some(extract_outer_body(trimmed))
}


// Parse a flat RON struct body into (key, raw_value) pairs, handling
// multi-line values and nested structs/arrays correctly.
fn collect_fields(body: &str) -> Vec<(String, String)>
{
    let mut fields: Vec<(String, String)> = Vec::new();
    let mut current_field: Option<String> = None;
    let mut current_value = String::new();
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escaped = false;

    for line in body.lines()
    {
        let trimmed = line.trim();

        // Skip blank lines and full-line comments only when not mid-value.
        if current_field.is_none()
        {
            if trimmed.is_empty() || trimmed.starts_with("//") { continue; }

            let Some(colon_pos) = find_field_colon(trimmed) else { continue };
            let field_name = trimmed[..colon_pos].trim().to_string();
            let value_part = trimmed[colon_pos + 1..].trim().trim_end_matches(',').to_string();
            depth = count_depth_change_stateful(&value_part, &mut in_string, &mut escaped);

            if depth <= 0 && !value_part.is_empty() && !in_string
            {
                fields.push((field_name, value_part));
            }
            else
            {
                current_field = Some(field_name);
                current_value = value_part;
            }
        }
        else
        {
            // Strip inline comments before accounting for depth.
            let code_part = strip_line_comment(trimmed, &in_string);
            depth += count_depth_change_stateful(code_part, &mut in_string, &mut escaped);
            if !current_value.is_empty() { current_value.push(' '); }
            current_value.push_str(code_part);

            if depth <= 0 && !in_string
            {
                let field_name = current_field.take().unwrap_or_default();
                let final_value = current_value.trim().trim_end_matches(',').to_string();
                fields.push((field_name, final_value));
                current_value.clear();
                in_string = false;
                escaped = false;
            }
        }
    }

    fields
}


// Remove a trailing `// ...` comment from a line, but only when not inside a string.
fn strip_line_comment<'a>(line: &'a str, in_string: &bool) -> &'a str
{
    if *in_string { return line; }
    let mut in_str = false;
    let mut escaped = false;
    let mut prev_slash = false;
    let mut prev_slash_pos = 0usize;

    for (byte_pos, c) in line.char_indices()
    {
        if escaped               { escaped = false; prev_slash = false; continue; }
        if c == '\\'             { escaped = true;  prev_slash = false; continue; }
        if c == '"'              { in_str = !in_str; prev_slash = false; continue; }
        if in_str                { prev_slash = false; continue; }
        if c == '/' && prev_slash { return &line[..prev_slash_pos]; }
        prev_slash = c == '/';
        if prev_slash { prev_slash_pos = byte_pos; }
    }
    line
}

// replaces count_depth_change — takes mutable string/escape state
fn count_depth_change_stateful(s: &str, in_string: &mut bool, escaped: &mut bool) -> i32
{
    let mut depth = 0i32;
    for c in s.chars()
    {
        if *escaped { *escaped = false; continue; }
        if c == '\\' { *escaped = true; continue; }
        if c == '"' { *in_string = !*in_string; continue; }
        if *in_string { continue; }
        match c
        {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            _ => {}
        }
    }
    depth
}



// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
 
    #[test]
    fn bar_config_default_has_nonzero_bar_height()
    {
        let config = BarConfig::default();
        assert!(config.general.bar_size[1] > 0, "default bar height should be > 0");
    }
 
    #[test]
    fn bar_config_default_has_nonempty_font_family()
    {
        let config = BarConfig::default();
        assert!(!config.general.font_family.is_empty());
    }
 
    #[test]
    fn bar_config_default_has_nonempty_font_style()
    {
        let config = BarConfig::default();
        assert!(!config.general.font_style.is_empty());
    }
 
    #[test]
    fn bar_config_default_workspace_text_has_ten_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.workspace.workspace_text.len(), 10);
    }
 
    #[test]
    fn bar_config_default_workspace_selected_text_has_ten_entries()
    {
        let config = BarConfig::default();
        let selected = config.workspace.workspace_selected_text.unwrap();
        assert_eq!(selected.len(), 10);
    }
 
    #[test]
    fn bar_config_default_output_volume_format_has_six_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.volume_output.output_volume_format.len(), 6);
    }
 
    #[test]
    fn bar_config_default_input_volume_format_has_six_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.volume_input.input_volume_format.len(), 6);
    }
 
    #[test]
    fn bar_config_default_media_player_buttons_format_has_four_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.media_player_button.media_player_buttons_format.len(), 4);
    }
 
    #[test]
    fn bar_config_default_network_level_format_has_four_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.network.network_level_format.len(), 4);
    }
 
    #[test]
    fn bar_config_default_connection_type_icons_has_three_entries()
    {
        let config = BarConfig::default();
        assert_eq!(config.network.network_connection_type_icons.len(), 3);
    }
 
    #[test]
    fn bar_config_default_position_is_up()
    {
        assert_eq!(BarConfig::default().general.bar_position, BarPosition::Up);
    }
 
    #[test]
    fn bar_config_default_clock_timezones_is_none()
    {
        assert!(BarConfig::default().clock.clock_timezones.is_none());
    }
 
    #[test]
    fn bar_config_default_ellipsis_text_is_three_dots()
    {
        assert_eq!(BarConfig::default().general.ellipsis_text, "...");
    }
 
    #[test]
    fn bar_config_default_incremental_steps_are_nonzero()
    {
        let config = BarConfig::default();
        assert!(config.volume_output.incremental_steps_output > 0);
        assert!(config.volume_input.incremental_steps_input > 0);
    }

        // ---- alt_network default arrays ----------------------------------------
 
    #[test]
    fn bar_config_default_alt_network_level_format_has_four_entries()
    {
        assert_eq!(BarConfig::default().alt_network.alt_network_level_format.len(), 4);
    }
 
    #[test]
    fn bar_config_default_alt_network_connection_type_icons_has_three_entries()
    {
        assert_eq!(BarConfig::default().alt_network.alt_network_connection_type_icons.len(), 3);
    }
 
    // ---- Option fields default to None / Some sentinel ---------------------
 
    #[test]
    fn bar_config_default_display_is_none()
    {
        assert!(BarConfig::default().general.display.is_none());
    }
 
    #[test]
    fn bar_config_default_persistent_workspaces_is_none()
    {
        assert!(BarConfig::default().workspace.persistent_workspaces.is_none());
    }
 
    #[test]
    fn bar_config_default_force_static_position_context_menu_is_none()
    {
        assert!(BarConfig::default().general.force_static_position_context_menu.is_none());
    }
 
    // ---- String defaults are non-empty -------------------------------------
 
    #[test]
    fn bar_config_default_ellipsis_text_is_nonempty()
    {
        assert!(!BarConfig::default().general.ellipsis_text.is_empty());
    }
 
    #[test]
    fn bar_config_default_network_disconnected_text_is_nonempty()
    {
        assert!(!BarConfig::default().network.network_disconnected_text.is_empty());
    }
 
    #[test]
    fn bar_config_default_clock_format_is_nonempty()
    {
        assert!(!BarConfig::default().clock.clock_format.is_empty());
    }
 
    #[test]
    fn bar_config_default_network_module_format_is_nonempty()
    {
        assert!(!BarConfig::default().network.network_module_format.is_empty());
    }
 
    // ---- Numeric defaults are sane -----------------------------------------
 
    #[test]
    fn bar_config_default_incremental_steps_output_is_positive()
    {
        assert!(BarConfig::default().volume_output.incremental_steps_output > 0);
    }
 
    #[test]
    fn bar_config_default_incremental_steps_input_is_positive()
    {
        assert!(BarConfig::default().volume_input.incremental_steps_input > 0);
    }
 
    #[test]
    fn bar_config_default_context_menu_item_size_is_positive()
    {
        assert!(BarConfig::default().context_menu.context_menu_item_size > 0);
    }
 
    #[test]
    fn bar_config_default_context_menu_size_is_positive()
    {
        assert!(BarConfig::default().context_menu.context_menu_size > 0);
    }
 
    #[test]
    fn bar_config_default_tray_icon_size_is_positive()
    {
        assert!(BarConfig::default().tray.tray_icon_size > 0);
    }
 
    // ---- Vec defaults are empty (user configures modules) ------------------
 
    #[test]
    fn bar_config_default_custom_modules_is_empty()
    {
        assert!(BarConfig::default().custom_module.custom_modules.is_empty());
    }
 
    //#[test]
    //fn bar_config_default_left_modules_is_empty()
    //{
    //    assert!(BarConfig::default().left_modules.is_empty());
    //}
 
    //#[test]
    //fn bar_config_default_center_modules_is_empty()
    //{
    //    assert!(BarConfig::default().center_modules.is_empty());
    //}
 
    //#[test]
    //fn bar_config_default_right_modules_is_empty()
    //{
    //    assert!(BarConfig::default().right_modules.is_empty());
    //}
 
    // ---- BarPosition equality ----------------------------------------------
 
    #[test]
    fn bar_position_same_variants_are_equal()
    {
        assert_eq!(BarPosition::Up,    BarPosition::Up);
        assert_eq!(BarPosition::Down,  BarPosition::Down);
        assert_eq!(BarPosition::Left,  BarPosition::Left);
        assert_eq!(BarPosition::Right, BarPosition::Right);
    }
 
    #[test]
    fn bar_position_different_variants_are_not_equal()
    {
        assert_ne!(BarPosition::Up,   BarPosition::Down);
        assert_ne!(BarPosition::Left, BarPosition::Right);
        assert_ne!(BarPosition::Up,   BarPosition::Left);
    }
}
