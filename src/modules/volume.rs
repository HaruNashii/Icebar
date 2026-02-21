// ============ IMPORTS ============
use std::process::Command;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct VolumeData
{
    pub output_volume_level: String,
    pub input_volume_level: String
}

pub enum VolumeAction<'a>
{
    GetOutput([&'a String;2]),
    GetInput([&'a String;2]),
    IncreaseOutput(u8),
    DecreaseOutput(u8),
    IncreaseInput(u8),
    DecreaseInput(u8),
    MuteOutput,
    MuteInput
}





// ============ FUNCTIONS ============
pub fn volume(volume_modifier: VolumeAction) -> String
{
    match volume_modifier 
    {
        VolumeAction::IncreaseOutput(v) => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg(format!("{}%+", v)).output().expect("Failed To Increase Volume With wpctl"),
        VolumeAction::DecreaseOutput(v) => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg(format!("{}%-", v)).output().expect("Failed To Decrease Volume With wpctl"),
        VolumeAction::MuteOutput => Command::new("wpctl").arg("set-mute").arg("@DEFAULT_SINK@").arg("toggle").output().expect("Failed To Toggle-Mute With wpctl"),
        VolumeAction::GetOutput([format, muted_format]) => 
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

        VolumeAction::IncreaseInput(v) => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SOURCE@").arg(format!("{}%+", v)).output().expect("Failed To Increase Volume With wpctl"),
        VolumeAction::DecreaseInput(v) => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SOURCE@").arg(format!("{}%-", v)).output().expect("Failed To Decrease Volume With wpctl"),
        VolumeAction::MuteInput => Command::new("wpctl").arg("set-mute").arg("@DEFAULT_SOURCE@").arg("toggle").output().expect("Failed To Toggle-Mute With wpctl"),
        VolumeAction::GetInput([format, muted_format]) => 
        {
            let output = Command::new("wpctl").arg("get-volume").arg("@DEFAULT_SOURCE@").output().expect("Failed To Get Current Volume With wpctl");
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
