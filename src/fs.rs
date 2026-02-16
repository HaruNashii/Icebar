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
            BarConfig(
                bar_position: "Up",
                bar_size: 100
            )
        "#;
        let mut file = File::create(ron_file_config_path).expect("Couldn't Create Default Config File");
        file.write_all(ron_default_data.as_bytes()).expect("Couldn't Create Default Config File");
    };
}
