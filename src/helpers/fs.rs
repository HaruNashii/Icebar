// ============ IMPORTS ============
use std::{io::Write, fs, fs::File, path::Path};





// ============ FUNCTIONS ============
pub fn check_if_config_file_exists()
{
    println!("\n=== FS CHECK RUNNING... ===");
    let home_path = home::home_dir().expect("Failed To Get Home Directory").display().to_string();
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
        fs::create_dir_all(ron_config_path).expect("Couldn't Create Ron Config Directory");
    };

    if Path::exists(ron_file_config_path)
    {
        println!("Ron Config File Exists!!!");
    }
    else
    {
        println!("Ron config file doesn't exist, Creating...");
        let ron_default_data = r#"// This File Is Auto-Generated When Icebar Detects That The Config File Or Config Directory Doesn't Exists.

// ===== WARNINGS =====
// WARNING!!!: THE ALPHA OF THE RGBA HAS THE RANGE BETWEEN 0 TO 100, PARSING MORE THAN 100 WILL RESULT IN CRASH
// WARNING!!!: "bar_size" FIRST OPTION DEFINED TO "0" WILL MAKE THE BAR FILL THE ENTIRE SCREEN X AXIS
// WARNING!!!: IF THE NUMBERS OF WORKSPACE IS GREATER THAN THE PARSED "hypr_workspace_text:" AND "hypr_workspace_selected_text:", THE NON-PARSED FORMAT WORKSPACE WILL HAVE THE NUMBER OF THE DETERMINED WORKSPACE
// WARNING!!!: IS VERY IMPORTANT TO SET THE DISPLAY VARIABLE, NOT SETING IT UP MAY CAUSE UNDEFINED BEHAVIOUR
// WARNING!!!: MISSING OPTIONS IS FINE AND WILL HAVE FALLBACK TO THE DEFAULT CONFIG, BUT MISSED SYNTAX WILL RESULT IN CRASH!!!
// WARNING!!!: THE FIELD: "continous_command" MAY GENERATA HIGH CPU USAGE, DEPENDING ON HOW HEAVY IS THE COMMAND YOU PARSED
// WARNING!!!: THE FIELD: "continous_command" MAY NOT BE SET TO RUN AN LOOP OF ANY KIND, THE PROCESS WILL HANG FOREVER IF YOU RUN AN LOOP WITH IT
// WARNING!!!: THE OPTION "bar_size: ()" HAS THE TYPE AS: (WIDTH, HEIGTH), IF YOU ARE CREATING AN SIDE BAR, THE VALUE OF 0 IN THE FIRST OPTION IS NOT VALID, AND WILL RESULT IN CRASH OR THE APP HANGING!!!
// WARNING!!!: "NiriWorkspaces" DOESN'T SUPPORT "persistent_workspaces", IF YOU PARSED IT, IT WILL BE IGNORED


     
// ===== TIPS =====
// All possible modules: "FocusedWindowSway", "FocusedWindowHypr", "FocusedWindowNiri", "CpuTemp", "Ram", "Cpu", "MediaPlayerMetaData", "MediaPlayerButtons", "NiriWorkspaces", "HyprWorkspaces", "SwayWorkspaces", "CustomModule(index)", "VolumeOutput", "VolumeInput", "Network", "Clock", "Tray".
//
// All texts supports per color flags per string, you just need to have "[Color=(Red, Green, Blue), String=YourString]", you can have multiples color flags for text 
// like: "[Color=(255, 0, 0), String=red_string], non_colored_String [Color=(0, 0, 255), String=blue_string]", this will display "red_string non_colored_String, blue_string", all with your respective colors
//
// Volume (output and input) format steps have an incremental of 25%, like this: "0%", 25%, 50%, 75%, 100%, > 100+%.
//
// If "clock_timezones" is not setted or is explicited setted to "None", your clock will use your local timezone 
//
// Available options for "bar_position" are: "Up", "Down", "Left" and "Right" 
//
// To configure diffents texts for diferents orientations, how can set the variables "text_orientation:" on any module (excluding "Tray"), with the values:
//Vertical:
//|A|
//|B|
//|C|
//
//Horizontal:
//|A|B|C|
//
//
// To see the correct "font_family" and "font_style" i recommend using "fc-scan $PATH_TO_FONT_FILE".
//
// every custom module you make will be assigned an index based on the position they are, from top to bottom, the first = 0, the second = 1
// so for parsing your custom_module to the position just put on the position modules your "custom_module[index]"
//
// if "bar_check_reload_interval_ms" is set to "None" the bar will not hot-reload, 
// it may make the bar lighter if it's turned off, so if you don't pretend to use this feature is reccommended to turn it off.
//
// The unique syntax for each some modules are: "display" = Some("HDMI-A-1"), "force_static_position_context_menu" = Some((x, y)) and "persistent_workspaces" = Some(number_of_persistent_elements)
//
// This file have all the currently available options, if you are not finding an option maybe is
// not implemented yet, make sure to make me know.
//
// If you notice some bug or want more features, please feel free to publish your thoughs on: https://github.com/HaruNashii/Icebar.git
// Or if you want talk directly to me to clear up any questions, my discord id is: harunashiii
// you can also join my contact server with: https://discord.gg/CRsz24Ts3a



BarConfig
(
    // ================= GENERAL =================
    display: None,
    bar_position: Up,
    floating_space: 0,
    increased_exclusive_bar_zone: 0,
    bar_check_reload_interval_ms: Some(500),
    bar_side_spaces_size: 0,
    bar_size: (0, 35),
    bar_border_radius: (0., 0., 0., 0.),
    bar_border_size: 1.0,
    bar_border_color_rgb: (90, 70, 100),
    bar_background_color_rgba: (18, 18, 22, 100),
    font_family: "JetBrains Mono",
    font_style: "Bold",


    // ================= MODULES =================
    left_modules: [CustomModule(0), Cpu, CpuTemp, Ram],
    center_modules: [Clock],
    right_modules: [Tray, Network, VolumeOutput, VolumeInput],


    // ================= MODULES CONFIGS =================
    clock_timezones: None,
    //clock_timezones: Some(["America/New_York", "Europe/London", "Asia/Tokyo", "America/Sao_Paulo"]),
    ellipsis_text: "...",
    player: "spotify",
    dont_show_metadata_if_empty: false,
    dont_show_focused_window_if_empty: true,
    text_when_metadata_is_empty: "No Media Found.",
    text_when_focused_window_is_empty: "No Window Focused",
    media_player_metadata_text_limit_len: 25,
    focused_window_text_limit_len: 25,
    spacing_between_all_modules: 5,
    force_static_position_context_menu: None,
    reverse_scroll_on_workspace: false,
    persistent_workspaces: Some(5),
    incremental_steps_output: 10,
    incremental_steps_input: 10,
    action_on_left_click_media_player_metadata: Nothing, 
    action_on_right_click_media_player_metadata: Nothing, 
    action_on_left_click_clock: DefaultAction,
    //action_on_right_click_clock: CycleClockTimezones,
    // action_on_right_click_clock: ToggleAltClockAndCycleClockTimezones,
    action_on_right_click_clock: CustomAction(["kitty", "bash", "-c", "cal && echo 'Press Enter To Exit' && read -n 1"]), 
    action_on_left_click_network: DefaultAction, 
    action_on_right_click_network: DefaultAction, 
    action_on_left_click_volume_output: DefaultAction, 
    action_on_right_click_volume_output: CustomAction(["kitty", "pulsemixer"]), 
    action_on_left_click_volume_input: DefaultAction, 
    action_on_right_click_volume_input: CustomAction(["kitty", "pulsemixer"]), 
	

    // ================= FORMATS =================
    media_player_buttons_format: ("󰒮", "⏸", "▶", "󰒭"),
    media_player_metadata_format: "{{artist}} | {{album}} | {{title}}",
    network_disconnected_text: "No Connection Found.",
    alt_network_module_format: "{level} | {connection_type} | [Color=(0, 255, 255), String={id}]  | [Color=(0, 255, 0), String={speed}MB/s]",
    network_module_format: "{level} ",
    network_level_format: 
    (
        "[Color=(150, 40, 80), String=󰖩]",
        "[Color=(150, 40, 80), String=󱚵]",
        "[Color=(150, 40, 80), String=󱚼]",
        "[Color=(150, 40, 80), String=󰖪]"
    ),
    alt_network_level_format: 
    (
        "󰖩",
        "󱚵",
        "󱚼",
        "󰖪"
    ),
    network_connection_type_icons: 
    (
        "[Color=(150, 40, 80), String=󰈀]", 
        "[Color=(150, 40, 80), String=]", 
        "[Color=(150, 40, 80), String=?]"
    ),
    alt_network_connection_type_icons: 
    (
        "󰈀", 
        "", 
        "?"
    ),
    output_volume_format: 
    (
        "[Color=(150, 40, 80), String=]   {}%", 
        "[Color=(150, 40, 80), String=󰖀]   {}%", 
        "[Color=(150, 40, 80), String=]   {}%", 
        "[Color=(150, 40, 80), String=]   {}%", 
        "[Color=(150, 40, 80), String=]   {}%", 
        "[Color=(150, 40, 80), String=]   + {}%"
    ),
    input_volume_format: 
    (
        "[Color=(150, 40, 80), String=]   {}%", 
        "[Color=(150, 40, 80), String=]  {}%", 
        "[Color=(150, 40, 80), String=]  {}%", 
        "[Color=(150, 40, 80), String=]  {}%", 
        "[Color=(150, 40, 80), String=]  {}%", 
        "[Color=(150, 40, 80), String=󰢴]  {}%"
    ),
    output_volume_muted_format: "   Muted",
    input_volume_muted_format: "   Muted",
    clock_format: "[Color=(150, 40, 80), String=󰥔]  %H:%M",
    clock_alt_format: "󰃭  %a %d %b |  󰥔  %H:%M:%S",
    focused_window_format: "{title}",
    cpu_format: "[Color=(150, 40, 80), String=CPU:] {usage}%",
    cpu_temp_format: "[Color=(150, 40, 80), String=CPU Temp:] {temp}°C",
    ram_format: "[Color=(150, 40, 80), String=Ram Used:] {used}MB / {percent}% | [Color=(150, 40, 80), String=Ram Total:] {total}MB",

    
    // ================= PADDING CONFIGS =================
    focused_window_padding: 0,
    cpu_padding: 0,
    cpu_temp_padding: 0,
    ram_padding: 0,
    media_player_metadata_padding: 0,
    media_player_button_padding: 0,
    network_padding: 0,
    alt_network_padding: 0,
    clock_padding: 0,
    alt_clock_padding: 0,
    volume_output_padding: 0,
    muted_volume_output_padding: 0,
    volume_input_padding: 0,
    muted_volume_input_padding: 0,


    // ================= SIDE SEPARATOR CONFIGS =================
    clock_side_separator: (false, false),
    clock_side_separator_color: (75, 75, 75),
    clock_side_separator_width: 1.,
    clock_side_separator_height: 16.,

    alt_clock_side_separator: (false, false),
    alt_clock_side_separator_color: (75, 75, 75),
    alt_clock_side_separator_width: 1.,
    alt_clock_side_separator_height: 16.,

    tray_side_separator:        (false, false),
    tray_side_separator_color:  (75, 75, 75),
    tray_side_separator_width:  1.,
    tray_side_separator_height: 16.,
 
    workspace_side_separator:        (false, false),
    workspace_side_separator_color:  (75, 75, 75),
    workspace_side_separator_width:  1.,
    workspace_side_separator_height: 16.,
 
    media_player_metadata_side_separator:        (false, false),
    media_player_metadata_side_separator_color:  (75, 75, 75),
    media_player_metadata_side_separator_width:  1.,
    media_player_metadata_side_separator_height: 16.,
 
    media_player_buttons_side_separator:        (false, false),
    media_player_buttons_side_separator_color:  (75, 75, 75),
    media_player_buttons_side_separator_width:  1.,
    media_player_buttons_side_separator_height: 16.,
 
    focused_window_side_separator:        (false, false),
    focused_window_side_separator_color:  (75, 75, 75),
    focused_window_side_separator_width:  1.,
    focused_window_side_separator_height: 16.,
 
    cpu_side_separator:        (false, false),
    cpu_side_separator_color:  (75, 75, 75),
    cpu_side_separator_width:  1.,
    cpu_side_separator_height: 16.,
 
    cpu_temp_side_separator:        (false, false),
    cpu_temp_side_separator_color:  (75, 75, 75),
    cpu_temp_side_separator_width:  1.,
    cpu_temp_side_separator_height: 16.,
 
    ram_side_separator:        (false, false),
    ram_side_separator_color:  (75, 75, 75),
    ram_side_separator_width:  1.,
    ram_side_separator_height: 16.,
 
    network_side_separator:        (false, false),
    network_side_separator_color:  (75, 75, 75),
    network_side_separator_width:  1.,
    network_side_separator_height: 16.,

    volume_output_side_separator:        (false, false),
    volume_output_side_separator_color:  (75, 75, 75),
    volume_output_side_separator_width:  1.,
    volume_output_side_separator_height: 20.,
 
    muted_volume_output_side_separator:        (false, false),
    muted_volume_output_side_separator_color:  (75, 75, 75),
    muted_volume_output_side_separator_width:  1.,
    muted_volume_output_side_separator_height: 20.,
 
    volume_input_side_separator:        (false, false),
    volume_input_side_separator_color:  (75, 75, 75),
    volume_input_side_separator_width:  1.,
    volume_input_side_separator_height: 20.,

    muted_volume_input_side_separator:        (false, false),
    muted_volume_input_side_separator_color:  (75, 75, 75),
    muted_volume_input_side_separator_width:  1.,
    muted_volume_input_side_separator_height: 20.,


    // ================= TRAY (STYLE) =================
    tray_icon_size: 20,
    tray_spacing: 5,
    tray_button_size: 5,
    tray_button_color_rgb: (60, 50, 70),
    tray_button_text_color_rgb: (220, 220, 230),
    tray_button_hovered_color_rgb: (110, 40, 80),
    tray_button_hovered_text_color_rgb: (255, 255, 255),
    tray_button_pressed_color_rgb: (70, 20, 40),
    tray_border_color_rgba: (90, 70, 100, 100),
    tray_border_size: 1.0,
    tray_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= FOCUSED WINDOW (STYLE) =================
    focused_window_text_size: 12,
    focused_window_text_color_rgb: (220, 220, 220),
    focused_window_text_orientation: Horizontal,
    focused_window_button_color_rgb: (40, 40, 50),
    focused_window_button_text_color_rgb: (220, 220, 220),
    focused_window_button_hovered_color_rgb: (60, 60, 75),
    focused_window_button_hovered_text_color_rgb: (255, 255, 255),
    focused_window_button_pressed_color_rgb: (30, 30, 40),
    focused_window_border_color_rgba: (80, 80, 100, 80),
    focused_window_border_size: 1.0,
    focused_window_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= CPU (STYLE) =================
    cpu_text_size: 12,
    cpu_text_color_rgb: (220, 220, 220),
    cpu_text_orientation: Horizontal,
    cpu_button_color_rgb: (40, 40, 50),
    cpu_button_text_color_rgb: (220, 220, 220),
    cpu_button_hovered_color_rgb: (60, 60, 75),
    cpu_button_hovered_text_color_rgb: (255, 255, 255),
    cpu_button_pressed_color_rgb: (30, 30, 40),
    cpu_border_color_rgba: (80, 80, 100, 80),
    cpu_border_size: 1.0,
    cpu_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= CPU TEMP (STYLE) =================
    cpu_temp_text_size: 12,
    cpu_temp_text_color_rgb: (220, 220, 220),
    cpu_temp_text_orientation: Horizontal,
    cpu_temp_button_color_rgb: (40, 40, 50),
    cpu_temp_button_text_color_rgb: (220, 220, 220),
    cpu_temp_button_hovered_color_rgb: (60, 60, 75),
    cpu_temp_button_hovered_text_color_rgb: (255, 255, 255),
    cpu_temp_button_pressed_color_rgb: (30, 30, 40),
    cpu_temp_border_color_rgba: (80, 80, 100, 80),
    cpu_temp_border_size: 1.0,
    cpu_temp_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= RAM (STYLE) =================
    ram_text_size: 12,
    ram_text_color_rgb: (220, 220, 220),
    ram_text_orientation: Horizontal,
    ram_button_color_rgb: (40, 40, 50),
    ram_button_text_color_rgb: (220, 220, 220),
    ram_button_hovered_color_rgb: (60, 60, 75),
    ram_button_hovered_text_color_rgb: (255, 255, 255),
    ram_button_pressed_color_rgb: (30, 30, 40),
    ram_border_color_rgba: (80, 80, 100, 80),
    ram_border_size: 1.0,
    ram_border_radius: (3.0, 3.0, 3.0, 3.0),

    
    // ================= MEDIA PLAYER (STYLE) =================
    media_player_metadata_text_size: 15,
    media_player_metadata_text_color_rgb: (255, 255, 255),
    media_player_metadata_text_orientation: Horizontal,
    media_player_metadata_button_color_rgb: (50, 45, 60),
    media_player_metadata_button_text_color_rgb: (235, 235, 240),
    media_player_metadata_button_hovered_color_rgb: (130, 35, 70),
    media_player_metadata_button_hovered_text_color_rgb: (255, 255, 255),
    media_player_metadata_button_pressed_color_rgb: (80, 25, 45),
    media_player_metadata_border_color_rgba: (120, 80, 130, 100),
    media_player_metadata_border_size: 1.0,
    media_player_metadata_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= MEDIA PLAYER BUTTONS (STYLE) =================
    media_player_button_spacing: 5,
    media_player_button_text_size: 15,
    media_player_button_text_color_rgb: (255, 255, 255),
    media_player_button_text_orientation: Horizontal,
    media_player_button_color_rgb: (50, 45, 60),
    media_player_button_hovered_color_rgb: (130, 35, 70),
    media_player_button_hovered_text_color_rgb: (255, 255, 255),
    media_player_button_pressed_color_rgb: (80, 25, 45),
    media_player_button_border_color_rgba: (120, 80, 130, 100),
    media_player_button_border_size: 1.0,
    media_player_button_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= NETWORK (STYLE) =================
    network_text_size: 15,
    network_text_color_rgb: (255, 255, 255),
    network_text_orientation: Horizontal,
    network_button_color_rgb: (50, 45, 60),
    network_button_text_color_rgb: (235, 235, 240),
    network_button_hovered_color_rgb: (130, 35, 70),
    network_button_hovered_text_color_rgb: (255, 255, 255),
    network_button_pressed_color_rgb: (80, 25, 45),
    network_border_color_rgba: (120, 80, 130, 100),
    network_border_size: 1.0,
    network_border_radius: (3.0, 3.0, 3.0, 3.0),

    // ================= ALT NETWORK (STYLE) =================
    alt_network_text_size: 15,
    alt_network_text_color_rgb: (255, 255, 255),
    alt_network_text_orientation: Horizontal,
    alt_network_button_color_rgb: (150, 40, 80),
    alt_network_button_text_color_rgb: (235, 235, 240),
    alt_network_button_hovered_color_rgb: (130, 35, 70),
    alt_network_button_hovered_text_color_rgb: (255, 255, 255),
    alt_network_button_pressed_color_rgb: (80, 25, 45),
    alt_network_border_color_rgba: (120, 80, 130, 100),
    alt_network_border_size: 1.0,
    alt_network_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= CLOCK (STYLE) =================
    clock_text_size: 15,
    clock_text_color_rgb: (255, 255, 255),
    clock_text_orientation: Horizontal,
    clock_button_color_rgb: (50, 45, 60),
    clock_button_text_color_rgb: (235, 235, 240),
    clock_button_hovered_color_rgb: (130, 35, 70),
    clock_button_hovered_text_color_rgb: (255, 255, 255),
    clock_button_pressed_color_rgb: (80, 25, 45),
    clock_border_color_rgba: (120, 80, 130, 100),
    clock_border_size: 1.0,
    clock_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= ALT CLOCK (STYLE) =================
    alt_clock_text_size: 15,
    alt_clock_text_color_rgb: (255, 255, 255),
    alt_clock_text_orientation: Horizontal,
    alt_clock_button_color_rgb: (150, 40, 80),
    alt_clock_button_text_color_rgb: (235, 235, 240),
    alt_clock_button_hovered_color_rgb: (130, 35, 70),
    alt_clock_button_hovered_text_color_rgb: (255, 255, 255),
    alt_clock_button_pressed_color_rgb: (80, 25, 45),
    alt_clock_border_color_rgba: (120, 80, 130, 100),
    alt_clock_border_size: 1.0,
    alt_clock_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= VOLUME/OUTPUT (STYLE) =================
    volume_output_text_size: 15,
    volume_output_text_color_rgb: (255, 255, 255),
    volume_output_text_orientation: Horizontal,
    volume_output_button_color_rgb: (55, 45, 65),
    volume_output_button_text_color_rgb: (220, 220, 230),
    volume_output_button_hovered_color_rgb: (150, 45, 85),
    volume_output_button_hovered_text_color_rgb: (255, 255, 255),
    volume_output_button_pressed_color_rgb: (85, 30, 50),
    volume_output_border_color_rgba: (110, 80, 120, 100),
    volume_output_border_size: 1.0,
    volume_output_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= MUTED VOLUME/OUTPUT (STYLE) =================
    muted_volume_output_text_size: 15,
    muted_volume_output_text_color_rgb: (255, 255, 255),
    muted_volume_output_text_orientation: Horizontal,
    muted_volume_output_button_color_rgb: (150, 40, 80),
    muted_volume_output_button_text_color_rgb: (220, 220, 230),
    muted_volume_output_button_hovered_color_rgb: (150, 45, 85),
    muted_volume_output_button_hovered_text_color_rgb: (255, 255, 255),
    muted_volume_output_button_pressed_color_rgb: (85, 30, 50),
    muted_volume_output_border_color_rgba: (110, 80, 120, 100),
    muted_volume_output_border_size: 1.0,
    muted_volume_output_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= VOLUME/INPUT (STYLE) =================
    volume_input_text_size: 15,
    volume_input_text_color_rgb: (255, 255, 255),
    volume_input_text_orientation: Horizontal,
    volume_input_button_color_rgb: (55, 45, 65),
    volume_input_button_text_color_rgb: (220, 220, 230),
    volume_input_button_hovered_color_rgb: (150, 45, 85),
    volume_input_button_hovered_text_color_rgb: (255, 255, 255),
    volume_input_button_pressed_color_rgb: (85, 30, 50),
    volume_input_border_color_rgba: (110, 80, 120, 100),
    volume_input_border_size: 1.0,
    volume_input_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= MUTED VOLUME/INPUT (STYLE) =================
    muted_volume_input_text_size: 15,
    muted_volume_input_text_color_rgb: (255, 255, 255),
    muted_volume_input_text_orientation: Horizontal,
    muted_volume_input_button_color_rgb: (150, 40, 80),
    muted_volume_input_button_text_color_rgb: (220, 220, 230),
    muted_volume_input_button_hovered_color_rgb: (150, 45, 85),
    muted_volume_input_button_hovered_text_color_rgb: (255, 255, 255),
    muted_volume_input_button_pressed_color_rgb: (85, 30, 50),
    muted_volume_input_border_color_rgba: (110, 80, 120, 100),
    muted_volume_input_border_size: 1.0,
    muted_volume_input_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= HYPR WORKSPACES (STYLE) =================
    workspace_heigth: 5,
    workspace_width: 5,
    workspace_different_selected_width: None,
    workspace_text_size: 15,
    workspace_selected_text_color_rgb: (255, 255, 255),
    workspace_text_color_rgb: (255, 255, 255),
    workspace_text_orientation: Horizontal,
    workspace_text: 
    [
        "1", 
        "2", 
        "3", 
        "4", 
        "5", 
        "6", 
        "7", 
        "8", 
        "9", 
        "10"
    ],
    workspace_selected_text: Some(
    [
        "●", 
        "●", 
        "●", 
        "●", 
        "●", 
        "●", 
        "●", 
        "●", 
        "●", 
        "●"
    ]),
    workspace_spacing: 3,
    workspace_button_color_rgb: (45, 40, 55),
    workspace_button_text_color_rgb: (200, 200, 210),
    workspace_button_selected_color_rgb: (150, 40, 80),
    workspace_button_hovered_color_rgb: (140, 35, 75),
    workspace_button_hovered_text_color_rgb: (255, 255, 255),
    workspace_button_pressed_color_rgb: (90, 25, 50),
    workspace_border_color_rgba: (120, 90, 135, 100),
    workspace_border_size: 1.0,
    workspace_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= CONTEXT MENU (STYLE) =================
    context_menu_background_color_rgba: (20, 20, 24, 98),
    context_menu_background_size: 5,
    context_menu_background_border_color_rgba: (255, 255, 255, 100),
    context_menu_background_border_size: 1.0,
    context_menu_background_border_radius: (3.0, 3.0, 3.0, 3.0),
    context_menu_text_size: 15,
    context_menu_text_color_rgb: (255, 255, 255),
    context_menu_orientation: Vertical,
    context_menu_size: 300,
    context_menu_item_size: 30,
    context_menu_button_color_rgb: (45, 40, 55),
    context_menu_button_text_color_rgb: (230, 230, 240),
    context_menu_button_hovered_color_rgb: (150, 40, 80),
    context_menu_button_hovered_text_color_rgb: (255, 255, 255),
    context_menu_button_pressed_color_rgb: (85, 30, 55),
    context_menu_border_color_rgba: (130, 90, 140, 100),
    context_menu_border_size: 1.0,
    context_menu_border_radius: (3.0, 3.0, 3.0, 3.0),

    // ================= CUSTOM MODULES =================
    custom_modules_spacing: 10,
    custom_modules: 
    [
    	//Example of an button that just runs an app or command
	(
                side_separator: (true, false),
                separator_color: (75, 75, 75),
                separator_width: 1.,
                separator_height: 16.,

		name: "Wofi Custom Module",
		text: "[Color=(150, 40, 80), String=󰣇] ",
                text_size: 15,
                text_color_rgb: (255, 255, 255),
		text_orientation: Horizontal,
		height: 30,
    		button_color_rgb: (45, 40, 55),
    		button_text_color_rgb: (230, 230, 240),
    		button_hovered_color_rgb: (150, 40, 80),
    		button_hovered_text_color_rgb: (255, 255, 255),
    		button_pressed_color_rgb: (85, 30, 55),
    		border_color_rgba: (130, 90, 140, 100),
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
    	//	button_color_rgb: (255, 40, 55),
    	//	button_text_color_rgb: (230, 230, 240),
    	//	button_hovered_color_rgb: (150, 40, 80),
    	//	button_hovered_text_color_rgb: (255, 255, 255),
    	//	button_pressed_color_rgb: (85, 30, 55),
    	//	border_color_rgba: (130, 90, 140, 100),
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
    	//	button_color_rgb: (255, 40, 55),
    	//	button_text_color_rgb: (230, 230, 240),
    	//	button_hovered_color_rgb: (150, 40, 80),
    	//	button_hovered_text_color_rgb: (255, 255, 255),
    	//	button_pressed_color_rgb: (85, 30, 55),
    	//	border_color_rgba: (130, 90, 140, 100),
    	//	border_size: 1.0,
    	//	border_radius: (3.0, 3.0, 3.0, 3.0),
        //	display_err_output_if_failed: false,
	//      dont_show_if_any_output_is_empty: true,
	//	use_output_as_text: false,
	//	use_continous_output_as_text: true,
	//	all_output_as_text_format: "    {continous_output}",
        //      output_text_limit_len: 50,
        //      continous_command_interval: 500,
	//	continous_command: ["playerctl", "--player=spotify", "metadata", "--format", "{{ artist }} - {{ title }}"]
	//),
    ],
)"#;
        let mut file = File::create(ron_file_config_path).expect("Couldn't Create Default Config File");
        file.write_all(ron_default_data.as_bytes()).expect("Couldn't Create Default Config File");
    };
}
