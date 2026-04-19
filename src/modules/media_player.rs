// ============ IMPORTS ============
use iced::{Task, Element, widget::container, Alignment, Theme, widget::button};
use serde::{Deserialize, Serialize};
use std::pin::Pin;





// ============ CRATES ============
use crate::helpers::{color::{ColorType, Gradient}, string::{convert_text_to_rich_text}, style::{UserStyle, orient_text, set_style, TextOrientation, SideOption}};
use crate::ron::ActionOnClick;
use crate::update::Message;
use crate::AppData;





// ============ CONST/STATIC ============
const STATUS_PREFIX: &str = "__STATUS__:";





// ============ ENUM/STRUCTS, ETC... ============
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MediaPlayerMetadataConfig
{
    pub player:                                              String,
    pub media_player_metadata_format:                        String,
    pub media_player_metadata_update_interval:               u64,
    pub dont_show_metadata_if_empty:                         bool,
    pub text_when_metadata_is_empty:                         String,
    pub media_player_metadata_text_limit_len:                usize,
    pub action_on_left_click_media_player_metadata:          ActionOnClick,
    pub action_on_right_click_media_player_metadata:         ActionOnClick,
    pub media_player_metadata_padding:                       u16,
    pub media_player_metadata_text_size:                     u32,
    pub media_player_metadata_text_color:                    ColorType,
    pub media_player_metadata_text_orientation:              TextOrientation,
    pub media_player_metadata_button_color:                  ColorType,
    pub media_player_metadata_button_hovered_color:          ColorType,
    pub media_player_metadata_button_hovered_text_color:     ColorType,
    pub media_player_metadata_button_pressed_text_color:     ColorType,
    pub media_player_metadata_button_pressed_color:          ColorType,
    pub media_player_metadata_border_color:                  ColorType,
    pub media_player_metadata_border_size:                   f32,
    pub media_player_metadata_border_radius:                 [f32; 4],
    pub media_player_metadata_side_separator:                Option<SideOption>,
    pub media_player_metadata_side_separator_color:          ColorType,
    pub media_player_metadata_side_separator_width:          f32,
    pub media_player_metadata_side_separator_height:         f32,
    pub media_player_metadata_button_gradient_color:         Option<Gradient>,
    pub media_player_metadata_button_hovered_gradient_color: Option<Gradient>,
    pub media_player_metadata_button_pressed_gradient_color: Option<Gradient>,
    pub media_player_metadata_button_shadow_color:           Option<ColorType>,
    pub media_player_metadata_button_shadow_x:               f32,
    pub media_player_metadata_button_shadow_y:               f32,
    pub media_player_metadata_button_shadow_blur:            f32
}

