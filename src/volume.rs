// ============ IMPORTS ============
use std::process::Command;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct VolumeData
{
    pub volume_level: String
}

pub enum VolumeAction<'a>
{
    Get([&'a String;2]),
    Increase,
    Decrease,
    Mute
}





// ============ FUNCTIONS ============
pub fn volume(volume_modifier: VolumeAction) -> String
{
    match volume_modifier 
    {
        VolumeAction::Increase => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg("5%+").output().expect("Failed To Increase Volume With wpctl"),
        VolumeAction::Decrease => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg("5%-").output().expect("Failed To Decrease Volume With wpctl"),
        VolumeAction::Mute => Command::new("wpctl").arg("set-mute").arg("@DEFAULT_SINK@").arg("toggle").output().expect("Failed To Toggle-Mute With wpctl"),
        VolumeAction::Get([format, muted_format]) => 
        {
            let output = Command::new("wpctl").arg("get-volume").arg("@DEFAULT_SINK@").output().expect("Failed To Get Current Volume With wpctl");
            let stdout_bytes = output.stdout;
            let get_volume_output = String::from_utf8(stdout_bytes).unwrap();
            let mut is_muted = false;
            if get_volume_output.contains("[MUTED]") { is_muted = true };

            if is_muted
            {
                return muted_format.to_string();
            }
            else
            {
                let parsed = get_volume_output.replace("Volume: ", "").replace("[MUTED]", "").replace(" ", "").replace("\n", "").parse::<f32>().unwrap();
                let rounded_result = ((parsed * 100.0).round() as u32).to_string();
                return format.to_string().replace("{}", &rounded_result);
            };
        }
    };
    String::new()
}
