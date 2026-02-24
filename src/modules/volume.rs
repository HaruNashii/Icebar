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
    GetOutput((&'a [String;6], &'a String)),
    GetInput((&'a [String;6], &'a String)),
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
        VolumeAction::IncreaseOutput(v) => {let _ = Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg(format!("{}%+", v)).output();},
        VolumeAction::DecreaseOutput(v) => {let _ = Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg(format!("{}%-", v)).output();},
        VolumeAction::MuteOutput => {let _ = Command::new("wpctl").arg("set-mute").arg("@DEFAULT_SINK@").arg("toggle").output();},
        VolumeAction::GetOutput((formats, muted_format)) => 
        {
            let output = Command::new("wpctl").arg("get-volume").arg("@DEFAULT_SINK@").output().expect("Failed To Get Current Volume With wpctl");
            let stdout_bytes = output.stdout;
            let get_volume_output = String::from_utf8(stdout_bytes).unwrap_or_default();
            let mut is_muted = false;
            if get_volume_output.contains("[MUTED]") { is_muted = true };

            if is_muted
            {
                return muted_format.to_string();
            }
            else
            {
                let parsed = get_volume_output.replace("Volume: ", "").replace("[MUTED]", "").replace(" ", "").replace("\n", "").parse::<f32>().unwrap_or_default();
                let thresholds = 
                [
                    (0.0, &formats[0]),
                    (0.240, &formats[1]),
                    (0.490, &formats[2]),
                    (0.900, &formats[3]),
                    (1.00, &formats[4]),
                    (999.9, &formats[5]),
                ];
                let format = thresholds.iter().find(|&&(max, _)| parsed <= max).map(|&(_, fmt)| fmt).unwrap_or(&formats[0]);
                let rounded_result = ((parsed * 100.0).round() as u32).to_string();
                return format.to_string().replace("{}", &rounded_result);
            };
        }

        VolumeAction::IncreaseInput(v) => {let _ = Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SOURCE@").arg(format!("{}%+", v)).output();},
        VolumeAction::DecreaseInput(v) => {let _ = Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SOURCE@").arg(format!("{}%-", v)).output();},
        VolumeAction::MuteInput => {let _ = Command::new("wpctl").arg("set-mute").arg("@DEFAULT_SOURCE@").arg("toggle").output();},
        VolumeAction::GetInput((formats, muted_format)) => 
        {
            let output = Command::new("wpctl").arg("get-volume").arg("@DEFAULT_SOURCE@").output().expect("Failed To Get Current Volume With wpctl");
            let stdout_bytes = output.stdout;
            let get_volume_output = String::from_utf8(stdout_bytes).unwrap_or_default();
            let mut is_muted = false;
            if get_volume_output.contains("[MUTED]") { is_muted = true };

            if is_muted
            {
                return muted_format.to_string();
            }
            else
            {
                let parsed = get_volume_output.replace("Volume: ", "").replace("[MUTED]", "").replace(" ", "").replace("\n", "").parse::<f32>().unwrap_or_default();
                let thresholds = 
                [
                    (0.0, &formats[0]),
                    (0.240, &formats[1]),
                    (0.490, &formats[2]),
                    (0.990, &formats[3]),
                    (1.00, &formats[4]),
                ];
                let format = thresholds.iter().find(|&&(max, _)| parsed <= max).map(|&(_, fmt)| fmt).unwrap_or(&formats[0]);
                let rounded_result = ((parsed * 100.0).round() as u32).to_string();
                return format.to_string().replace("{}", &rounded_result);
            };
        }
    };
    String::new()
}