impl Default for MediaPlayerMetadataConfig
{
    fn default() -> Self
    {
        Self
        {
            player:                                              "spotify".into(),
            media_player_metadata_format:                        "{{artist}} | {{album}} | {{title}}".into(),
            media_player_metadata_update_interval:               750,
            dont_show_metadata_if_empty:                         false,
            text_when_metadata_is_empty:                         "No Media Found.".into(),
            media_player_metadata_text_limit_len:                25,
            action_on_left_click_media_player_metadata:          ActionOnClick::DefaultAction,
            action_on_right_click_media_player_metadata:         ActionOnClick::DefaultAction,
            media_player_metadata_padding:                       0,
            media_player_metadata_text_size:                     15,
            media_player_metadata_text_color:                    ColorType::RGB([255, 255, 255]),
            media_player_metadata_text_orientation:              TextOrientation::Horizontal,
            media_player_metadata_button_color:                  ColorType::RGB([50, 45, 60]),
            media_player_metadata_button_hovered_color:          ColorType::RGB([130, 35, 70]),
            media_player_metadata_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            media_player_metadata_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            media_player_metadata_button_pressed_color:          ColorType::RGB([80, 25, 45]),
            media_player_metadata_border_color:                  ColorType::RGB([120, 80, 130]),
            media_player_metadata_border_size:                   1.0,
            media_player_metadata_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            media_player_metadata_side_separator:                None,
            media_player_metadata_side_separator_color:          ColorType::RGB([75, 75, 75]),
            media_player_metadata_side_separator_width:          1.,
            media_player_metadata_side_separator_height:         16.,
            media_player_metadata_button_gradient_color:         None,
            media_player_metadata_button_hovered_gradient_color: None,
            media_player_metadata_button_pressed_gradient_color: None,
            media_player_metadata_button_shadow_color:           None,
            media_player_metadata_button_shadow_x:               0.0,
            media_player_metadata_button_shadow_y:               0.0,
            media_player_metadata_button_shadow_blur:            0.0
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MediaPlayerButtonConfig
{
    pub media_player_buttons_format:                       [String; 4],
    pub media_player_button_spacing:                       u32,
    pub media_player_button_padding:                       u16,
    pub media_player_button_text_size:                     u32,
    pub media_player_button_text_color:                    ColorType,
    pub media_player_button_text_orientation:              TextOrientation,
    pub media_player_button_color:                         ColorType,
    pub media_player_button_hovered_color:                 ColorType,
    pub media_player_button_hovered_text_color:            ColorType,
    pub media_player_button_pressed_text_color:            ColorType,
    pub media_player_button_pressed_color:                 ColorType,
    pub media_player_button_border_color:                  ColorType,
    pub media_player_button_border_size:                   f32,
    pub media_player_button_border_radius:                 [f32; 4],
    pub media_player_buttons_side_separator:               Option<SideOption>,
    pub media_player_buttons_side_separator_color:         ColorType,
    pub media_player_buttons_side_separator_width:         f32,
    pub media_player_buttons_side_separator_height:        f32,
    pub media_player_button_gradient_color:                Option<Gradient>,
    pub media_player_button_hovered_gradient_color:        Option<Gradient>,
    pub media_player_button_pressed_gradient_color:        Option<Gradient>,
    pub media_player_button_shadow_color:                  Option<ColorType>,
    pub media_player_button_shadow_x:                      f32,
    pub media_player_button_shadow_y:                      f32,
    pub media_player_button_shadow_blur:                   f32
}

impl Default for MediaPlayerButtonConfig
{
    fn default() -> Self
    {
        Self
        {
            media_player_buttons_format:                       ["󰒮".into(),"⏸".into(),"▶".into(),"󰒭".into()],
            media_player_button_spacing:                       5,
            media_player_button_padding:                       0,
            media_player_button_text_size:                     15,
            media_player_button_text_color:                    ColorType::RGB([255, 255, 255]),
            media_player_button_text_orientation:              TextOrientation::Horizontal,
            media_player_button_color:                         ColorType::RGB([50, 45, 60]),
            media_player_button_hovered_color:                 ColorType::RGB([130, 35, 70]),
            media_player_button_hovered_text_color:            ColorType::RGB([255, 255, 255]),
            media_player_button_pressed_text_color:            ColorType::RGB([255, 255, 255]),
            media_player_button_pressed_color:                 ColorType::RGB([80, 25, 45]),
            media_player_button_border_color:                  ColorType::RGB([120, 80, 130]),
            media_player_button_border_size:                   1.0,
            media_player_button_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            media_player_buttons_side_separator:               None,
            media_player_buttons_side_separator_color:         ColorType::RGB([75, 75, 75]),
            media_player_buttons_side_separator_width:         1.,
            media_player_buttons_side_separator_height:        16.,
            media_player_button_gradient_color:                None,
            media_player_button_hovered_gradient_color:        None,
            media_player_button_pressed_gradient_color:        None,
            media_player_button_shadow_color:                  None,
            media_player_button_shadow_x:                      0.0,
            media_player_button_shadow_y:                      0.0,
            media_player_button_shadow_blur:                   0.0
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct MediaPlayerData
{
    pub is_hovering_media_player_meta_data: bool,
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
pub async fn get_player_data_with_format(player: &str, format: &str) -> MediaPlayerData
{
    const PLAYERCTL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

    let result_metadata_output = tokio::time::timeout(
        PLAYERCTL_TIMEOUT,
        tokio::process::Command::new("playerctl").arg(format!("--player={}", player)).arg("metadata").arg("--format").arg(format).output()
    ).await.unwrap_or_else(|_| { eprintln!("[icebar] playerctl metadata timed out"); Err(std::io::Error::other("timeout")) });

    let result_status_output = tokio::time::timeout(
        PLAYERCTL_TIMEOUT,
        tokio::process::Command::new("playerctl").arg(format!("--player={}", player)).arg("status").output()
    ).await.unwrap_or_else(|_| { eprintln!("[icebar] playerctl status timed out"); Err(std::io::Error::other("timeout")) });

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
        is_hovering_media_player_meta_data: false,
        metadata: metadata_string, 
        status: status_string
    }
}



pub fn media_player_action(player: &str, action: MediaPlayerAction) -> Task<crate::update::Message>
{
    let player = player.to_string();
    let arg = match action
    {
        MediaPlayerAction::PlayPause => "play-pause",
        MediaPlayerAction::Next      => "next",
        MediaPlayerAction::Prev      => "previous",
        MediaPlayerAction::VolumeUp  => "volume",
        MediaPlayerAction::VolumeDown => "volume"
    };
    let extra_arg = match action
    {
        MediaPlayerAction::VolumeUp   => Some("0.1+"),
        MediaPlayerAction::VolumeDown => Some("0.1-"),
        _                             => None
    };
    Task::perform(async move 
    {
        let mut cmd = tokio::process::Command::new("playerctl");
        cmd.arg(format!("--player={}", player)).arg(arg);
        if let Some(extra) = extra_arg { cmd.arg(extra); }
        let _ = tokio::time::timeout(std::time::Duration::from_secs(5), cmd.output()).await;
    },|_| Message::Nothing)
}



pub fn define_media_player_metadata_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let hovered =              app.ron_config.media_player_metadata.media_player_metadata_button_hovered_color;
    let hovered_text =         app.ron_config.media_player_metadata.media_player_metadata_button_hovered_text_color;
    let pressed_text =         app.ron_config.media_player_metadata.media_player_metadata_button_pressed_text_color;
    let pressed =              app.ron_config.media_player_metadata.media_player_metadata_button_pressed_color;
    let normal =               app.ron_config.media_player_metadata.media_player_metadata_button_color;
    let normal_text =          app.ron_config.media_player_metadata.media_player_metadata_text_color;
    let border_size =              app.ron_config.media_player_metadata.media_player_metadata_border_size;
    let border_color =    app.ron_config.media_player_metadata.media_player_metadata_border_color;
    let border_radius =       app.ron_config.media_player_metadata.media_player_metadata_border_radius;
    set_style(UserStyle { status, hovered, hovered_text, pressed_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.media_player_metadata.media_player_metadata_button_gradient_color.clone(), hovered_gradient: app.ron_config.media_player_metadata.media_player_metadata_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.media_player_metadata.media_player_metadata_button_pressed_gradient_color.clone(), shadow_color: app.ron_config.media_player_metadata.media_player_metadata_button_shadow_color, shadow_x: app.ron_config.media_player_metadata.media_player_metadata_button_shadow_x, shadow_y: app.ron_config.media_player_metadata.media_player_metadata_button_shadow_y, shadow_blur: app.ron_config.media_player_metadata.media_player_metadata_button_shadow_blur })
}



pub fn define_media_player_buttons_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let hovered =              app.ron_config.media_player_button.media_player_button_hovered_color;
    let hovered_text =         app.ron_config.media_player_button.media_player_button_hovered_text_color;
    let pressed_text =         app.ron_config.media_player_button.media_player_button_pressed_text_color;
    let pressed =              app.ron_config.media_player_button.media_player_button_pressed_color;
    let normal =               app.ron_config.media_player_button.media_player_button_color;
    let normal_text =          app.ron_config.media_player_button.media_player_button_text_color;
    let border_size =              app.ron_config.media_player_button.media_player_button_border_size;
    let border_color =    app.ron_config.media_player_button.media_player_button_border_color;
    let border_radius =       app.ron_config.media_player_button.media_player_button_border_radius;
    set_style(UserStyle { status, hovered, hovered_text, pressed_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.media_player_button.media_player_button_gradient_color.clone(), hovered_gradient: app.ron_config.media_player_button.media_player_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.media_player_button.media_player_button_pressed_gradient_color.clone(), shadow_color: app.ron_config.media_player_button.media_player_button_shadow_color, shadow_x: app.ron_config.media_player_button.media_player_button_shadow_x, shadow_y: app.ron_config.media_player_button.media_player_button_shadow_y, shadow_blur: app.ron_config.media_player_button.media_player_button_shadow_blur })
}



pub fn define_media_player_metadata_text(app: &AppData) -> String
{
    let metadata = &app.modules_data.media_player_data.metadata;

    if app.modules_data.media_player_data.metadata.is_empty()
    {
        if app.ron_config.media_player_metadata.dont_show_metadata_if_empty
        {
            return String::new();
        }
        return orient_text(
            &app.ron_config.media_player_metadata.text_when_metadata_is_empty,
            &app.ron_config.media_player_metadata.media_player_metadata_text_orientation,
        );
    }

    orient_text(metadata, &app.ron_config.media_player_metadata.media_player_metadata_text_orientation)
}



pub fn define_media_player_buttons_text(app: &AppData) -> (String, String, String)
{
    let previous_text = &app.ron_config.media_player_button.media_player_buttons_format[0];
    let play_pause_text = if app.modules_data.media_player_data.status.contains("Playing")
    {
        &app.ron_config.media_player_button.media_player_buttons_format[1]
    }
    else
    {
        &app.ron_config.media_player_button.media_player_buttons_format[2]
    };
    let next_text = &app.ron_config.media_player_button.media_player_buttons_format[3];

    (
        orient_text(previous_text,     &app.ron_config.media_player_button.media_player_button_text_orientation),
        orient_text(play_pause_text,   &app.ron_config.media_player_button.media_player_button_text_orientation),
        orient_text(next_text,         &app.ron_config.media_player_button.media_player_button_text_orientation)
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



pub fn create_media_button<'a>(app: &'a AppData, padding: u16, label: String, message: Message) -> Element<'a, Message> 
{
    let colored_label = convert_text_to_rich_text::<Message>(&label);
    container
    (
        button
        (
            colored_label
            .wrapping(iced::widget::text::Wrapping::Word)
            .font(app.default_font)
            .size(app.ron_config.media_player_button.media_player_button_text_size)
            .center()
        )
        .style(|_: &Theme, status: button::Status| 
        {
            define_media_player_buttons_style(app, status)
        }).on_press(message)).align_y(Alignment::Center).padding(padding).into()
}



pub fn media_player_subscription(player: String, format: String) -> Pin<Box<dyn futures::Stream<Item = crate::update::Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        let init = get_player_data_with_format(&player, &format).await;
        yield crate::update::Message::MediaPlayerDataFetched(init);

        loop
        {
            let combined_format = format!("{}{}\t{}", STATUS_PREFIX, "{{status}}", format);
            let player_arg = format!("--player={}", player);

            let child = tokio::process::Command::new("playerctl")
                .args([&player_arg, "--follow", "metadata", "--format", &combined_format])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn();

            let mut child = match child
            {
                Ok(c) => c,
                Err(e) =>
                {
                    eprintln!("[icebar] media_player_subscription: spawn error: {e}");
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    continue;
                }
            };

            let stdout = match child.stdout.take()
            {
                Some(s) => s,
                None =>
                {
                    child.kill().await.ok();
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    continue;
                }
            };

            use tokio::io::{AsyncBufReadExt, BufReader};
            let mut lines = BufReader::new(stdout).lines();

            while let Ok(Some(line)) = lines.next_line().await
            {
                if let Some(rest) = line.strip_prefix(STATUS_PREFIX)
                {
                    let (status_str, metadata_str) = if let Some(idx) = rest.find('\t')
                    {
                        (&rest[..idx], &rest[idx + 1..])
                    }
                    else
                    {
                        (rest, "")
                    };

                    yield crate::update::Message::MediaPlayerDataFetched(MediaPlayerData
                    {
                        is_hovering_media_player_meta_data: false,
                        metadata: metadata_str.to_owned(),
                        status:   status_str.to_owned()
                    });
                }
            }

            child.kill().await.ok();
            let _ = child.wait().await;
            eprintln!("[icebar] media_player_subscription: playerctl exited — retrying in 3s");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    })
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use crate::helpers::{color::ColorType, string::ellipsize};
    use crate::modules::media_player::MediaPlayerData;
    use iced::{widget::button, Background, Color};
 
    fn make_style_app() -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.ron_config.media_player_metadata.media_player_metadata_button_color = ColorType::RGB([10, 20, 30]);
        app.ron_config.media_player_metadata.media_player_metadata_button_hovered_color = ColorType::RGB([50, 60, 70]);
        app.ron_config.media_player_metadata.media_player_metadata_button_pressed_color = ColorType::RGB([80, 90, 100]);
        app.ron_config.media_player_metadata.media_player_metadata_text_color = ColorType::RGB([200, 210, 220]);
        app.ron_config.media_player_metadata.media_player_metadata_button_hovered_text_color = ColorType::RGB([255, 255, 255]);
        app.ron_config.media_player_button.media_player_button_color = ColorType::RGB([1, 2, 3]);
        app.ron_config.media_player_button.media_player_button_hovered_color = ColorType::RGB([4, 5, 6]);
        app.ron_config.media_player_button.media_player_button_pressed_color = ColorType::RGB([7, 8, 9]);
        app.ron_config.media_player_button.media_player_button_text_color = ColorType::RGB([100, 100, 100]);
        app.ron_config.media_player_button.media_player_button_hovered_text_color = ColorType::RGB([150, 150, 150]);
        app
    }
 
 
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
        let app = make_style_app();
        let meta    = define_media_player_metadata_style(&app, button::Status::Active);
        let buttons = define_media_player_buttons_style( &app, button::Status::Active);
        assert_ne!(meta.background, buttons.background);
    }
 
    fn make_app(metadata: &str, status: &str) -> AppData
    {
        let mut app = AppData { ..Default::default() };
        app.modules_data.media_player_data = MediaPlayerData
        {
            is_hovering_media_player_meta_data: false,
            metadata: metadata.into(),
            status: status.into()
        };
        app.ron_config.media_player_metadata.media_player_metadata_text_limit_len = 20;
        app.ron_config.general.ellipsis_text = "...".into();
        app.ron_config.media_player_metadata.dont_show_metadata_if_empty = false;
        app.ron_config.media_player_metadata.text_when_metadata_is_empty = "No Media".into();
        app.ron_config.media_player_button.media_player_buttons_format = ["<<".into(), "||".into(), ">".into(), ">>".into()];
        app
    }
 
 
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
        assert!(result.chars().count() <= 23);
    }
 
