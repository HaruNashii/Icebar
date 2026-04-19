// ============ IMPORTS ============
use libpulse_binding::{callbacks::ListResult, mainloop::threaded::Mainloop, volume::{ChannelVolumes, Volume}, context::{Context, FlagSet as ContextFlagSet, subscribe::InterestMaskSet}};
use iced::{Alignment, Element, Length, Task, Theme, widget::{button, column, container, row, scrollable, slider, text, Space}};
use iced_layershell::reexport::{Anchor, Layer, NewLayerShellSettings};
use std::{pin::Pin, sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};





// ============ CRATES ============
use crate::helpers::{color::ColorType, style::{UserStyle, set_style}, string::convert_text_to_rich_text};
use crate::context_menu::smart_popup_position;
use crate::{AppData, WindowInfo};
use crate::ron::BarPosition;
use crate::update::Message;





// ============ ENUMS / STRUCTS ============
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum MixerKind 
{ 
    Output,
    Input 
}

#[derive(Debug, Clone)]
pub struct AudioDevice
{
    pub index:      u32,
    pub name:       String,
    pub description: String,
    pub volume:     f32,   
    pub muted:      bool,
    pub is_default: bool
}

#[derive(Debug, Clone)]
pub struct AppStream
{
    pub index:       u32,
    pub name:        String,  
    pub volume:      f32,
    pub muted:       bool,
    pub sink_index:  u32
}

#[derive(Debug, Clone, Default)]
pub struct MixerState
{
    pub output_devices: Vec<AudioDevice>,
    pub input_devices:  Vec<AudioDevice>,
    pub output_streams: Vec<AppStream>,
    pub input_streams:  Vec<AppStream>
}

#[derive(Clone, Debug)]
pub struct VolumeMixerData
{
    pub output_mixer_open:          bool,
    pub input_mixer_open:           bool,
    pub output_cursor_inside:       bool,
    pub input_cursor_inside:        bool,
    pub output_device_cat_open:     bool,  
    pub input_device_cat_open:      bool,
    pub output_app_cat_open:        bool,
    pub input_app_cat_open:         bool,
    pub mouse_pos:                  (i32, i32)
}

impl Default for VolumeMixerData
{
    fn default() -> Self
    {
        Self
        {
            output_mixer_open:      false,
            input_mixer_open:       false,
            output_cursor_inside:   false,
            input_cursor_inside:    false,
            output_device_cat_open: true,
            input_device_cat_open:  true,
            output_app_cat_open:    true,
            input_app_cat_open:     true,
            mouse_pos:              (0, 0)
        }
    }
}

