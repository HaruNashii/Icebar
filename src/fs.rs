use std::{io::Write, fs, fs::File, path::Path};





pub fn check_if_config_file_exists()
{
    let home_path = home::home_dir().expect("Failed To Get Home Directory").display().to_string();
    let ron_config_dir = format!("{}/.config/icebar", home_path);
    let ron_config_file_dir = format!("{}/config.ron", ron_config_dir);

    let ron_file_config_path = Path::new(&ron_config_file_dir);
    let ron_config_path = Path::new(&ron_config_dir);

    if !Path::exists(ron_config_path)
    {
        println!("Ron config directory doesn't exist, Creating...");
        fs::create_dir_all(ron_config_path).expect("Couldn't Create Ron Config Directory");
    };

    if !Path::exists(ron_file_config_path)
    {
        println!("Ron config file doesn't exist, Creating...");
        let ron_default_data = r#"
// WARNING!!!: THE ALPHA OF THE RGBA HAS THE RANGE BETWEEN 0 TO 100, PARSING MORE THAN 100 WILL RESULT IN CRASH

BarConfig(
    display: Some("DP-2"),
    bar_position: "Up",
    // THE BAR SIZE FIRST OPTION DEFINED TO "0" WILL MAKE THE BAR FILL THE ENTIRE SCREEN X AXIS
    bar_size: (0, 45),
    bar_general_padding: 5,
    bar_background_color_rgba: (134, 206, 203, 90),

    left_modules: ["tray", "hypr/workspaces", "volume/output"],
    center_modules: ["clock", "tray"],
    right_modules: ["tray"],

    volume_format: " {}%",
    volume_muted_format: "[MUTED]",

    clock_format: "%H:%M",
    clock_alt_format: "%H:%M:%S | %a %b %e %Y",

    tray_background_color_rgba: (255, 255, 255, 100),
    tray_button: (0, 70, 255),
    tray_button_text: (0, 0, 0),
    tray_button_hovered: (0, 50, 255),
    tray_button_hovered_text: (255, 255, 255),
    tray_button_pressed: (0, 30, 70),
    tray_border_color: (0, 0, 0),
    tray_border_size: 1.0,

    clock_background_color_rgba: (255, 255, 255, 100),
    clock_button: (0, 70, 255),
    clock_button_text: (0, 0, 0),
    clock_button_hovered: (0, 50, 255),
    clock_button_hovered_text: (255, 255, 255),
    clock_button_pressed: (0, 30, 70),
    clock_border_color: (0, 0, 0),
    clock_border_size: 1.0,

    volume_output_background_color_rgba: (255, 255, 255, 100),
    volume_output_button: (0, 70, 255),
    volume_output_button_text: (0, 0, 0),
    volume_output_button_hovered: (0, 50, 255),
    volume_output_button_hovered_text: (255, 255, 255),
    volume_output_button_pressed: (0, 30, 70),
    volume_output_border_color: (0, 0, 0),
    volume_output_border_size: 1.0,

    hypr_workspace_background_color_rgba: (255, 255, 255, 100),
    hypr_workspace_button: (0, 70, 255),
    hypr_workspace_button_text: (0, 0, 0),
    hypr_workspace_button_hovered: (0, 50, 255),
    hypr_workspace_button_hovered_text: (255, 255, 255),
    hypr_workspace_button_pressed: (0, 30, 70),
    hypr_workspace_border_color: (0, 0, 0),
    hypr_workspace_border_size: 1.0,

    context_menu_width: 200,
    context_menu_background_color_rgba: (255, 255, 255, 100),
    context_menu_button: (0, 70, 255),
    context_menu_button_text: (0, 0, 0),
    context_menu_button_hovered: (0, 50, 255),
    context_menu_button_hovered_text: (255, 255, 255),
    context_menu_button_pressed: (0, 30, 70),
    context_menu_border_color: (0, 0, 0),
    context_menu_border_size: 1.0
)"#;
        let mut file = File::create(ron_file_config_path).expect("Couldn't Create Default Config File");
        file.write_all(ron_default_data.as_bytes()).expect("Couldn't Create Default Config File");
    };
}