    #[test]
    fn metadata_text_empty_shows_fallback()
    {
        let app = make_app("", "Stopped");
        let result = define_media_player_metadata_text(&app);
        assert_eq!(result, "No Media");
    }
 
 
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
 
 
    #[test]
    fn metadata_text_vertical_orientation_inserts_newlines()
    {
        use crate::helpers::style::TextOrientation;
        let mut app = make_app("abc", "Playing");
        app.ron_config.media_player_metadata.media_player_metadata_text_orientation = TextOrientation::Vertical;
        let result = define_media_player_metadata_text(&app);
        assert!(result.contains('\n'));
    }
 
    #[test]
    fn metadata_text_dont_show_if_empty_flag_hides_fallback()
    {
        let mut app = make_app("", "Stopped");
        app.ron_config.media_player_metadata.dont_show_metadata_if_empty = true;
        let result = define_media_player_metadata_text(&app);
        assert_eq!(result, "");
    }



    #[test]
    fn define_button_data_returns_three_entries()
    {
        let data = define_button_data("prev".into(), "play".into(), "next".into());
        assert_eq!(data.len(), 3);
    }

    #[test]
    fn define_button_data_first_entry_is_previous()
    {
        let data = define_button_data("PREV".into(), "PLAY".into(), "NEXT".into());
        assert_eq!(data[0].0, "PREV");
        assert!(matches!(data[0].1, Message::MediaPlayerClickPrev));
    }

