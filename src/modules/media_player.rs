// ============ imports ============
use std::process::Command;





// ============ CRATES ============
use crate::ron::BarConfig;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct MediaPlayerData
{
    pub metadata: String,
    pub status: String 
}

pub enum MediaPlayerAction
{
    PlayPause,
    Next,
    Prev
}





// ============ FUNCTIONS ============
pub fn get_player_data_with_format(ron_config: &BarConfig) -> MediaPlayerData
{
    let result_metadata_output = Command::new("playerctl").arg(format!("--player={}", ron_config.player)).arg("metadata").arg("--format").arg(&ron_config.media_player_metadata_format).output();
    let result_status_output = Command::new("playerctl").arg(format!("--player={}", ron_config.player)).arg("status").output();

    let metadata_string = if let Ok(metadata_output) = result_metadata_output
    {
        String::from_utf8_lossy(&metadata_output.stdout).to_string().replace("\n", "")
    }
    else
    {
        String::new()
    };

    let status_string = if let Ok(status_output) = result_status_output
    {
        String::from_utf8_lossy(&status_output.stdout).to_string().replace("\n", "")
    }
    else
    {
        String::new()
    };

    MediaPlayerData 
    {
        metadata: metadata_string, 
        status: status_string
    }
}



pub fn media_player_action(player: &String, action: MediaPlayerAction)
{
    match action
    {
        MediaPlayerAction::PlayPause => {let _ = Command::new("playerctl").arg(format!("--player={}", player)).arg("play-pause").output();}
        MediaPlayerAction::Next => {let _ = Command::new("playerctl").arg(format!("--player={}", player)).arg("next").output();}
        MediaPlayerAction::Prev => {let _ = Command::new("playerctl").arg(format!("--player={}", player)).arg("previous").output();}
    }
}
