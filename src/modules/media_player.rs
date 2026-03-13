// ============ IMPORTS ============
use iced::{Element, widget::container, Alignment, Theme, widget::button};
use std::process::Command;





// ============ CRATES ============
use crate::helpers::{string::{convert_text_to_rich_text}, style::{UserStyle, orient_text, set_style}};
use crate::update::Message;
use crate::ron::BarConfig;
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct MediaPlayerData
{
    pub metadata: String,
    pub status: String 
}

pub enum MediaPlayerAction
{
    VolumeDown,
    PlayPause,
    VolumeUp,
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
    let mut binding = Command::new("playerctl");
    let base_command = binding.arg(format!("--player={}", player));
    match action
    {
        MediaPlayerAction::PlayPause => {let _ = base_command.arg(format!("--player={}", player)).arg("play-pause").output();}
        MediaPlayerAction::Next => {let _ = base_command.arg(format!("--player={}", player)).arg("next").output();}
        MediaPlayerAction::Prev => {let _ = base_command.arg("previous").output();}
        MediaPlayerAction::VolumeUp => {let _ = base_command.arg("volume").arg("0.1+").output();}
        MediaPlayerAction::VolumeDown => {let _ = base_command.arg("volume").arg("0.1-").output();}
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



pub fn define_media_player_metadata_text(app: &AppData) -> String
{
    let mut metadata = &app.modules_data.media_player_data.metadata;
    if !app.ron_config.dont_show_metadata_if_empty && app.modules_data.media_player_data.metadata.is_empty()
    {
        metadata = &app.ron_config.text_when_metadata_is_empty;
    }
    orient_text(metadata, &app.ron_config.media_player_metadata_text_orientation)
}



pub fn define_media_player_buttons_text(app: &AppData) -> (String, String, String)
{
    let previous_text = &app.ron_config.media_player_buttons_format[0];
    let play_pause_text = if app.modules_data.media_player_data.status.contains("Playing")
    {
        &app.ron_config.media_player_buttons_format[1]
    }
    else
    {
        &app.ron_config.media_player_buttons_format[2]
    };
    let next_text = &app.ron_config.media_player_buttons_format[3];

    (
        orient_text(previous_text,     &app.ron_config.media_player_button_text_orientation),
        orient_text(play_pause_text,   &app.ron_config.media_player_button_text_orientation),
        orient_text(next_text,         &app.ron_config.media_player_button_text_orientation)
    ) 
}



pub fn define_button_data(previous_text: String, play_pause_text: String, next_text: String) -> Vec<(String, Message)>
{
    vec!
    [
        (
            previous_text,
            Message::MediaPlayerClickPrev
        ),
        (
            play_pause_text,
            Message::MediaPlayerClickPlayPause
        ),
        (
            next_text,
            Message::MediaPlayerClickNext
        ),
    ]
}



pub fn create_media_button<'a>(app: &'a AppData, padding: u16, label: String, message: Message, color: iced::Color) -> Element<'a, Message> 
{
    let colored_label = convert_text_to_rich_text::<Message>(&label, Some(color));
    container
    (
        button
        (
            colored_label
            .wrapping(iced::widget::text::Wrapping::Word)
            .font(app.default_font)
            .size(app.ron_config.media_player_button_text_size)
            .center()
        )
        .style(|_: &Theme, status: button::Status| 
        {
            define_media_player_buttons_style(app, status)
        }).on_press(message)).align_y(Alignment::Center).padding(padding).into()
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::helpers::string::ellipsize;
    use crate::modules::media_player::MediaPlayerData;
    use iced::{Background, Color};
    use iced::widget::button;
 
    fn make_style_app() -> AppData
    {
        let mut app = AppData::default();
        // metadata style colors
        app.ron_config.media_player_metadata_button_color_rgb         = [10, 20, 30];
        app.ron_config.media_player_metadata_button_hovered_color_rgb = [50, 60, 70];
        app.ron_config.media_player_metadata_button_pressed_color_rgb = [80, 90, 100];
        app.ron_config.media_player_metadata_button_text_color_rgb    = [200, 210, 220];
        app.ron_config.media_player_metadata_button_hovered_text_color_rgb = [255, 255, 255];
        // buttons style colors
        app.ron_config.media_player_button_color_rgb         = [1, 2, 3];
        app.ron_config.media_player_button_hovered_color_rgb = [4, 5, 6];
        app.ron_config.media_player_button_pressed_color_rgb = [7, 8, 9];
        app.ron_config.media_player_button_text_color_rgb    = [100, 100, 100];
        app.ron_config.media_player_button_hovered_text_color_rgb = [150, 150, 150];
        app
    }
 
    // ---- define_media_player_metadata_style ---------------------------------
 
    #[test]
    fn metadata_style_active_uses_metadata_normal_color()
    {
        let style = define_media_player_metadata_style(&make_style_app(), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(10, 20, 30))));
    }
 
    #[test]
    fn metadata_style_hovered_uses_metadata_hovered_color()
    {
        let style = define_media_player_metadata_style(&make_style_app(), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(50, 60, 70))));
    }
 
    #[test]
    fn metadata_style_pressed_uses_metadata_pressed_color()
    {
        let style = define_media_player_metadata_style(&make_style_app(), button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(80, 90, 100))));
    }
 
    #[test]
    fn metadata_style_active_text_color_correct()
    {
        let style = define_media_player_metadata_style(&make_style_app(), button::Status::Active);
        assert_eq!(style.text_color, Color::from_rgb8(200, 210, 220));
    }
 
    #[test]
    fn metadata_style_all_statuses_produce_background()
    {
        let app = make_style_app();
        for status in [button::Status::Active, button::Status::Hovered, button::Status::Pressed, button::Status::Disabled]
        {
            let style = define_media_player_metadata_style(&app, status);
            assert!(style.background.is_some(), "Expected background for {:?}", status);
        }
    }
 
    // ---- define_media_player_buttons_style ----------------------------------
 
    #[test]
    fn buttons_style_active_uses_button_normal_color()
    {
        let style = define_media_player_buttons_style(&make_style_app(), button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(1, 2, 3))));
    }
 
    #[test]
    fn buttons_style_hovered_uses_button_hovered_color()
    {
        let style = define_media_player_buttons_style(&make_style_app(), button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(4, 5, 6))));
    }
 
    #[test]
    fn buttons_style_pressed_uses_button_pressed_color()
    {
        let style = define_media_player_buttons_style(&make_style_app(), button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(7, 8, 9))));
    }
 
    #[test]
    fn metadata_style_and_buttons_style_have_different_active_backgrounds()
    {
        // The two style functions must not be accidentally swapped
        let app = make_style_app();
        let meta    = define_media_player_metadata_style(&app, button::Status::Active);
        let buttons = define_media_player_buttons_style( &app, button::Status::Active);
        assert_ne!(meta.background, buttons.background);
    }
 
    fn make_app(metadata: &str, status: &str) -> AppData
    {
        let mut app = AppData::default();
        app.modules_data.media_player_data = MediaPlayerData
        {
            metadata: metadata.into(),
            status: status.into(),
        };
        app.ron_config.media_player_metadata_text_limit_len = 20;
        app.ron_config.ellipsis_text = "...".into();
        app.ron_config.dont_show_metadata_if_empty = false;
        app.ron_config.text_when_metadata_is_empty = "No Media".into();
        app.ron_config.media_player_buttons_format = ["<<".into(), "||".into(), ">".into(), ">>".into()];
        app
    }
 
    // ---- define_media_player_metadata_text ----------------------------------
 
    #[test]
    fn metadata_text_short_returned_as_is()
    {
        let app = make_app("short title", "Playing");
        let result = define_media_player_metadata_text(&app);
        assert_eq!(result, "short title");
    }
 
    #[test]
    fn metadata_text_long_gets_ellipsized()
    {
        let app = make_app("this is a very long title that exceeds the limit", "Playing");
        let text = define_media_player_metadata_text(&app);
        let result  = ellipsize(&"...".to_string(), &text, 20);
        assert!(result.ends_with("..."));
        assert!(result.chars().count() <= 23); // 20 + "...".len()
    }
 
    #[test]
    fn metadata_text_empty_shows_fallback()
    {
        let app = make_app("", "Stopped");
        let result = define_media_player_metadata_text(&app);
        assert_eq!(result, "No Media");
    }
 
    // ---- define_media_player_buttons_text -----------------------------------
 
    #[test]
    fn buttons_text_playing_returns_pause_symbol()
    {
        let app = make_app("", "Playing");
        let (_prev, play_pause, _next) = define_media_player_buttons_text(&app);
        assert_eq!(play_pause, "||");
    }
 
    #[test]
    fn buttons_text_paused_returns_play_symbol()
    {
        let app = make_app("", "Paused");
        let (_prev, play_pause, _next) = define_media_player_buttons_text(&app);
        assert_eq!(play_pause, ">");
    }
 
    #[test]
    fn buttons_text_prev_and_next_always_same()
    {
        let app = make_app("", "Playing");
        let (prev, _pp, next) = define_media_player_buttons_text(&app);
        assert_eq!(prev, "<<");
        assert_eq!(next, ">>");
    }

    // ---- define_button_data -------------------------------------------------
 
    #[test]
    fn button_data_vec_has_three_entries()
    {
        let data = define_button_data("<<".into(), "||".into(), ">>".into());
        assert_eq!(data.len(), 3);
    }
 
    #[test]
    fn button_data_labels_are_correct()
    {
        let data = define_button_data("PREV".into(), "PLAY".into(), "NEXT".into());
        assert_eq!(data[0].0, "PREV");
        assert_eq!(data[1].0, "PLAY");
        assert_eq!(data[2].0, "NEXT");
    }
 
    #[test]
    fn button_data_messages_are_correct_variants()
    {
        use crate::update::Message;
        let data = define_button_data("".into(), "".into(), "".into());
        assert!(matches!(data[0].1, Message::MediaPlayerClickPrev));
        assert!(matches!(data[1].1, Message::MediaPlayerClickPlayPause));
        assert!(matches!(data[2].1, Message::MediaPlayerClickNext));
    }
 
    // ---- metadata text with orientation ------------------------------------
 
    #[test]
    fn metadata_text_vertical_orientation_inserts_newlines()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("abc", "Playing");
        app.ron_config.media_player_metadata_text_orientation = TextOrientation::Vertical;
        let result = define_media_player_metadata_text(&app);
        assert!(result.contains('\n'));
    }
 
    #[test]
    fn metadata_text_dont_show_if_empty_flag_hides_fallback()
    {
        let mut app = make_app("", "Stopped");
        app.ron_config.dont_show_metadata_if_empty = true;
        // When dont_show is true and metadata is empty, it should still use
        // the empty string (not the fallback), then ellipsize — result is empty.
        let result = define_media_player_metadata_text(&app);
        assert_eq!(result, "");
    }
}