    #[test]
    fn define_button_data_second_entry_is_play_pause()
    {
        let data = define_button_data("p".into(), "pp".into(), "n".into());
        assert_eq!(data[1].0, "pp");
        assert!(matches!(data[1].1, Message::MediaPlayerClickPlayPause));
    }

    #[test]
    fn define_button_data_third_entry_is_next()
    {
        let data = define_button_data("p".into(), "pp".into(), "NEXT".into());
        assert_eq!(data[2].0, "NEXT");
        assert!(matches!(data[2].1, Message::MediaPlayerClickNext));
    }


    #[test]
    fn buttons_text_playing_returns_pause_format()
    {
        let mut app = make_app("track", "Playing");
        app.ron_config.media_player_button.media_player_buttons_format = ["<<".into(), "||".into(), ">".into(), ">>".into()];
        let (_, play_pause, _) = define_media_player_buttons_text(&app);
        assert_eq!(play_pause, "||");
    }

    #[test]
    fn buttons_text_not_playing_returns_play_format()
    {
        let mut app = make_app("track", "Stopped");
        app.ron_config.media_player_button.media_player_buttons_format = ["<<".into(), "||".into(), ">".into(), ">>".into()];
        let (_, play_pause, _) = define_media_player_buttons_text(&app);
        assert_eq!(play_pause, ">");
    }

