// ============ IMPORTS ============
use iced::{Alignment, Element, Length, Task, Theme, widget::{button, column, container, mouse_area, row, scrollable, stack, Space}};
use iced_layershell::reexport::{Anchor, Layer, NewLayerShellSettings, KeyboardInteractivity};
use serde::{Deserialize, Serialize};





// ============ CRATES ============
use crate::helpers::{color::{ColorType, Gradient, hex_color}, string::convert_text_to_rich_text, style::{UserStyle, set_style}};
use crate::windows::context_menu::smart_popup_position;
use crate::{AppData, WindowInfo};
use crate::ron::BarPosition;
use crate::update::Message;





// ============ ENUM/STRUCT, ETC ============
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum PlayerRowElement
{
    Prev,
    PlayPause,
    Next,
    VolumeDown,
    VolumeUp,
    Metadata,
    Fill,
    ProgressBar,
    VolumeBar
}

#[derive(Clone, Debug, Default)]
pub struct MediaPlayerWindowData
{
    pub is_open:          bool,
    pub cursor_inside:    bool,
    pub mouse_pos:        (i32, i32),
    pub vol_bar_mouse_x:  f32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MediaWindowButtonStyle
{
    pub color:                  ColorType,
    pub hovered_color:          ColorType,
    pub pressed_color:          ColorType,
    pub text_color:             ColorType,
    pub hovered_text_color:     ColorType,
    pub pressed_text_color:     ColorType,
    pub border_color:           ColorType,
    pub border_size:            f32,
    pub border_radius:          [f32; 4],
    pub gradient_color:         Option<Gradient>,
    pub hovered_gradient_color: Option<Gradient>,
    pub pressed_gradient_color: Option<Gradient>
}

impl Default for MediaWindowButtonStyle
{
    fn default() -> Self
    {
        Self
        {
            color:                  hex_color("303030"),
            hovered_color:          hex_color("3d3d3d"),
            pressed_color:          hex_color("1c1c1c"),
            text_color:             hex_color("ffffff"),
            hovered_text_color:     hex_color("ffffff"),
            pressed_text_color:     hex_color("ffffff"),
            border_color:           hex_color("ffffff"),
            border_size:            0.0,
            border_radius:          [6., 6., 6., 6.],
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        }
    }
}

impl MediaWindowButtonStyle
{
    pub fn to_iced_style(&self, status: button::Status) -> iced::widget::button::Style
    {
        set_style(UserStyle
        {
            status,
            normal_text:        self.text_color,
            hovered_text:       self.hovered_text_color,
            pressed_text:       self.pressed_text_color,
            border_color:       self.border_color,
            border_size:        self.border_size,
            border_radius:      self.border_radius,
            normal_background:  crate::helpers::style::match_color_or_gradient(self.gradient_color.as_ref(),         self.color),
            hovered_background: crate::helpers::style::match_color_or_gradient(self.hovered_gradient_color.as_ref(), self.hovered_color),
            pressed_background: crate::helpers::style::match_color_or_gradient(self.pressed_gradient_color.as_ref(), self.pressed_color),
            shadow_color:       None,
            shadow_x:           0.,
            shadow_y:           0.,
            shadow_blur:        0.,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MediaWindowScrollbarConfig
{
    pub show:                    bool,
    pub width:                   u16,
    pub margin:                  u16,
    pub scroller_width:          u16,
    pub border_radius:           [f32; 4],
    pub rail_color:              ColorType,
    pub rail_border_color:       ColorType,
    pub rail_border_width:       f32,
    pub scroller_color:          ColorType,
    pub scroller_hovered_color:  ColorType,
    pub scroller_dragging_color: ColorType,
    pub scroller_border_color:   ColorType,
    pub scroller_border_width:   f32
}

impl Default for MediaWindowScrollbarConfig
{
    fn default() -> Self
    {
        Self
        {
            show:                    false,
            width:                   6,
            margin:                  2,
            scroller_width:          6,
            border_radius:           [3., 3., 3., 3.],
            rail_color:              ColorType::RGBA([0, 0, 0, 0]),
            rail_border_color:       ColorType::RGBA([0, 0, 0, 0]),
            rail_border_width:       0.,
            scroller_color:          ColorType::RGBA([100, 100, 100, 60]),
            scroller_hovered_color:  ColorType::RGBA([130, 130, 130, 80]),
            scroller_dragging_color: ColorType::RGBA([160, 160, 160, 100]),
            scroller_border_color:   ColorType::RGBA([0, 0, 0, 0]),
            scroller_border_width:   0.
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum AlbumArtPosition
{
    Left,
    Right,
    #[default]
    Top,
    Bottom
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum VolumeBarLabelPosition
{
    Left,
    Right
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MediaPlayerWindowConfig
{
    pub window_size:                  [u32; 2],
    pub window_padding:               u16,
    pub background_color:             ColorType,
    pub background_border_color:      ColorType,
    pub background_border_size:       f32,
    pub background_border_radius:     [f32; 4],
    pub row_order:                    Vec<PlayerRowElement>,
    pub row_height:                   u32,
    pub row_spacing:                  u16,
    pub section_spacing:              u16,
    pub metadata_text_size:           u32,
    pub metadata_text_color:          ColorType,
    pub metadata_max_chars:           usize,
    pub metadata_button_style:        MediaWindowButtonStyle,
    pub metadata_button_height:       u32,
    pub metadata_button_width:        f32,
    pub prev_label:                   String,
    pub play_label:                   String,
    pub pause_label:                  String,
    pub next_label:                   String,
    pub vol_down_label:               String,
    pub vol_up_label:                 String,
    pub button_text_size:             u32,
    pub button_width:                 u32,
    pub button_height:                u32,
    pub button_style:                 MediaWindowButtonStyle,
    pub play_button_style:            MediaWindowButtonStyle,
    pub pause_button_style:           MediaWindowButtonStyle,
    pub show_album_art:               bool,
    pub album_art_size:               u32,
    pub album_art_border_radius:      [f32; 4],
    pub album_art_border_color:       ColorType,
    pub album_art_border_size:        f32,
    pub album_art_spacing:            u16,
    pub album_art_placeholder_color:  ColorType,
    pub album_art_position:           AlbumArtPosition,
    pub progress_bar_height:          u32,
    pub progress_bar_filled_color:    ColorType,
    pub progress_bar_track_color:     ColorType,
    pub progress_bar_border_color:    ColorType,
    pub progress_bar_border_size:     f32,
    pub progress_bar_border_radius:   [f32; 4],
    pub progress_bar_seekable:        bool,
    pub progress_and_volume_bar_poll_interval_ms: u64,
    pub volume_bar_width:             f32,
    pub volume_bar_height:            u32,
    pub volume_bar_filled_color:      ColorType,
    pub volume_bar_track_color:       ColorType,
    pub volume_bar_border_color:      ColorType,
    pub volume_bar_border_size:       f32,
    pub volume_bar_border_radius:     [f32; 4],
    pub volume_bar_clickable:         bool,
    pub volume_bar_label_format:      Option<String>,
    pub volume_bar_label_position:    VolumeBarLabelPosition,
    pub volume_bar_label_size:        u16,
    pub volume_bar_label_color:       ColorType,
    pub volume_bar_label_spacing:     f32,
    pub scrollbar:                    MediaWindowScrollbarConfig
}

impl Default for MediaPlayerWindowConfig
{
    fn default() -> Self
    {
        Self
        {
            window_size:                  [340, 500],
            window_padding:               18,
            background_color:             hex_color("1c1c1c"),
            background_border_color:      hex_color("3d3d3d"),
            background_border_size:       1.0,
            background_border_radius:     [16., 16., 16., 16.],
            row_order: vec!
            [
                PlayerRowElement::Fill,
                PlayerRowElement::Prev,
                PlayerRowElement::PlayPause,
                PlayerRowElement::Next,
                PlayerRowElement::Fill,
            ],
            row_height:                   54,
            row_spacing:                  14,
            section_spacing:              18,
            metadata_text_size:           15,
            metadata_text_color:          hex_color("ffffff"),
            metadata_max_chars:           80,
            metadata_button_style: MediaWindowButtonStyle
            {
                color:              ColorType::RGBA([0, 0, 0, 0]),
                hovered_color:      ColorType::RGBA([0, 0, 0, 0]),
                pressed_color:      ColorType::RGBA([0, 0, 0, 0]),
                text_color:         hex_color("ffffff"),
                hovered_text_color: hex_color("ffffff"),
                pressed_text_color: hex_color("ffffff"),
                border_color:       ColorType::RGBA([0, 0, 0, 0]),
                border_size:        0.0,
                border_radius:      [0., 0., 0., 0.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            metadata_button_height:       24,
            metadata_button_width:        0.0,
            button_text_size:             16,
            button_width:                 40,
            button_height:                40,
            button_style: MediaWindowButtonStyle
            {
                color:              ColorType::RGBA([0, 0, 0, 0]),
                hovered_color:      hex_color("2b2b2b"),
                pressed_color:      hex_color("1c1c1c"),
                text_color:         hex_color("d8d8d8"),
                hovered_text_color: hex_color("ffffff"),
                pressed_text_color: hex_color("a1a1a1"),
                border_color:       ColorType::RGBA([0, 0, 0, 0]),
                border_size:        0.0,
                border_radius:      [20., 20., 20., 20.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            play_button_style: MediaWindowButtonStyle
            {
                color:              hex_color("ffffff"),
                hovered_color:      hex_color("e6e6e6"),
                pressed_color:      hex_color("cfcfcf"),
                text_color:         hex_color("1c1c1c"),
                hovered_text_color: hex_color("1c1c1c"),
                pressed_text_color: hex_color("1c1c1c"),
                border_color:       ColorType::RGBA([0, 0, 0, 0]),
                border_size:        0.0,
                border_radius:      [20., 20., 20., 20.],
                gradient_color: Some(Gradient::Gradient((
                    180.0,
                    vec![
                        (0.0, hex_color("ffffff")),
                        (1.0, hex_color("d0d0d0")),
                    ],
                ))),
                hovered_gradient_color: Some(Gradient::Gradient((
                    180.0,
                    vec![
                        (0.0, hex_color("ffffff")),
                        (1.0, hex_color("dedede")),
                    ],
                ))),
                pressed_gradient_color: None
            },
            pause_button_style: MediaWindowButtonStyle
            {
                color:              hex_color("ffffff"),
                hovered_color:      hex_color("e6e6e6"),
                pressed_color:      hex_color("cfcfcf"),
                text_color:         hex_color("1c1c1c"),
                hovered_text_color: hex_color("1c1c1c"),
                pressed_text_color: hex_color("1c1c1c"),
                border_color:       ColorType::RGBA([0, 0, 0, 0]),
                border_size:        0.0,
                border_radius:      [20., 20., 20., 20.],
                gradient_color: Some(Gradient::Gradient((
                    180.0,
                    vec![
                        (0.0, hex_color("ffffff")),
                        (1.0, hex_color("d0d0d0")),
                    ],
                ))),
                hovered_gradient_color: Some(Gradient::Gradient((
                    180.0,
                    vec![
                        (0.0, hex_color("ffffff")),
                        (1.0, hex_color("dedede")),
                    ],
                ))),
                pressed_gradient_color: None
            },
            prev_label:     "󰒮".to_string(),
            play_label:     "▶".to_string(),
            pause_label:    "󰏤".to_string(),
            next_label:     "󰒭".to_string(),
            vol_down_label: "󰝞".to_string(),
            vol_up_label:   "󰝝".to_string(),
            show_album_art:               true,
            album_art_size:               260,
            album_art_border_radius:      [14., 14., 14., 14.],
            album_art_border_color:       hex_color("3d3d3d"),
            album_art_border_size:        1.0,
            album_art_spacing:            18,
            album_art_placeholder_color:  hex_color("2b2b2b"),
            album_art_position:           AlbumArtPosition::Top,
            progress_bar_height:           4,
            progress_bar_filled_color:     hex_color("ffffff"),
            progress_bar_track_color:      hex_color("3d3d3d"),
            progress_bar_border_color:     ColorType::RGBA([0, 0, 0, 0]),
            progress_bar_border_size:      0.0,
            progress_bar_border_radius:    [2., 2., 2., 2.],
            progress_bar_seekable:         true,
            progress_and_volume_bar_poll_interval_ms: 90,
            volume_bar_width:              90.0,
            volume_bar_height:             4,
            volume_bar_filled_color:       hex_color("ffffff"),
            volume_bar_track_color:        hex_color("3d3d3d"),
            volume_bar_border_color:       ColorType::RGBA([0, 0, 0, 0]),
            volume_bar_border_size:        0.0,
            volume_bar_border_radius:      [2., 2., 2., 2.],
            volume_bar_clickable:          true,
            volume_bar_label_format:       Some("{vol}%".to_string()),
            volume_bar_label_position:     VolumeBarLabelPosition::Right,
            volume_bar_label_size:         12,
            volume_bar_label_color:        hex_color("a1a1a1"),
            volume_bar_label_spacing:      8.0,
            scrollbar: MediaWindowScrollbarConfig::default()
        }
    }
}

impl MediaPlayerWindowConfig
{
    fn metadata_width(&self) -> Length
    {
        if self.metadata_button_width <= 0.0 { Length::Fill }
        else                                 { Length::Fixed(self.metadata_button_width) }
    }
}





// ============ FUNCTIONS ============
pub fn create_media_player_window(app: &mut AppData) -> Task<Message>
{
    let cfg    = &app.ron_config.media_player_window;
    let [w, h] = cfg.window_size;
    let anchor = bar_anchor(&app.ron_config.general.bar_position);

    let (mx, my) = app.modules_data.media_player_window_data.mouse_pos;
    let (px, py) = smart_popup_position(
        mx, my,
        app.monitor_size.0 as i32, app.monitor_size.1 as i32,
        w as i32, h as i32,
    );

    let backdrop_id = iced::window::Id::unique();
    app.ids.insert(backdrop_id, WindowInfo::ContextMenuBackdrop);

    let id = iced::window::Id::unique();
    app.ids.insert(id, WindowInfo::MediaPlayerWindow);

    let backdrop_settings = NewLayerShellSettings
    {
        layer:                  Layer::Overlay,
        size:                   Some((app.monitor_size.0, app.monitor_size.1)),
        exclusive_zone:         Some(0),
        keyboard_interactivity: KeyboardInteractivity::None,
        anchor:                 Anchor::Top | Anchor::Left,
        margin:                 Some((0, 0, 0, 0)),
        ..Default::default()
    };

    let window_settings = NewLayerShellSettings
    {
        layer:                  Layer::Overlay,
        size:                   Some((w, h)),
        exclusive_zone:         Some(0),
        keyboard_interactivity: KeyboardInteractivity::Exclusive,
        anchor,
        margin:                 Some((py, 0, 0, px)),
        ..Default::default()
    };

    Task::batch([
        Task::done(Message::NewLayerShell { settings: backdrop_settings, id: backdrop_id }),
        Task::done(Message::NewLayerShell { settings: window_settings,   id }),
    ])
}



pub fn media_player_window_view<'a>(app: &'a AppData) -> Element<'a, Message>
{
    let cfg         = &app.ron_config.media_player_window;
    let mp_data     = &app.modules_data.media_player_data;
    let bg_color    = cfg.background_color.to_iced_color();
    let border_col  = cfg.background_border_color.to_iced_color();
    let border_size = cfg.background_border_size;
    let brad        = cfg.background_border_radius;
    let pad         = cfg.window_padding;
    let section_gap = cfg.section_spacing as f32;

    let raw_meta = if mp_data.metadata.is_empty()
    {
        app.ron_config.media_player_metadata.text_when_metadata_is_empty.clone()
    }
    else
    {
        mp_data.metadata.clone()
    };

    let truncated       = truncate_str(&raw_meta, cfg.metadata_max_chars);
    let meta_txt_size   = cfg.metadata_text_size as f32;
    let meta_txt_color  = cfg.metadata_text_color.to_iced_color();
    let meta_btn_h      = cfg.metadata_button_height as f32;
    let meta_btn_w      = cfg.metadata_width();
    let meta_style      = cfg.metadata_button_style.clone();

    let metadata_btn = button
    (
        convert_text_to_rich_text::<Message>(&truncated)
            .size(meta_txt_size)
            .color(meta_txt_color)
            .wrapping(iced::widget::text::Wrapping::None),
    )
    .height(meta_btn_h)
    .width(meta_btn_w)
    .clip(true)
    .style(move |_, status| meta_style.to_iced_style(status))
    .on_press(Message::Nothing);

    let metadata_row: Element<'_, Message> = row![metadata_btn]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into();

    let is_playing   = mp_data.status.contains("Playing");
    let btn_h        = cfg.button_height as f32;
    let btn_w        = cfg.button_width  as f32;
    let btn_txt_size = cfg.button_text_size as f32;
    let win_w        = cfg.window_size[0] as f32;
    let win_pad      = cfg.window_padding as f32;
    let inner_w      = (win_w - win_pad * 2.0).max(1.0);
    let mouse_x      = (app.modules_data.media_player_window_data.mouse_pos.0 as f32 - win_pad).clamp(0.0, inner_w);

    let mut ctrl_items: Vec<Element<'_, Message>> = Vec::new();

    for element in &cfg.row_order
    {
        match element
        {
            PlayerRowElement::Prev =>
            {
                let label = cfg.prev_label.clone();
                let style = cfg.button_style.clone();
                ctrl_items.push(
                    button(
                        convert_text_to_rich_text::<Message>(&label)
                            .size(btn_txt_size)
                            .center(),
                    )
                    .width(btn_w)
                    .height(btn_h)
                    .style(move |_, st| style.to_iced_style(st))
                    .on_press(Message::MediaPlayerClickPrev)
                    .into(),
                );
            }

            PlayerRowElement::PlayPause =>
            {
                let (label, style) = if is_playing
                {
                    (cfg.pause_label.clone(), cfg.pause_button_style.clone())
                }
                else
                {
                    (cfg.play_label.clone(), cfg.play_button_style.clone())
                };
                ctrl_items.push(
                    button(
                        convert_text_to_rich_text::<Message>(&label)
                            .size(btn_txt_size)
                            .center(),
                    )
                    .width(btn_w)
                    .height(btn_h)
                    .style(move |_, st| style.to_iced_style(st))
                    .on_press(Message::MediaPlayerClickPlayPause)
                    .into(),
                );
            }

            PlayerRowElement::Next =>
            {
                let label = cfg.next_label.clone();
                let style = cfg.button_style.clone();
                ctrl_items.push(
                    button(
                        convert_text_to_rich_text::<Message>(&label)
                            .size(btn_txt_size)
                            .center(),
                    )
                    .width(btn_w)
                    .height(btn_h)
                    .style(move |_, st| style.to_iced_style(st))
                    .on_press(Message::MediaPlayerClickNext)
                    .into(),
                );
            }

            PlayerRowElement::VolumeDown =>
            {
                let label = cfg.vol_down_label.clone();
                let style = cfg.button_style.clone();
                ctrl_items.push(
                    button(
                        convert_text_to_rich_text::<Message>(&label)
                            .size(btn_txt_size)
                            .center(),
                    )
                    .width(btn_w)
                    .height(btn_h)
                    .style(move |_, st| style.to_iced_style(st))
                    .on_press(Message::MediaPlayerVolumeDown)
                    .into(),
                );
            }

            PlayerRowElement::VolumeUp =>
            {
                let label = cfg.vol_up_label.clone();
                let style = cfg.button_style.clone();
                ctrl_items.push(
                    button(
                        convert_text_to_rich_text::<Message>(&label)
                            .size(btn_txt_size)
                            .center(),
                    )
                    .width(btn_w)
                    .height(btn_h)
                    .style(move |_, st| style.to_iced_style(st))
                    .on_press(Message::MediaPlayerVolumeUp)
                    .into(),
                );
            }

            PlayerRowElement::Metadata =>
            {
                let raw  = if mp_data.metadata.is_empty()
                {
                    app.ron_config.media_player_metadata.text_when_metadata_is_empty.clone()
                }
                else
                {
                    mp_data.metadata.clone()
                };
                let trunc = truncate_str(&raw, cfg.metadata_max_chars);
                let ts    = cfg.metadata_text_size as f32;
                let tc    = cfg.metadata_text_color.to_iced_color();
                let bh    = cfg.metadata_button_height as f32;
                let bw    = cfg.metadata_width();
                let bs    = cfg.metadata_button_style.clone();
                ctrl_items.push(
                    button(
                        convert_text_to_rich_text::<Message>(&trunc)
                            .size(ts)
                            .color(tc),
                    )
                    .height(bh)
                    .width(bw)
                    .clip(true)
                    .style(move |_, st| bs.to_iced_style(st))
                    .on_press(Message::Nothing)
                    .into(),
                );
            }

            PlayerRowElement::Fill =>
            {
                ctrl_items.push(Space::new().width(Length::Fill).into());
            }

            PlayerRowElement::ProgressBar =>
            {
                ctrl_items.push(build_progress_bar(mp_data, cfg, inner_w, mouse_x));
            }

            PlayerRowElement::VolumeBar =>
            {
                ctrl_items.push(build_volume_bar(mp_data, cfg, app.modules_data.media_player_window_data.vol_bar_mouse_x));
            }
        }
    }

    let ctrl_row: Element<'_, Message> = row(ctrl_items)
        .spacing(cfg.row_spacing as f32)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into();

    let progress_bar_row = build_progress_bar(mp_data, cfg, inner_w, mouse_x);

    let fmt_time = |s: f64| -> String
    {
        let s = s as u64;
        format!("{}:{:02}", s / 60, s % 60)
    };

    let time_label_str = if mp_data.duration_secs > 0.0
    {
        format!("{} / {}", fmt_time(mp_data.position_secs), fmt_time(mp_data.duration_secs))
    }
    else
    {
        String::new()
    };

    let time_label: Element<'_, Message> = if !time_label_str.is_empty()
    {
        convert_text_to_rich_text::<Message>(&time_label_str)
            .size(cfg.metadata_text_size as f32 - 3.0)
            .color(hex_color("808080").to_iced_color())
            .into()
    }
    else
    {
        Space::new().into()
    };

    let vol_btn_style = cfg.button_style.clone();
    let vol_down_btn: Element<'_, Message> = button(
        convert_text_to_rich_text::<Message>(&cfg.vol_down_label)
            .size(cfg.button_text_size as f32 - 2.0)
            .center(),
    )
    .width(28.0)
    .height(28.0)
    .style({
        let s = vol_btn_style.clone();
        move |_, st| s.to_iced_style(st)
    })
    .on_press(Message::MediaPlayerVolumeDown)
    .into();

    let vol_up_btn: Element<'_, Message> = button(
        convert_text_to_rich_text::<Message>(&cfg.vol_up_label)
            .size(cfg.button_text_size as f32 - 2.0)
            .center(),
    )
    .width(28.0)
    .height(28.0)
    .style({
        let s = vol_btn_style;
        move |_, st| s.to_iced_style(st)
    })
    .on_press(Message::MediaPlayerVolumeUp)
    .into();

    let volume_row: Element<'_, Message> = if cfg.volume_bar_width > 0.0
    {
        row![
            vol_down_btn,
            Space::new().width(8.0),
            container(build_volume_bar(mp_data, cfg, app.modules_data.media_player_window_data.vol_bar_mouse_x))
                .width(Length::Fill)
                .center_x(Length::Fill),
            Space::new().width(8.0),
            vol_up_btn,
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into()
    }
    else
    {
        Space::new().height(0).into()
    };

    let text_col: Element<'_, Message> = column![
        metadata_row,
        Space::new().height(section_gap),
        ctrl_row,
        Space::new().height(section_gap),
        progress_bar_row,
        Space::new().height(6.0),
        time_label,
        Space::new().height(section_gap),
        volume_row,
    ]
    .width(Length::Fill)
    .into();

    let art_size   = cfg.album_art_size as f32;
    let art_brad   = cfg.album_art_border_radius;
    let art_border = cfg.album_art_border_color.to_iced_color();
    let art_bw     = cfg.album_art_border_size;
    let art_ph     = cfg.album_art_placeholder_color.to_iced_color();
    let art_gap    = cfg.album_art_spacing as f32;

    let art_element: Element<'_, Message> = if cfg.show_album_art
    {
        let inner_art: Element<'_, Message> = if let Some(handle) = &app.modules_data.album_art
        {
            iced::widget::image(handle.clone())
                .width(art_size)
                .height(art_size)
                .content_fit(iced::ContentFit::Cover)
                .into()
        }
        else
        {
            Space::new().width(art_size).height(art_size).into()
        };

        container(inner_art)
            .width(art_size)
            .height(art_size)
            .style(move |_: &Theme|
            {
                iced::widget::container::Style
                {
                    background: Some(iced::Background::Color(art_ph)),
                    border: iced::Border
                    {
                        color:  art_border,
                        width:  art_bw,
                        radius: iced::border::Radius
                        {
                            top_left:     art_brad[0],
                            top_right:    art_brad[1],
                            bottom_right: art_brad[2],
                            bottom_left:  art_brad[3],
                        },
                    },
                    ..Default::default()
                }
            })
            .into()
    }
    else
    {
        Space::new().width(0).height(0).into()
    };

    let body: Element<'_, Message> = if cfg.show_album_art
    {
        match cfg.album_art_position
        {
            AlbumArtPosition::Left =>
                row![art_element, Space::new().width(art_gap), text_col]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),

            AlbumArtPosition::Right =>
                row![text_col, Space::new().width(art_gap), art_element]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),

            AlbumArtPosition::Top =>
                column![
                    container(art_element).center_x(Length::Fill),
                    Space::new().height(art_gap),
                    text_col,
                ]
                .width(Length::Fill)
                .into(),

            AlbumArtPosition::Bottom =>
                column![
                    text_col,
                    Space::new().height(art_gap),
                    container(art_element).center_x(Length::Fill),
                ]
                .width(Length::Fill)
                .into(),
        }
    }
    else
    {
        text_col
    };

    let scroll_cfg = &cfg.scrollbar;
    let content: Element<'_, Message> = if scroll_cfg.show
    {
        let scrollbar = scrollable::Scrollbar::new()
            .width(scroll_cfg.width as f32)
            .margin(scroll_cfg.margin as f32)
            .scroller_width(scroll_cfg.scroller_width as f32);

        let sc = scroll_cfg.clone();
        scrollable(body)
            .direction(scrollable::Direction::Vertical(scrollbar))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme, status| make_scrollbar_style(&sc, theme, status))
            .into()
    }
    else
    {
        body
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(pad)
        .style(move |_: &Theme|
        {
            iced::widget::container::Style
            {
                background: Some(iced::Background::Color(bg_color)),
                border: iced::Border
                {
                    color:  border_col,
                    width:  border_size,
                    radius: iced::border::Radius
                    {
                        top_left:     brad[0],
                        top_right:    brad[1],
                        bottom_right: brad[2],
                        bottom_left:  brad[3],
                    },
                },
                ..Default::default()
            }
        })
        .into()
}



fn bar_anchor(pos: &BarPosition) -> Anchor
{
    match pos
    {
        BarPosition::Down  => Anchor::Bottom | Anchor::Left,
        BarPosition::Up    => Anchor::Top    | Anchor::Left,
        BarPosition::Left  => Anchor::Left   | Anchor::Top,
        BarPosition::Right => Anchor::Right  | Anchor::Top
    }
}



fn truncate_str(s: &str, max: usize) -> String
{
    if s.chars().count() <= max { s.to_string() }
    else { format!("{}…", s.chars().take(max.saturating_sub(1)).collect::<String>()) }
}



fn build_progress_bar<'a>(mp_data: &'a crate::modules::media_player::MediaPlayerData, cfg: &'a MediaPlayerWindowConfig, inner_w: f32, mouse_x: f32) -> Element<'a, Message>
{
    let pos      = mp_data.position_secs;
    let dur      = mp_data.duration_secs;
    let ratio    = if dur > 0.0 { (pos / dur).clamp(0.0, 1.0) as f32 } else { 0.0 };
    let bar_w    = inner_w.max(1.0);
    let bar_h    = cfg.progress_bar_height as f32;
    let filled_c = cfg.progress_bar_filled_color.to_iced_color();
    let track_c  = cfg.progress_bar_track_color.to_iced_color();
    let b_col    = cfg.progress_bar_border_color.to_iced_color();
    let b_size   = cfg.progress_bar_border_size;
    let b_rad    = cfg.progress_bar_border_radius;
    let seekable = cfg.progress_bar_seekable;

    let min_filled_w = bar_h.clamp(2.0, 4.0);
    let filled_w = if ratio <= 0.0
    {
        0.0
    }
    else
    {
        (bar_w * ratio).clamp(min_filled_w, bar_w)
    };

    let filled_part: Element<'_, Message> = container(Space::new())
        .width(Length::Fixed(filled_w))
        .height(Length::Fixed(bar_h))
        .style(move |_: &Theme| iced::widget::container::Style
        {
            background: Some(iced::Background::Color(filled_c)),
            ..Default::default()
        })
        .into();

    let remaining_w = (bar_w - filled_w).max(0.0);

    let track_part: Element<'_, Message> = container(Space::new())
        .width(Length::Fixed(remaining_w))
        .height(Length::Fixed(bar_h))
        .style(move |_: &Theme| iced::widget::container::Style
        {
            background: Some(iced::Background::Color(track_c)),
            ..Default::default()
        })
        .into();

    let bar_border_style = move |_: &Theme| iced::widget::container::Style
    {
        border: iced::Border
        {
            color:  b_col,
            width:  b_size,
            radius: iced::border::Radius
            {
                top_left:     b_rad[0],
                top_right:    b_rad[1],
                bottom_right: b_rad[2],
                bottom_left:  b_rad[3],
            },
        },
        ..Default::default()
    };

    let bar_inner: Element<'_, Message> = row![filled_part, track_part]
        .width(Length::Fixed(bar_w))
        .height(Length::Fixed(bar_h))
        .into();

    let bar_box: Element<'_, Message> = container(bar_inner)
        .width(Length::Fixed(bar_w))
        .height(Length::Fixed(bar_h))
        .style(bar_border_style)
        .clip(true)
        .into();

    let (knob_layer, knob_d) = build_knob(bar_w, bar_h, filled_w, filled_c);

    let track_layer: Element<'_, Message> = container(bar_box)
        .width(Length::Fixed(bar_w))
        .height(Length::Fixed(knob_d))
        .align_y(iced::alignment::Vertical::Center)
        .into();

    let stacked: Element<'_, Message> = stack![track_layer, knob_layer].into();

    if seekable && dur > 0.0
    {
        mouse_area(stacked)
            .on_press(Message::MediaPlayerSeekByFraction(seek_fraction_from_cursor(
                mouse_x,
                inner_w,
            )))
            .into()
    }
    else
    {
        stacked
    }
}



fn build_volume_bar<'a>(mp_data: &'a crate::modules::media_player::MediaPlayerData, cfg: &'a MediaPlayerWindowConfig, local_mouse_x: f32) -> Element<'a, Message>
{
    let vol       = mp_data.volume.clamp(0.0, 1.0) as f32;
    let bar_w     = cfg.volume_bar_width.max(1.0);
    let bar_h     = cfg.volume_bar_height as f32;
    let filled_c  = cfg.volume_bar_filled_color.to_iced_color();
    let track_c   = cfg.volume_bar_track_color.to_iced_color();
    let b_col     = cfg.volume_bar_border_color.to_iced_color();
    let b_size    = cfg.volume_bar_border_size;
    let b_rad     = cfg.volume_bar_border_radius;
    let clickable = cfg.volume_bar_clickable;

    let min_filled_w = bar_h.clamp(2.0, 4.0);
    let filled_w = if vol <= 0.0
    {
        0.0
    }
    else
    {
        (bar_w * vol).clamp(min_filled_w, bar_w)
    };

    let filled_part: Element<'_, Message> = container(Space::new())
        .width(Length::Fixed(filled_w))
        .height(Length::Fixed(bar_h))
        .style(move |_: &Theme| iced::widget::container::Style
        {
            background: Some(iced::Background::Color(filled_c)),
            ..Default::default()
        })
        .into();

    let remaining_w = (bar_w - filled_w).max(0.0);

    let track_part: Element<'_, Message> = container(Space::new())
        .width(Length::Fixed(remaining_w))
        .height(Length::Fixed(bar_h))
        .style(move |_: &Theme| iced::widget::container::Style
        {
            background: Some(iced::Background::Color(track_c)),
            ..Default::default()
        })
        .into();

    let bar_border_style = move |_: &Theme| iced::widget::container::Style
    {
        border: iced::Border
        {
            color:  b_col,
            width:  b_size,
            radius: iced::border::Radius
            {
                top_left:     b_rad[0],
                top_right:    b_rad[1],
                bottom_right: b_rad[2],
                bottom_left:  b_rad[3],
            },
        },
        ..Default::default()
    };

    let bar_inner: Element<'_, Message> = row![filled_part, track_part]
        .width(Length::Fixed(bar_w))
        .height(Length::Fixed(bar_h))
        .into();

    let bar_box: Element<'_, Message> = container(bar_inner)
        .width(Length::Fixed(bar_w))
        .height(Length::Fixed(bar_h))
        .style(bar_border_style)
        .clip(true)
        .into();

    let (knob_layer, knob_d) = build_knob(bar_w, bar_h, filled_w, filled_c);

    let track_layer: Element<'_, Message> = container(bar_box)
        .width(Length::Fixed(bar_w))
        .height(Length::Fixed(knob_d))
        .align_y(iced::alignment::Vertical::Center)
        .into();

    let stacked: Element<'_, Message> = stack![track_layer, knob_layer].into();

    let clamped_x = local_mouse_x.clamp(0.0, bar_w);

    let bar_element: Element<'_, Message> = if clickable
    {
        mouse_area(stacked)
            .on_move(|pt| Message::MediaPlayerVolumeBarMouseMoved(pt.x))
            .on_press(Message::MediaPlayerVolumeSet(seek_fraction_from_cursor(
                clamped_x,
                bar_w,
            )))
            .into()
    }
    else
    {
        stacked
    };

    match &cfg.volume_bar_label_format
    {
        Some(fmt) =>
        {
            let percent  = (vol * 100.0).round() as i64;
            let label_str = fmt.replace("{vol}", &percent.to_string());
            let label_ts  = cfg.volume_bar_label_size as f32;
            let label_c   = cfg.volume_bar_label_color.to_iced_color();

            let label: Element<'_, Message> = convert_text_to_rich_text::<Message>(&label_str)
                .size(label_ts)
                .color(label_c)
                .into();

            let spacing = cfg.volume_bar_label_spacing;

            match cfg.volume_bar_label_position
            {
                VolumeBarLabelPosition::Left =>
                {
                    row![label, Space::new().width(spacing), bar_element]
                        .align_y(Alignment::Center)
                        .into()
                }
                VolumeBarLabelPosition::Right =>
                {
                    row![bar_element, Space::new().width(spacing), label]
                        .align_y(Alignment::Center)
                        .into()
                }
            }
        }
        None => bar_element,
    }
}



fn seek_fraction_from_cursor(cursor_x: f32, bar_width: f32) -> f64
{
    if bar_width <= 0.0 { return 0.0; }
    (cursor_x / bar_width).clamp(0.0, 1.0) as f64
}



fn build_knob<'a>(bar_w: f32, bar_h: f32, fill_x: f32, knob_color: iced::Color) -> (Element<'a, Message>, f32)
{
    let knob_d = (bar_h * 2.2).max(8.0);
    let left   = (fill_x - knob_d / 2.0).clamp(0.0, (bar_w - knob_d).max(0.0));

    let dot: Element<'_, Message> = container(Space::new())
        .width(Length::Fixed(knob_d))
        .height(Length::Fixed(knob_d))
        .style(move |_: &Theme| iced::widget::container::Style
        {
            background: Some(iced::Background::Color(knob_color)),
            border: iced::Border
            {
                color:  knob_color,
                width:  0.0,
                radius: iced::border::Radius
                {
                    top_left:     knob_d / 2.0,
                    top_right:    knob_d / 2.0,
                    bottom_right: knob_d / 2.0,
                    bottom_left:  knob_d / 2.0,
                },
            },
            ..Default::default()
        })
        .into();

    let knob_layer: Element<'_, Message> = container(dot)
        .width(Length::Fixed(bar_w))
        .height(Length::Fixed(knob_d))
        .padding(iced::Padding { top: 0.0, right: 0.0, bottom: 0.0, left })
        .into();

    (knob_layer, knob_d)
}



fn make_scrollbar_style(cfg: &MediaWindowScrollbarConfig, theme: &Theme, status: scrollable::Status) -> scrollable::Style
{
    let brad = iced::border::Radius
    {
        top_left:     cfg.border_radius[0],
        top_right:    cfg.border_radius[1],
        bottom_right: cfg.border_radius[2],
        bottom_left:  cfg.border_radius[3]
    };

    let scroller_color = match status
    {
        scrollable::Status::Dragged { .. } => cfg.scroller_dragging_color.to_iced_color(),
        scrollable::Status::Hovered
        {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } if is_horizontal_scrollbar_hovered || is_vertical_scrollbar_hovered =>
        {
            cfg.scroller_hovered_color.to_iced_color()
        }
        _ => cfg.scroller_color.to_iced_color(),
    };

    let rail = scrollable::Rail
    {
        background: Some(iced::Background::Color(cfg.rail_color.to_iced_color())),
        border: iced::Border
        {
            color:  cfg.rail_border_color.to_iced_color(),
            width:  cfg.rail_border_width,
            radius: brad,
        },
        scroller: scrollable::Scroller
        {
            background: iced::Background::Color(scroller_color),
            border: iced::Border
            {
                color:  cfg.scroller_border_color.to_iced_color(),
                width:  cfg.scroller_border_width,
                radius: brad,
            },
        },
    };

    let base = scrollable::default(theme, status);
    scrollable::Style
    {
        container:       iced::widget::container::Style::default(),
        vertical_rail:   rail,
        horizontal_rail: rail,
        gap:             None,
        ..base
    }
}
