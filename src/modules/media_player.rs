// ============ imports ============
use std::process::Command;
use iced::widget::button;





// ============ CRATES ============
use crate::AppData;
use crate::helpers::style::{UserStyle, set_style};
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



pub fn define_media_player_metadata_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let hovered =              app.ron_config.media_player_metadata_button_hovered_color_rgb;
    let hovered_text =         app.ron_config.media_player_metadata_button_hovered_text_color_rgb;
    let pressed =              app.ron_config.media_player_metadata_button_pressed_color_rgb;
    let normal =               app.ron_config.media_player_metadata_button_color_rgb;
    let normal_text =          app.ron_config.media_player_metadata_button_text_color_rgb;
    let border_size =              app.ron_config.media_player_metadata_border_size;
    let border_color_rgba =    app.ron_config.media_player_metadata_border_color_rgba;
    let border_radius =       app.ron_config.media_player_metadata_border_radius;
    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
}



pub fn define_media_player_buttons_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let hovered =              app.ron_config.media_player_button_hovered_color_rgb;
    let hovered_text =         app.ron_config.media_player_button_hovered_text_color_rgb;
    let pressed =              app.ron_config.media_player_button_pressed_color_rgb;
    let normal =               app.ron_config.media_player_button_color_rgb;
    let normal_text =          app.ron_config.media_player_button_text_color_rgb;
    let border_size =              app.ron_config.media_player_button_border_size;
    let border_color_rgba =    app.ron_config.media_player_button_border_color_rgba;
    let border_radius =       app.ron_config.media_player_button_border_radius;
    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
}
