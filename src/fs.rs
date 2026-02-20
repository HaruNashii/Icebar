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
        let ron_default_data = r#"// WARNING!!!: THE ALPHA OF THE RGBA HAS THE RANGE BETWEEN 0 TO 100, PARSING MORE THAN 100 WILL RESULT IN CRASH
BarConfig(
    // ================= GENERAL =================
    display: Some("DP-2"),
    bar_position: "Up",
    // THE BAR SIZE FIRST OPTION DEFINED TO "0" WILL MAKE THE BAR FILL THE ENTIRE SCREEN X AXIS
    bar_size: (0, 45),
    bar_general_padding: 6,
    bar_background_color_rgba: (18, 18, 22, 92),


    // ================= MODULES =================
    left_modules: ["hypr/workspaces"],
    center_modules: ["clock"],
    right_modules: ["tray", "volume/output"],


    // ================= FORMATS =================
    volume_format: " {}%",
    volume_muted_format: "󰝟 muted",
    clock_format: "%H:%M",
    clock_alt_format: "%H:%M:%S | %a %d %b %Y",


    // ================= TRAY =================
    tray_background_color_rgba: (30, 30, 36, 0),
    tray_button_color_rgb: (60, 50, 70),
    tray_button_text_color_rgb: (220, 220, 230),
    tray_button_hovered_color_rgb: (110, 40, 80),
    tray_button_hovered_text_color_rgb: (255, 255, 255),
    tray_button_pressed_color_rgb: (70, 20, 40),
    tray_border_color_rgba: (90, 70, 100, 100),
    tray_border_size: 1.0,
    tray_border_radius: (6, 6, 6, 6),


    // ================= CLOCK =================
    clock_background_color_rgba: (25, 25, 30, 95),
    clock_button_color_rgb: (50, 45, 60),
    clock_button_text_color_rgb: (235, 235, 240),
    clock_button_hovered_color_rgb: (130, 35, 70),
    clock_button_hovered_text_color_rgb: (255, 255, 255),
    clock_button_pressed_color_rgb: (80, 25, 45),
    clock_border_color_rgba: (120, 80, 130, 100),
    clock_border_size: 1.0,
    clock_border_radius: (8, 8, 8, 8),


    // ================= VOLUME =================
    volume_output_background_color_rgba: (30, 30, 36, 95),
    volume_output_button_color_rgb: (55, 45, 65),
    volume_output_button_text_color_rgb: (220, 220, 230),
    volume_output_button_hovered_color_rgb: (150, 45, 85),
    volume_output_button_hovered_text_color_rgb: (255, 255, 255),
    volume_output_button_pressed_color_rgb: (85, 30, 50),
    volume_output_border_color_rgba: (110, 80, 120, 100),
    volume_output_border_size: 1.0,
    volume_output_border_radius: (6, 6, 6, 6),


    // ================= HYPR WORKSPACES =================
    hypr_workspace_text: ("A", "B", "C", "D", "E", "F", "G", "H", "I", "J"),
    hypr_workspace_background_color_rgba: (28, 28, 34, 95),
    hypr_workspace_button_color_rgb: (45, 40, 55),
    hypr_workspace_button_text_color_rgb: (200, 200, 210),
    hypr_workspace_button_selected_color_rgb: (150, 40, 80),
    hypr_workspace_button_hovered_color_rgb: (140, 35, 75),
    hypr_workspace_button_hovered_text_color_rgb: (255, 255, 255),
    hypr_workspace_button_pressed_color_rgb: (90, 25, 50),
    hypr_workspace_border_color_rgba: (120, 90, 135, 100),
    hypr_workspace_border_size: 1.0,
    hypr_workspace_border_radius: (6, 6, 6, 6),


    // ================= CONTEXT MENU =================
    context_menu_width: 200,
    context_menu_background_color_rgba: (20, 20, 24, 98),
    context_menu_button_color_rgb: (45, 40, 55),
    context_menu_button_text_color_rgb: (230, 230, 240),
    context_menu_button_hovered_color_rgb: (150, 40, 80),
    context_menu_button_hovered_text_color_rgb: (255, 255, 255),
    context_menu_button_pressed_color_rgb: (85, 30, 55),
    context_menu_border_color_rgba: (130, 90, 140, 100),
    context_menu_border_size: 1.0,
    context_menu_border_radius: (8, 8, 8, 8)
)"#;
        let mut file = File::create(ron_file_config_path).expect("Couldn't Create Default Config File");
        file.write_all(ron_default_data.as_bytes()).expect("Couldn't Create Default Config File");
    };
}
