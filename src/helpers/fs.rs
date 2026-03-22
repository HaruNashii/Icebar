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
        display: None,
        bar_position: Up,
        floating_space: 0,
        increased_exclusive_bar_zone: 0,
        bar_check_reload_interval_ms: Some(500),
        bar_side_spaces_size: 0,
        bar_size: (0, 35),
        bar_border_radius: (0., 0., 0., 0.),
        bar_border_size: 1.0,
        bar_border_color: RGB((90, 70, 100)),
        bar_background_color: RGBA((18, 18, 22, 100)),
        font_family: "JetBrains Mono",
        font_style: "Bold",
        spacing_between_all_modules: 5,
        force_static_position_context_menu: None,
        left_modules: [CustomModule(0), Cpu, CpuTemp, Ram],
        center_modules: [Clock],
        right_modules: [Tray, Network, VolumeOutput, VolumeInput],
    ),


    // ================= GENERAL STYLE =================
    general_style:
    (
        general_padding: Some(0),
        general_text_size: Some(15),
        general_text_orientation: Some(Horizontal),
        general_text_color: Some(HEX("FFFFFF")),
        general_button_color: Some(HEX("322d3c")),
        general_button_hovered_color: Some(RGB((130, 35, 70))),
        general_button_hovered_text_color: Some(RGB((255, 255, 255))),
        general_button_pressed_text_color: Some(RGB((255, 255, 255))),
        general_button_pressed_color: Some(RGB((80, 25, 45))),
        general_button_gradient_color: None,
        general_button_pressed_gradient_color: None,
        general_button_hovered_gradient_color: None,
        general_border_color: Some(RGB((120, 80, 130))),
        general_border_size: Some(1.0),
        general_border_radius: Some((3.0, 3.0, 3.0, 3.0)),
        general_side_separator: None,
        general_side_separator_color: Some(RGB((75, 75, 75))),
        general_side_separator_width: Some(1.0),
        general_side_separator_height: Some(16.0),
        general_alt_padding: None,
        general_alt_text_size: None,
        general_alt_text_orientation: None,
        general_alt_text_color: None,
        general_alt_button_color: None,
        general_alt_button_hovered_color: None,
        general_alt_button_hovered_text_color: None,
        general_alt_button_pressed_text_color: None,
        general_alt_button_pressed_color: None,
        general_alt_border_color: None,
        general_alt_border_size: None,
        general_alt_border_radius: None,
        general_alt_side_separator: None,
        general_alt_side_separator_color: None,
        general_alt_side_separator_width: None,
        general_alt_side_separator_height: None,
        general_alt_button_gradient_color: Some ( Gradient (( 180.0, [ (0.0, RGB((175, 75, 75))), (1.0, RGB((0, 0, 0))), ] )) ),
        general_alt_button_pressed_gradient_color: None,
        general_alt_button_hovered_gradient_color: None,
    ),


    // ================= CLOCK =================
    clock:
    (
        clock_timezones: None,
        clock_update_interval: 400,
        clock_format: "[Color=(150, 40, 80), String=󰥔]  %H:%M",
        clock_alt_format: "󰃭  %a %d %b |  󰥔  %H:%M:%S",
        action_on_left_click_clock: DefaultAction,
        action_on_right_click_clock: CustomAction(["kitty", "bash", "-c", "cal && echo 'Press Enter To Exit' && read -n 1"]),
        clock_side_separator: None,
        clock_side_separator_color: RGB((75, 75, 75)),
        clock_side_separator_width: 1.,
        clock_side_separator_height: 16.,
        alt_clock_side_separator: None,
        alt_clock_side_separator_color: RGB((75, 75, 75)),
        alt_clock_side_separator_width: 1.,
        alt_clock_side_separator_height: 16.,
        clock_button_hovered_gradient_color: None,
        clock_button_pressed_gradient_color: None,
        clock_button_gradient_color: None,
        alt_clock_button_gradient_color: None,
        alt_clock_button_hovered_gradient_color: None,
        alt_clock_button_pressed_gradient_color: None,
        clock_padding: 0,
        clock_text_size: 15,
        clock_text_color: RGB((255, 255, 255)),
        clock_text_orientation: Horizontal,
        clock_button_color: RGB((50, 45, 60)),
        clock_button_hovered_color: RGB((130, 35, 70)),
        clock_button_hovered_text_color: RGB((255, 255, 255)),
        clock_button_pressed_text_color: RGB((255, 255, 255)),
        clock_button_pressed_color: RGB((80, 25, 45)),
        clock_border_color: RGB((120, 80, 130)),
        clock_border_size: 1.0,
        clock_border_radius: (3.0, 3.0, 3.0, 3.0),
        alt_clock_padding: 0,
        alt_clock_text_size: 15,
        alt_clock_text_color: RGB((255, 255, 255)),
        alt_clock_text_orientation: Horizontal,
        alt_clock_button_color: RGB((150, 40, 80)),
        alt_clock_button_hovered_color: RGB((130, 35, 70)),
        alt_clock_button_hovered_text_color: RGB((255, 255, 255)),
        alt_clock_button_pressed_text_color: RGB((255, 255, 255)),
        alt_clock_button_pressed_color: RGB((80, 25, 45)),
        alt_clock_border_color: RGB((120, 80, 130)),
        alt_clock_border_size: 1.0,
        alt_clock_border_radius: (3.0, 3.0, 3.0, 3.0),
    ),


    // ================= VOLUME OUTPUT =================
    volume_output:
    (
        output_volume_format:
        (
            "[Color=(150, 40, 80), String=]   {}%",
            "[Color=(150, 40, 80), String=󰖀]   {}%",
            "[Color=(150, 40, 80), String=]   {}%",
            "[Color=(150, 40, 80), String=]   {}%",
            "[Color=(150, 40, 80), String=]   {}%",
            "[Color=(150, 40, 80), String= +] {}%"
        ),
        output_volume_muted_format: "  Muted",
        incremental_steps_output: 10,
        action_on_left_click_volume_output: DefaultAction,
        action_on_right_click_volume_output: CustomAction(["kitty", "pulsemixer"]),
        volume_output_padding: 0,
        volume_output_text_size: 15,
        volume_output_text_color: RGB((255, 255, 255)),
        volume_output_text_orientation: Horizontal,
        volume_output_button_color: RGB((55, 45, 65)),
        volume_output_button_hovered_color: RGB((150, 45, 85)),
        volume_output_button_hovered_text_color: RGB((255, 255, 255)),
        volume_output_button_pressed_text_color: RGB((255, 255, 255)),
        volume_output_button_pressed_color: RGB((85, 30, 50)),
        volume_output_border_color: RGB((110, 80, 120)),
        volume_output_border_size: 1.0,
        volume_output_border_radius: (3.0, 3.0, 3.0, 3.0),
        volume_output_side_separator: None,
        volume_output_side_separator_color: RGB((75, 75, 75)),
        volume_output_side_separator_width: 1.,
        volume_output_side_separator_height: 20.,
        volume_output_button_gradient_color: None,
        volume_output_button_hovered_gradient_color: None,
        volume_output_button_pressed_gradient_color: None,
    ),


    // ================= MUTED VOLUME OUTPUT =================
    muted_volume_output:
    (
        muted_volume_output_padding: 0,
        muted_volume_output_text_size: 15,
        muted_volume_output_text_color: RGB((255, 255, 255)),
        muted_volume_output_text_orientation: Horizontal,
        muted_volume_output_button_color: RGB((150, 40, 80)),
        muted_volume_output_button_hovered_color: RGB((150, 45, 85)),
        muted_volume_output_button_hovered_text_color: RGB((255, 255, 255)),
        muted_volume_output_button_pressed_text_color: RGB((255, 255, 255)),
        muted_volume_output_button_pressed_color: RGB((85, 30, 50)),
        muted_volume_output_border_color: RGB((110, 80, 120)),
        muted_volume_output_border_size: 1.0,
        muted_volume_output_border_radius: (3.0, 3.0, 3.0, 3.0),
        muted_volume_output_side_separator: None,
        muted_volume_output_side_separator_color: RGB((75, 75, 75)),
        muted_volume_output_side_separator_width: 1.,
        muted_volume_output_side_separator_height: 20.,
        muted_volume_output_button_gradient_color: None,
        muted_volume_output_button_hovered_gradient_color: None,
        muted_volume_output_button_pressed_gradient_color: None,
    ),


    // ================= VOLUME INPUT =================
    volume_input:
    (
        input_volume_format:
        (
            "[Color=(150, 40, 80), String=]  {}%",
            "[Color=(150, 40, 80), String=]  {}%",
            "[Color=(150, 40, 80), String=]  {}%",
            "[Color=(150, 40, 80), String=]  {}%",
            "[Color=(150, 40, 80), String=]  {}%",
            "[Color=(150, 40, 80), String=󰢴]  {}%"
        ),
        input_volume_muted_format: "  Muted",
        incremental_steps_input: 10,
        action_on_left_click_volume_input: DefaultAction,
        action_on_right_click_volume_input: CustomAction(["kitty", "pulsemixer"]),
        volume_input_padding: 0,
        volume_input_text_size: 15,
        volume_input_text_color: RGB((255, 255, 255)),
        volume_input_text_orientation: Horizontal,
        volume_input_button_color: RGB((55, 45, 65)),
        volume_input_button_hovered_color: RGB((150, 45, 85)),
        volume_input_button_hovered_text_color: RGB((255, 255, 255)),
        volume_input_button_pressed_text_color: RGB((255, 255, 255)),
        volume_input_button_pressed_color: RGB((85, 30, 50)),
        volume_input_border_color: RGB((110, 80, 120)),
        volume_input_border_size: 1.0,
        volume_input_border_radius: (3.0, 3.0, 3.0, 3.0),
        volume_input_side_separator: None,
        volume_input_side_separator_color: RGB((75, 75, 75)),
        volume_input_side_separator_width: 1.,
        volume_input_side_separator_height: 20.,
        volume_input_button_gradient_color: None,
        volume_input_button_hovered_gradient_color: None,
        volume_input_button_pressed_gradient_color: None,
    ),


    // ================= MUTED VOLUME INPUT =================
    muted_volume_input:
    (
        muted_volume_input_padding: 0,
        muted_volume_input_text_size: 15,
        muted_volume_input_text_color: RGB((255, 255, 255)),
        muted_volume_input_text_orientation: Horizontal,
        muted_volume_input_button_color: RGB((150, 40, 80)),
        muted_volume_input_button_hovered_color: RGB((150, 45, 85)),
        muted_volume_input_button_hovered_text_color: RGB((255, 255, 255)),
        muted_volume_input_button_pressed_text_color: RGB((255, 255, 255)),
        muted_volume_input_button_pressed_color: RGB((85, 30, 50)),
        muted_volume_input_border_color: RGB((110, 80, 120)),
        muted_volume_input_border_size: 1.0,
        muted_volume_input_border_radius: (3.0, 3.0, 3.0, 3.0),
        muted_volume_input_side_separator: None,
        muted_volume_input_side_separator_color: RGB((75, 75, 75)),
        muted_volume_input_side_separator_width: 1.,
        muted_volume_input_side_separator_height: 20.,
        muted_volume_input_button_gradient_color: None,
        muted_volume_input_button_hovered_gradient_color: None,
        muted_volume_input_button_pressed_gradient_color: None,
    ),


    // ================= NETWORK =================
    network:
    (
        network_module_format: "{level}[Tuning=4]",
        network_disconnected_text: "No Connection Found.",
        network_level_format:
        (
            "[Color=(150, 40, 80), String=󰖩]",
            "[Color=(150, 40, 80), String=󱚵]",
            "[Color=(150, 40, 80), String=󱚼]",
            "[Color=(150, 40, 80), String=󰖪]"
        ),
        network_connection_type_icons:
        (
            "[Color=(150, 40, 80), String=󰈀]",
            "[Color=(150, 40, 80), String=]",
            "[Color=(150, 40, 80), String=?]"
        ),
        action_on_left_click_network: DefaultAction,
        action_on_right_click_network: DefaultAction,
        network_padding: 0,
        network_text_size: 15,
        network_text_color: RGB((255, 255, 255)),
        network_text_orientation: Horizontal,
        network_button_color: RGB((50, 45, 60)),
        network_button_hovered_color: RGB((130, 35, 70)),
        network_button_hovered_text_color: RGB((255, 255, 255)),
        network_button_pressed_text_color: RGB((255, 255, 255)),
        network_button_pressed_color: RGB((80, 25, 45)),
        network_border_color: RGB((120, 80, 130)),
        network_border_size: 1.0,
        network_border_radius: (3.0, 3.0, 3.0, 3.0),
        network_side_separator: None,
        network_side_separator_color: RGB((75, 75, 75)),
        network_side_separator_width: 1.,
        network_side_separator_height: 16.,
        network_button_gradient_color: None,
        network_button_hovered_gradient_color: None,
        network_button_pressed_gradient_color: None,
    ),


    // ================= ALT NETWORK =================
    alt_network:
    (
        alt_network_module_format: "{level} | {connection_type} | [Color=(0, 255, 255), String={id}]  | [Color=(0, 255, 0), String={speed}MB/s] | [Color=(0, 0, 255), String={sent}KB/s] | [Color=(0, 0, 255), String={received}KB/s]",
        alt_network_level_format: ( "󰖩", "󱚵", "󱚼", "󰖪" ),
        alt_network_connection_type_icons: ( "󰈀", "", "?" ),
        alt_network_padding: 0,
        alt_network_text_size: 15,
        alt_network_text_color: RGB((255, 255, 255)),
        alt_network_text_orientation: Horizontal,
        alt_network_button_color: RGB((150, 40, 80)),
        alt_network_button_hovered_color: RGB((130, 35, 70)),
        alt_network_button_hovered_text_color: RGB((255, 255, 255)),
        alt_network_button_pressed_text_color: RGB((255, 255, 255)),
        alt_network_button_pressed_color: RGB((80, 25, 45)),
        alt_network_border_color: RGB((120, 80, 130)),
        alt_network_border_size: 1.0,
        alt_network_border_radius: (3.0, 3.0, 3.0, 3.0),
        alt_network_side_separator: None,
        alt_network_side_separator_color: RGB((75, 75, 75)),
        alt_network_side_separator_width: 1.,
        alt_network_side_separator_height: 16.,
        alt_network_button_gradient_color: None,
        alt_network_button_hovered_gradient_color: None,
        alt_network_button_pressed_gradient_color: None,
    ),


    // ================= WORKSPACE =================
    workspace:
    (
        niri_workspaces_update_interval: 225,
        reverse_scroll_on_workspace: false,
        persistent_workspaces: Some(5),
        workspace_height: 30,
        workspace_width: 30,
        workspace_different_selected_width: None,
        workspace_different_selected_height: None,
        workspace_text_size: 15,
        workspace_text_color: RGB((255, 255, 255)),
        workspace_selected_text_color: RGB((255, 255, 255)),
        workspace_text_orientation: Horizontal,
        workspace_text: [ "1", "2", "3", "4", "5", "6", "7", "8", "9", "10" ],
        workspace_selected_text: Some( [ "●", "●", "●", "●", "●", "●", "●", "●", "●", "●" ]),
        workspace_spacing: 3,
        workspace_padding: 0,
        workspace_button_color: RGB((45, 40, 55)),
        workspace_button_selected_color: RGB((150, 40, 80)),
        workspace_button_hovered_color: RGB((140, 35, 75)),
        workspace_button_hovered_text_color: RGB((255, 255, 255)),
        workspace_button_pressed_text_color: RGB((255, 255, 255)),
        workspace_button_pressed_color: RGB((90, 25, 50)),
        workspace_border_color: RGB((120, 90, 135)),
        workspace_border_size: 1.0,
        workspace_border_radius: (3.0, 3.0, 3.0, 3.0),
        workspace_side_separator: None,
        workspace_side_separator_color: RGB((75, 75, 75)),
        workspace_side_separator_width: 1.,
        workspace_side_separator_height: 16.,
        workspace_button_gradient_color: None,
        workspace_button_selected_gradient_color: None,
        workspace_button_hovered_gradient_color: None,
        workspace_button_pressed_gradient_color: None,
    ),


    // ================= TRAY =================
    tray:
    (
        tray_icon_size: 20,
        tray_spacing: 5,
        tray_button_size: 5,
        tray_button_color: RGB((60, 50, 70)),
        tray_button_hovered_color: RGB((110, 40, 80)),
        tray_button_hovered_text_color: RGB((255, 255, 255)),
        tray_button_pressed_text_color: RGB((255, 255, 255)),
        tray_button_pressed_color: RGB((70, 20, 40)),
        tray_border_color: RGB((90, 70, 100)),
        tray_border_size: 1.0,
        tray_border_radius: (3.0, 3.0, 3.0, 3.0),
        tray_side_separator: None,
        tray_side_separator_color: RGB((75, 75, 75)),
        tray_side_separator_width: 1.,
        tray_side_separator_height: 16.,
        tray_button_gradient_color: None,
        tray_button_hovered_gradient_color: None,
        tray_button_pressed_gradient_color: None,
    ),


    // ================= MEDIA PLAYER METADATA =================
    media_player_metadata:
    (
        player: "spotify",
        media_player_metadata_format: "{{artist}} | {{album}} | {{title}}",
        media_player_metadata_update_interval: 750,
        dont_show_metadata_if_empty: false,
        text_when_metadata_is_empty: "No Media Found.",
        media_player_metadata_text_limit_len: 25,
        ellipsis_text: "...",
        action_on_left_click_media_player_metadata: Nothing,
        action_on_right_click_media_player_metadata: Nothing,
        media_player_metadata_padding: 0,
        media_player_metadata_text_size: 15,
        media_player_metadata_text_color: RGB((255, 255, 255)),
        media_player_metadata_text_orientation: Horizontal,
        media_player_metadata_button_color: RGB((50, 45, 60)),
        media_player_metadata_button_hovered_color: RGB((130, 35, 70)),
        media_player_metadata_button_hovered_text_color: RGB((255, 255, 255)),
        media_player_metadata_button_pressed_text_color: RGB((255, 255, 255)),
        media_player_metadata_button_pressed_color: RGB((80, 25, 45)),
        media_player_metadata_border_color: RGB((120, 80, 130)),
        media_player_metadata_border_size: 1.0,
        media_player_metadata_border_radius: (3.0, 3.0, 3.0, 3.0),
        media_player_metadata_side_separator: None,
        media_player_metadata_side_separator_color: RGB((75, 75, 75)),
        media_player_metadata_side_separator_width: 1.,
        media_player_metadata_side_separator_height: 16.,
        media_player_metadata_button_gradient_color: None,
        media_player_metadata_button_hovered_gradient_color: None,
        media_player_metadata_button_pressed_gradient_color: None,
    ),


    // ================= MEDIA PLAYER BUTTONS =================
    media_player_button:
    (
        media_player_buttons_format: ("󰒮", "⏸", "▶", "󰒭"),
        media_player_button_spacing: 5,
        media_player_button_padding: 0,
        media_player_button_text_size: 15,
        media_player_button_text_color: RGB((50, 45, 60)),
        media_player_button_text_orientation: Horizontal,
        media_player_button_color: RGB((50, 45, 60)),
        media_player_button_hovered_color: RGB((130, 35, 70)),
        media_player_button_hovered_text_color: RGB((255, 255, 255)),
        media_player_button_pressed_text_color: RGB((255, 255, 255)),
        media_player_button_pressed_color: RGB((80, 25, 45)),
        media_player_button_border_color: RGB((120, 80, 130)),
        media_player_button_border_size: 1.0,
        media_player_button_border_radius: (3.0, 3.0, 3.0, 3.0),
        media_player_buttons_side_separator: None,
        media_player_buttons_side_separator_color: RGB((75, 75, 75)),
        media_player_buttons_side_separator_width: 1.,
        media_player_buttons_side_separator_height: 16.,
        media_player_button_gradient_color: None,
        media_player_button_hovered_gradient_color: None,
        media_player_button_pressed_gradient_color: None,
    ),


    // ================= CPU =================
    cpu:
    (
        cpu_format: "[Color=(150, 40, 80), String=CPU:] {usage}%",
        cpu_update_interval: 1050,
        action_on_left_click_cpu: DefaultAction,
        action_on_right_click_cpu: DefaultAction,
        cpu_padding: 0,
        cpu_text_size: 12,
        cpu_text_color: RGB((220, 220, 220)),
        cpu_text_orientation: Horizontal,
        cpu_button_color: RGB((40, 40, 50)),
        cpu_button_hovered_color: RGB((60, 60, 75)),
        cpu_button_hovered_text_color: RGB((255, 255, 255)),
        cpu_button_pressed_text_color: RGB((255, 255, 255)),
        cpu_button_pressed_color: RGB((30, 30, 40)),
        cpu_border_color: RGB((80, 80, 100)),
        cpu_border_size: 1.0,
        cpu_border_radius: (3.0, 3.0, 3.0, 3.0),
        cpu_side_separator: None,
        cpu_side_separator_color: RGB((75, 75, 75)),
        cpu_side_separator_width: 1.,
        cpu_side_separator_height: 16.,
        cpu_button_gradient_color: None,
        cpu_button_hovered_gradient_color: None,
        cpu_button_pressed_gradient_color: None,
    ),


    // ================= CPU TEMP =================
    cpu_temp:
    (
        cpu_temp_format: "[Color=(150, 40, 80), String=CPU Temp:] {temp}°C",
        cpu_temp_update_interval: 1050,
        action_on_left_click_cpu_temp: DefaultAction,
        action_on_right_click_cpu_temp: DefaultAction,
        cpu_temp_padding: 0,
        cpu_temp_text_size: 12,
        cpu_temp_text_color: RGB((220, 220, 220)),
        cpu_temp_text_orientation: Horizontal,
        cpu_temp_button_color: RGB((40, 40, 50)),
        cpu_temp_button_hovered_color: RGB((60, 60, 75)),
        cpu_temp_button_hovered_text_color: RGB((255, 255, 255)),
        cpu_temp_button_pressed_text_color: RGB((255, 255, 255)),
        cpu_temp_button_pressed_color: RGB((30, 30, 40)),
        cpu_temp_border_color: RGB((80, 80, 100)),
        cpu_temp_border_size: 1.0,
        cpu_temp_border_radius: (3.0, 3.0, 3.0, 3.0),
        cpu_temp_side_separator: None,
        cpu_temp_side_separator_color: RGB((75, 75, 75)),
        cpu_temp_side_separator_width: 1.,
        cpu_temp_side_separator_height: 16.,
        cpu_temp_button_gradient_color: None,
        cpu_temp_button_hovered_gradient_color: None,
        cpu_temp_button_pressed_gradient_color: None,
    ),


    // ================= RAM =================
    ram:
    (
        ram_format: "[Color=(150, 40, 80), String=Ram Used:] {used}MB / {percent}% | [Color=(150, 40, 80), String=Ram Total:] {total}MB",
        ram_update_interval: 1050,
        ram_padding: 0,
        ram_text_size: 12,
        ram_text_color: RGB((220, 220, 220)),
        ram_text_orientation: Horizontal,
        ram_button_color: RGB((40, 40, 50)),
        ram_button_hovered_color: RGB((60, 60, 75)),
        ram_button_hovered_text_color: RGB((255, 255, 255)),
        ram_button_pressed_text_color: RGB((255, 255, 255)),
        ram_button_pressed_color: RGB((30, 30, 40)),
        ram_border_color: RGB((80, 80, 100)),
        ram_border_size: 1.0,
        ram_border_radius: (3.0, 3.0, 3.0, 3.0),
        ram_side_separator: None,
        ram_side_separator_color: RGB((75, 75, 75)),
        ram_side_separator_width: 1.,
        ram_side_separator_height: 16.,
        ram_button_gradient_color: None,
        ram_button_hovered_gradient_color: None,
        ram_button_pressed_gradient_color: None,
    ),


    // ================= DISK =================
    disk:
    (
        disk_format: "[Color=(150, 40, 80), String=Disk Used:] {used}GB / {percent}% | [Color=(150, 40, 80), String=Disk Total:] {total}GB",
        disk_mount: "/",
        disk_update_interval: 10000,
        disk_padding: 0,
        disk_text_size: 12,
        disk_text_color: RGB((220, 220, 220)),
        disk_text_orientation: Horizontal,
        disk_button_color: RGB((40, 40, 50)),
        disk_button_hovered_color: RGB((60, 60, 75)),
        disk_button_hovered_text_color: RGB((255, 255, 255)),
        disk_button_pressed_text_color: RGB((255, 255, 255)),
        disk_button_pressed_color: RGB((30, 30, 40)),
        disk_border_color: RGB((80, 80, 100)),
        disk_border_size: 1.0,
        disk_border_radius: (3.0, 3.0, 3.0, 3.0),
        disk_side_separator: None,
        disk_side_separator_color: RGB((75, 75, 75)),
        disk_side_separator_width: 1.,
        disk_side_separator_height: 20.,
        disk_button_gradient_color: None,
        disk_button_hovered_gradient_color: None,
        disk_button_pressed_gradient_color: None,
    ),


    // ================= CONTEXT MENU =================
    context_menu:
    (
        context_menu_background_color: RGBA((20, 20, 24, 98)),
        context_menu_background_size: 5,
        context_menu_background_border_color: RGB((255, 255, 255)),
        context_menu_background_border_size: 1.0,
        context_menu_background_border_radius: (3.0, 3.0, 3.0, 3.0),
        context_menu_text_size: 15,
        context_menu_text_color: RGB((255, 255, 255)),
        context_menu_orientation: Vertical,
        context_menu_size: 300,
        context_menu_item_size: 30,
        context_menu_button_color: RGB((45, 40, 55)),
        context_menu_button_hovered_color: RGB((150, 40, 80)),
        context_menu_button_hovered_text_color: RGB((255, 255, 255)),
        context_menu_button_pressed_text_color: RGB((255, 255, 255)),
        context_menu_button_pressed_color: RGB((85, 30, 55)),
        context_menu_border_color: RGB((130, 90, 140)),
        context_menu_border_size: 1.0,
        context_menu_border_radius: (3.0, 3.0, 3.0, 3.0),
    ),


    // ================= FOCUSED WINDOW =================
    focused_window:
    (
        focused_window_format: "{title}",
        focused_window_update_interval: 500,
        dont_show_focused_window_if_empty: true,
        text_when_focused_window_is_empty: "No Window Focused",
        focused_window_text_limit_len: 25,
        focused_window_padding: 0,
        focused_window_text_size: 12,
        focused_window_text_color: RGB((220, 220, 220)),
        focused_window_text_orientation: Horizontal,
        focused_window_button_color: RGB((40, 40, 50)),
        focused_window_button_hovered_color: RGB((60, 60, 75)),
        focused_window_button_hovered_text_color: RGB((255, 255, 255)),
        focused_window_button_pressed_text_color: RGB((255, 255, 255)),
        focused_window_button_pressed_color: RGB((30, 30, 40)),
        focused_window_border_color: RGB((80, 80, 100)),
        focused_window_border_size: 1.0,
        focused_window_border_radius: (3.0, 3.0, 3.0, 3.0),
        focused_window_side_separator: None,
        focused_window_side_separator_color: RGB((75, 75, 75)),
        focused_window_side_separator_width: 1.,
        focused_window_side_separator_height: 16.,
        focused_window_button_gradient_color: None,
        focused_window_button_hovered_gradient_color: None,
        focused_window_button_pressed_gradient_color: None,
    ),


    // ================= IMAGE =================
    //image:
    //(
    //    images_spacing: 5,
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
    //            (
    //                image_path: "/path/to/your/image.png",
    //                content_fit: Fill,
    //                message_image_missing: "Warning!!!: Image Not Found.",
    //                side_separator: None,
    //                separator_color: RGB((75, 75, 75)),
    //                separator_width:  1.,
    //                separator_height: 16.,
    //                padding: 1,
    //                height: 30,
    //                width: 50,
    //                button_color: RGBA((60, 50, 70, 0)),
    //                button_hovered_color: RGB((110, 40, 80)),
    //                button_pressed_color: RGB((70, 20, 40)),
    //                border_color: RGB((90, 70, 100)),
    //                border_size: 0.0,
    //                border_radius: (3., 3., 3., 3.),
    //                command_to_exec_on_left_click: ["kitty", "pulsemixer"],
    //                command_to_exec_on_right_click: ["kitty", "pulsemixer"],
    //            )
    //        ],
    //),


    // ================= CUSTOM MODULE =================
    custom_module:
    (
        custom_modules_spacing: 10,
        custom_modules:
        [
            //Example of an button that just runs an app or command
            (
                side_separator: None,
                separator_color: RGB((75, 75, 75)),
                separator_width: 1.,
                separator_height: 16.,
                name: "Wofi Custom Module",
                text: " [Color=(150, 40, 80), String=󰣇][Tuning=4]",
                text_size: 15,
                text_color: RGB((255, 255, 255)),
                text_orientation: Horizontal,
                height: 30,
                button_color: RGB((45, 40, 55)),
                button_hovered_color: RGB((150, 40, 80)),
                button_hovered_text_color: RGB((255, 255, 255)),
                button_pressed_text_color: RGB((255, 255, 255)),
                button_pressed_color: RGB((85, 30, 55)),
                border_color: RGBA((130, 90, 140, 100)),
                border_size: 1.0,
                border_radius: (3.0, 3.0, 3.0, 3.0),
                display_err_output_if_failed: false,
                dont_show_if_any_output_is_empty: false,
                use_output_as_text: false,
                use_continous_output_as_text: false,
                all_output_as_text_format: "",
                output_text_limit_len: 0,
                command_to_exec_on_left_click: ["wofi", "--show", "drun"],
                command_to_exec_on_right_click: ["wofi", "--show", "run"],
                continous_command_interval: 500,
                continous_command: []
            ),
            // Example of an button that displays the output on click
            //(
                //	name: "print",
                //	text: "print output:",
                //	text_size: 15,
                //	height: 30,
                //	button_color: RGB((255, 40, 55)),
                //	button_hovered_color: RGB((150, 40, 80)),
                //	button_hovered_text_color: RGB((255, 255, 255)),
                //	button_pressed_text_color: RGB((255, 255, 255)),
                //	button_pressed_color: RGB((85, 30, 55)),
                //	border_color: RGBA((130, 90, 140, 100)),
                //	border_size: 1.0,
                //	border_radius: (3.0, 3.0, 3.0, 3.0),
                //	use_output_as_text: true,
                //	all_output_as_text_format: "{text} {output}",
                //	command_to_exec_on_left_click: ["echo", "YAAAYYY"],
                //	command_to_exec_on_right_click: ["echo", "IT'S WORKING!!!"],
                //),
            // Example of an button that displays the continous output
            //(
                //	name: "Playerctl Status - Artist - Media",
                //	text: "",
                //	text_size: 15,
                //	height: 30,
                //	button_color: RGB((255, 40, 55)),
                //	button_hovered_color: RGB((150, 40, 80)),
                //	button_hovered_text_color: RGB((255, 255, 255)),
                //	button_pressed_text_color: RGB((255, 255, 255)),
                //	button_pressed_color: RGB((85, 30, 55)),
                //	border_color: RGBA((130, 90, 140, 100)),
                //	border_size: 1.0,
                //	border_radius: (3.0, 3.0, 3.0, 3.0),
                //	display_err_output_if_failed: true,
                //      dont_show_if_any_output_is_empty: false,
                //	use_output_as_text: false,
                //	use_continous_output_as_text: true,
                //	all_output_as_text_format: "    {continous_output}",
                //      output_text_limit_len: 50,
                //      continous_command_interval: 500,
                //	continous_command: ["playerctl", "--player=spotify", "metadata", "--format", "{{ artist }} - {{ title }}"]
                //),
        ],
    ),
)
"#;
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