impl VolumeMixerData
{
    pub fn from_config(output_cfg: &VolumeMixerConfig, input_cfg: &VolumeMixerConfig) -> Self
    {
        Self
        {
            output_device_cat_open: !output_cfg.device_category.start_collapsed,
            output_app_cat_open:    !output_cfg.app_category.start_collapsed,
            input_device_cat_open:  !input_cfg.device_category.start_collapsed,
            input_app_cat_open:     !input_cfg.app_category.start_collapsed,
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MixerButtonStyle
{
    pub color:               ColorType,
    pub hovered_color:       ColorType,
    pub pressed_color:       ColorType,
    pub text_color:          ColorType,
    pub hovered_text_color:  ColorType,
    pub pressed_text_color:  ColorType,
    pub border_color:        ColorType,
    pub border_size:         f32,
    pub border_radius:       [f32; 4]
}

impl Default for MixerButtonStyle
{
    fn default() -> Self
    {
        Self
        {
            color:              ColorType::RGB([48, 48, 48]),
            hovered_color:      ColorType::RGB([61, 61, 61]),
            pressed_color:      ColorType::RGB([28, 28, 28]),
            text_color:         ColorType::RGB([255, 255, 255]),
            hovered_text_color: ColorType::RGB([255, 255, 255]),
            pressed_text_color: ColorType::RGB([255, 255, 255]),
            border_color:       ColorType::RGB([61, 61, 61]),
            border_size:        1.0,
            border_radius:      [6., 6., 6., 6.]
        }
    }
}

impl MixerButtonStyle
{
    pub fn to_iced_style(&self, status: button::Status) -> iced::widget::button::Style
    {
        set_style(UserStyle
        {
            status,
            normal:          self.color,
            hovered:         self.hovered_color,
            pressed:         self.pressed_color,
            normal_text:     self.text_color,
            hovered_text:    self.hovered_text_color,
            pressed_text:    self.pressed_text_color,
            border_color:    self.border_color,
            border_size:     self.border_size,
            border_radius:   self.border_radius,
            normal_gradient:  None,
            hovered_gradient: None,
            pressed_gradient: None,
            shadow_color:     None,
            shadow_x:         0.,
            shadow_y:         0.,
            shadow_blur:      0.,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MixerSliderStyle
{
    pub rail_color:          ColorType,
    pub rail_filled_color:   ColorType,
    pub rail_border_radius:  [f32; 4],
    pub rail_width:          f32,
    pub handle_color:        ColorType,
    pub handle_border_color: ColorType,
    pub handle_border_width: f32,
    pub handle_border_radius:[f32; 4],
    pub handle_shape:        SliderHandleShape,
    pub handle_circle_r:     f32,
    pub handle_rect_w:       f32,
    pub handle_rect_h:       f32
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum SliderHandleShape { #[default] Circle, Rectangle }

impl Default for MixerSliderStyle
{
    fn default() -> Self
    {
        Self
        {
            rail_color:           ColorType::RGB([48, 48, 48]),
            rail_filled_color:    ColorType::RGB([120, 174, 237]),
            rail_border_radius:   [2., 2., 2., 2.],
            rail_width:           4.0,
            handle_color:         ColorType::RGB([255, 255, 255]),
            handle_border_color:  ColorType::RGB([255, 255, 255]),
            handle_border_width:  1.5,
            handle_border_radius: [10., 10., 10., 10.],
            handle_shape:         SliderHandleShape::Circle,
            handle_circle_r:      8.0,
            handle_rect_w:        10.0,
            handle_rect_h:        20.0
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MixerCategoryConfig
{
    pub show:                      bool,
    pub show_header:               bool,
    pub start_collapsed:           bool,
    pub header_label:              String,
    pub header_text_size:          u32,
    pub header_text_color:         ColorType,
    pub header_button_style:       MixerButtonStyle,
    pub header_button_height:      u32,
    pub header_collapsed_label:    String,   
    pub header_expanded_label:     String,   
    pub header_arrow_text_size:    u32,
    pub spacing:                   u16
}

impl MixerCategoryConfig
{
    fn default_device() -> Self
    {
        Self
        {
            show:                   true,
            show_header:            true,
            start_collapsed:        false,
            header_label:           " Devices".to_string(),
            header_collapsed_label: "›".to_string(),
            header_expanded_label:  "⌄".to_string(),
            header_text_size:       13,
            header_arrow_text_size: 14,
            header_button_height:   24,
            spacing:                15,
            header_text_color:      ColorType::RGB([255, 255, 255]),
            header_button_style:    MixerButtonStyle::default()
        }
    }

    fn default_app() -> Self
    {
        Self
        {
            show:                   true,
            show_header:            true,
            start_collapsed:        false,
            header_label:           " Applications".to_string(),
            header_collapsed_label: "›".to_string(),
            header_expanded_label:  "⌄".to_string(),
            header_arrow_text_size: 14,
            header_text_size:       13,
            header_text_color:      ColorType::RGB([255, 255, 255]),
            spacing:                15,
            header_button_height:   24,
            header_button_style:    MixerButtonStyle::default()
        }
    }
}

impl Default for MixerCategoryConfig
{
    fn default() -> Self { Self::default_device() }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub enum CategoryPosition
{
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum DeviceRowElement
{
    DeviceButton,
    Slider,
    Mute,
    IncreaseVolume,
    DecreaseVolume,
    Fill
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MixerDeviceRowConfig
{
    pub device_row_order:          Vec<DeviceRowElement>,
    pub show_only_default_device_name:  bool,

    pub row_height:                u32,
    pub row_spacing:               u16,
    pub name_text_size:            u32,
    pub name_text_color:           ColorType,
    pub name_max_chars:            usize,

    pub volume_step:               u8,
    pub slider_width:              f32,
    pub slider_style:              MixerSliderStyle,

    pub inc_button_label:          String,
    pub dec_button_label:          String,
    pub inc_dec_text_size:         u32,
    pub inc_dec_button_width:      u32,
    pub inc_dec_button_height:     u32,
    pub inc_dec_button_style:      MixerButtonStyle,

    pub mute_label:                String,
    pub unmute_label:              String,
    pub mute_text_size:            u32,
    pub mute_button_width:         u32,
    pub mute_button_height:        u32,
    pub mute_button_style:         MixerButtonStyle,
    pub muted_button_style:        MixerButtonStyle,

    pub default_label:             String,
    pub non_default_button_style:  MixerButtonStyle,
    pub default_button_style:      MixerButtonStyle,
    pub device_name_button_width:  u32
}

impl Default for MixerDeviceRowConfig
{
    fn default() -> Self
    {
        Self
        {
            device_row_order: vec!
            [
                DeviceRowElement::DeviceButton,
                DeviceRowElement::Slider,
                DeviceRowElement::DecreaseVolume,
                DeviceRowElement::IncreaseVolume,
                DeviceRowElement::Mute,
            ],
            show_only_default_device_name: false,

            row_height:                 30,
            row_spacing:                6,
            name_text_size:             13,
            name_text_color:            ColorType::RGB([255, 255, 255]),
            name_max_chars:             18,

            volume_step:                1,
            slider_width:               250.0,
            slider_style:               MixerSliderStyle::default(),

            inc_button_label:           "+".to_string(),
            dec_button_label:           "−".to_string(),
            inc_dec_text_size:          13,
            inc_dec_button_width:       30,
            inc_dec_button_height:      22,
            inc_dec_button_style:       MixerButtonStyle::default(),

            mute_label:                 "󰕾".to_string(),
            unmute_label:               "󰖁".to_string(),
            mute_text_size:             14,
            mute_button_width:          30,
            mute_button_height:         22,
            mute_button_style:          MixerButtonStyle::default(),
            muted_button_style:         MixerButtonStyle::default(),

            default_label:              "⬤".to_string(),
            non_default_button_style:   MixerButtonStyle::default(),
            default_button_style:       MixerButtonStyle::default(),
            device_name_button_width:   160
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AppRowElement
{
    AppName,
    Slider,
    Mute,
    IncreaseVolume,
    DecreaseVolume,
    Fill
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MixerAppRowConfig
{
    pub app_row_order:         Vec<AppRowElement>,

    pub row_height:            u32,
    pub row_spacing:           u16,
    pub name_text_size:        u32,
    pub name_text_color:       ColorType,
    pub name_max_chars:        usize,

    pub volume_step:           u8,
    pub slider_width:          f32,
    pub slider_style:          MixerSliderStyle,

    pub inc_button_label:      String,
    pub dec_button_label:      String,
    pub inc_dec_text_size:     u32,
    pub inc_dec_button_width:  u32,
    pub inc_dec_button_height: u32,
    pub inc_dec_button_style:  MixerButtonStyle,

    pub mute_label:            String,
    pub unmute_label:          String,
    pub mute_text_size:        u32,
    pub mute_button_width:     u32,
    pub mute_button_height:    u32,
    pub mute_button_style:     MixerButtonStyle,
    pub muted_button_style:    MixerButtonStyle,

    pub name_button_width:     u32
}

impl Default for MixerAppRowConfig
{
    fn default() -> Self
    {
        Self
        {
            app_row_order: vec!
            [
                AppRowElement::AppName,
                AppRowElement::Slider,
                AppRowElement::DecreaseVolume,
                AppRowElement::IncreaseVolume,
                AppRowElement::Mute,
            ],

            row_height:                 30,
            row_spacing:                6,
            name_text_size:             13,
            name_text_color:            ColorType::RGB([255, 255, 255]),
            name_max_chars:             18,

            volume_step:                1,
            slider_width:               120.0,
            slider_style:               MixerSliderStyle::default(),

            inc_button_label:           "+".to_string(),
            dec_button_label:           "−".to_string(),
            inc_dec_text_size:          13,
            inc_dec_button_width:       30,
            inc_dec_button_height:      22,
            inc_dec_button_style:       MixerButtonStyle::default(),

            mute_label:                 "󰕾".to_string(),
            unmute_label:               "󰖁".to_string(),
            mute_text_size:             14,
            mute_button_width:          30,
            mute_button_height:         22,
            mute_button_style:          MixerButtonStyle::default(),
            muted_button_style:         MixerButtonStyle::default(),

            name_button_width:          160
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MixerScrollbarConfig
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

impl Default for MixerScrollbarConfig
{
    fn default() -> Self
    {
        Self
        {
            show:                    true,
            width:                   6,
            margin:                  2,
            scroller_width:          6,
            border_radius:           [3.0, 3.0, 3.0, 3.0],
            rail_color:              ColorType::RGBA([0, 0, 0, 0]),
            rail_border_color:       ColorType::RGBA([0, 0, 0, 0]),
            rail_border_width:       0.0,
            scroller_color:          ColorType::RGBA([100, 100, 100, 60]),
            scroller_hovered_color:  ColorType::RGBA([130, 130, 130, 80]),
            scroller_dragging_color: ColorType::RGBA([160, 160, 160, 100]),
            scroller_border_color:   ColorType::RGBA([0, 0, 0, 0]),
            scroller_border_width:   0.0
        }
    }
}



#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct VolumeMixerConfig
{
    pub mixer_window_size:              [u32; 2],
    pub mixer_background_color:         ColorType,
    pub mixer_background_border_color:  ColorType,
    pub mixer_background_border_size:   f32,
    pub mixer_background_border_radius: [f32; 4],
    pub mixer_padding:                  u16,
    pub mixer_section_spacing:          u16,
    pub mixer_show_only_active_devices: bool,
    pub categories_position:            CategoryPosition,
    pub scrollbar:                      MixerScrollbarConfig,

    pub device_category:                MixerCategoryConfig,
    pub app_category:                   MixerCategoryConfig,
    pub device_row:                     MixerDeviceRowConfig,
    pub app_row:                        MixerAppRowConfig
}

impl Default for VolumeMixerConfig
{
    fn default() -> Self
    {
        Self
        {
            mixer_window_size:              [550, 400],
            mixer_background_color:         ColorType::RGB([36, 36, 36]),
            mixer_background_border_color:  ColorType::RGB([161, 161, 161]),
            mixer_background_border_size:   1.0,
            mixer_background_border_radius: [6., 6., 6., 6.],
            mixer_padding:                  10,
            mixer_section_spacing:          8,
            mixer_show_only_active_devices: false,
            categories_position:            CategoryPosition::Up,
            scrollbar:                      MixerScrollbarConfig::default(),

            device_category: MixerCategoryConfig::default_device(),
            app_category:    MixerCategoryConfig::default_app(),
            device_row:      MixerDeviceRowConfig::default(),
            app_row:         MixerAppRowConfig::default()
        }
    }
}

#[derive(Default, Clone)]
struct PulseStateInternal
{
    output_devices: Vec<AudioDevice>,
    input_devices:  Vec<AudioDevice>,
    output_streams: Vec<AppStream>,
    input_streams:  Vec<AppStream>,
    default_sink_name:   String,
    default_source_name: String,
    sinks_done:    bool,
    sources_done:  bool,
    sink_in_done:  bool,
    src_out_done:  bool
}





// ============ FUNCTIONS ============
pub fn volume_mixer_subscription() -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        let state: Arc<Mutex<PulseStateInternal>> = Arc::new(Mutex::new(PulseStateInternal::default()));
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        let state_cb  = Arc::clone(&state);
        let tx_clone  = tx.clone();

        let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel::<()>();

        std::thread::spawn(move ||
        {
            let mut mainloop = match Mainloop::new() { Some(m) => m, None => return };
            if mainloop.start().is_err() { return; }

            mainloop.lock();

            let context = match Context::new(&mainloop, "icebar-volume-mixer")
            {
                Some(c) => Arc::new(Mutex::new(c)),
                None    => { mainloop.unlock(); return; }
            };

            if context.lock().unwrap().connect(None, ContextFlagSet::NOFLAGS, None).is_err()
            {
                mainloop.unlock();
                return;
            }

            loop
            {
                match context.lock().unwrap().get_state()
                {
                    libpulse_binding::context::State::Ready => break,
                    libpulse_binding::context::State::Failed |
                    libpulse_binding::context::State::Terminated => { mainloop.unlock(); return; }
                    _ => {}
                }
                mainloop.unlock();
                std::thread::sleep(std::time::Duration::from_millis(10));
                mainloop.lock();
            }

            {
                let s = Arc::clone(&state_cb);
                let t = tx_clone.clone();
                fetch_all(Arc::clone(&context), Arc::clone(&s), t);
            }

            {
                let ctx = Arc::clone(&context);
                let s   = Arc::clone(&state_cb);
                let t   = tx_clone.clone();

                context.lock().unwrap().subscribe
                (
                    InterestMaskSet::SINK
                    | InterestMaskSet::SOURCE
                    | InterestMaskSet::SINK_INPUT
                    | InterestMaskSet::SOURCE_OUTPUT
                    | InterestMaskSet::SERVER,
                    |_| {}
                );

                context.lock().unwrap().set_subscribe_callback
                (
                    Some
                    (
                        Box::new
                        (
                            move |_facility, _op, _index|
                            {
                                fetch_all(Arc::clone(&ctx), Arc::clone(&s), t.clone());
                            }
                        )
                    )
                );
            }

            mainloop.unlock();

            let _ = shutdown_rx.recv();

            mainloop.lock();
            context.lock().unwrap().disconnect();
            mainloop.unlock();
            mainloop.stop();
        });

        let _shutdown_guard = shutdown_tx;

        while rx.recv().await.is_some()
        {
            let s = state.lock().unwrap().clone();
            if s.sinks_done && s.sources_done && s.sink_in_done && s.src_out_done
            {
                let mixer_state = MixerState
                {
                    output_devices:      s.output_devices,
                    input_devices:       s.input_devices,
                    output_streams:      s.output_streams,
                    input_streams:       s.input_streams
                };
                yield Message::MixerStateUpdated(mixer_state);
            }
        }
    })
}




fn fetch_all(ctx: Arc<Mutex<Context>>, state: Arc<Mutex<PulseStateInternal>>, tx: tokio::sync::mpsc::UnboundedSender<()>)
{
    {
        let mut s = state.lock().unwrap();
        s.sinks_done   = false;
        s.sources_done = false;
        s.sink_in_done = false;
        s.src_out_done = false;
    }

    let state_srv = Arc::clone(&state);
    let ctx2 = Arc::clone(&ctx);
    let tx2  = tx.clone();
    let insp = ctx.lock().unwrap().introspect();
    insp.get_server_info(move |info|
    {
        let def_sink   = info.default_sink_name.as_deref().unwrap_or("").to_string();
        let def_source = info.default_source_name.as_deref().unwrap_or("").to_string();
        {
            let mut s = state_srv.lock().unwrap();
            s.default_sink_name   = def_sink;
            s.default_source_name = def_source;
        }
        fetch_sinks(          Arc::clone(&ctx2), Arc::clone(&state_srv), tx2.clone());
        fetch_sources(        Arc::clone(&ctx2), Arc::clone(&state_srv), tx2.clone());
        fetch_sink_inputs(    Arc::clone(&ctx2), Arc::clone(&state_srv), tx2.clone());
        fetch_source_outputs( Arc::clone(&ctx2), Arc::clone(&state_srv), tx2.clone());
    });
}



fn vol_to_f32(cv: &ChannelVolumes) -> f32
{
    cv.avg().0 as f32 / Volume::NORMAL.0 as f32
}



fn fetch_sinks(ctx: Arc<Mutex<Context>>, state: Arc<Mutex<PulseStateInternal>>, tx: tokio::sync::mpsc::UnboundedSender<()>)
{
    let mut devs: Vec<AudioDevice> = Vec::new();
    let state2 = Arc::clone(&state);
    let tx2 = tx.clone();

    let insp = ctx.lock().unwrap().introspect();
    insp.get_sink_info_list(move |list|
    {
        match list
        {
            ListResult::Item(info) =>
            {
                let name = info.name.as_deref().unwrap_or("").to_string();
                let desc = info.description.as_deref().unwrap_or(&name).to_string();
                let default_name = state2.lock().unwrap().default_sink_name.clone();
                devs.push(AudioDevice
                {
                    index:       info.index,
                    name:        name.clone(),
                    description: desc,
                    volume:      vol_to_f32(&info.volume),
                    muted:       info.mute,
                    is_default:  name == default_name,
                });
            }
            ListResult::End =>
            {
                let mut s = state2.lock().unwrap();
                let def = s.default_sink_name.clone();
                for d in &mut devs { d.is_default = d.name == def; }
                s.output_devices = devs.clone();
                s.sinks_done = true;
                let _ = tx2.send(());
            }
            ListResult::Error => {}
        }
    });
}



fn fetch_sources(ctx: Arc<Mutex<Context>>, state: Arc<Mutex<PulseStateInternal>>, tx: tokio::sync::mpsc::UnboundedSender<()>)
{
    let mut devs: Vec<AudioDevice> = Vec::new();
    let state2 = Arc::clone(&state);
    let tx2 = tx.clone();

    let insp = ctx.lock().unwrap().introspect();
    insp.get_source_info_list(move |list|
    {
        match list
        {
            ListResult::Item(info) =>
            {
                if info.name.as_deref().unwrap_or("").ends_with(".monitor") { return; }
                let name = info.name.as_deref().unwrap_or("").to_string();
                let desc = info.description.as_deref().unwrap_or(&name).to_string();
                let def  = state2.lock().unwrap().default_source_name.clone();
                devs.push(AudioDevice
                {
                    index:       info.index,
                    name:        name.clone(),
                    description: desc,
                    volume:      vol_to_f32(&info.volume),
                    muted:       info.mute,
                    is_default:  name == def,
                });
            }
            ListResult::End =>
            {
                let mut s = state2.lock().unwrap();
                let def = s.default_source_name.clone();
                for d in &mut devs { d.is_default = d.name == def; }
                s.input_devices = devs.clone();
                s.sources_done = true;
                let _ = tx2.send(());
            }
            ListResult::Error => {}
        }
    });
}



fn fetch_sink_inputs(ctx: Arc<Mutex<Context>>, state: Arc<Mutex<PulseStateInternal>>, tx: tokio::sync::mpsc::UnboundedSender<()>)
{
    let mut streams: Vec<AppStream> = Vec::new();
    let state2 = Arc::clone(&state);
    let tx2 = tx.clone();

    let insp = ctx.lock().unwrap().introspect();
    insp.get_sink_input_info_list(move |list|
    {
        match list
        {
            ListResult::Item(info) =>
            {
                let name = info.proplist.get_str("application.name")
                    .or_else(|| info.proplist.get_str("media.name"))
                    .unwrap_or_else(|| format!("Stream #{}", info.index));
                streams.push(AppStream
                {
                    index:      info.index,
                    name,
                    volume:     vol_to_f32(&info.volume),
                    muted:      info.mute,
                    sink_index: info.sink,
                });
            }
            ListResult::End =>
            {
                let mut s = state2.lock().unwrap();
                s.output_streams = streams.clone();
                s.sink_in_done = true;
                let _ = tx2.send(());
            }
            ListResult::Error => {}
        }
    });
}



fn fetch_source_outputs(ctx: Arc<Mutex<Context>>, state: Arc<Mutex<PulseStateInternal>>, tx: tokio::sync::mpsc::UnboundedSender<()>)
{
    let mut streams: Vec<AppStream> = Vec::new();
    let state2 = Arc::clone(&state);
    let tx2 = tx.clone();

    let insp = ctx.lock().unwrap().introspect();
    insp.get_source_output_info_list(move |list|
    {
        match list
        {
            ListResult::Item(info) =>
            {
                let app_name = info.proplist.get_str("application.name").unwrap_or_default();
                if app_name == "PulseAudio Volume Control"
                    || app_name.starts_with("peak detect")
                    || info.name.as_deref().unwrap_or("").contains("peak detect")
                {
                    return;
                }
                let name = if !app_name.is_empty()
                {
                    app_name
                }
                else
                {
                    info.proplist.get_str("media.name")
                        .unwrap_or_else(|| format!("Stream #{}", info.index))
                };
                streams.push(AppStream
                {
                    index:      info.index,
                    name,
                    volume:     vol_to_f32(&info.volume),
                    muted:      info.mute,
                    sink_index: info.source,
                });
            }
            ListResult::End =>
            {
                let mut s = state2.lock().unwrap();
                s.input_streams = streams.clone();
                s.src_out_done = true;
                let _ = tx2.send(());
            }
            ListResult::Error => {}
        }
    });
}





// ============ CONTROL ACTIONS ============
pub async fn set_device_volume_cmd(kind: MixerKind, index: u32, volume_pct: u8)
{
    let type_str = match kind { MixerKind::Output => "sink", MixerKind::Input => "source" };
    let _ = tokio::process::Command::new("pactl")
        .args
        ([
            &format!("set-{}-volume", type_str),
            &index.to_string(),
            &format!("{}%", volume_pct),
        ])
        .output().await;
}

pub async fn toggle_device_mute_cmd(kind: MixerKind, index: u32)
{
    let type_str = match kind { MixerKind::Output => "sink", MixerKind::Input => "source" };
    let _ = tokio::process::Command::new("pactl")
        .args([&format!("set-{}-mute", type_str), &index.to_string(), "toggle"])
        .output().await;
}

pub async fn set_default_device_cmd(kind: MixerKind, name: String)
{
    let type_str = match kind { MixerKind::Output => "default-sink", MixerKind::Input => "default-source" };
    let _ = tokio::process::Command::new("pactl")
        .args([&format!("set-{}", type_str), &name])
        .output().await;
}

pub async fn set_app_volume_cmd(kind: MixerKind, index: u32, volume_pct: u8)
{
    let type_str = match kind { MixerKind::Output => "sink-input", MixerKind::Input => "source-output" };
    let _ = tokio::process::Command::new("pactl")
        .args([
            &format!("set-{}-volume", type_str),
            &index.to_string(),
            &format!("{}%", volume_pct),
        ])
        .output().await;
}

pub async fn toggle_app_mute_cmd(kind: MixerKind, index: u32)
{
    let type_str = match kind { MixerKind::Output => "sink-input", MixerKind::Input => "source-output" };
    let _ = tokio::process::Command::new("pactl")
        .args([&format!("set-{}-mute", type_str), &index.to_string(), "toggle"])
        .output().await;
}





// ============ TASK HELPERS ============
pub fn task_set_device_volume(kind: MixerKind, index: u32, volume_pct: u8) -> Task<Message>
{
    Task::perform(set_device_volume_cmd(kind, index, volume_pct), |_| Message::Nothing)
}

pub fn task_toggle_device_mute(kind: MixerKind, index: u32) -> Task<Message>
{
    Task::perform(toggle_device_mute_cmd(kind, index), |_| Message::Nothing)
}

pub fn task_set_default_device(kind: MixerKind, name: String) -> Task<Message>
{
    Task::perform(set_default_device_cmd(kind, name), |_| Message::Nothing)
}

pub fn task_set_app_volume(kind: MixerKind, index: u32, volume_pct: u8) -> Task<Message>
{
    Task::perform(set_app_volume_cmd(kind, index, volume_pct), |_| Message::Nothing)
}

pub fn task_toggle_app_mute(kind: MixerKind, index: u32) -> Task<Message>
{
    Task::perform(toggle_app_mute_cmd(kind, index), |_| Message::Nothing)
}





// ============ WINDOW CREATION ============
pub fn create_output_mixer_window(app: &mut AppData) -> Task<Message>
{
    let cfg = &app.ron_config.volume_output_mixer;
    let [w, h] = cfg.mixer_window_size;
    let anchor  = bar_anchor(&app.ron_config.general.bar_position);
    let (mx, my) = app.modules_data.volume_mixer_data.mouse_pos;
    let (px, py) = smart_popup_position(mx, my, app.monitor_size.0 as i32, app.monitor_size.1 as i32, w as i32, h as i32);
    let id = iced::window::Id::unique();
    app.ids.insert(id, WindowInfo::VolumeOutputMixer);
    Task::done(Message::NewLayerShell
    {
        settings: NewLayerShellSettings
        {
            layer:                  Layer::Overlay,
            size:                   Some((w, h)),
            exclusive_zone:         Some(0),
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::Exclusive,
            anchor,
            margin:                 Some((py, 0, 0, px)),
            ..Default::default()
        },
        id
    })
}

pub fn create_input_mixer_window(app: &mut AppData) -> Task<Message>
{
    let cfg = &app.ron_config.volume_input_mixer;
    let [w, h] = cfg.mixer_window_size;
    let anchor  = bar_anchor(&app.ron_config.general.bar_position);
    let (mx, my) = app.modules_data.volume_mixer_data.mouse_pos;
    let (px, py) = smart_popup_position(mx, my, app.monitor_size.0 as i32, app.monitor_size.1 as i32, w as i32, h as i32);
    let id = iced::window::Id::unique();
    app.ids.insert(id, WindowInfo::VolumeInputMixer);
    Task::done(Message::NewLayerShell
    {
        settings: NewLayerShellSettings
        {
            layer:                  Layer::Overlay,
            size:                   Some((w, h)),
            exclusive_zone:         Some(0),
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::Exclusive,
            anchor,
            margin:                 Some((py, 0, 0, px)),
            ..Default::default()
        },
        id
    })
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



pub fn volume_mixer_view<'a>(app: &'a AppData, kind: MixerKind) -> Element<'a, Message>
{
    let cfg = match kind
    {
        MixerKind::Output => &app.ron_config.volume_output_mixer,
        MixerKind::Input  => &app.ron_config.volume_input_mixer
    };

    let mixer_data = &app.modules_data.volume_mixer_data;
    let state = &app.modules_data.mixer_state;
    let bg_color    = cfg.mixer_background_color.to_iced_color();
    let border_col  = cfg.mixer_background_border_color.to_iced_color();
    let border_size   = cfg.mixer_background_border_size;
    let brad     = cfg.mixer_background_border_radius;
    let pad           = cfg.mixer_padding;

    let devices: Vec<&AudioDevice> = match kind
    {
        MixerKind::Output =>
        {
            if cfg.mixer_show_only_active_devices
            {
                let active_sink_indices: std::collections::HashSet<u32> = state.output_streams.iter().map(|s| s.sink_index).collect();
                state.output_devices.iter().filter(|d| d.is_default || active_sink_indices.contains(&d.index)).collect()
            }
            else 
            { 
                state.output_devices.iter().collect() 
            }
        }
        MixerKind::Input =>
        {
            if cfg.mixer_show_only_active_devices
            {
                let active_src_indices: std::collections::HashSet<u32> = state.input_streams.iter().map(|s| s.sink_index).collect();
                state.input_devices.iter().filter(|d| d.is_default || active_src_indices.contains(&d.index)).collect()
            }
            else 
            {
                state.input_devices.iter().collect() 
            }
        }
    };

    let streams: Vec<&AppStream> = match kind
    {
        MixerKind::Output => state.output_streams.iter().collect(),
        MixerKind::Input  => state.input_streams.iter().collect()
    };

    let dev_cat_cfg = &cfg.device_category;
    let dev_row_cfg = &cfg.device_row;
    let (dev_cat_open, app_cat_open) = match kind
    {
        MixerKind::Output => (mixer_data.output_device_cat_open, mixer_data.output_app_cat_open),
        MixerKind::Input  => (mixer_data.input_device_cat_open,  mixer_data.input_app_cat_open)
    };

    let mut top_sections:    Vec<Element<'_, Message>> = Vec::new();
    let mut bottom_sections: Vec<Element<'_, Message>> = Vec::new();

    if dev_cat_cfg.show
    {
        if dev_cat_cfg.show_header
        {
            let arrow = if dev_cat_open { &dev_cat_cfg.header_expanded_label } else { &dev_cat_cfg.header_collapsed_label };
            let arrow_size   = dev_cat_cfg.header_arrow_text_size;
            let label_size   = dev_cat_cfg.header_text_size;
            let label_color  = dev_cat_cfg.header_text_color.to_iced_color();
            let btn_h        = dev_cat_cfg.header_button_height as f32;
            let btn_style    = dev_cat_cfg.header_button_style.clone();

            let toggle_msg = match kind
            {
                MixerKind::Output => Message::ToggleOutputDeviceCategory,
                MixerKind::Input  => Message::ToggleInputDeviceCategory
            };

            let header_btn = button
            (
                row!
                [
                    convert_text_to_rich_text::<Message>(arrow).size(arrow_size as f32)
                        .wrapping(text::Wrapping::None),
                    Space::new(),
                    convert_text_to_rich_text::<Message>(&dev_cat_cfg.header_label)
                        .wrapping(text::Wrapping::None)
                        .size(label_size as f32)
                        .color(label_color),
                ]
                .align_y(Alignment::Center)
            )
            .height(btn_h)
            .clip(true)
            .width(Length::Fill)
            .style(move |_, status| btn_style.to_iced_style(status))
            .on_press(toggle_msg);

            top_sections.push(header_btn.into());
        }

        if dev_cat_open
        {
            for dev in &devices
            {
                top_sections.push(build_device_row(dev, kind, dev_row_cfg));
            }
        }
    }

    let app_cat_cfg = &cfg.app_category;
    let app_row_cfg = &cfg.app_row;

    if app_cat_cfg.show
    {
        if app_cat_cfg.show_header
        {
            let arrow = if app_cat_open { &app_cat_cfg.header_expanded_label } else { &app_cat_cfg.header_collapsed_label };
            let arrow_size  = app_cat_cfg.header_arrow_text_size;
            let label_size  = app_cat_cfg.header_text_size;
            let label_color = app_cat_cfg.header_text_color.to_iced_color();
            let btn_h       = app_cat_cfg.header_button_height as f32;
            let btn_style   = app_cat_cfg.header_button_style.clone();

            let toggle_msg = match kind
            {
                MixerKind::Output => Message::ToggleOutputAppCategory,
                MixerKind::Input  => Message::ToggleInputAppCategory
            };

            let header_btn = button
            (
                row!
                [
                    convert_text_to_rich_text::<Message>(arrow).size(arrow_size as f32)
                        .wrapping(text::Wrapping::None),
                    Space::new().width(6.0),
                    convert_text_to_rich_text::<Message>(&app_cat_cfg.header_label)
                        .wrapping(text::Wrapping::None)
                        .size(label_size as f32)
                        .color(label_color),
                ]
                .align_y(Alignment::Center)
            )
            .height(btn_h)
            .clip(true)
            .width(Length::Fill)
            .style(move |_, status| btn_style.to_iced_style(status))
            .on_press(toggle_msg);

            bottom_sections.push(header_btn.into());
        }

        if app_cat_open
        {
            for stream in &streams
            {
                bottom_sections.push(build_app_row(stream, kind, app_row_cfg));
            }

            if streams.is_empty()
            {
                bottom_sections.push
                (
                    text("No active streams")
                        .size(11.0)
                        .color(iced::Color::from_rgb8(130, 120, 150))
                        .into()
                );
            }
        }
    }

    let dev_col = column(top_sections)
        .spacing(dev_cat_cfg.spacing as f32)
        .width(Length::Fill);

    let app_col = column(bottom_sections)
        .spacing(app_cat_cfg.spacing as f32)
        .width(Length::Fill);

    let section_gap = cfg.mixer_section_spacing as f32;

    let inner: Element<'_, Message> = match cfg.categories_position
    {
        CategoryPosition::Up =>
        {
            column![dev_col, app_col]
                .spacing(section_gap)
                .width(Length::Fill)
                .into()
        }
        CategoryPosition::Down =>
        {
            column![app_col, dev_col]
                .spacing(section_gap)
                .width(Length::Fill)
                .into()
        }
        CategoryPosition::Left =>
        {
            row![dev_col, app_col]
                .spacing(section_gap)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        CategoryPosition::Right =>
        {
            row![app_col, dev_col]
                .spacing(section_gap)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    };

    let scroll_cfg = &cfg.scrollbar;
    let scrollbar  = scrollable::Scrollbar::new()
        .width(scroll_cfg.width as f32)
        .margin(scroll_cfg.margin as f32)
        .scroller_width(scroll_cfg.scroller_width as f32);

    let content: Element<'_, Message> = if scroll_cfg.show
    {
        let scroll_cfg_clone = scroll_cfg.clone();
        scrollable(inner)
            .direction(scrollable::Direction::Vertical(scrollbar))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme: &Theme, status| make_scrollbar_style(&scroll_cfg_clone, theme, status))
            .into()
    }
    else
    {
        inner
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(pad)
        .style(move |_: &Theme|
        {
            iced::widget::container::Style
            {
                background:  Some(iced::Background::Color(bg_color)),
                border:      iced::Border
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




fn build_device_row<'a>(dev: &AudioDevice, kind: MixerKind, cfg: &'a MixerDeviceRowConfig) -> Element<'a, Message>
{
    let vol_pct = (dev.volume * 100.0).round().clamp(0.0, 150.0) as u8;
    let h       = cfg.row_height as f32;
    let spacing = cfg.row_spacing;
    let mut row_items: Vec<Element<'_, Message>> = Vec::new();

    for element in &cfg.device_row_order
    {
        match element
        {
            DeviceRowElement::DeviceButton =>
            {
                if cfg.show_only_default_device_name && !dev.is_default { continue; }

                let label = truncate(&dev.description, cfg.name_max_chars);
                let full_label: String = if dev.is_default
                {
                    format!("{} {}", cfg.default_label, label)
                }
                else
                {
                    label
                };

                let style_normal  = cfg.non_default_button_style.clone();
                let style_default = cfg.default_button_style.clone();
                let is_default    = dev.is_default;
                let dev_name      = dev.name.clone();
                let btn_w         = cfg.device_name_button_width as f32;
                let txt_size      = cfg.name_text_size as f32;
                let txt_color     = cfg.name_text_color.to_iced_color();
                let set_default_msg = task_msg_set_default(kind, dev_name);

                let name_btn = button(convert_text_to_rich_text::<Message>(&full_label).size(txt_size).color(txt_color))
                    .width(btn_w)
                    .height(h)
                    .style(move |_, status|
                    {
                        if is_default { style_default.to_iced_style(status) }
                        else          { style_normal.to_iced_style(status)  }
                    })
                    .on_press(set_default_msg);

                row_items.push(name_btn.into());
            }

            DeviceRowElement::Slider =>
            {
                let slider_w = cfg.slider_width;
                let s_style  = cfg.slider_style.clone();
                let idx      = dev.index;

                let vol_slider = slider(0..=150u8, vol_pct, move |v|
                {
                    match kind
                    {
                        MixerKind::Output => Message::SetDeviceVolume(MixerKind::Output, idx, v),
                        MixerKind::Input  => Message::SetDeviceVolume(MixerKind::Input,  idx, v)
                    }
                })
                .width(slider_w)
                .style(move |_, status| make_slider_style(&s_style, status));

                row_items.push(vol_slider.into());
            }

            DeviceRowElement::DecreaseVolume =>
            {
                let step   = cfg.volume_step;
                let idx    = dev.index;
                let bw     = cfg.inc_dec_button_width as f32;
                let bh     = cfg.inc_dec_button_height as f32;
                let ts     = cfg.inc_dec_text_size as f32;
                let bstyle = cfg.inc_dec_button_style.clone();
                let dec_msg = match kind
                {
                    MixerKind::Output => Message::SetDeviceVolume(MixerKind::Output, idx, vol_pct.saturating_sub(step)),
                    MixerKind::Input  => Message::SetDeviceVolume(MixerKind::Input,  idx, vol_pct.saturating_sub(step))
                };
                let dec_btn = button(convert_text_to_rich_text::<Message>(&cfg.dec_button_label).size(ts))
                    .width(bw)
                    .height(bh)
                    .style(move |_, st| bstyle.to_iced_style(st))
                    .on_press(dec_msg);
                row_items.push(dec_btn.into());
            }

            DeviceRowElement::IncreaseVolume =>
            {
                let step   = cfg.volume_step;
                let idx    = dev.index;
                let bw     = cfg.inc_dec_button_width as f32;
                let bh     = cfg.inc_dec_button_height as f32;
                let ts     = cfg.inc_dec_text_size as f32;
                let bstyle = cfg.inc_dec_button_style.clone();
                let inc_msg = match kind
                {
                    MixerKind::Output => Message::SetDeviceVolume(MixerKind::Output, idx, (vol_pct as u16 + step as u16).min(150) as u8),
                    MixerKind::Input  => Message::SetDeviceVolume(MixerKind::Input,  idx, (vol_pct as u16 + step as u16).min(150) as u8)
                };
                let inc_btn = button(convert_text_to_rich_text::<Message>(&cfg.inc_button_label).size(ts))
                    .width(bw)
                    .height(bh)
                    .style(move |_, st| bstyle.to_iced_style(st))
                    .on_press(inc_msg);
                row_items.push(inc_btn.into());
            }

            DeviceRowElement::Mute =>
            {
                let muted    = dev.muted;
                let idx      = dev.index;
                let bw       = cfg.mute_button_width as f32;
                let bh       = cfg.mute_button_height as f32;
                let ts       = cfg.mute_text_size as f32;
                let label    = if muted { cfg.unmute_label.clone() } else { cfg.mute_label.clone() };
                let ms       = cfg.mute_button_style.clone();
                let mds      = cfg.muted_button_style.clone();
                let mute_msg = match kind
                {
                    MixerKind::Output => Message::ToggleDeviceMute(MixerKind::Output, idx),
                    MixerKind::Input  => Message::ToggleDeviceMute(MixerKind::Input,  idx)
                };
                let mute_btn = button(convert_text_to_rich_text::<Message>(&label).size(ts))
                    .width(bw)
                    .height(bh)
                    .style(move |_, st| if muted { mds.to_iced_style(st) } else { ms.to_iced_style(st) })
                    .on_press(mute_msg);
                row_items.push(mute_btn.into());
            }

            DeviceRowElement::Fill =>
            {
                row_items.push(Space::new().width(Length::Fill).into());
            }
        }
    }

    row(row_items)
        .spacing(spacing as f32)
        .align_y(Alignment::Center)
        .into()
}




fn build_app_row<'a>(stream: &AppStream, kind: MixerKind, cfg: &'a MixerAppRowConfig) -> Element<'a, Message>
{
    let vol_pct = (stream.volume * 100.0).round().clamp(0.0, 150.0) as u8;
    let h       = cfg.row_height as f32;
    let spacing = cfg.row_spacing;
    let mut row_items: Vec<Element<'_, Message>> = Vec::new();

    for element in &cfg.app_row_order
    {
        match element
        {
            AppRowElement::AppName =>
            {
                let label = truncate(&stream.name, cfg.name_max_chars);
                let ts    = cfg.name_text_size as f32;
                let tc    = cfg.name_text_color.to_iced_color();
                let bw    = cfg.name_button_width as f32;
                row_items.push(convert_text_to_rich_text::<Message>(&label).size(ts).color(tc).width(bw).height(h).into());
            }

            AppRowElement::Slider =>
            {
                let slider_w = cfg.slider_width;
                let s_style  = cfg.slider_style.clone();
                let idx      = stream.index;
                let vol_slider = slider(0..=150u8, vol_pct, move |v|
                {
                    match kind
                    {
                        MixerKind::Output => Message::SetAppVolume(MixerKind::Output, idx, v),
                        MixerKind::Input  => Message::SetAppVolume(MixerKind::Input,  idx, v)
                    }
                })
                .width(slider_w)
                .style(move |_, status| make_slider_style(&s_style, status));
                row_items.push(vol_slider.into());
            }

            AppRowElement::DecreaseVolume =>
            {
                let step   = cfg.volume_step;
                let idx    = stream.index;
                let bw     = cfg.inc_dec_button_width as f32;
                let bh     = cfg.inc_dec_button_height as f32;
                let ts     = cfg.inc_dec_text_size as f32;
                let bstyle = cfg.inc_dec_button_style.clone();
                let dec_msg = match kind
                {
                    MixerKind::Output => Message::SetAppVolume(MixerKind::Output, idx, vol_pct.saturating_sub(step)),
                    MixerKind::Input  => Message::SetAppVolume(MixerKind::Input,  idx, vol_pct.saturating_sub(step))
                };
                let dec_btn = button(convert_text_to_rich_text::<Message>(&cfg.dec_button_label).size(ts))
                    .width(bw)
                    .height(bh)
                    .style(move |_, st| bstyle.to_iced_style(st))
                    .on_press(dec_msg);
                row_items.push(dec_btn.into());
            }

            AppRowElement::IncreaseVolume =>
            {
                let step   = cfg.volume_step;
                let idx    = stream.index;
                let bw     = cfg.inc_dec_button_width as f32;
                let bh     = cfg.inc_dec_button_height as f32;
                let ts     = cfg.inc_dec_text_size as f32;
                let bstyle = cfg.inc_dec_button_style.clone();
                let inc_msg = match kind
                {
                    MixerKind::Output => Message::SetAppVolume(MixerKind::Output, idx, (vol_pct as u16 + step as u16).min(150) as u8),
                    MixerKind::Input  => Message::SetAppVolume(MixerKind::Input,  idx, (vol_pct as u16 + step as u16).min(150) as u8)
                };
                let inc_btn = button(convert_text_to_rich_text::<Message>(&cfg.inc_button_label).size(ts))
                    .width(bw)
                    .height(bh)
                    .style(move |_, st| bstyle.to_iced_style(st))
                    .on_press(inc_msg);
                row_items.push(inc_btn.into());
            }

            AppRowElement::Mute =>
            {
                let muted    = stream.muted;
                let idx      = stream.index;
                let bw       = cfg.mute_button_width as f32;
                let bh       = cfg.mute_button_height as f32;
                let ts       = cfg.mute_text_size as f32;
                let label    = if muted { cfg.unmute_label.clone() } else { cfg.mute_label.clone() };
                let ms       = cfg.mute_button_style.clone();
                let mds      = cfg.muted_button_style.clone();
                let mute_msg = match kind
                {
                    MixerKind::Output => Message::ToggleAppMute(MixerKind::Output, idx),
                    MixerKind::Input  => Message::ToggleAppMute(MixerKind::Input,  idx)
                };
                let mute_btn = button(convert_text_to_rich_text::<Message>(&label).size(ts))
                    .width(bw)
                    .height(bh)
                    .style(move |_, st| if muted { mds.to_iced_style(st) } else { ms.to_iced_style(st) })
                    .on_press(mute_msg);
                row_items.push(mute_btn.into());
            }

            AppRowElement::Fill =>
            {
                row_items.push(Space::new().width(Length::Fill).into());
            }
        }
    }

    row(row_items)
        .spacing(spacing as f32)
        .align_y(Alignment::Center)
        .into()
}



fn truncate(s: &str, max: usize) -> String
{
    if s.chars().count() <= max { s.to_string() }
    else { format!("{}…", s.chars().take(max.saturating_sub(1)).collect::<String>()) }
}



fn task_msg_set_default(kind: MixerKind, name: String) -> Message
{
    Message::SetDefaultDevice(kind, name)
}



fn make_slider_style(s: &MixerSliderStyle, _status: iced::widget::slider::Status) -> iced::widget::slider::Style
{
    let rail_color   = s.rail_color.to_iced_color();
    let filled_color = s.rail_filled_color.to_iced_color();
    let rail_brad    = s.rail_border_radius;
    let rail_w       = s.rail_width;
    let handle_col   = s.handle_color.to_iced_color();
    let handle_bord  = s.handle_border_color.to_iced_color();
    let handle_bw    = s.handle_border_width;
    let handle_brad  = s.handle_border_radius;

    let handle_shape = match s.handle_shape
    {
        SliderHandleShape::Circle    => iced::widget::slider::HandleShape::Circle { radius: s.handle_circle_r },
        SliderHandleShape::Rectangle => iced::widget::slider::HandleShape::Rectangle
        {
            width:        s.handle_rect_w as u16,
            border_radius: iced::border::Radius
            {
                top_left:     handle_brad[0],
                top_right:    handle_brad[1],
                bottom_right: handle_brad[2],
                bottom_left:  handle_brad[3],
            },
        }
    };

    iced::widget::slider::Style
    {
        rail: iced::widget::slider::Rail
        {
            backgrounds: 
            (
                iced::Background::Color(filled_color),
                iced::Background::Color(rail_color),
            ),
            width:  rail_w,
            border: iced::Border
            {
                color:  iced::Color::TRANSPARENT,
                width:  0.,
                radius: iced::border::Radius
                {
                    top_left:     rail_brad[0],
                    top_right:    rail_brad[1],
                    bottom_right: rail_brad[2],
                    bottom_left:  rail_brad[3],
                },
            },
        },
        handle: iced::widget::slider::Handle
        {
            shape:        handle_shape,
            background:   iced::Background::Color(handle_col),
            border_color: handle_bord,
            border_width: handle_bw,
        }
    }
}



fn make_scrollbar_style(cfg: &MixerScrollbarConfig, theme: &Theme, status: scrollable::Status) -> scrollable::Style
{
    let brad = iced::border::Radius
    {
        top_left:     cfg.border_radius[0],
        top_right:    cfg.border_radius[1],
        bottom_right: cfg.border_radius[2],
        bottom_left:  cfg.border_radius[3]
    };

    let rail_bg   = cfg.rail_color.to_iced_color();
    let rail_bord = cfg.rail_border_color.to_iced_color();
    let rail_bw   = cfg.rail_border_width;

    let scroller_color = match status
    {
        scrollable::Status::Dragged { .. } =>
        {
            cfg.scroller_dragging_color.to_iced_color()
        }
        scrollable::Status::Hovered
        {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
            ..
        } if is_horizontal_scrollbar_hovered || is_vertical_scrollbar_hovered =>
        {
            cfg.scroller_hovered_color.to_iced_color()
        }
        _ => cfg.scroller_color.to_iced_color()
    };

    let scroller_bord_color = cfg.scroller_border_color.to_iced_color();
    let scroller_bw         = cfg.scroller_border_width;

    let rail = scrollable::Rail
    {
        background: Some(iced::Background::Color(rail_bg)),
        border: iced::Border
        {
            color:  rail_bord,
            width:  rail_bw,
            radius: brad,
        },
        scroller: scrollable::Scroller
        {
            background: iced::Background::Color(scroller_color),
            border: iced::Border
            {
                color:  scroller_bord_color,
                width:  scroller_bw,
                radius: brad,
            },
        }
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