    #[test]
    fn buttons_text_prev_and_next_always_same_regardless_of_status()
    {
        for status in ["Playing", "Stopped", "Paused"]
        {
            let mut app = make_app("x", status);
            app.ron_config.media_player_button.media_player_buttons_format = ["<<".into(), "||".into(), ">".into(), ">>".into()];
            let (prev, _, next) = define_media_player_buttons_text(&app);
            assert_eq!(prev, "<<");
            assert_eq!(next, ">>");
        }
    }


    #[test]
    fn metadata_text_empty_metadata_with_dont_show_false_returns_placeholder()
    {
        let app = make_app("", "Stopped");
        let result = define_media_player_metadata_text(&app);
        assert_eq!(result, "No Media");
    }

    #[test]
    fn metadata_text_nonempty_metadata_returns_metadata_itself()
    {
        let app = make_app("Artist - Song", "Playing");
        let result = define_media_player_metadata_text(&app);
        assert_eq!(result, "Artist - Song");
    }

    #[test]
    fn metadata_text_empty_metadata_with_dont_show_true_returns_empty()
    {
        let mut app = make_app("", "Stopped");
        app.ron_config.media_player_metadata.dont_show_metadata_if_empty = true;
        assert_eq!(define_media_player_metadata_text(&app), "");
    }


    #[test]
    fn media_player_metadata_config_default_text_size_is_positive()
    {
        assert!(MediaPlayerMetadataConfig::default().media_player_metadata_text_size > 0);
    }

    #[test]
    fn media_player_button_config_default_has_four_format_entries()
    {
        use crate::modules::media_player::MediaPlayerButtonConfig;
        assert_eq!(MediaPlayerButtonConfig::default().media_player_buttons_format.len(), 4);
    }
}
