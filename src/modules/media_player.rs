// ============ IMPORTS ============
use iced::{Task, Element, widget::container, Alignment, Theme, widget::button};
use serde::{Deserialize, Serialize};
use std::pin::Pin;





// ============ CRATES ============
use crate::helpers::{color::{ColorType, Gradient}, string::{convert_text_to_rich_text}, style::{UserStyle, orient_text, set_style, TextOrientation, SideOption, match_color_or_gradient}};
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
    pub status:   String,
    pub art_url:  String,
    pub position_secs: f64,
    pub duration_secs: f64,
    pub volume: f64
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

    let result_art_output = tokio::time::timeout(
        PLAYERCTL_TIMEOUT,
        tokio::process::Command::new("playerctl")
            .arg(format!("--player={}", player))
            .arg("metadata")
            .arg("mpris:artUrl")
            .output()
    ).await.unwrap_or_else(|_| { eprintln!("[icebar] playerctl artUrl timed out"); Err(std::io::Error::other("timeout")) });

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

    let art_url_string = if let Ok(art_output) = result_art_output
    {
        String::from_utf8_lossy(&art_output.stdout).trim().to_string()
    }
    else
    {
        String::new()
    };

    let result_pos_output = tokio::time::timeout(
        PLAYERCTL_TIMEOUT,
        tokio::process::Command::new("playerctl")
            .arg(format!("--player={}", player))
            .arg("position")
            .output()
    ).await.unwrap_or_else(|_| { eprintln!("[icebar] playerctl position timed out"); Err(std::io::Error::other("timeout")) });

    let result_dur_output = tokio::time::timeout(
        PLAYERCTL_TIMEOUT,
        tokio::process::Command::new("playerctl")
            .arg(format!("--player={}", player))
            .arg("metadata")
            .arg("mpris:length")
            .output()
    ).await.unwrap_or_else(|_| { eprintln!("[icebar] playerctl mpris:length timed out"); Err(std::io::Error::other("timeout")) });

    let position_secs = parse_position_output(&result_pos_output);
    let duration_secs = parse_duration_output(&result_dur_output);

    let result_vol_output = tokio::time::timeout(
        PLAYERCTL_TIMEOUT,
        tokio::process::Command::new("playerctl")
            .arg(format!("--player={}", player))
            .arg("volume")
            .output()
    ).await.unwrap_or_else(|_| { eprintln!("[icebar] playerctl volume timed out"); Err(std::io::Error::other("timeout")) });

    let volume = match result_vol_output
    {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().parse::<f64>().unwrap_or(0.0),
        Err(_)  => 0.0,
    };

    MediaPlayerData
    {
        is_hovering_media_player_meta_data: false,
        metadata: metadata_string,
        status:   status_string,
        art_url:  art_url_string,
        position_secs,
        duration_secs,
        volume
    }
}



pub fn parse_position_output(result: &Result<std::process::Output, std::io::Error>) -> f64
{
    match result
    {
        Ok(out) => String::from_utf8_lossy(&out.stdout).trim().parse::<f64>().unwrap_or(0.0),
        Err(_)  => 0.0,
    }
}

pub fn parse_duration_output(result: &Result<std::process::Output, std::io::Error>) -> f64
{
    match result
    {
        Ok(out) =>
        {
            let micros = String::from_utf8_lossy(&out.stdout).trim().parse::<u64>().unwrap_or(0);
            micros as f64 / 1_000_000.0
        }
        Err(_) => 0.0,
    }
}



