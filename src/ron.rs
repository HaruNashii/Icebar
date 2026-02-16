use ron::from_str;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};





#[derive(Debug, Deserialize, Serialize)]
pub struct BarConfig
{
   pub bar_position: String,
   pub bar_size: u32,
}

pub fn read_ron_config() -> BarConfig
{
    let home_path = home::home_dir().expect("Failed To Get Home Directory").display().to_string();
    let ron_config_file_dir = format!("{}/.config/icebar/config.ron", home_path);
    let ron_file_config_path = Path::new(&ron_config_file_dir);

    let ron_content = fs::read_to_string(ron_file_config_path).expect("Couldn't Read Config File");
    from_str(&ron_content).expect("Coudln't Translate Config File")
}
