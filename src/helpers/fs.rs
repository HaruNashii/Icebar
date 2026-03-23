// ============ IMPORTS ============
use std::{io::Write, fs, fs::File, path::Path};





// ============ FUNCTIONS ============
pub fn check_if_config_file_exists() -> Option<String>
{
    println!("\n=== FS CHECK RUNNING... ===");
    let home_path = match home::home_dir()
    {
        Some(home_dir) => home_dir.display().to_string(),
        None => return Some("Warning!!!: Failed to get Home directory".to_string()),
    };
    let ron_config_dir = format!("{}/.config/icebar", home_path);
    let ron_config_file_dir = format!("{}/config.ron", ron_config_dir);
    let ron_file_config_path = Path::new(&ron_config_file_dir);
    let ron_config_path = Path::new(&ron_config_dir);

    if Path::exists(ron_config_path)
    {
        println!("Ron Config Directory Exists!!!");
    }
    else
    {
        println!("Ron config directory doesn't exist, Creating...");
        if let Err(err) = fs::create_dir_all(ron_config_path)
        {
            return Some(format!("Warning!!!: Couldn't create ron config directory, ERR: {err}"));
        };
    };

    if Path::exists(ron_file_config_path)
    {
        println!("Ron Config File Exists!!!");
    }
    else
    {
        println!("Ron config file doesn't exist, Creating...");
        let ron_default_data = r#"//==============================================================================================================================================
// This file is auto-generated when icebar detects that the config file or config directory doesn't exist.
//
//===============================================================[  WARNINGS  ]=================================================================
// - The alpha channel of RGBA values has a range of 0 to 100. Parsing a value greater than 100 will crash.
// - Setting the first value of "bar_size" to 0 will make the bar fill the entire screen on the X axis.
// - It is very important to set the "display" variable. Not setting it may cause undefined behaviour.
// - Missing options are fine and will fall back to their default values, but invalid syntax will crash.
// - The "continous_command" field may generate high CPU usage depending on how heavy the command is.
// - The "continous_command" field must not run a loop of any kind — the process will hang forever if it does.
// - "bar_size" has the format (width, height). For side bars, a width of 0 is not valid and will crash.
// - "NiriWorkspaces" does not support "persistent_workspaces". If set, it will be ignored.
// - If the number of workspaces exceeds the number of entries in "workspace_text" or "workspace_selected_text",
//   the extra workspaces will display their number as text instead.
//
//==============================================================================================================================================
//
//
//=================================================================[  TIPS  ]===================================================================
// All possible modules:
//   "FocusedWindowSway", "FocusedWindowHypr", "FocusedWindowNiri",
//   "NiriWorkspaces", "HyprWorkspaces", "SwayWorkspaces",
//   "MediaPlayerMetaData", "MediaPlayerButtons",
//   "CustomModule(index)", "Image(index)",
//   "CpuTemp", "Ram", "Cpu", "Disk",
//   "VolumeOutput", "VolumeInput",
//   "Network", "Clock", "Tray"
//
// =============================================================================================================================================
// All color fields now use the ColorType system. Three formats are supported:
//   RGB((R, G, B))         — standard RGB color. Values range from 0 to 255.
//   RGBA((R, G, B, A))     — RGB with alpha. Alpha ranges from 0 (transparent) to 100 (opaque).
//   HEX("RRGGBB")          — hex color string. Supports 6-digit (FF0000) and 8-digit (FF0000FF) formats.
//
// =============================================================================================================================================
// Gradient fields accept either "None" (no gradient) or a Gradient value with this syntax: Gradient((angle, [(offset, color), ...]))
//
// =============================================================================================================================================
// Fields with unique syntax:
//   general: ( display: Some("HDMI-A-1"), ... )
//   clock: ( clock_timezones: Some(["America/New_York", "Europe/London"]), ... )
//   workspace: ( persistent_workspaces: Some(5), ... )
//   image: ( images_spacing: 5, images: [ (...), (...) ] )
//   custom_module: ( custom_modules_spacing: 10, custom_modules: [ (...) ] )
//
// =============================================================================================================================================
// - The "general_style" fields overwrite every respective per-module option.
// - Available options for "MODULENAME_side_separator": Some(Left), Some(Right), Some(Up), Some(Down), Some(LeftAndRight), Some(UpAndDown).
// - Available options for "bar_position": "Up", "Down", "Left", "Right".
// - To find the correct "font_family" and "font_style" values, run: fc-scan $PATH_TO_FONT_FILE
// - Setting "bar_check_reload_interval_ms" to "None" disables hot-reload.
// - Images and Custom modules are assigned an index based on their position (top to bottom): first = 0, second = 1, etc...
//   Reference them in the module lists as "Image(index)" or "CustomModule(index)".
//
// - Report bugs or request features at: https://github.com/HaruNashii/Icebar
// - Contact directly on Discord: harunashiii
// - Join the contact server: https://discord.gg/CRsz24Ts3a
//
// =============================================================================================================================================



BarConfig
(

    // ================= GENERAL =================
    general:
    (
        display:                            None,
        bar_position:                       Up,
        floating_space:                     0,
        increased_exclusive_bar_zone:       0,
        bar_check_reload_interval_ms:       Some(500),
        bar_side_spaces_size:               8,
        bar_size:                           (0, 40),
        bar_border_radius:                  (0., 0., 0., 0.),
        bar_border_size:                    1.0,
        bar_border_color:                   HEX("1a1a1a"),
        bar_background_color:               HEX("242424"),
        font_family:                        "JetBrains",
        font_style:                         "Bold",
        spacing_between_all_modules:        5,
        force_static_position_context_menu: None,
        left_modules:                       [CustomModule(0), Cpu, CpuTemp, Ram, Disk],
        center_modules:                     [Clock, MediaPlayerMetaData, MediaPlayerButtons],
        right_modules:                      [Tray, Network, VolumeOutput, VolumeInput],
    ),


    // ================= GENERAL STYLE =================
    general_style:
    (
        general_padding:                        Some(0),
        general_text_size:                      Some(14),
        general_text_orientation:               Some(Horizontal),
        general_text_color:                     Some(HEX("ffffff")),
        general_button_color:                   Some(HEX("303030")),
        general_button_hovered_color:           Some(HEX("3d3d3d")),
        general_button_hovered_text_color:      Some(HEX("ffffff")),
        general_button_pressed_text_color:      Some(HEX("ffffff")),
        general_button_pressed_color:           Some(HEX("1c1c1c")),
        general_button_gradient_color:          None,
        general_button_pressed_gradient_color:  None,
        general_button_hovered_gradient_color:  None,
        general_button_shadow_color:            Some(RGBA((0, 0, 0, 50))),
        general_button_shadow_x:                Some(1.0),
        general_button_shadow_y:                Some(2.0),
        general_button_shadow_blur:             Some(5.0),
        general_border_color:                   Some(HEX("3d3d3d")),
        general_border_size:                    Some(1.0),
        general_border_radius:                  Some((6.0, 6.0, 6.0, 6.0)),
        general_side_separator:                 None,
        general_side_separator_color:           Some(HEX("3d3d3d")),
        general_side_separator_width:           Some(1.0),
        general_side_separator_height:          Some(18.0),
        general_alt_padding:                    None,
        general_alt_text_size:                  None,
        general_alt_text_orientation:           None,
        general_alt_text_color:                 None,
        general_alt_button_color:               None,
        general_alt_button_hovered_color:       None,
        general_alt_button_hovered_text_color:  None,
        general_alt_button_pressed_text_color:  None,
        general_alt_button_pressed_color:       None,
        general_alt_border_color:               None,
        general_alt_border_size:                None,
        general_alt_border_radius:              None,
        general_alt_side_separator:             None,
        general_alt_side_separator_color:       None,
        general_alt_side_separator_width:       None,
        general_alt_side_separator_height:      None,
        general_alt_button_gradient_color:      None,
        general_alt_button_pressed_gradient_color:  None,
        general_alt_button_hovered_gradient_color:  None,
        general_alt_button_shadow_color:            Some(RGBA((0, 0, 0, 50))),
        general_alt_button_shadow_x:                Some(1.0),
        general_alt_button_shadow_y:                Some(2.0),
        general_alt_button_shadow_blur:             Some(5.0),
    ),


    // ================= CLOCK =================
    clock:
    (
        clock_timezones:                        None,
        clock_update_interval:                  400,
        clock_format:                           "[Color=(120, 174, 237), String=󰥔]  %H:%M",
        clock_alt_format:                       "󰃭  %a %d %b",
        action_on_left_click_clock:             DefaultAction,
        action_on_right_click_clock:            CustomAction(["kitty", "bash", "-c", "cal && echo 'Press Enter To Exit' && read -n 1"]),
        clock_side_separator:                   None,
        clock_side_separator_color:             HEX("3d3d3d"),
        clock_side_separator_width:             1.,
        clock_side_separator_height:            18.,
        alt_clock_side_separator:               None,
        alt_clock_side_separator_color:         HEX("3d3d3d"),
        alt_clock_side_separator_width:         1.,
        alt_clock_side_separator_height:        18.,
        clock_button_hovered_gradient_color:    None,
        clock_button_pressed_gradient_color:    None,
        clock_button_gradient_color:            None,
        clock_button_shadow_color:              Some(RGBA((0, 0, 0, 50))),
        clock_button_shadow_x:                  0.0,
        clock_button_shadow_y:                  1.0,
        clock_button_shadow_blur:               3.0,
        alt_clock_button_gradient_color:        None,
        alt_clock_button_hovered_gradient_color: None,
        alt_clock_button_pressed_gradient_color: None,
        //alt_clock_button_shadow_color: Some(RGBA((50, 255, 50, 70))),
        //alt_clock_button_shadow_x: 2.0,
        //alt_clock_button_shadow_y: 2.0,
        //alt_clock_button_shadow_blur: 4.0,
        clock_padding:                          7,
        clock_text_size:                        14,
        clock_text_color:                       HEX("ffffff"),
        clock_text_orientation:                 Horizontal,
        clock_button_color:                     HEX("303030"),
        clock_button_hovered_color:             HEX("3d3d3d"),
        clock_button_hovered_text_color:        HEX("ffffff"),
        clock_button_pressed_text_color:        HEX("ffffff"),
        clock_button_pressed_color:             HEX("1c1c1c"),
        clock_border_color:                     HEX("3d3d3d"),
        clock_border_size:                      1.0,
        clock_border_radius:                    (6.0, 6.0, 6.0, 6.0),
        alt_clock_padding:                      7,
        alt_clock_text_size:                    14,
        alt_clock_text_color:                   HEX("ffffff"),
        alt_clock_text_orientation:             Horizontal,
        alt_clock_button_color:                 HEX("1c71d8"),
        alt_clock_button_hovered_color:         RGB((143, 191, 245)),
        alt_clock_button_hovered_text_color:    HEX("ffffff"),
        alt_clock_button_pressed_text_color:    HEX("ffffff"),
        alt_clock_button_pressed_color:         RGB((86, 148, 210)),
        alt_clock_border_color:                 HEX("5a8fc7"),
        alt_clock_border_size:                  1.0,
        alt_clock_border_radius:                (6.0, 6.0, 6.0, 6.0),
    ),


    // ================= VOLUME OUTPUT =================
    volume_output:
    (
        output_volume_format:
        (
            "[Color=(120, 174, 237), String=]  {}%",
            "[Color=(120, 174, 237), String=󰖀]  {}%",
            "[Color=(120, 174, 237), String=]  {}%",
            "[Color=(120, 174, 237), String=]  {}%",
            "[Color=(120, 174, 237), String=]  {}%",
            "[Color=(120, 174, 237), String= +] {}%"
        ),
        output_volume_muted_format:                     "  Muted",
        incremental_steps_output:                       5,
        action_on_left_click_volume_output:             DefaultAction,
        action_on_right_click_volume_output:            CustomAction(["kitty", "pulsemixer"]),
        volume_output_padding:                          7,
        volume_output_text_size:                        14,
        volume_output_text_color:                       HEX("ffffff"),
        volume_output_text_orientation:                 Horizontal,
        volume_output_button_color:                     HEX("303030"),
        volume_output_button_hovered_color:             HEX("3d3d3d"),
        volume_output_button_hovered_text_color:        HEX("ffffff"),
        volume_output_button_pressed_text_color:        HEX("ffffff"),
        volume_output_button_pressed_color:             HEX("1c1c1c"),
        volume_output_border_color:                     HEX("3d3d3d"),
        volume_output_border_size:                      1.0,
        volume_output_border_radius:                    (6.0, 6.0, 6.0, 6.0),
        volume_output_side_separator:                   None,
        volume_output_side_separator_color:             HEX("3d3d3d"),
        volume_output_side_separator_width:             1.,
        volume_output_side_separator_height:            18.,
        volume_output_button_gradient_color:            None,
        volume_output_button_hovered_gradient_color:    None,
        volume_output_button_pressed_gradient_color:    None,
        volume_output_button_shadow_color:              Some(RGBA((0, 0, 0, 50))),
        volume_output_button_shadow_x:                  0.0,
        volume_output_button_shadow_y:                  1.0,
        volume_output_button_shadow_blur:               3.0,
    ),


    // ================= MUTED VOLUME OUTPUT =================
    muted_volume_output:
    (
        muted_volume_output_padding:                            7,
        muted_volume_output_text_size:                          14,
        muted_volume_output_text_color:                         HEX("ffffff"),
        muted_volume_output_text_orientation:                   Horizontal,
        muted_volume_output_button_color:                       RGB((192, 28, 40)),
        muted_volume_output_button_hovered_color:               RGB((210, 38, 52)),
        muted_volume_output_button_hovered_text_color:          HEX("ffffff"),
        muted_volume_output_button_pressed_text_color:          HEX("ffffff"),
        muted_volume_output_button_pressed_color:               RGB((161, 21, 32)),
        muted_volume_output_border_color:                       HEX("8b1a22"),
        muted_volume_output_border_size:                        1.0,
        muted_volume_output_border_radius:                      (6.0, 6.0, 6.0, 6.0),
        muted_volume_output_side_separator:                     None,
        muted_volume_output_side_separator_color:               HEX("3d3d3d"),
        muted_volume_output_side_separator_width:               1.,
        muted_volume_output_side_separator_height:              18.,
        muted_volume_output_button_gradient_color:              None,
        muted_volume_output_button_hovered_gradient_color:      None,
        muted_volume_output_button_pressed_gradient_color:      None,
        muted_volume_output_button_shadow_color:                Some(RGBA((0, 0, 0, 50))),
        muted_volume_output_button_shadow_x:                    0.0,
        muted_volume_output_button_shadow_y:                    1.0,
        muted_volume_output_button_shadow_blur:                 3.0,
    ),


    // ================= VOLUME INPUT =================
    volume_input:
    (
        input_volume_format:
        (
            "[Color=(120, 174, 237), String=] {}%",
            "[Color=(120, 174, 237), String=] {}%",
            "[Color=(120, 174, 237), String=] {}%",
            "[Color=(120, 174, 237), String=] {}%",
            "[Color=(120, 174, 237), String=] {}%",
            "[Color=(120, 174, 237), String=󰢴] {}%"
        ),
        input_volume_muted_format:                      "  Muted",
        incremental_steps_input:                        5,
        action_on_left_click_volume_input:              DefaultAction,
        action_on_right_click_volume_input:             CustomAction(["kitty", "pulsemixer"]),
        volume_input_padding:                           7,
        volume_input_text_size:                         14,
        volume_input_text_color:                        HEX("ffffff"),
        volume_input_text_orientation:                  Horizontal,
        volume_input_button_color:                      HEX("303030"),
        volume_input_button_hovered_color:              HEX("3d3d3d"),
        volume_input_button_hovered_text_color:         HEX("ffffff"),
        volume_input_button_pressed_text_color:         HEX("ffffff"),
        volume_input_button_pressed_color:              HEX("1c1c1c"),
        volume_input_border_color:                      HEX("3d3d3d"),
        volume_input_border_size:                       1.0,
        volume_input_border_radius:                     (6.0, 6.0, 6.0, 6.0),
        volume_input_side_separator:                    None,
        volume_input_side_separator_color:              HEX("3d3d3d"),
        volume_input_side_separator_width:              1.,
        volume_input_side_separator_height:             18.,
        volume_input_button_gradient_color:             None,
        volume_input_button_hovered_gradient_color:     None,
        volume_input_button_pressed_gradient_color:     None,
        volume_input_button_shadow_color:               Some(RGBA((0, 0, 0, 50))),
        volume_input_button_shadow_x:                   0.0,
        volume_input_button_shadow_y:                   1.0,
        volume_input_button_shadow_blur:                3.0,
    ),


    // ================= MUTED VOLUME INPUT =================
    muted_volume_input:
    (
        muted_volume_input_padding:                         7,
        muted_volume_input_text_size:                       14,
        muted_volume_input_text_color:                      HEX("ffffff"),
        muted_volume_input_text_orientation:                Horizontal,
        muted_volume_input_button_color:                    RGB((192, 28, 40)),
        muted_volume_input_button_hovered_color:            RGB((210, 38, 52)),
        muted_volume_input_button_hovered_text_color:       HEX("ffffff"),
        muted_volume_input_button_pressed_text_color:       HEX("ffffff"),
        muted_volume_input_button_pressed_color:            RGB((161, 21, 32)),
        muted_volume_input_border_color:                    HEX("8b1a22"),
        muted_volume_input_border_size:                     1.0,
        muted_volume_input_border_radius:                   (6.0, 6.0, 6.0, 6.0),
        muted_volume_input_side_separator:                  None,
        muted_volume_input_side_separator_color:            HEX("3d3d3d"),
        muted_volume_input_side_separator_width:            1.,
        muted_volume_input_side_separator_height:           18.,
        muted_volume_input_button_gradient_color:           None,
        muted_volume_input_button_hovered_gradient_color:   None,
        muted_volume_input_button_pressed_gradient_color:   None,
        muted_volume_input_button_shadow_color:             Some(RGBA((0, 0, 0, 50))),
        muted_volume_input_button_shadow_x:                 0.0,
        muted_volume_input_button_shadow_y:                 1.0,
        muted_volume_input_button_shadow_blur:              3.0,
    ),


    // ================= NETWORK =================
    network:
    (
        network_module_format:          "{level}[Tuning=4]",
        network_disconnected_text:      "[Color=(192, 28, 40), String=󰖪]  No Connection",
        network_level_format:
        (
            "[Color=(120, 174, 237), String=󰖩]",
            "[Color=(120, 174, 237), String=󱚵]",
            "[Color=(120, 174, 237), String=󱚼]",
            "[Color=(192, 28, 40), String=󰖪]"
        ),
        network_connection_type_icons:
        (
            "[Color=(120, 174, 237), String=󰈀]",
            "[Color=(120, 174, 237), String=]",
            "[Color=(120, 174, 237), String=?]"
        ),
        action_on_left_click_network:           DefaultAction,
        action_on_right_click_network:          DefaultAction,
        network_padding:                        7,
        network_text_size:                      14,
        network_text_color:                     HEX("ffffff"),
        network_text_orientation:               Horizontal,
        network_button_color:                   HEX("303030"),
        network_button_hovered_color:           HEX("3d3d3d"),
        network_button_hovered_text_color:      HEX("ffffff"),
        network_button_pressed_text_color:      HEX("ffffff"),
        network_button_pressed_color:           HEX("1c1c1c"),
        network_border_color:                   HEX("3d3d3d"),
        network_border_size:                    1.0,
        network_border_radius:                  (6.0, 6.0, 6.0, 6.0),
        network_side_separator:                 None,
        network_side_separator_color:           HEX("3d3d3d"),
        network_side_separator_width:           1.,
        network_side_separator_height:          18.,
        network_button_gradient_color:          None,
        network_button_hovered_gradient_color:  None,
        network_button_pressed_gradient_color:  None,
        network_button_shadow_color:            Some(RGBA((0, 0, 0, 50))),
        network_button_shadow_x:                0.0,
        network_button_shadow_y:                1.0,
        network_button_shadow_blur:             3.0,
    ),


    // ================= ALT NETWORK =================
    alt_network:
    (
        //with all information: alt_network_module_format:          "{level} {connection_type} • {id} • [Color=(87, 227, 137), String={speed}]MB/s • [Color=(255, 190, 111), String={sent}]KB/s • [Color=(255, 190, 111), String={received}]KB/s",
        alt_network_module_format:          "{id} • {speed}MB/s",
        alt_network_level_format:           ("󰖩", "󱚵", "󱚼", "󰖪"),
        alt_network_connection_type_icons:  ("󰈀", "", "?"),
        alt_network_padding:                        7,
        alt_network_text_size:                      14,
        alt_network_text_color:                     HEX("ffffff"),
        alt_network_text_orientation:               Horizontal,
        alt_network_button_color:                   HEX("1c71d8"),
        alt_network_button_hovered_color:           RGB((143, 191, 245)),
        alt_network_button_hovered_text_color:      HEX("ffffff"),
        alt_network_button_pressed_text_color:      HEX("ffffff"),
        alt_network_button_pressed_color:           RGB((86, 148, 210)),
        alt_network_border_color:                   HEX("5a8fc7"),
        alt_network_border_size:                    1.0,
        alt_network_border_radius:                  (6.0, 6.0, 6.0, 6.0),
        alt_network_side_separator:                 None,
        alt_network_side_separator_color:           HEX("3d3d3d"),
        alt_network_side_separator_width:           1.,
        alt_network_side_separator_height:          18.,
        alt_network_button_gradient_color:          None,
        alt_network_button_hovered_gradient_color:  None,
        alt_network_button_pressed_gradient_color:  None,
        alt_network_button_shadow_color:            None,
        alt_network_button_shadow_x:                0.0,
        alt_network_button_shadow_y:                0.0,
        alt_network_button_shadow_blur:             0.0,
    ),


    // ================= WORKSPACE =================
    workspace:
    (
        niri_workspaces_update_interval:        225,
        reverse_scroll_on_workspace:            false,
        persistent_workspaces:                  Some(5),
        workspace_height:                       22,
        workspace_width:                        22,
        workspace_different_selected_width:     Some(28),
        workspace_different_selected_height:    Some(22),
        workspace_text_size:                    12,
        workspace_text_color:                   RGBA((255, 255, 255, 35)),
        workspace_selected_text_color:          RGB((120, 174, 237)),
        workspace_text_orientation:             Horizontal,
        workspace_text:                         ["•", "•", "•", "•", "•", "•", "•", "•", "•", "•"],
        workspace_selected_text:                Some(["⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤"]),
        workspace_spacing:                      4,
        workspace_padding:                      0,
        workspace_button_color:                 RGBA((0, 0, 0, 0)),
        workspace_button_selected_color:        RGBA((0, 0, 0, 0)),
        workspace_button_hovered_color:         RGBA((255, 255, 255, 8)),
        workspace_button_hovered_text_color:    HEX("ffffff"),
        workspace_button_pressed_text_color:    HEX("ffffff"),
        workspace_button_pressed_color:         RGBA((255, 255, 255, 12)),
        workspace_border_color:                 RGBA((0, 0, 0, 0)),
        workspace_border_size:                  0.0,
        workspace_border_radius:                (0.0, 0.0, 0.0, 0.0),
        workspace_side_separator:               None,
        workspace_side_separator_color:         HEX("3d3d3d"),
        workspace_side_separator_width:         1.,
        workspace_side_separator_height:        18.,
        workspace_button_gradient_color:            None,
        workspace_button_selected_gradient_color:   None,
        workspace_button_hovered_gradient_color:    None,
        workspace_button_pressed_gradient_color:    None,
        workspace_button_shadow_color:              None,
        workspace_button_shadow_x:                  0.0,
        workspace_button_shadow_y:                  0.0,
        workspace_button_shadow_blur:               0.0,
    ),


    // ================= TRAY =================
    tray:
    (
        tray_icon_size:                     19,
        tray_spacing:                       4,
        tray_button_size:                   4,
        tray_button_color:                  HEX("303030"),
        tray_button_hovered_color:          HEX("3d3d3d"),
        tray_button_hovered_text_color:     HEX("ffffff"),
        tray_button_pressed_text_color:     HEX("ffffff"),
        tray_button_pressed_color:          HEX("1c1c1c"),
        tray_border_color:                  HEX("3d3d3d"),
        tray_border_size:                   1.0,
        tray_border_radius:                 (6.0, 6.0, 6.0, 6.0),
        tray_side_separator:                None,
        tray_side_separator_color:          HEX("3d3d3d"),
        tray_side_separator_width:          1.,
        tray_side_separator_height:         18.,
        tray_button_gradient_color:         None,
        tray_button_hovered_gradient_color: None,
        tray_button_pressed_gradient_color: None,
        tray_button_shadow_color:           Some(RGBA((0, 0, 0, 50))),
        tray_button_shadow_x:               0.0,
        tray_button_shadow_y:               1.0,
        tray_button_shadow_blur:            3.0,
    ),

    // ================= MEDIA PLAYER METADATA =================
    media_player_metadata:
    (
        player:                                         "spotify",
        media_player_metadata_format:                   "[Color=(120, 174, 237), String=] [Tuning=5]{{artist}} | {{title}}",
        media_player_metadata_update_interval:          750,
        dont_show_metadata_if_empty:                    false,
        text_when_metadata_is_empty:                    "No Media Playing",
        media_player_metadata_text_limit_len:           45,
        ellipsis_text:                                  "…",
        action_on_left_click_media_player_metadata:     Nothing,
        action_on_right_click_media_player_metadata:    Nothing,
        media_player_metadata_padding:                  7,
        media_player_metadata_text_size:                14,
        media_player_metadata_text_color:               HEX("ffffff"),
        media_player_metadata_text_orientation:         Horizontal,
        media_player_metadata_button_color:             HEX("303030"),
        media_player_metadata_button_hovered_color:     HEX("3d3d3d"),
        media_player_metadata_button_hovered_text_color:    HEX("ffffff"),
        media_player_metadata_button_pressed_text_color:    HEX("ffffff"),
        media_player_metadata_button_pressed_color:         HEX("1c1c1c"),
        media_player_metadata_border_color:             HEX("3d3d3d"),
        media_player_metadata_border_size:              1.0,
        media_player_metadata_border_radius:            (6.0, 6.0, 6.0, 6.0),
        media_player_metadata_side_separator:           None,
        media_player_metadata_side_separator_color:     HEX("3d3d3d"),
        media_player_metadata_side_separator_width:     1.,
        media_player_metadata_side_separator_height:    18.,
        media_player_metadata_button_gradient_color:            None,
        media_player_metadata_button_hovered_gradient_color:    None,
        media_player_metadata_button_pressed_gradient_color:    None,
        media_player_metadata_button_shadow_color:              Some(RGBA((0, 0, 0, 50))),
        media_player_metadata_button_shadow_x:                  0.0,
        media_player_metadata_button_shadow_y:                  1.0,
        media_player_metadata_button_shadow_blur:               3.0,
    ),


    // ================= MEDIA PLAYER BUTTONS =================
    media_player_button:
    (
        media_player_buttons_format:                    ("⏮", "⏸", "▶", "⏭"),
        media_player_button_spacing:                    4,
        media_player_button_padding:                    7,
        media_player_button_text_size:                  14,
        media_player_button_text_color:                 HEX("ffffff"),
        media_player_button_text_orientation:           Horizontal,
        media_player_button_color:                      HEX("303030"),
        media_player_button_hovered_color:              HEX("3d3d3d"),
        media_player_button_hovered_text_color:         HEX("ffffff"),
        media_player_button_pressed_text_color:         HEX("ffffff"),
        media_player_button_pressed_color:              HEX("1c1c1c"),
        media_player_button_border_color:               HEX("3d3d3d"),
        media_player_button_border_size:                1.0,
        media_player_button_border_radius:              (6.0, 6.0, 6.0, 6.0),
        media_player_buttons_side_separator:            None,
        media_player_buttons_side_separator_color:      HEX("3d3d3d"),
        media_player_buttons_side_separator_width:      1.,
        media_player_buttons_side_separator_height:     18.,
        media_player_button_gradient_color:             None,
        media_player_button_hovered_gradient_color:     None,
        media_player_button_pressed_gradient_color:     None,
        media_player_button_shadow_color:               Some(RGBA((0, 0, 0, 50))),
        media_player_button_shadow_x:                   0.0,
        media_player_button_shadow_y:                   1.0,
        media_player_button_shadow_blur:                3.0,
    ),


    // ================= CPU =================
    cpu:
    (
        cpu_format:                         "[Color=(120, 174, 237), String=CPU]  {usage}%",
        cpu_update_interval:                1000,
        action_on_left_click_cpu:           DefaultAction,
        action_on_right_click_cpu:          DefaultAction,
        cpu_padding:                        7,
        cpu_text_size:                      13,
        cpu_text_color:                     HEX("ffffff"),
        cpu_text_orientation:               Horizontal,
        cpu_button_color:                   HEX("303030"),
        cpu_button_hovered_color:           HEX("3d3d3d"),
        cpu_button_hovered_text_color:      HEX("ffffff"),
        cpu_button_pressed_text_color:      HEX("ffffff"),
        cpu_button_pressed_color:           HEX("1c1c1c"),
        cpu_border_color:                   HEX("3d3d3d"),
        cpu_border_size:                    1.0,
        cpu_border_radius:                  (6.0, 6.0, 6.0, 6.0),
        cpu_side_separator:                 None,
        cpu_side_separator_color:           HEX("3d3d3d"),
        cpu_side_separator_width:           1.,
        cpu_side_separator_height:          18.,
        cpu_button_gradient_color:          None,
        cpu_button_hovered_gradient_color:  None,
        cpu_button_pressed_gradient_color:  None,
        cpu_button_shadow_color:            Some(RGBA((0, 0, 0, 50))),
        cpu_button_shadow_x:                0.0,
        cpu_button_shadow_y:                1.0,
        cpu_button_shadow_blur:             3.0,
    ),


    // ================= CPU TEMP =================
    cpu_temp:
    (
        cpu_temp_format:                        "[Color=(120, 174, 237), String=TEMP]  {temp}°C",
        cpu_temp_update_interval:               1000,
        action_on_left_click_cpu_temp:          DefaultAction,
        action_on_right_click_cpu_temp:         DefaultAction,
        cpu_temp_padding:                       7,
        cpu_temp_text_size:                     13,
        cpu_temp_text_color:                    HEX("ffffff"),
        cpu_temp_text_orientation:              Horizontal,
        cpu_temp_button_color:                  HEX("303030"),
        cpu_temp_button_hovered_color:          HEX("3d3d3d"),
        cpu_temp_button_hovered_text_color:     HEX("ffffff"),
        cpu_temp_button_pressed_text_color:     HEX("ffffff"),
        cpu_temp_button_pressed_color:          HEX("1c1c1c"),
        cpu_temp_border_color:                  HEX("3d3d3d"),
        cpu_temp_border_size:                   1.0,
        cpu_temp_border_radius:                 (6.0, 6.0, 6.0, 6.0),
        cpu_temp_side_separator:                None,
        cpu_temp_side_separator_color:          HEX("3d3d3d"),
        cpu_temp_side_separator_width:          1.,
        cpu_temp_side_separator_height:         18.,
        cpu_temp_button_gradient_color:         None,
        cpu_temp_button_hovered_gradient_color: None,
        cpu_temp_button_pressed_gradient_color: None,
        cpu_temp_button_shadow_color:           Some(RGBA((0, 0, 0, 50))),
        cpu_temp_button_shadow_x:               0.0,
        cpu_temp_button_shadow_y:               1.0,
        cpu_temp_button_shadow_blur:            3.0,
    ),


    // ================= RAM =================
    ram:
    (
        ram_format:             "[Color=(120, 174, 237), String=RAM]  {used}MB / {percent}%",
        ram_update_interval:    1000,
        ram_padding:                    7,
        ram_text_size:                  13,
        ram_text_color:                 HEX("ffffff"),
        ram_text_orientation:           Horizontal,
        ram_button_color:               HEX("303030"),
        ram_button_hovered_color:       HEX("3d3d3d"),
        ram_button_hovered_text_color:  HEX("ffffff"),
        ram_button_pressed_text_color:  HEX("ffffff"),
        ram_button_pressed_color:       HEX("1c1c1c"),
        ram_border_color:               HEX("3d3d3d"),
        ram_border_size:                1.0,
        ram_border_radius:              (6.0, 6.0, 6.0, 6.0),
        ram_side_separator:             None,
        ram_side_separator_color:       HEX("3d3d3d"),
        ram_side_separator_width:       1.,
        ram_side_separator_height:      18.,
        ram_button_gradient_color:          None,
        ram_button_hovered_gradient_color:  None,
        ram_button_pressed_gradient_color:  None,
        ram_button_shadow_color:            Some(RGBA((0, 0, 0, 50))),
        ram_button_shadow_x:               0.0,
        ram_button_shadow_y:               1.0,
        ram_button_shadow_blur:            3.0,
    ),


    // ================= DISK =================
    disk:
    (
        disk_format:            "[Color=(120, 174, 237), String=DISK]  {used}GB / {percent}%",
        disk_mount:             "/",
        disk_update_interval:   10000,
        disk_padding:                   7,
        disk_text_size:                 13,
        disk_text_color:                HEX("ffffff"),
        disk_text_orientation:          Horizontal,
        disk_button_color:              HEX("303030"),
        disk_button_hovered_color:      HEX("3d3d3d"),
        disk_button_hovered_text_color: HEX("ffffff"),
        disk_button_pressed_text_color: HEX("ffffff"),
        disk_button_pressed_color:      HEX("1c1c1c"),
        disk_border_color:              HEX("3d3d3d"),
        disk_border_size:               1.0,
        disk_border_radius:             (6.0, 6.0, 6.0, 6.0),
        disk_side_separator:            None,
        disk_side_separator_color:      HEX("3d3d3d"),
        disk_side_separator_width:      1.,
        disk_side_separator_height:     18.,
        disk_button_gradient_color:         None,
        disk_button_hovered_gradient_color: None,
        disk_button_pressed_gradient_color: None,
        disk_button_shadow_color:           Some(RGBA((0, 0, 0, 50))),
        disk_button_shadow_x:               0.0,
        disk_button_shadow_y:               1.0,
        disk_button_shadow_blur:            3.0,
    ),


    // ================= CONTEXT MENU =================
    context_menu:
    (
        context_menu_background_color:          RGBA((30, 30, 30, 98)),
        context_menu_background_size:           6,
        context_menu_background_border_color:   HEX("3d3d3d"),
        context_menu_background_border_size:    1.0,
        context_menu_background_border_radius:  (10.0, 10.0, 10.0, 10.0),
        context_menu_text_size:                 14,
        context_menu_text_color:                HEX("ffffff"),
        context_menu_orientation:               Vertical,
        context_menu_size:                      280,
        context_menu_item_size:                 32,
        context_menu_button_color:              HEX("2a2a2a"),
        context_menu_button_hovered_color:      HEX("3d3d3d"),
        context_menu_button_hovered_text_color: HEX("ffffff"),
        context_menu_button_pressed_text_color: HEX("ffffff"),
        context_menu_button_pressed_color:      HEX("1c1c1c"),
        context_menu_border_color:              HEX("3d3d3d"),
        context_menu_border_size:               1.0,
        context_menu_border_radius:             (6.0, 6.0, 6.0, 6.0),
    ),


    // ================= FOCUSED WINDOW =================
    focused_window:
    (
        focused_window_format:                      "{title}",
        focused_window_update_interval:             500,
        dont_show_focused_window_if_empty:          true,
        text_when_focused_window_is_empty:          "Desktop",
        focused_window_text_limit_len:              30,
        focused_window_padding:                     7,
        focused_window_text_size:                   13,
        focused_window_text_color:                  HEX("c0bfbc"),
        focused_window_text_orientation:            Horizontal,
        focused_window_button_color:                HEX("2a2a2a"),
        focused_window_button_hovered_color:        HEX("3d3d3d"),
        focused_window_button_hovered_text_color:   HEX("ffffff"),
        focused_window_button_pressed_text_color:   HEX("ffffff"),
        focused_window_button_pressed_color:        HEX("1c1c1c"),
        focused_window_border_color:                HEX("2a2a2a"),
        focused_window_border_size:                 1.0,
        focused_window_border_radius:               (6.0, 6.0, 6.0, 6.0),
        focused_window_side_separator:              None,
        focused_window_side_separator_color:        HEX("3d3d3d"),
        focused_window_side_separator_width:        1.,
        focused_window_side_separator_height:       18.,
        focused_window_button_gradient_color:           None,
        focused_window_button_hovered_gradient_color:   None,
        focused_window_button_pressed_gradient_color:   None,
        focused_window_button_shadow_color:             None,
        focused_window_button_shadow_x:                 0.0,
        focused_window_button_shadow_y:                 0.0,
        focused_window_button_shadow_blur:              0.0,
    ),


    // ================= IMAGE =================
    //image:
    //(\n    //    images_spacing: 5,
    //    images:
    //        [
    //            (
    //                image_path: "path/to/your/gif",
    //                content_fit: Fill,
    //                message_image_missing: "Warning!!!: GIF Not Found.",
    //                side_separator: None,
    //                separator_color: RGB((75, 75, 75)),
    //                separator_width:  1.,
    //                separator_height: 16.,
    //                padding: 1,
    //                height: 30,
    //                width: 50,
    //                button_color: RGB((60, 50, 70)),
    //                button_hovered_color: RGB((110, 40, 80)),
    //                button_pressed_color: RGB((70, 20, 40)),
    //                border_color: RGB((45, 55, 100)),
    //                border_size: 1.0,
    //                border_radius: (3., 3., 3., 3.),
    //                command_to_exec_on_left_click: ["do", "a", "flip"],
    //                command_to_exec_on_right_click: ["kitty", "echo", "Meow"],
    //            ),
    //        ],
    //),


    // ================= CUSTOM MODULE =================
    custom_module:
    (
        custom_modules_spacing: 5,
        custom_modules:
        [
            // === Example of an button that just runs an app or command ===
            (
                side_separator:                 None,
                separator_color:                HEX("3d3d3d"),
                separator_width:                1.,
                separator_height:               18.,
                name:                           "App Launcher",
                text:                           "[Color=(120, 174, 237), String=󱗼]",
                text_size:                      15,
                text_color:                     HEX("ffffff"),
                text_orientation:               Horizontal,
                height:                         30,
                button_color:                   HEX("303030"),
                button_hovered_color:           HEX("3d3d3d"),
                button_hovered_text_color:      HEX("ffffff"),
                button_pressed_text_color:      HEX("ffffff"),
                button_pressed_color:           HEX("1c1c1c"),
                border_color:                   HEX("3d3d3d"),
                border_size:                    1.0,
                border_radius:                  (6.0, 6.0, 6.0, 6.0),
                button_gradient_color:          None,
                button_hovered_gradient_color:  None,
                button_pressed_gradient_color:  None,
                button_shadow_color:            Some(RGBA((0, 0, 0, 50))),
                button_shadow_x:                0.0,
                button_shadow_y:                1.0,
                button_shadow_blur:             3.0,
                display_err_output_if_failed:       false,
                dont_show_if_any_output_is_empty:   false,
                use_output_as_text:                 false,
                use_continous_output_as_text:       false,
                all_output_as_text_format:          "",
                output_text_limit_len:              0,
                command_to_exec_on_left_click:      ["wofi", "--show", "drun"],
                command_to_exec_on_right_click:     ["wofi", "--show", "run"],
                continous_command_interval:         500,
                continous_command:                  []
            ),
            // === Example of an button that displays the output on click ===
            //(
	    //		name: "print",
            //		text: "print output:",
            //		text_size: 15,
            //		height: 30,
            //		button_color: RGB((255, 40, 55)),
            //		button_hovered_color: RGB((150, 40, 80)),
            //		button_hovered_text_color: RGB((255, 255, 255)),
            //		button_pressed_text_color: RGB((255, 255, 255)),
            //		button_pressed_color: RGB((85, 30, 55)),
            //		border_color: RGBA((130, 90, 140, 100)),
            //		border_size: 1.0,
            //		border_radius: (3.0, 3.0, 3.0, 3.0),
            //		use_output_as_text: true,
            //		all_output_as_text_format: "{text} {output}",
            //		command_to_exec_on_left_click: ["echo", "YAAAYYY"],
            //		command_to_exec_on_right_click: ["echo", "IT'S WORKING!!!"],
            //),
            // === Example of an button that displays the continous output ===
            // 		name: "Playerctl Status - Artist - Media",
            //		text: "",
            //		text_size: 15,
            //		height: 30,
            //		button_color: RGB((255, 40, 55)),
            //		button_hovered_color: RGB((150, 40, 80)),
            //		button_hovered_text_color: RGB((255, 255, 255)),
            //		button_pressed_text_color: RGB((255, 255, 255)),
            //		button_pressed_color: RGB((85, 30, 55)),
            //		border_color: RGBA((130, 90, 140, 100)),
            //		border_size: 1.0,
            //		border_radius: (3.0, 3.0, 3.0, 3.0),
            //		display_err_output_if_failed: true,
            //		dont_show_if_any_output_is_empty: false,
            //		use_output_as_text: false,
            //		use_continous_output_as_text: true,
            //		all_output_as_text_format: "    {continous_output}",
            //      	output_text_limit_len: 50,
            //      	continous_command_interval: 500,
            //		continous_command: ["playerctl", "--player=spotify", "metadata", "--format", "{{ artist }} - {{ title }}"]
	    //), 
	],
    ),
)"#;
        let result_file = File::create(ron_file_config_path);
        match result_file
        {
            Ok(mut file) => 
            {
                if let Err(err) = file.write_all(ron_default_data.as_bytes())
                {
                    return Some(format!("Warning!!!: Couldn't create default config file, Err: {err}"));
                };
            },
            Err(err) => return Some(format!("Warning!!!: Couldn't create default config file, Err: {err}"))
        }
    };
    None
}
