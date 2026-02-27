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

// ===== TIPS =====
// All possible modules: ""HyprWorkspaces", "SwayWorkspaces", "CustomModule(index)", "VolumeOutput", "VolumeInput", "Clock", "Tray".
//
// Volume (output and input) format steps have an incremental of 25%, like this: "0%", 25%, 50%, 75%, 100%, > 100+%.
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
// The unique syntax for each some modules are: "display" = Some("HDMI-A-1"), "force_static_position_context_menu" = Some((x, y)) and "persistent_workspaces" = Some(number_of_persistent_elements)
//
// If you notice some bug or want more features, please feel free to publish your thoughs on: https://github.com/HaruNashii/Icebar.git
// Or if you want talk directly to me to clear up any questions, my discord id is: harunashiii
// you can also join my contact server with: https://discord.gg/CRsz24Ts3a



BarConfig
(
    // ================= GENERAL =================
    display: None,
    bar_position: Up,
    bar_size: (0, 35),
    bar_background_color_rgba: (18, 18, 22, 100),
    font_family: "JetBrains Mono",
    font_style: "Bold",


    // ================= MODULES =================
    left_modules: [CustomModule(0)],
    center_modules: [Clock],
    right_modules: [Tray, VolumeOutput, VolumeInput],


    // ================= MODULES CONFIGS =================
    spacing_between_all_modules: 5,
    force_static_position_context_menu: None,
    reverse_scroll_on_workspace: false,
    persistent_workspaces: Some(5),
    incremental_steps_output: 10,
    incremental_steps_input: 10,
    action_on_left_click_clock: DefaultAction,
    action_on_right_click_clock: CustomAction(["kitty", "bash", "-c", "cal && echo 'Press Enter To Exit' && read -n 1"]), 
    action_on_right_click_volume_output: CustomAction(["kitty", "pulsemixer"]), 
    action_on_right_click_volume_input: CustomAction(["kitty", "pulsemixer"]), 



    // ================= FORMATS =================
    output_volume_format: 
    (
    "   {}%", "󰖀   {}%", "   {}%", "   {}%", "   {}%", "   + {}%"
    ),
    output_volume_muted_format: "  Muted",
    input_volume_format: ("  {}%", "  {}%", "  {}%", "  {}%", "  {}%", "󰢴  {}%"),
    input_volume_muted_format: "  Muted",
    clock_format: "󰥔  %H:%M",
    clock_alt_format: "󰃭  %a %d %b |  󰥔  %H:%M:%S",


    // ================= TRAY (STYLE) =================
    tray_icon_size: 20,
    tray_button_size: 5,
    tray_background_color_rgba: (30, 30, 36, 0),
    tray_button_color_rgb: (60, 50, 70),
    tray_button_text_color_rgb: (220, 220, 230),
    tray_button_hovered_color_rgb: (110, 40, 80),
    tray_button_hovered_text_color_rgb: (255, 255, 255),
    tray_button_pressed_color_rgb: (70, 20, 40),
    tray_border_color_rgba: (90, 70, 100, 100),
    tray_border_size: 1.0,
    tray_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= CLOCK (STYLE) =================
    clock_text_size: 15,
    clock_text_orientation: Horizontal,
    clock_background_color_rgba: (25, 25, 30, 95),
    clock_button_color_rgb: (50, 45, 60),
    clock_button_text_color_rgb: (235, 235, 240),
    clock_button_hovered_color_rgb: (130, 35, 70),
    clock_button_hovered_text_color_rgb: (255, 255, 255),
    clock_button_pressed_color_rgb: (80, 25, 45),
    clock_border_color_rgba: (120, 80, 130, 100),
    clock_border_size: 1.0,
    clock_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= VOLUME/OUTPUT (STYLE) =================
    volume_output_text_size: 15,
    volume_output_text_orientation: Horizontal,
    volume_output_background_color_rgba: (30, 30, 36, 95),
    volume_output_button_color_rgb: (55, 45, 65),
    volume_output_button_text_color_rgb: (220, 220, 230),
    volume_output_button_hovered_color_rgb: (150, 45, 85),
    volume_output_button_hovered_text_color_rgb: (255, 255, 255),
    volume_output_button_pressed_color_rgb: (85, 30, 50),
    volume_output_border_color_rgba: (110, 80, 120, 100),
    volume_output_border_size: 1.0,
    volume_output_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= VOLUME/INPUT (STYLE) =================
    volume_input_text_size: 15,
    volume_input_text_orientation: Horizontal,
    volume_input_background_color_rgba: (30, 30, 36, 95),
    volume_input_button_color_rgb: (55, 45, 65),
    volume_input_button_text_color_rgb: (220, 220, 230),
    volume_input_button_hovered_color_rgb: (150, 45, 85),
    volume_input_button_hovered_text_color_rgb: (255, 255, 255),
    volume_input_button_pressed_color_rgb: (85, 30, 50),
    volume_input_border_color_rgba: (110, 80, 120, 100),
    volume_input_border_size: 1.0,
    volume_input_border_radius: (3.0, 3.0, 3.0, 3.0),


    // ================= HYPR WORKSPACES (STYLE) =================
    workspace_heigth: 30,
    workspace_width: 15,
    workspace_different_selected_width: None,
    workspace_text_size: 15,
    workspace_text_orientation: Horizontal,
    workspace_text: ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
    workspace_selected_text: Some(["●", "●", "●", "●", "●", "●", "●", "●", "●", "●"]),
    workspace_spacing: 3,
    workspace_background_color_rgba: (28, 28, 34, 95),
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
    custom_modules: [
    	//Example of an button that just runs an app or command
	(
		name: "Wofi Custom Module",
		text: "󰣇",
		text_orientation: Horizontal,
    		text_size: 15,
		height: 30,
    		button_color_rgb: (45, 40, 55),
    		button_text_color_rgb: (230, 230, 240),
    		button_hovered_color_rgb: (150, 40, 80),
    		button_hovered_text_color_rgb: (255, 255, 255),
    		button_pressed_color_rgb: (85, 30, 55),
    		border_color_rgba: (130, 90, 140, 100),
    		border_size: 1.0,
    		border_radius: (3.0, 3.0, 3.0, 3.0),
		command_to_exec_on_left_click: ["wofi", "--show", "drun"],
		command_to_exec_on_right_click: ["wofi", "--show", "run"],
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
	//	text: "Continous Output:",
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
	//	use_output_as_text: false,
	//	use_continous_output_as_text: true,
	//	all_output_as_text_format: "    {continous_output}",
        //      	output_text_limit_len: 50,
	//	continous_command: ["playerctl", "--player=spotify", "metadata", "--format", "{{ artist }} - {{ title }}"]
	//),
	//(
	//	name: "Playerctl Previous Button",
	//	text: "󰒮",
    	//	text_size: 15,
	//	height: 30,
    	//	button_color_rgb: (45, 40, 55),
    	//	button_text_color_rgb: (230, 230, 240),
    	//	button_hovered_color_rgb: (150, 40, 80),
    	//	button_hovered_text_color_rgb: (255, 255, 255),
    	//	button_pressed_color_rgb: (85, 30, 55),
    	//	border_color_rgba: (130, 90, 140, 100),
    	//	border_size: 1.0,
    	//	border_radius: (3.0, 3.0, 3.0, 3.0),
	//	command_to_exec_on_left_click: ["playerctl", "--player=spotify", "previous"],
	//),
	//(
	//	name: "Playerctl Play-Pause Button",
	//	text: "󰏤",
    	//	text_size: 15,
	//	height: 30,
    	//	button_color_rgb: (45, 40, 55),
    	//	button_text_color_rgb: (230, 230, 240),
    	//	button_hovered_color_rgb: (150, 40, 80),
    	//	button_hovered_text_color_rgb: (255, 255, 255),
    	//	button_pressed_color_rgb: (85, 30, 55),
    	//	border_color_rgba: (130, 90, 140, 100),
	//	border_size: 1.0,
	//	border_radius: (3.0, 3.0, 3.0, 3.0),
	//	all_output_as_text_format: "{continous_output}",
	//	use_continous_output_as_text: true,
	//	continous_command: ["bash", "-c", "case \"$(playerctl --player=spotify status 2>/dev/null)\" in Playing) printf \" ⏸ \" ;; Paused) printf \"▶\" ;; *) printf \"▶\" ;; esac"],
	//	command_to_exec_on_left_click: ["playerctl", "--player=spotify", "play-pause"],
	//),
	//(
	//	name: "Playerctl Next Button",
	//	text: "󰒭",
    	//	text_size: 15,
	//	height: 30,
    	//	button_color_rgb: (45, 40, 55),
    	//	button_text_color_rgb: (230, 230, 240),
    	//	button_hovered_color_rgb: (150, 40, 80),
    	//	button_hovered_text_color_rgb: (255, 255, 255),
    	//	button_pressed_color_rgb: (85, 30, 55),
    	//	border_color_rgba: (130, 90, 140, 100),
    	//	border_size: 1.0,
    	//	border_radius: (3.0, 3.0, 3.0, 3.0),
	//	command_to_exec_on_left_click: ["playerctl", "--player=spotify", "next"],
	//)
    ],
)"#;
        let mut file = File::create(ron_file_config_path).expect("Couldn't Create Default Config File");
        file.write_all(ron_default_data.as_bytes()).expect("Couldn't Create Default Config File");
    };
}
