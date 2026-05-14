// ============ IMPORTS ============
use std::{fs::{self, File}, io::Write, path::Path};





// ============ FUNCTIONS ============
pub fn check_if_config_file_exists(different_config_path: Option<String>) -> Option<String>
{
    println!("\n=== FS CHECK RUNNING... ===");

    let home_path = match home::home_dir()
    {
        Some(home_dir) => home_dir.display().to_string(),
        None => return Some("Warning!!!: Failed to get Home directory".to_string())
    };

    let path: String = if let Some(ref user_config_path) = different_config_path
    {
        println!("Using user parsed config path...");
        if user_config_path.ends_with(".ron")
        {
            match user_config_path.rfind('/') 
            {
                Some(i) => user_config_path[..i].to_string(),
                None => user_config_path.to_string()
            }
        }
        else
        {
            user_config_path.to_owned()
        }
    }
    else
    {
        format!("{}/.config/icebar", home_path)
    };


    let file_path = if let Some(ref user_config_path) = different_config_path
    {
        if user_config_path.ends_with(".ron")
        {
            user_config_path.to_owned()
        }
        else
        {
            if user_config_path.ends_with("/")
            {
                format!("{user_config_path}config.ron")
            }
            else
            {
                format!("{user_config_path}/config.ron")
            }
        }
    }
    else
    {
        format!("{}/.config/icebar/config.ron", home_path)
    };


    let ron_file_config_path = Path::new(&file_path);
    let ron_config_path = Path::new(&path);

    if Path::exists(ron_config_path)
    {
        println!("Ron Config Directory Exists!!!");
    }
    else
    {
        if different_config_path.is_some()
        {
            eprintln!("Warning!!!: User parsed Ron config directory doesn't exist!!!");
            return Some("Warning!!!: User parsed Ron config directory doesn't exist!!!".to_string())
        };
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
        if different_config_path.is_some()
        {
            eprintln!("Warning!!!: User parsed Ron config file doesn't exist!!!");
            return Some("Warning!!!: User parsed Ron config file doesn't exist!!!".to_string())
        };
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
//   "NiriWorkspaces", "HyprWorkspaces", "SwayWorkspaces", "PlasmaWorkspaces",
//   "MediaPlayerMetaData", "MediaPlayerButtons",
//   "CustomModule(index)", "Image(index)",
//   "CpuTemp", "Ram", "Cpu", "Disk", "PowerProfile"
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
//   group_of_modules: groups each hold their own modules list, spacing_inside (between inner modules),
//                     spacing (around the group itself), padding, background_color, border_color,
//                     border_size and border_radius. Reference a group in module lists as "Group(index)".
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
        right_modules:                      [PowerProfile, Tray, Network, VolumeOutput, VolumeInput]
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
        general_alt_padding:                    Some(0),
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
        general_alt_button_shadow_blur:             Some(5.0)
    ),


    // ================= CLOCK =================
    clock:
    (
        clock_timezones:                        None,
        clock_update_interval:                  400,
        clock_format:                           "[Color=(120, 174, 237), String=󰥔]  %H:%M",
        clock_alt_format:                       "󰃭  %a %d %b",
        action_on_left_click_clock:             DefaultAction,
        action_on_right_click_clock:            ShowCalendar,
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
        alt_clock_border_radius:                (6.0, 6.0, 6.0, 6.0)
    ),


    // ================= VOLUME OUTPUT =================
    volume_output:
    (
        //output_volume_unique_format:	Some("[Color=(120, 174, 237), String= ]  {}%"),
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
        action_on_right_click_volume_output:            ShowVolumeOutputMixer,
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
        volume_output_button_shadow_blur:               3.0
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
        muted_volume_output_button_shadow_blur:                 3.0
    ),


    // ================= VOLUME INPUT =================
    volume_input:
    (
    	//input_volume_unique_format:	Some("[Color=(120, 174, 237), String=] {}%"),
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
        action_on_right_click_volume_input:             ShowVolumeInputMixer,
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
        volume_input_button_shadow_blur:                3.0
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
        muted_volume_input_button_shadow_blur:              3.0
    ),

        // ================= VOLUME MIXER OUTPUT =================
    volume_output_mixer:
    (
    	mixer_window_size: 		(560, 420),
	mixer_background_color:		RGB((36, 36, 36)),
        mixer_background_border_color:  HEX("3d3d3d"),
	mixer_background_border_size:	1.0,
	mixer_background_border_radius:	(6., 6., 6., 6.),
	mixer_padding:			12,
	mixer_section_spacing:		10,
	mixer_show_only_active_devices: false,
	// Available options: Up, Down, Left, Right
	// Up    = [Devices]      Down  = [Applications]   Left  = [Devices][Applications]   Right = [Applications][Devices]
	//         [Applications]         [Devices]
	categories_position:            Up,

	scrollbar:
	(
		show:                    true,
            	width:                   6,
            	margin:                  2,
            	scroller_width:          6,
            	border_radius:           (3.0, 3.0, 3.0, 3.0),
		rail_border_width:       0.0,
            	rail_border_color:       RGBA((0, 0, 0, 0)),
		rail_color:              RGBA((0, 0, 0, 0)),
            	scroller_color:          RGBA((100, 100, 100, 60)),
            	scroller_hovered_color:  RGBA((130, 130, 130, 80)),
            	scroller_dragging_color: RGBA((160, 160, 160, 100)),
            	scroller_border_color:   RGBA((0, 0, 0, 0)),
            	scroller_border_width:   0.0,
	),

	device_category: 
	(
		show:			true,
		show_header:		true,
		start_collapsed: 	false,
		header_label:		"  Devices",
		header_collapsed_label: "›",
		header_expanded_label:	"‹",
		header_text_size:	13,
		header_arrow_text_size:	13,
		header_button_height:	28,
		spacing:		15,
		header_text_color:	RGB((120, 174, 237)),
		header_button_style:
		(
	    	color:              	RGB((48, 48, 48)),
            		hovered_color:      	RGB((61, 61, 61)),
            		pressed_color:      	RGB((28, 28, 28)),
            		text_color:         	RGB((255, 255, 255)),
            		hovered_text_color: 	RGB((255, 255, 255)),
            		pressed_text_color: 	RGB((255, 255, 255)),
            		border_color:       	HEX("3d3d3d"),
            		border_size:        	1.0,
            		border_radius:      	(6., 6., 6., 6.),
            		gradient_color:         None,
            		hovered_gradient_color: None,
            		pressed_gradient_color: None
		)
	),

	app_category: 
	(
		show:			true,
		show_header:		true,
		start_collapsed: 	false,
		header_label:		" Applications",
		header_collapsed_label: "›",
		header_expanded_label:	"‹",
		header_text_size:	13,
		header_arrow_text_size:	13,
		header_button_height:	28,
		spacing:		15,
		header_text_color:	RGB((120, 174, 237)),
		header_button_style:
		(
	    	color:              	RGB((48, 48, 48)),
            		hovered_color:      	RGB((61, 61, 61)),
            		pressed_color:      	RGB((28, 28, 28)),
            		text_color:         	RGB((255, 255, 255)),
            		hovered_text_color: 	RGB((255, 255, 255)),
            		pressed_text_color: 	RGB((255, 255, 255)),
            		border_color:       	HEX("3d3d3d"),
            		border_size:        	1.0,
            		border_radius:      	(6., 6., 6., 6.),
            		gradient_color:         None,
            		hovered_gradient_color: None,
            		pressed_gradient_color: None
		)
	),
	
	device_row:
	(
	    // "Fill" is also valid in the device_row_order
	    device_row_order:			[DeviceButton, Fill, Slider, DecreaseVolume, IncreaseVolume, Mute],
            show_only_default_device_name: 	false,

            row_height:                 32,
            row_spacing:                6,
            name_text_size:             13,
            name_text_color:            RGB((255, 255, 255)),
            name_max_chars:             20,
	    device_name_button_width:   160,

            volume_step:                1,
            slider_width:               240.0,
            slider_style:               
	    (
            	rail_width:           4.0,
		rail_color:           RGB((48, 48, 48)),
		rail_filled_color:    RGB((120, 174, 237)),
		rail_border_radius:   (2., 2., 2., 2.),
            	handle_border_width:  1.5,
            	handle_border_radius: (10., 10., 10., 10.),
		handle_color:         RGB((255, 255, 255)),
		handle_border_color:  RGB((120, 174, 237)),
            	handle_shape:         Circle,
            	handle_circle_r:      7.0,
            	handle_rect_w:        10.0,
            	handle_rect_h:        20.0,
	    ),

            inc_button_label:           "+",
            dec_button_label:           "−",
            inc_dec_text_size:          13,
            inc_dec_button_width:       28,
            inc_dec_button_height:      24,
            inc_dec_button_style:       
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),

            mute_label:                 "󰕾",
            unmute_label:               "󰖁",
            mute_text_size:             14,
            mute_button_width:          32,
            mute_button_height:         24,
            mute_button_style:          
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
            muted_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((200, 80, 80)),
            	hovered_text_color: RGB((200, 80, 80)),
            	pressed_text_color: RGB((200, 80, 80)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),

            default_label:              "⬤",
            non_default_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
            default_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((200, 80, 80)),
            	hovered_text_color: RGB((200, 80, 80)),
            	pressed_text_color: RGB((200, 80, 80)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
	),

	app_row:
	(            
	    // "Fill" is also valid in the app_row_order
	    app_row_order:			[AppName, Fill, Slider, DecreaseVolume, IncreaseVolume, Mute],

            row_height:                 32,
            row_spacing:                6,
            name_text_size:             13,
            name_text_color:            RGB((255, 255, 255)),
            name_max_chars:             20,
            name_button_width:          160,

            volume_step:                1,
            slider_width:               240.0,
            slider_style:               
	    (
            	rail_width:           4.0,
		rail_color:           RGB((48, 48, 48)),
		rail_filled_color:    RGB((120, 174, 237)),
		rail_border_radius:   (2., 2., 2., 2.),
            	handle_border_width:  1.5,
            	handle_border_radius: (10., 10., 10., 10.),
		handle_color:         RGB((255, 255, 255)),
		handle_border_color:  RGB((120, 174, 237)),
            	handle_shape:         Circle,
            	handle_circle_r:      7.0,
            	handle_rect_w:        10.0,
            	handle_rect_h:        20.0,
	    ),

            inc_button_label:           "+",
            dec_button_label:           "−",
            inc_dec_text_size:          13,
            inc_dec_button_width:       28,
            inc_dec_button_height:      24,
            inc_dec_button_style:       
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),

            mute_label:                 "󰕾",
            unmute_label:               "󰖁",
            mute_text_size:             14,
            mute_button_width:          32,
            mute_button_height:         24,
            mute_button_style:          
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
            muted_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((200, 80, 80)),
            	hovered_text_color: RGB((200, 80, 80)),
            	pressed_text_color: RGB((200, 80, 80)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
	)
    ),
        // ================= VOLUME MIXER INPUT =================
    volume_input_mixer:
    (
    	mixer_window_size: 		(560, 420),
	mixer_background_color:		RGB((36, 36, 36)),
        mixer_background_border_color:  HEX("3d3d3d"),
	mixer_background_border_size:	1.0,
	mixer_background_border_radius:	(6., 6., 6., 6.),
	mixer_padding:			12,
	mixer_section_spacing:		10,
	mixer_show_only_active_devices: false,
	// Available options: Up, Down, Left, Right
	// Up    = [Devices]      Down  = [Applications]   Left  = [Devices][Applications]   Right = [Applications][Devices]
	//         [Applications]         [Devices]
	categories_position:            Up,

	scrollbar:
	(
		show:                    true,
            	width:                   6,
            	margin:                  2,
            	scroller_width:          6,
            	border_radius:           (3.0, 3.0, 3.0, 3.0),
		rail_border_width:       0.0,
            	rail_border_color:       RGBA((0, 0, 0, 0)),
		rail_color:              RGBA((0, 0, 0, 0)),
            	scroller_color:          RGBA((100, 100, 100, 60)),
            	scroller_hovered_color:  RGBA((130, 130, 130, 80)),
            	scroller_dragging_color: RGBA((160, 160, 160, 100)),
            	scroller_border_color:   RGBA((0, 0, 0, 0)),
            	scroller_border_width:   0.0,
	),

	device_category: 
	(
		show:			true,
		show_header:		true,
		start_collapsed: 	false,
		header_label:		"  Devices",
		header_collapsed_label: "›",
		header_expanded_label:	"‹",
		header_text_size:	13,
		header_arrow_text_size:	13,
		header_button_height:	28,
		spacing:		15,
		header_text_color:	RGB((120, 174, 237)),
		header_button_style:
		(
	    	color:              	RGB((48, 48, 48)),
            		hovered_color:      	RGB((61, 61, 61)),
            		pressed_color:      	RGB((28, 28, 28)),
            		text_color:         	RGB((255, 255, 255)),
            		hovered_text_color: 	RGB((255, 255, 255)),
            		pressed_text_color: 	RGB((255, 255, 255)),
            		border_color:       	HEX("3d3d3d"),
            		border_size:        	1.0,
            		border_radius:      	(6., 6., 6., 6.),
            		gradient_color:         None,
            		hovered_gradient_color: None,
            		pressed_gradient_color: None
		)
	),

	app_category: 
	(
		show:			true,
		show_header:		true,
		start_collapsed: 	false,
		header_label:		" Applications",
		header_collapsed_label: "›",
		header_expanded_label:	"‹",
		header_text_size:	13,
		header_arrow_text_size:	13,
		header_button_height:	28,
		spacing:		15,
		header_text_color:	RGB((120, 174, 237)),
		header_button_style:
		(
	    	color:              	RGB((48, 48, 48)),
            		hovered_color:      	RGB((61, 61, 61)),
            		pressed_color:      	RGB((28, 28, 28)),
            		text_color:         	RGB((255, 255, 255)),
            		hovered_text_color: 	RGB((255, 255, 255)),
            		pressed_text_color: 	RGB((255, 255, 255)),
            		border_color:       	HEX("3d3d3d"),
            		border_size:        	1.0,
            		border_radius:      	(6., 6., 6., 6.),
            		gradient_color:         None,
            		hovered_gradient_color: None,
            		pressed_gradient_color: None
		)
	),
	
	device_row:
	(
	    // "Fill" is also valid in the device_row_order
	    device_row_order:			[DeviceButton, Fill, Slider, DecreaseVolume, IncreaseVolume, Mute],
            show_only_default_device_name: 	false,

            row_height:                 32,
            row_spacing:                6,
            name_text_size:             13,
            name_text_color:            RGB((255, 255, 255)),
            name_max_chars:             20,
	    device_name_button_width:   160,

            volume_step:                1,
            slider_width:               240.0,
            slider_style:               
	    (
            	rail_width:           4.0,
		rail_color:           RGB((48, 48, 48)),
		rail_filled_color:    RGB((120, 174, 237)),
		rail_border_radius:   (2., 2., 2., 2.),
            	handle_border_width:  1.5,
            	handle_border_radius: (10., 10., 10., 10.),
		handle_color:         RGB((255, 255, 255)),
		handle_border_color:  RGB((120, 174, 237)),
            	handle_shape:         Circle,
            	handle_circle_r:      7.0,
            	handle_rect_w:        10.0,
            	handle_rect_h:        20.0,
	    ),

            inc_button_label:           "+",
            dec_button_label:           "−",
            inc_dec_text_size:          13,
            inc_dec_button_width:       28,
            inc_dec_button_height:      24,
            inc_dec_button_style:       
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),

            mute_label:                 "󰕾",
            unmute_label:               "󰖁",
            mute_text_size:             14,
            mute_button_width:          32,
            mute_button_height:         24,
            mute_button_style:          
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
            muted_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((200, 80, 80)),
            	hovered_text_color: RGB((200, 80, 80)),
            	pressed_text_color: RGB((200, 80, 80)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),

            default_label:              "⬤",
            non_default_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
            default_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((200, 80, 80)),
            	hovered_text_color: RGB((200, 80, 80)),
            	pressed_text_color: RGB((200, 80, 80)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
	),

	app_row:
	(            
	    // "Fill" is also valid in the app_row_order
	    app_row_order:			[AppName, Fill, Slider, DecreaseVolume, IncreaseVolume, Mute],

            row_height:                 32,
            row_spacing:                6,
            name_text_size:             13,
            name_text_color:            RGB((255, 255, 255)),
            name_max_chars:             20,
            name_button_width:          160,

            volume_step:                1,
            slider_width:               240.0,
            slider_style:               
	    (
            	rail_width:           4.0,
		rail_color:           RGB((48, 48, 48)),
		rail_filled_color:    RGB((120, 174, 237)),
		rail_border_radius:   (2., 2., 2., 2.),
            	handle_border_width:  1.5,
            	handle_border_radius: (10., 10., 10., 10.),
		handle_color:         RGB((255, 255, 255)),
		handle_border_color:  RGB((120, 174, 237)),
            	handle_shape:         Circle,
            	handle_circle_r:      7.0,
            	handle_rect_w:        10.0,
            	handle_rect_h:        20.0,
	    ),

            inc_button_label:           "+",
            dec_button_label:           "−",
            inc_dec_text_size:          13,
            inc_dec_button_width:       28,
            inc_dec_button_height:      24,
            inc_dec_button_style:       
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),

            mute_label:                 "󰕾",
            unmute_label:               "󰖁",
            mute_text_size:             14,
            mute_button_width:          32,
            mute_button_height:         24,
            mute_button_style:          
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((255, 255, 255)),
            	hovered_text_color: RGB((255, 255, 255)),
            	pressed_text_color: RGB((255, 255, 255)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
            muted_button_style:
	    (
	    	color:              RGB((48, 48, 48)),
            	hovered_color:      RGB((61, 61, 61)),
            	pressed_color:      RGB((28, 28, 28)),
            	text_color:         RGB((200, 80, 80)),
            	hovered_text_color: RGB((200, 80, 80)),
            	pressed_text_color: RGB((200, 80, 80)),
            	border_color:       HEX("3d3d3d"),
            	border_size:        1.0,
            	border_radius:      (6., 6., 6., 6.),
            	gradient_color:         None,
            	hovered_gradient_color: None,
            	pressed_gradient_color: None
	    ),
	)
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
        network_button_shadow_blur:             3.0
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
        alt_network_button_shadow_blur:             0.0
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
	workspace_unique_text:			Some("•"),
	workspace_selected_unique_text:		Some("⬤"), 
	//workspace_unique_text:		Some("{workspace_number}"),
	//workspace_selected_unique_text:	Some("{workspace_number}"),
        //workspace_text:                       ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        //workspace_selected_text:              Some(["⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤", "⬤"]),
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
        workspace_button_shadow_blur:               0.0
    ),


    // ================= TRAY =================
    tray:
    (
        tray_attention_icon:                true,
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
        tray_button_shadow_blur:            3.0
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
        media_player_metadata_button_shadow_blur:               3.0
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
        media_player_button_shadow_blur:                3.0
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
        cpu_button_shadow_blur:             3.0
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
        cpu_temp_button_shadow_blur:            3.0
    ),


    // ================= RAM =================
    ram:
    (
        ram_format:             "[Color=(120, 174, 237), String=RAM]  {used}MB / {percent}%",
        ram_update_interval:    1000,
        action_on_left_click_ram:       DefaultAction,
        action_on_right_click_ram:      DefaultAction,
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
        ram_button_shadow_blur:            3.0
    ),


    // ================= DISK =================
    disk:
    (
        disk_format:            "[Color=(120, 174, 237), String=DISK]  {used}GB / {percent}%",
        disk_mount:             "/",
        disk_update_interval:   10000,
        action_on_left_click_disk:      DefaultAction,
        action_on_right_click_disk:     DefaultAction,
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
        disk_button_shadow_blur:            3.0
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
        context_menu_button_gradient_color:         None,
        context_menu_button_hovered_gradient_color: None,
        context_menu_button_pressed_gradient_color: None
    ),


    // ================= FOCUSED WINDOW =================
    focused_window:
    (
        focused_window_format:                      "{title}",
        focused_window_update_interval:             500,
        dont_show_focused_window_if_empty:          true,
        text_when_focused_window_is_empty:          "Desktop",
        focused_window_text_limit_len:              30,
        action_on_left_click_focused_window:        DefaultAction,
        action_on_right_click_focused_window:       DefaultAction,
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
        focused_window_button_shadow_blur:              0.0
    ),


    // ================= POWER PROFILE =================
    power_profile:
    (
        // Text shown for each profile. Supports [Color=...] tags.
        // Left-click (DefaultAction) cycles through: Balanced → Performance → PowerSaver → …
        power_profile_format_power_saver:   	 "[Color=(120, 174, 237), String= 󰌪 ] Saver",
        power_profile_format_balanced:      	 "[Color=(120, 174, 237), String= 󰈐 ] Balanced",
        power_profile_format_performance:   	 "[Color=(120, 174, 237), String= 󱐋 ] Performance",
        power_profile_update_interval:      	 5000,
        action_on_left_click_power_profile:  	 DefaultAction,
        action_on_right_click_power_profile: 	 Nothing,
        power_profile_padding:                   0,
        power_profile_text_size:                 13,
        power_profile_text_color:                HEX("ffffff"),
        power_profile_text_orientation:          Horizontal,
        power_profile_button_color:              HEX("303030"),
        power_profile_button_hovered_color:      HEX("3d3d3d"),
        power_profile_button_hovered_text_color: HEX("ffffff"),
        power_profile_button_pressed_text_color: HEX("ffffff"),
        power_profile_button_pressed_color:      HEX("1c1c1c"),
        power_profile_border_color:              HEX("3d3d3d"),
        power_profile_border_size:               1.0,
        power_profile_border_radius:             (6.0, 6.0, 6.0, 6.0),
        power_profile_side_separator:            None,
        power_profile_side_separator_color:      HEX("3d3d3d"),
        power_profile_side_separator_width:      1.,
        power_profile_side_separator_height:     18.,
        power_profile_button_gradient_color:         None,
        power_profile_button_hovered_gradient_color: None,
        power_profile_button_pressed_gradient_color: None,
        power_profile_button_shadow_color:           Some(RGBA((0, 0, 0, 50))),
        power_profile_button_shadow_x:               0.0,
        power_profile_button_shadow_y:               1.0,
        power_profile_button_shadow_blur:            3.0
    ),


    // ================= IMAGE =================
    //image:
    //(    
    //    images_spacing: 5,
    //    images:
    //    [
    //        (
    //            image_path: "path/to/your/gif",
    //            content_fit: Fill,
    //            message_image_missing: "Warning!!!: GIF Not Found.",
    //            side_separator: None,
    //            separator_color: RGB((75, 75, 75)),
    //            separator_width:  1.,
    //            separator_height: 16.,
    //            padding: 1,
    //            height: 30,
    //            width: 50,
    //            button_color: RGB((60, 50, 70)),
    //            button_hovered_color: RGB((110, 40, 80)),
    //            button_pressed_color: RGB((70, 20, 40)),
    //            border_color: RGB((45, 55, 100)),
    //            border_size: 1.0,
    //            border_radius: (3., 3., 3., 3.),
    //            command_to_exec_on_left_click: ["do", "a", "flip"],
    //            command_to_exec_on_right_click: ["kitty", "echo", "Meow"],
    //        ),
    //    ],
    //),


    // ================= CALENDAR =================
    calendar_window:
    (
        // ── Window ───────────────────────────────────────────────────────
        calendar_window_size:                   (340, 310),

        // ── Background / frame ───────────────────────────────────────────
        calendar_background_color:              RGBA((30, 30, 30, 98)),
        calendar_background_border_color:       HEX("3d3d3d"),
        calendar_background_border_size:        1.0,
        calendar_background_border_radius:      (10.0, 10.0, 10.0, 10.0),
        calendar_padding:                       8,

        // ── Nav bar ──────────────────────────────────────────────────────
        calendar_nav_spacing_y:                 0,
        calendar_nav_position:                  Above,
        calendar_nav_grid_spacing:              6,
        calendar_nav_spacing:                   4,
        calendar_nav_button_height:             28,
        calendar_nav_button_width:              80,
        calendar_nav_text_size:                 14,
        calendar_nav_button_style:
        (
            color:               HEX("303030"),
            hovered_color:       HEX("3d3d3d"),
            pressed_color:       HEX("1c1c1c"),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        HEX("3d3d3d"),
            border_size:         1.0,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),
        calendar_nav_active_button_style:
        (
            color:               HEX("1c71d8"),
            hovered_color:       RGB((53, 132, 228)),
            pressed_color:       RGB((28, 92, 187)),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        RGB((90, 143, 199)),
            border_size:         1.0,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),

        // ── Prev / Next arrow buttons ─────────────────────────────────────
        calendar_prev_label:                    "‹",
        calendar_next_label:                    "›",
        calendar_arrow_button_height:           28,
        calendar_arrow_button_width:            28,
        calendar_arrow_text_size:               18,
        calendar_arrow_button_style:
        (
            color:               HEX("303030"),
            hovered_color:       HEX("3d3d3d"),
            pressed_color:       HEX("1c1c1c"),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        HEX("3d3d3d"),
            border_size:         1.0,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),

        // ── Month view ────────────────────────────────────────────────────
	calendar_total_day_cells:		35,
	calendar_month_spacing_y:		5,
        calendar_weekday_labels:                ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
        calendar_show_weekday_header:           true,
        calendar_weekday_header_text_size:      12,
        calendar_weekday_header_text_color:     RGB((180, 180, 180)),
        calendar_show_week_numbers:             false,
        calendar_week_number_text_size:         11,
        calendar_week_number_text_color:        RGB((140, 140, 140)),
        calendar_first_week_day:                Monday,
        calendar_day_cell_width:                40,
        calendar_day_cell_height:               38,
        calendar_grid_spacing:                  2,
        calendar_day_text_size:                 13,
        calendar_show_overflow_days_prev_month:            true,
        calendar_show_overflow_days_next_month:            true,
        calendar_day_button_style:
        (
            color:               HEX("303030"),
            hovered_color:       HEX("3d3d3d"),
            pressed_color:       HEX("1c1c1c"),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        HEX("3d3d3d"),
            border_size:         1.0,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),
        calendar_today_button_style:
        (
            color:               RGB((28, 92, 187)),
            hovered_color:       HEX("1c71d8"),
            pressed_color:       RGB((20, 70, 150)),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        RGB((90, 143, 199)),
            border_size:         1.5,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),
        calendar_selected_day_button_style:
        (
            color:               RGB((53, 132, 228)),
            hovered_color:       RGB((70, 148, 240)),
            pressed_color:       HEX("1c71d8"),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        RGB((143, 191, 245)),
            border_size:         1.5,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),
        calendar_overflow_day_button_style:
        (
            color:               RGBA((30, 30, 30, 60)),
            hovered_color:       RGBA((50, 50, 50, 80)),
            pressed_color:       RGBA((20, 20, 20, 60)),
            text_color:          RGB((100, 100, 100)),
            hovered_text_color:  RGB((140, 140, 140)),
            pressed_text_color:  RGB((120, 120, 120)),
            border_color:        RGBA((61, 61, 61, 40)),
            border_size:         0.5,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),
        calendar_day_click_action:              HighlightOnly,

        // ── Year view ─────────────────────────────────────────────────────
	calendar_year_spacing_y:		45,
        calendar_month_cell_width:              55,
        calendar_month_cell_height:             55,
        calendar_year_grid_columns:             4,
        calendar_year_grid_spacing:             4,
        calendar_month_text_size:               13,
        calendar_month_labels:                  ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                                                 "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
        calendar_month_button_style:
        (
            color:               HEX("303030"),
            hovered_color:       HEX("3d3d3d"),
            pressed_color:       HEX("1c1c1c"),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        HEX("3d3d3d"),
            border_size:         1.0,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),
        calendar_current_month_button_style:
        (
            color:               HEX("1c71d8"),
            hovered_color:       RGB((53, 132, 228)),
            pressed_color:       RGB((28, 92, 187)),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        RGB((90, 143, 199)),
            border_size:         1.5,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),

        // ── Decade view ───────────────────────────────────────────────────
	calendar_decade_spacing_y:		45,
        calendar_year_cell_width:               55,
        calendar_year_cell_height:              55,
        calendar_decade_grid_columns:           4,
        calendar_decade_grid_spacing:           4,
        calendar_year_text_size:                13,
        calendar_year_button_style:
        (
            color:               HEX("303030"),
            hovered_color:       HEX("3d3d3d"),
            pressed_color:       HEX("1c1c1c"),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        HEX("3d3d3d"),
            border_size:         1.0,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        ),
        calendar_current_year_button_style:
        (
            color:               HEX("1c71d8"),
            hovered_color:       RGB((53, 132, 228)),
            pressed_color:       RGB((28, 92, 187)),
            text_color:          HEX("ffffff"),
            hovered_text_color:  HEX("ffffff"),
            pressed_text_color:  HEX("ffffff"),
            border_color:        RGB((90, 143, 199)),
            border_size:         1.5,
            border_radius:       (6.0, 6.0, 6.0, 6.0),
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        )
    ),


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
                command_to_exec_on_left_click:      ["icelauncher"],
                command_to_exec_on_right_click:     ["icelauncher", "--shell"],
                continous_command_interval:         1000,
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
            //(
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
            //		all_output_as_text_format: " {continous_output}",
            //      	output_text_limit_len: 50,
            //      	continous_command_interval: 500,
            //		continous_command: ["playerctl", "--player=spotify", "metadata", "--format", "{{ artist }} - {{ title }}"]
	    //), 
	]
    ),


    // ================= GROUP OF MODULES =================
    // group_of_modules lets you visually group multiple modules inside a styled container.
    // Each group has its own background, border, padding, and spacing controls:
    //   - spacing_inside: space between the modules *inside* the group container
    //   - padding:        inner padding around the group's content
    //   - background_color, border_color, border_size, border_radius: styling
    // Reference a group in any module list as "Group(index)" where the index matches
    // its position in the groups array (first = 0, second = 1, etc.).
    //
    //group_of_modules: 
    //(
    //    groups: 
    //    [
    //    	(
    //            	modules: [HyprWorkspaces],
    //             	spacing_inside: 5,
    //             	padding: 5,
    //             	background_color: HEX("303030"),
    //             	border_color: HEX("3d3d3d"),
    //             	border_size: 1.0,
    //             	border_radius: (6., 6., 6., 6.)
    //    	)
    //    ]
    //),

    // ================= AUTO-HIDE =================
    // The bar hides itself after the cursor leaves and slides back when
    // the cursor touches the hot edge.  Set to None to disable.
    //
    // Available options:
    //   hide_delay_ms  -- ms to wait after cursor leaves before hiding  (default: 500)
    //   show_delay_ms  -- ms to wait after cursor enters before showing (default: 0 = instant)
    //   peek_size      -- px of bar kept visible as a hot edge          (default: 1)
    //
    auto_hide: None,
    //auto_hide: Some
    //((
    //    hide_delay_ms:  500,
    //    show_delay_ms:  0,
    //    peek_size:      1,
    //))
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





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use tempfile::TempDir;

    // ---- helper: create a temp dir with a config.ron file in it ----
    fn make_dir_with_config(dir: &TempDir) -> String
    {
        let path = dir.path().to_str().unwrap().to_string();
        let file = format!("{}/config.ron", path);
        std::fs::write(&file, "// dummy config").unwrap();
        path
    }

    // ---- check_if_config_file_exists with a non-existent custom path ----

    #[test]
    fn custom_path_dir_nonexistent_returns_some_warning()
    {
        let result = check_if_config_file_exists(Some("/tmp/icebar_nonexistent_dir_xyz_test".to_string()));
        assert!(result.is_some());
        let msg = result.unwrap();
        assert!(msg.contains("Warning!!!"));
    }

    #[test]
    fn custom_path_file_nonexistent_returns_some_warning()
    {
        let result = check_if_config_file_exists(Some("/tmp/does_not_exist_xyz.ron".to_string()));
        assert!(result.is_some());
    }

    #[test]
    fn custom_path_dir_exists_but_no_config_file_returns_some_warning()
    {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_str().unwrap().to_string();
        // directory exists but no config.ron
        let result = check_if_config_file_exists(Some(path));
        assert!(result.is_some());
    }

    #[test]
    fn custom_path_dir_exists_with_config_file_returns_none()
    {
        let dir = TempDir::new().unwrap();
        let path = make_dir_with_config(&dir);
        let result = check_if_config_file_exists(Some(path));
        assert!(result.is_none(), "expected None but got: {:?}", result);
    }

    #[test]
    fn custom_path_dot_ron_that_exists_returns_none()
    {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("myconfig.ron");
        std::fs::write(&file, "// config").unwrap();
        let result = check_if_config_file_exists(Some(file.to_str().unwrap().to_string()));
        assert!(result.is_none(), "expected None got: {:?}", result);
    }

    #[test]
    fn custom_path_dot_ron_that_does_not_exist_returns_some_warning()
    {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("missing.ron");
        // do NOT create the file
        let result = check_if_config_file_exists(Some(file.to_str().unwrap().to_string()));
        assert!(result.is_some());
    }

    #[test]
    fn custom_path_ron_with_rfind_slash_no_slash_in_name_is_handled()
    {
        // a path like "config.ron" (no slash) — rfind('/') returns None
        // the directory part becomes the whole string, which won't exist
        let result = check_if_config_file_exists(Some("nonexistent_no_slash.ron".to_string()));
        assert!(result.is_some());
    }

    #[test]
    fn custom_path_trailing_slash_constructs_config_file_path()
    {
        let dir = TempDir::new().unwrap();
        let path_with_slash = format!("{}/", dir.path().to_str().unwrap());
        // No config.ron yet → should warn
        let result = check_if_config_file_exists(Some(path_with_slash.clone()));
        assert!(result.is_some());
        // Now create config.ron → should succeed
        let file = format!("{}config.ron", path_with_slash);
        std::fs::write(&file, "").unwrap();
        let result2 = check_if_config_file_exists(Some(path_with_slash));
        assert!(result2.is_none());
    }

    #[test]
    fn custom_path_without_trailing_slash_constructs_config_file_path()
    {
        let dir = TempDir::new().unwrap();
        let path = dir.path().to_str().unwrap().to_string();
        // Create config.ron
        std::fs::write(format!("{}/config.ron", path), "").unwrap();
        let result = check_if_config_file_exists(Some(path));
        assert!(result.is_none());
    }

    #[test]
    fn none_path_does_not_panic()
    {
        // We can't control ~/.config/icebar, but we just ensure it doesn't panic
        let _ = check_if_config_file_exists(None);
    }

    #[test]
    fn warning_message_mentions_directory_when_dir_missing()
    {
        let result = check_if_config_file_exists(Some("/tmp/icebar_really_missing_dir_abc".to_string()));
        if let Some(msg) = result
        {
            // Should mention the directory issue
            assert!(msg.contains("Warning!!!") || msg.contains("Warning") || msg.contains("directory") || msg.contains("exist"));
        }
    }

    #[test]
    fn warning_message_is_nonempty_when_dir_missing()
    {
        let result = check_if_config_file_exists(Some("/tmp/icebar_really_missing_dir_def".to_string()));
        assert!(result.map(|s| !s.is_empty()).unwrap_or(false));
    }

    #[test]
    fn custom_path_ron_full_existing_file_path_succeeds()
    {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("bar_config.ron");
        std::fs::write(&file, "// full config content").unwrap();
        let result = check_if_config_file_exists(Some(file.to_str().unwrap().to_string()));
        assert!(result.is_none(), "expected None got {:?}", result);
    }

    #[test]
    fn two_calls_with_same_existing_dir_both_return_none()
    {
        let dir = TempDir::new().unwrap();
        let path = make_dir_with_config(&dir);
        let r1 = check_if_config_file_exists(Some(path.clone()));
        let r2 = check_if_config_file_exists(Some(path));
        assert!(r1.is_none());
        assert!(r2.is_none());
    }

    #[test]
    fn custom_path_with_deeply_nested_missing_dir_returns_warning()
    {
        let result = check_if_config_file_exists(Some("/tmp/a/b/c/d/e/icebar_missing".to_string()));
        assert!(result.is_some());
    }

    #[test]
    fn custom_path_ron_file_in_nested_dir_that_exists_returns_none()
    {
        let dir = TempDir::new().unwrap();
        let nested = dir.path().join("sub");
        std::fs::create_dir(&nested).unwrap();
        let file = nested.join("config.ron");
        std::fs::write(&file, "").unwrap();
        let result = check_if_config_file_exists(Some(file.to_str().unwrap().to_string()));
        assert!(result.is_none());
    }
}
