use std::process::Command;





#[derive(Default, Clone)]
pub struct VolumeData
{
    pub volume_level: String,
}

pub enum VolumeAction
{
    Increase,
    Decrease,
    Mute,
    Get
}





pub fn volume(volume_modifier: VolumeAction) -> String
{
    let mut get_volume_output = String::new();

    match volume_modifier 
    {
        VolumeAction::Increase => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg("5%+").output().expect("Failed To Increase Volume With wpctl"),
        VolumeAction::Decrease => Command::new("wpctl").arg("set-volume").arg("@DEFAULT_SINK@").arg("5%-").output().expect("Failed To Decrease Volume With wpctl"),
        VolumeAction::Mute => Command::new("wpctl").arg("set-mute").arg("@DEFAULT_SINK@").arg("toggle").output().expect("Failed To Toggle-Mute With wpctl"),
        VolumeAction::Get => 
        {
            let output = Command::new("wpctl").arg("get-volume").arg("@DEFAULT_SINK@").output().expect("Failed To Get Current Volume With wpctl");
            let stdout_bytes = output.stdout;
            get_volume_output = String::from_utf8(stdout_bytes).unwrap();
            return get_volume_output
        }
    };

    get_volume_output
}