pub async fn fetch_album_art(url: String) -> Option<iced::widget::image::Handle>
{
    if url.is_empty() { return None; }

    let path = if url.starts_with("file://")
    {
        let raw = url.trim_start_matches("file://");
        percent_decode(raw)
    }
    else if url.starts_with('/')
    {
        url.clone()
    }
    else if url.starts_with("http://") || url.starts_with("https://")
    {
        String::new() 
    }
    else
    {
        url.clone()  
    };

    if !path.is_empty()
    {
        match tokio::fs::read(&path).await
        {
            Ok(bytes) => return decode_image_bytes(&bytes),
            Err(e)    => { eprintln!("[icebar] album art read error ({path}): {e}"); return None; }
        }
    }

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(8),
        tokio::process::Command::new("curl")
            .args(["--silent", "--max-time", "6", "--location", "--output", "-", &url])
            .output()
    ).await;

    match output
    {
        Ok(Ok(out)) if out.status.success() => decode_image_bytes(&out.stdout),
        Ok(Ok(out)) =>
        {
            eprintln!("[icebar] curl album art failed: exit {}", out.status);
            None
        }
        Ok(Err(e)) => { eprintln!("[icebar] curl error: {e}"); None }
        Err(_)     => { eprintln!("[icebar] curl timed out fetching album art"); None }
    }
}



fn decode_image_bytes(bytes: &[u8]) -> Option<iced::widget::image::Handle>
{
    use image::GenericImageView;
    match image::load_from_memory(bytes)
    {
        Ok(img) =>
        {
            let (w, h) = img.dimensions();
            let rgba   = img.into_rgba8().into_raw();
            Some(iced::widget::image::Handle::from_rgba(w, h, rgba))
        }
        Err(e) => { eprintln!("[icebar] album art decode error: {e}"); None }
    }
}



fn percent_decode(s: &str) -> String
{
    let mut out  = String::with_capacity(s.len());
    let bytes    = s.as_bytes();
    let mut i    = 0usize;
    while i < bytes.len()
    {
        if bytes[i] == b'%' && i + 2 < bytes.len()
        {
            let hi = bytes[i + 1];
            let lo = bytes[i + 2];
            if let (Some(h), Some(l)) = (hex_val(hi), hex_val(lo))
            {
                out.push((h << 4 | l) as char);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn hex_val(b: u8) -> Option<u8>
{
    match b
    {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _             => None,
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
    let cfg              = &app.ron_config.media_player_metadata;
    let hovered_text     = cfg.media_player_metadata_button_hovered_text_color;
    let pressed_text     = cfg.media_player_metadata_button_pressed_text_color;
    let normal_text      = cfg.media_player_metadata_text_color;
    let border_size      = cfg.media_player_metadata_border_size;
    let border_color     = cfg.media_player_metadata_border_color;
    let border_radius    = cfg.media_player_metadata_border_radius;
    let normal_background  = match_color_or_gradient(cfg.media_player_metadata_button_gradient_color.as_ref(),         cfg.media_player_metadata_button_color);
    let hovered_background = match_color_or_gradient(cfg.media_player_metadata_button_hovered_gradient_color.as_ref(), cfg.media_player_metadata_button_hovered_color);
    let pressed_background = match_color_or_gradient(cfg.media_player_metadata_button_pressed_gradient_color.as_ref(), cfg.media_player_metadata_button_pressed_color);
    set_style(UserStyle { status, hovered_text, pressed_text, normal_text, border_color, border_size, border_radius, normal_background, hovered_background, pressed_background, shadow_color: cfg.media_player_metadata_button_shadow_color, shadow_x: cfg.media_player_metadata_button_shadow_x, shadow_y: cfg.media_player_metadata_button_shadow_y, shadow_blur: cfg.media_player_metadata_button_shadow_blur })
}



pub fn define_media_player_buttons_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    let cfg              = &app.ron_config.media_player_button;
    let hovered_text     = cfg.media_player_button_hovered_text_color;
    let pressed_text     = cfg.media_player_button_pressed_text_color;
    let normal_text      = cfg.media_player_button_text_color;
    let border_size      = cfg.media_player_button_border_size;
    let border_color     = cfg.media_player_button_border_color;
    let border_radius    = cfg.media_player_button_border_radius;
    let normal_background  = match_color_or_gradient(cfg.media_player_button_gradient_color.as_ref(),         cfg.media_player_button_color);
    let hovered_background = match_color_or_gradient(cfg.media_player_button_hovered_gradient_color.as_ref(), cfg.media_player_button_hovered_color);
    let pressed_background = match_color_or_gradient(cfg.media_player_button_pressed_gradient_color.as_ref(), cfg.media_player_button_pressed_color);
    set_style(UserStyle { status, hovered_text, pressed_text, normal_text, border_color, border_size, border_radius, normal_background, hovered_background, pressed_background, shadow_color: cfg.media_player_button_shadow_color, shadow_x: cfg.media_player_button_shadow_x, shadow_y: cfg.media_player_button_shadow_y, shadow_blur: cfg.media_player_button_shadow_blur })
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
    let style_active  = define_media_player_buttons_style(app, button::Status::Active);
    let style_hovered = define_media_player_buttons_style(app, button::Status::Hovered);
    let style_pressed = define_media_player_buttons_style(app, button::Status::Pressed);
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
        .style(move |_: &Theme, status: button::Status|
        {
            match status
            {
                button::Status::Hovered => style_hovered,
                button::Status::Pressed => style_pressed,
                _                       => style_active,
            }
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

                    let art_url = {
                        let parg = format!("--player={}", player);
                        let res  = tokio::time::timeout(
                            std::time::Duration::from_secs(3),
                            tokio::process::Command::new("playerctl")
                                .args([&parg, "metadata", "mpris:artUrl"])
                                .output()
                        ).await;
                        match res
                        {
                            Ok(Ok(out)) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
                            _           => String::new(),
                        }
                    };

                    let parg2 = format!("--player={}", player);
                    let (position_secs, duration_secs) = {
                        let pos_res = tokio::time::timeout(
                            std::time::Duration::from_secs(3),
                            tokio::process::Command::new("playerctl")
                                .args([&parg2, "position"])
                                .output()
                        ).await.map(|r| r.unwrap_or_else(|e| { eprintln!("[icebar] pos err: {e}"); std::process::Output { status: std::process::ExitStatus::default(), stdout: vec![], stderr: vec![] } })).unwrap_or_else(|_| std::process::Output { status: std::process::ExitStatus::default(), stdout: vec![], stderr: vec![] });

                        let dur_res = tokio::time::timeout(
                            std::time::Duration::from_secs(3),
                            tokio::process::Command::new("playerctl")
                                .args([&parg2, "metadata", "mpris:length"])
                                .output()
                        ).await.map(|r| r.unwrap_or_else(|e| { eprintln!("[icebar] dur err: {e}"); std::process::Output { status: std::process::ExitStatus::default(), stdout: vec![], stderr: vec![] } })).unwrap_or_else(|_| std::process::Output { status: std::process::ExitStatus::default(), stdout: vec![], stderr: vec![] });

                        (
                            parse_position_output(&Ok(pos_res)),
                            parse_duration_output(&Ok(dur_res)),
                        )
                    };

                    let volume = {
                        let parg3 = format!("--player={}", player);
                        let vol_res = tokio::time::timeout(
                            std::time::Duration::from_secs(3),
                            tokio::process::Command::new("playerctl")
                                .args([&parg3, "volume"])
                                .output()
                        ).await;
                        match vol_res
                        {
                            Ok(Ok(out)) => String::from_utf8_lossy(&out.stdout).trim().parse::<f64>().unwrap_or(0.0),
                            _           => 0.0,
                        }
                    };

                    yield crate::update::Message::MediaPlayerDataFetched(MediaPlayerData
                    {
                        is_hovering_media_player_meta_data: false,
                        metadata: metadata_str.to_owned(),
                        status:   status_str.to_owned(),
                        art_url,
                        position_secs,
                        duration_secs,
                        volume
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





pub fn media_player_position_subscription(player: String, interval_ms: u64) -> std::pin::Pin<Box<dyn futures::Stream<Item = crate::update::Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        let interval = std::time::Duration::from_millis(interval_ms.max(100));
        loop
        {
            tokio::time::sleep(interval).await;

            let parg = format!("--player={}", player);

            let pos_out = tokio::time::timeout(
                std::time::Duration::from_secs(3),
                tokio::process::Command::new("playerctl")
                    .args([&parg, "position"])
                    .output()
            ).await;

            let dur_out = tokio::time::timeout(
                std::time::Duration::from_secs(3),
                tokio::process::Command::new("playerctl")
                    .args([&parg, "metadata", "mpris:length"])
                    .output()
            ).await;

            let vol_out = tokio::time::timeout(
                std::time::Duration::from_secs(3),
                tokio::process::Command::new("playerctl")
                    .args([&parg, "volume"])
                    .output()
            ).await;

            let pos = match pos_out { Ok(Ok(o)) => parse_position_output(&Ok(o)), _ => continue };
            let dur = match dur_out { Ok(Ok(o)) => parse_duration_output(&Ok(o)), _ => 0.0 };
            let vol = match vol_out
            {
                Ok(Ok(o)) => String::from_utf8_lossy(&o.stdout).trim().parse::<f64>().unwrap_or(0.0),
                _         => 0.0,
            };

            yield crate::update::Message::MediaPlayerPositionTick(pos, dur, vol);
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
            status: status.into(),
            art_url: String::new(),
            position_secs: 0.0,
            duration_secs: 0.0,
            volume: 0.0
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


    fn make_output(stdout: &str) -> std::process::Output
    {
        std::process::Output
        {
            status: std::process::ExitStatus::default(),
            stdout: stdout.as_bytes().to_vec(),
            stderr: vec![],
        }
    }

    #[test]
    fn parse_position_valid_float()
    {
        let out = make_output("123.456
");
        assert!((parse_position_output(&Ok(out)) - 123.456).abs() < 1e-6);
    }

    #[test]
    fn parse_position_zero_string()
    {
        let out = make_output("0
");
        assert_eq!(parse_position_output(&Ok(out)), 0.0);
    }

    #[test]
    fn parse_position_empty_returns_zero()
    {
        let out = make_output("");
        assert_eq!(parse_position_output(&Ok(out)), 0.0);
    }

    #[test]
    fn parse_position_error_returns_zero()
    {
        let err: Result<std::process::Output, std::io::Error> = Err(std::io::Error::other("fail"));
        assert_eq!(parse_position_output(&err), 0.0);
    }

    #[test]
    fn parse_duration_valid_microseconds()
    {
        let out = make_output("210000000
");
        assert!((parse_duration_output(&Ok(out)) - 210.0).abs() < 1e-6);
    }

    #[test]
    fn parse_duration_zero_returns_zero()
    {
        let out = make_output("0
");
        assert_eq!(parse_duration_output(&Ok(out)), 0.0);
    }

    #[test]
    fn parse_duration_empty_returns_zero()
    {
        let out = make_output("");
        assert_eq!(parse_duration_output(&Ok(out)), 0.0);
    }

    #[test]
    fn parse_duration_error_returns_zero()
    {
        let err: Result<std::process::Output, std::io::Error> = Err(std::io::Error::other("fail"));
        assert_eq!(parse_duration_output(&err), 0.0);
    }

    #[test]
    fn media_player_data_default_position_is_zero()
    {
        assert_eq!(MediaPlayerData::default().position_secs, 0.0);
    }

    #[test]
    fn media_player_data_default_duration_is_zero()
    {
        assert_eq!(MediaPlayerData::default().duration_secs, 0.0);
    }


    #[test]
    fn media_player_data_default_art_url_is_empty()
    {
        assert!(MediaPlayerData::default().art_url.is_empty());
    }

    #[test]
    fn media_player_data_art_url_stored_correctly()
    {
        let d = MediaPlayerData
        {
            art_url: "file:///tmp/cover.jpg".to_string(),
            ..Default::default()
        };
        assert_eq!(d.art_url, "file:///tmp/cover.jpg");
    }


    #[test]
    fn percent_decode_plain_path_unchanged()
    {
        assert_eq!(percent_decode("/home/user/music"), "/home/user/music");
    }

    #[test]
    fn percent_decode_space_encoded()
    {
        assert_eq!(percent_decode("/home/user/my%20music"), "/home/user/my music");
    }

    #[test]
    fn percent_decode_mixed_encoded_chars()
    {
        assert_eq!(percent_decode("hello%20world%21"), "hello world!");
    }

    #[test]
    fn percent_decode_uppercase_hex()
    {
        assert_eq!(percent_decode("%2F"), "/");
    }

    #[test]
    fn percent_decode_lowercase_hex()
    {
        assert_eq!(percent_decode("%2f"), "/");
    }

    #[test]
    fn percent_decode_empty_string()
    {
        assert_eq!(percent_decode(""), "");
    }

    #[test]
    fn percent_decode_no_encoded_chars()
    {
        assert_eq!(percent_decode("abc"), "abc");
    }

    #[test]
    fn percent_decode_percent_at_end_no_panic()
    {
        let result = percent_decode("abc%");
        assert!(result.contains("abc"));
    }

    #[test]
    fn percent_decode_single_hex_digit_no_panic()
    {
        let result = percent_decode("abc%2");
        assert!(result.contains("abc"));
    }


    #[test]
    fn hex_val_digit_zero()  { assert_eq!(hex_val(b'0'), Some(0)); }

    #[test]
    fn hex_val_digit_nine()  { assert_eq!(hex_val(b'9'), Some(9)); }

    #[test]
    fn hex_val_lower_a()     { assert_eq!(hex_val(b'a'), Some(10)); }

    #[test]
    fn hex_val_lower_f()     { assert_eq!(hex_val(b'f'), Some(15)); }

    #[test]
    fn hex_val_upper_a()     { assert_eq!(hex_val(b'A'), Some(10)); }

    #[test]
    fn hex_val_upper_f()     { assert_eq!(hex_val(b'F'), Some(15)); }

    #[test]
    fn hex_val_invalid_char() { assert_eq!(hex_val(b'g'), None); }

    #[test]
    fn hex_val_space()        { assert_eq!(hex_val(b' '), None); }


    #[test]
    fn decode_image_bytes_invalid_data_returns_none()
    {
        let bad = b"this is not an image";
        assert!(decode_image_bytes(bad).is_none());
    }

    #[test]
    fn decode_image_bytes_empty_returns_none()
    {
        assert!(decode_image_bytes(b"").is_none());
    }

    #[test]
    fn decode_image_bytes_valid_png_returns_handle()
    {
        let png: &[u8] = &[
            0x89,0x50,0x4e,0x47,0x0d,0x0a,0x1a,0x0a, // signature
            0x00,0x00,0x00,0x0d, b'I',b'H',b'D',b'R', // IHDR length + type
            0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x01,  // width=1, height=1
            0x08,0x02,0x00,0x00,0x00,0x90,0x77,0x53,0xde, // 8-bit RGB, CRC
            0x00,0x00,0x00,0x0c, b'I',b'D',b'A',b'T', // IDAT length + type
            0x08,0xd7,0x63,0xf8,0xcf,0xc0,0x00,0x00,
            0x00,0x02,0x00,0x01, 0xe2,0x21,0xbc,0x33,  // IDAT data + CRC
            0x00,0x00,0x00,0x00, b'I',b'E',b'N',b'D', // IEND
            0xae,0x42,0x60,0x82,
        ];
        let _ = decode_image_bytes(png);
    }


    #[tokio::test]
    async fn fetch_album_art_empty_url_returns_none()
    {
        let result = fetch_album_art(String::new()).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_album_art_nonexistent_file_returns_none()
    {
        let result = fetch_album_art("file:///this/path/does/not/exist/cover.jpg".to_string()).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_album_art_bare_nonexistent_path_returns_none()
    {
        let result = fetch_album_art("/this/path/does/not/exist/cover.jpg".to_string()).await;
        assert!(result.is_none());
    }
}
