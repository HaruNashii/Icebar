// ============ IMPORTS ============
use libpulse_binding::{callbacks::ListResult, context::{Context, FlagSet as ContextFlagSet, introspect::Introspector, subscribe::{Facility, InterestMaskSet}}, mainloop::threaded::Mainloop, volume::Volume};
use std::{pin::Pin, sync::{Arc, Mutex}};
use serde::{Deserialize, Serialize};
use iced::{Task, widget::button};
 
 


 
 
// ============ CRATES ============
use crate::helpers::{color::{ColorType, Gradient}, style::{TextOrientation, UserStyle, orient_text, set_style, SideOption}};
use crate::ron::ActionOnClick;
use crate::update::Message;
use crate::AppData;

 
 




// ============ CONFIG ============
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct VolumeOutputConfig
{
    pub output_volume_format:                        [String; 6],
    pub output_volume_muted_format:                  String,
    pub incremental_steps_output:                    u8,
    pub action_on_left_click_volume_output:          ActionOnClick,
    pub action_on_right_click_volume_output:         ActionOnClick,
    pub volume_output_padding:                       u16,
    pub volume_output_text_size:                     u32,
    pub volume_output_text_color:                    ColorType,
    pub volume_output_text_orientation:              TextOrientation,
    pub volume_output_button_color:                  ColorType,
    pub volume_output_button_hovered_color:          ColorType,
    pub volume_output_button_hovered_text_color:     ColorType,
    pub volume_output_button_pressed_text_color:     ColorType,
    pub volume_output_button_pressed_color:          ColorType,
    pub volume_output_border_color:                  ColorType,
    pub volume_output_border_size:                   f32,
    pub volume_output_border_radius:                 [f32; 4],
    pub volume_output_side_separator:                Option<SideOption>,
    pub volume_output_side_separator_color:          ColorType,
    pub volume_output_side_separator_width:          f32,
    pub volume_output_side_separator_height:         f32,
    pub volume_output_button_gradient_color:         Option<Gradient>,
    pub volume_output_button_hovered_gradient_color: Option<Gradient>,
    pub volume_output_button_pressed_gradient_color: Option<Gradient>,
    pub volume_output_button_shadow_color:           Option<ColorType>,
    pub volume_output_button_shadow_x:               f32,
    pub volume_output_button_shadow_y:               f32,
    pub volume_output_button_shadow_blur:            f32,
}

impl Default for VolumeOutputConfig
{
    fn default() -> Self
    {
        Self
        {
            output_volume_format:                        ["   {}%".into(),"󰖀   {}%".into(),"   {}%".into(),"   {}%".into(),"   {}%".into(),"   + {}%".into()],
            output_volume_muted_format:                  "   Muted".into(),
            incremental_steps_output:                    10,
            action_on_left_click_volume_output:          ActionOnClick::DefaultAction,
            action_on_right_click_volume_output:         ActionOnClick::DefaultAction,
            volume_output_padding:                       0,
            volume_output_text_size:                     15,
            volume_output_text_color:                    ColorType::RGB([255, 255, 255]),
            volume_output_text_orientation:              TextOrientation::Horizontal,
            volume_output_button_color:                  ColorType::RGB([55, 45, 65]),
            volume_output_button_hovered_color:          ColorType::RGB([150, 45, 85]),
            volume_output_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            volume_output_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            volume_output_button_pressed_color:          ColorType::RGB([85, 30, 50]),
            volume_output_border_color:                  ColorType::RGB([110, 80, 120]),
            volume_output_border_size:                   1.0,
            volume_output_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            volume_output_side_separator:                None,
            volume_output_side_separator_color:          ColorType::RGB([75, 75, 75]),
            volume_output_side_separator_width:          1.,
            volume_output_side_separator_height:         20.,
            volume_output_button_gradient_color:         None,
            volume_output_button_hovered_gradient_color: None,
            volume_output_button_pressed_gradient_color: None,
            volume_output_button_shadow_color:           None,
            volume_output_button_shadow_x:               0.0,
            volume_output_button_shadow_y:               0.0,
            volume_output_button_shadow_blur:            0.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MutedVolumeOutputConfig
{
    pub muted_volume_output_padding:                       u16,
    pub muted_volume_output_text_size:                     u32,
    pub muted_volume_output_text_color:                    ColorType,
    pub muted_volume_output_text_orientation:              TextOrientation,
    pub muted_volume_output_button_color:                  ColorType,
    pub muted_volume_output_button_hovered_color:          ColorType,
    pub muted_volume_output_button_hovered_text_color:     ColorType,
    pub muted_volume_output_button_pressed_text_color:     ColorType,
    pub muted_volume_output_button_pressed_color:          ColorType,
    pub muted_volume_output_border_color:                  ColorType,
    pub muted_volume_output_border_size:                   f32,
    pub muted_volume_output_border_radius:                 [f32; 4],
    pub muted_volume_output_side_separator:                Option<SideOption>,
    pub muted_volume_output_side_separator_color:          ColorType,
    pub muted_volume_output_side_separator_width:          f32,
    pub muted_volume_output_side_separator_height:         f32,
    pub muted_volume_output_button_gradient_color:         Option<Gradient>,
    pub muted_volume_output_button_hovered_gradient_color: Option<Gradient>,
    pub muted_volume_output_button_pressed_gradient_color: Option<Gradient>,
    pub muted_volume_output_button_shadow_color:           Option<ColorType>,
    pub muted_volume_output_button_shadow_x:               f32,
    pub muted_volume_output_button_shadow_y:               f32,
    pub muted_volume_output_button_shadow_blur:            f32,
}

impl Default for MutedVolumeOutputConfig
{
    fn default() -> Self
    {
        Self
        {
            muted_volume_output_padding:                       0,
            muted_volume_output_text_size:                     15,
            muted_volume_output_text_color:                    ColorType::RGB([255, 255, 255]),
            muted_volume_output_text_orientation:              TextOrientation::Horizontal,
            muted_volume_output_button_color:                  ColorType::RGB([150, 40, 80]),
            muted_volume_output_button_hovered_color:          ColorType::RGB([150, 45, 85]),
            muted_volume_output_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            muted_volume_output_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            muted_volume_output_button_pressed_color:          ColorType::RGB([85, 30, 50]),
            muted_volume_output_border_color:                  ColorType::RGB([110, 80, 120]),
            muted_volume_output_border_size:                   1.0,
            muted_volume_output_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            muted_volume_output_side_separator:                None,
            muted_volume_output_side_separator_color:          ColorType::RGB([75, 75, 75]),
            muted_volume_output_side_separator_width:          1.,
            muted_volume_output_side_separator_height:         20.,
            muted_volume_output_button_gradient_color:         None,
            muted_volume_output_button_hovered_gradient_color: None,
            muted_volume_output_button_pressed_gradient_color: None,
            muted_volume_output_button_shadow_color:           None,
            muted_volume_output_button_shadow_x:               0.0,
            muted_volume_output_button_shadow_y:               0.0,
            muted_volume_output_button_shadow_blur:            0.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct VolumeInputConfig
{
    pub input_volume_format:                        [String; 6],
    pub input_volume_muted_format:                  String,
    pub incremental_steps_input:                    u8,
    pub action_on_left_click_volume_input:          ActionOnClick,
    pub action_on_right_click_volume_input:         ActionOnClick,
    pub volume_input_padding:                       u16,
    pub volume_input_text_size:                     u32,
    pub volume_input_text_color:                    ColorType,
    pub volume_input_text_orientation:              TextOrientation,
    pub volume_input_button_color:                  ColorType,
    pub volume_input_button_hovered_color:          ColorType,
    pub volume_input_button_hovered_text_color:     ColorType,
    pub volume_input_button_pressed_text_color:     ColorType,
    pub volume_input_button_pressed_color:          ColorType,
    pub volume_input_border_color:                  ColorType,
    pub volume_input_border_size:                   f32,
    pub volume_input_border_radius:                 [f32; 4],
    pub volume_input_side_separator:                Option<SideOption>,
    pub volume_input_side_separator_color:          ColorType,
    pub volume_input_side_separator_width:          f32,
    pub volume_input_side_separator_height:         f32,
    pub volume_input_button_gradient_color:         Option<Gradient>,
    pub volume_input_button_hovered_gradient_color: Option<Gradient>,
    pub volume_input_button_pressed_gradient_color: Option<Gradient>,
    pub volume_input_button_shadow_color:           Option<ColorType>,
    pub volume_input_button_shadow_x:               f32,
    pub volume_input_button_shadow_y:               f32,
    pub volume_input_button_shadow_blur:            f32,
}

impl Default for VolumeInputConfig
{
    fn default() -> Self
    {
        Self
        {
            input_volume_format:                        ["   {}%".into(),"  {}%".into(),"  {}%".into(),"  {}%".into(),"  {}%".into(),"󰢴  {}%".into()],
            input_volume_muted_format:                  "   Muted".into(),
            incremental_steps_input:                    10,
            action_on_left_click_volume_input:          ActionOnClick::DefaultAction,
            action_on_right_click_volume_input:         ActionOnClick::DefaultAction,
            volume_input_padding:                       0,
            volume_input_text_size:                     15,
            volume_input_text_color:                    ColorType::RGB([255, 255, 255]),
            volume_input_text_orientation:              TextOrientation::Horizontal,
            volume_input_button_color:                  ColorType::RGB([55, 45, 65]),
            volume_input_button_hovered_color:          ColorType::RGB([150, 45, 85]),
            volume_input_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            volume_input_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            volume_input_button_pressed_color:          ColorType::RGB([85, 30, 50]),
            volume_input_border_color:                  ColorType::RGB([110, 80, 120]),
            volume_input_border_size:                   1.0,
            volume_input_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            volume_input_side_separator:                None,
            volume_input_side_separator_color:          ColorType::RGB([75, 75, 75]),
            volume_input_side_separator_width:          1.,
            volume_input_side_separator_height:         20.,
            volume_input_button_gradient_color:         None,
            volume_input_button_hovered_gradient_color: None,
            volume_input_button_pressed_gradient_color: None,
            volume_input_button_shadow_color:           None,
            volume_input_button_shadow_x:               0.0,
            volume_input_button_shadow_y:               0.0,
            volume_input_button_shadow_blur:            0.0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MutedVolumeInputConfig
{
    pub muted_volume_input_padding:                       u16,
    pub muted_volume_input_text_size:                     u32,
    pub muted_volume_input_text_color:                    ColorType,
    pub muted_volume_input_text_orientation:              TextOrientation,
    pub muted_volume_input_button_color:                  ColorType,
    pub muted_volume_input_button_hovered_color:          ColorType,
    pub muted_volume_input_button_hovered_text_color:     ColorType,
    pub muted_volume_input_button_pressed_text_color:     ColorType,
    pub muted_volume_input_button_pressed_color:          ColorType,
    pub muted_volume_input_border_color:                  ColorType,
    pub muted_volume_input_border_size:                   f32,
    pub muted_volume_input_border_radius:                 [f32; 4],
    pub muted_volume_input_side_separator:                Option<SideOption>,
    pub muted_volume_input_side_separator_color:          ColorType,
    pub muted_volume_input_side_separator_width:          f32,
    pub muted_volume_input_side_separator_height:         f32,
    pub muted_volume_input_button_gradient_color:         Option<Gradient>,
    pub muted_volume_input_button_hovered_gradient_color: Option<Gradient>,
    pub muted_volume_input_button_pressed_gradient_color: Option<Gradient>,
    pub muted_volume_input_button_shadow_color:           Option<ColorType>,
    pub muted_volume_input_button_shadow_x:               f32,
    pub muted_volume_input_button_shadow_y:               f32,
    pub muted_volume_input_button_shadow_blur:            f32,
}

impl Default for MutedVolumeInputConfig
{
    fn default() -> Self
    {
        Self
        {
            muted_volume_input_padding:                       0,
            muted_volume_input_text_size:                     15,
            muted_volume_input_text_color:                    ColorType::RGB([255, 255, 255]),
            muted_volume_input_text_orientation:              TextOrientation::Horizontal,
            muted_volume_input_button_color:                  ColorType::RGB([150, 40, 80]),
            muted_volume_input_button_hovered_color:          ColorType::RGB([150, 45, 85]),
            muted_volume_input_button_hovered_text_color:     ColorType::RGB([255, 255, 255]),
            muted_volume_input_button_pressed_text_color:     ColorType::RGB([255, 255, 255]),
            muted_volume_input_button_pressed_color:          ColorType::RGB([85, 30, 50]),
            muted_volume_input_border_color:                  ColorType::RGB([110, 80, 120]),
            muted_volume_input_border_size:                   1.0,
            muted_volume_input_border_radius:                 [3.0, 3.0, 3.0, 3.0],
            muted_volume_input_side_separator:                None,
            muted_volume_input_side_separator_color:          ColorType::RGB([75, 75, 75]),
            muted_volume_input_side_separator_width:          1.,
            muted_volume_input_side_separator_height:         20.,
            muted_volume_input_button_gradient_color:         None,
            muted_volume_input_button_hovered_gradient_color: None,
            muted_volume_input_button_pressed_gradient_color: None,
            muted_volume_input_button_shadow_color:           None,
            muted_volume_input_button_shadow_x:               0.0,
            muted_volume_input_button_shadow_y:               0.0,
            muted_volume_input_button_shadow_blur:            0.0,
        }
    }
}

// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct VolumeData
{
    pub output_volume_level: String,
    pub input_volume_level: String,
    pub volume_output_is_muted: bool,
    pub volume_input_is_muted: bool,
    pub is_hovering_volume_output: bool,
    pub is_hovering_volume_input: bool,
    pub volume_output_raw: f32,
    pub volume_input_raw: f32,
}
 
#[derive(Default, Clone)]
struct PulseState
{
    output_volume: f32,   
    output_muted:  bool,
    input_volume:  f32,
    input_muted:   bool,
}
 
pub enum VolumeAction
{
    IncreaseOutput(u8),
    DecreaseOutput(u8),
    IncreaseInput(u8),
    DecreaseInput(u8),
    MuteOutput,
    MuteInput,
}
 


 
 
pub fn volume_subscription() -> Pin<Box<dyn futures::Stream<Item = Message> + Send>>
{
    Box::pin(async_stream::stream!
    {
        let state: Arc<Mutex<PulseState>> = Arc::new(Mutex::new(PulseState::default()));
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        let state_cb  = Arc::clone(&state);
        let tx_clone  = tx.clone();
 
        std::thread::spawn(move ||
        {
            let mut mainloop = match Mainloop::new()
            {
                Some(m) => m,
                None    => return,
            };
            if mainloop.start().is_err() { return; }

            // SAFETY: All PA operations called from outside a PA callback
            // must be performed while holding the threaded mainloop lock.
            // Without it, concurrent pushes into PA's internal queue trigger
            // the `pa_queue_push` assertion and abort the process.
            mainloop.lock();

            let context = match Context::new(&mainloop, "icebar-volume")
            {
                Some(c) => Arc::new(Mutex::new(c)),
                None    =>
                {
                    mainloop.unlock();
                    return;
                }
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
                    libpulse_binding::context::State::Failed | libpulse_binding::context::State::Terminated =>
                    {
                        mainloop.unlock();
                        return;
                    }
                    _ => {}
                }
                mainloop.unlock();
                std::thread::sleep(std::time::Duration::from_millis(10));
                mainloop.lock();
            }

            // ── fetch initial state ─────────────────────────────────────────
            {
                let s = Arc::clone(&state_cb);
                let t = tx_clone.clone();
                let introspector = context.lock().unwrap().introspect();
                fetch_sink(&introspector, Arc::clone(&s), t.clone());
                fetch_source(&introspector, Arc::clone(&s), t.clone());
            }

            // ── subscribe to sink + source change events ────────────────────
            {
                let ctx = Arc::clone(&context);
                let s   = Arc::clone(&state_cb);
                let t   = tx_clone.clone();

                context.lock().unwrap().subscribe(
                    InterestMaskSet::SINK | InterestMaskSet::SOURCE,
                    |_| {},
                );

                context.lock().unwrap().set_subscribe_callback(Some(Box::new(
                    move |facility, _op, _index|
                    {
                        let introspector = ctx.lock().unwrap().introspect();
                        match facility
                        {
                            Some(Facility::Sink)   => fetch_sink(&introspector, Arc::clone(&s), t.clone()),
                            Some(Facility::Source) => fetch_source(&introspector, Arc::clone(&s), t.clone()),
                            _ => {}
                        }
                    }
                )));
            }

            mainloop.unlock();
            loop { std::thread::sleep(std::time::Duration::from_secs(60)); }
        });
 
        // ── yield a Message each time the callback fires ────────────────────
        while rx.recv().await.is_some()
        {
            let s = state.lock().unwrap().clone();
            yield Message::VolumeUpdated(s.output_volume, s.output_muted, s.input_volume, s.input_muted);
        }
    })
}
 


fn fetch_sink(introspector: &Introspector, state: Arc<Mutex<PulseState>>, tx: tokio::sync::mpsc::UnboundedSender<()>)
{
    introspector.get_sink_info_by_name("@DEFAULT_SINK@", move |list|
    {
        if let ListResult::Item(info) = list
        {
            let vol = info.volume.avg().0 as f32 / Volume::NORMAL.0 as f32;
            let mut s = state.lock().unwrap();
            s.output_volume = vol;
            s.output_muted  = info.mute;
            let _ = tx.send(());
        }
    });
}
 


fn fetch_source(introspector: &Introspector, state: Arc<Mutex<PulseState>>, tx: tokio::sync::mpsc::UnboundedSender<()>)
{
    introspector.get_source_info_by_name("@DEFAULT_SOURCE@", move |list|
    {
        if let ListResult::Item(info) = list
        {
            let vol = info.volume.avg().0 as f32 / Volume::NORMAL.0 as f32;
            let mut s = state.lock().unwrap();
            s.input_volume = vol;
            s.input_muted  = info.mute;
            let _ = tx.send(());
        }
    });
}
 
 

pub fn volume(volume_modifier: VolumeAction) -> Task<crate::update::Message>
{
    let args: (&'static str, &'static str, String) = match volume_modifier
    {
        VolumeAction::IncreaseOutput(v) => ("set-volume", "@DEFAULT_SINK@",   format!("{}%+", v)),
        VolumeAction::DecreaseOutput(v) => ("set-volume", "@DEFAULT_SINK@",   format!("{}%-", v)),
        VolumeAction::MuteOutput        => ("set-mute",   "@DEFAULT_SINK@",   "toggle".into()),
        VolumeAction::IncreaseInput(v)  => ("set-volume", "@DEFAULT_SOURCE@", format!("{}%+", v)),
        VolumeAction::DecreaseInput(v)  => ("set-volume", "@DEFAULT_SOURCE@", format!("{}%-", v)),
        VolumeAction::MuteInput         => ("set-mute",   "@DEFAULT_SOURCE@", "toggle".into()),
    };
    Task::perform(async move 
    {
        let _ = tokio::process::Command::new("wpctl").args([args.0, args.1, &args.2]).output().await;
    },|_| Message::Nothing,)
}
 


pub fn define_volume_output_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    if app.modules_data.volume_data.volume_output_is_muted
    {
        let hovered =           app.ron_config.muted_volume_output.muted_volume_output_button_hovered_color;
        let hovered_text =      app.ron_config.muted_volume_output.muted_volume_output_button_hovered_text_color;
        let pressed_text =      app.ron_config.muted_volume_output.muted_volume_output_button_pressed_text_color;
        let pressed =           app.ron_config.muted_volume_output.muted_volume_output_button_pressed_color;
        let normal =            app.ron_config.muted_volume_output.muted_volume_output_button_color;
        let normal_text =       app.ron_config.muted_volume_output.muted_volume_output_text_color;
        let border_size =       app.ron_config.muted_volume_output.muted_volume_output_border_size;
        let border_color =  app.ron_config.muted_volume_output.muted_volume_output_border_color;
        let border_radius =     app.ron_config.muted_volume_output.muted_volume_output_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.muted_volume_output.muted_volume_output_button_gradient_color.clone(), hovered_gradient: app.ron_config.muted_volume_output.muted_volume_output_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.muted_volume_output.muted_volume_output_button_pressed_gradient_color.clone(), shadow_color: app.ron_config.muted_volume_output.muted_volume_output_button_shadow_color, shadow_x: app.ron_config.muted_volume_output.muted_volume_output_button_shadow_x, shadow_y: app.ron_config.muted_volume_output.muted_volume_output_button_shadow_y, shadow_blur: app.ron_config.muted_volume_output.muted_volume_output_button_shadow_blur })
    }
    else
    {
        let hovered =           app.ron_config.volume_output.volume_output_button_hovered_color;
        let hovered_text =      app.ron_config.volume_output.volume_output_button_hovered_text_color;
        let pressed_text =      app.ron_config.volume_output.volume_output_button_pressed_text_color;
        let pressed =           app.ron_config.volume_output.volume_output_button_pressed_color;
        let normal =            app.ron_config.volume_output.volume_output_button_color;
        let normal_text =       app.ron_config.volume_output.volume_output_text_color;
        let border_size =       app.ron_config.volume_output.volume_output_border_size;
        let border_color =  app.ron_config.volume_output.volume_output_border_color;
        let border_radius =     app.ron_config.volume_output.volume_output_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.volume_output.volume_output_button_gradient_color.clone(), hovered_gradient: app.ron_config.volume_output.volume_output_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.volume_output.volume_output_button_pressed_gradient_color.clone(), shadow_color: app.ron_config.volume_output.volume_output_button_shadow_color, shadow_x: app.ron_config.volume_output.volume_output_button_shadow_x, shadow_y: app.ron_config.volume_output.volume_output_button_shadow_y, shadow_blur: app.ron_config.volume_output.volume_output_button_shadow_blur })
    }
}



pub fn define_volume_input_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    if app.modules_data.volume_data.volume_input_is_muted
    {
        let hovered =              app.ron_config.muted_volume_input.muted_volume_input_button_hovered_color;
        let hovered_text =         app.ron_config.muted_volume_input.muted_volume_input_button_hovered_text_color;
        let pressed_text =         app.ron_config.muted_volume_input.muted_volume_input_button_pressed_text_color;
        let pressed =              app.ron_config.muted_volume_input.muted_volume_input_button_pressed_color;
        let normal =               app.ron_config.muted_volume_input.muted_volume_input_button_color;
        let normal_text =          app.ron_config.muted_volume_input.muted_volume_input_text_color;
        let border_size =          app.ron_config.muted_volume_input.muted_volume_input_border_size;
        let border_color =     app.ron_config.muted_volume_input.muted_volume_input_border_color;
        let border_radius =        app.ron_config.muted_volume_input.muted_volume_input_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.muted_volume_input.muted_volume_input_button_gradient_color.clone(), hovered_gradient: app.ron_config.muted_volume_input.muted_volume_input_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.muted_volume_input.muted_volume_input_button_pressed_gradient_color.clone(), shadow_color: app.ron_config.muted_volume_input.muted_volume_input_button_shadow_color, shadow_x: app.ron_config.muted_volume_input.muted_volume_input_button_shadow_x, shadow_y: app.ron_config.muted_volume_input.muted_volume_input_button_shadow_y, shadow_blur: app.ron_config.muted_volume_input.muted_volume_input_button_shadow_blur })
    }
    else
    {
        let hovered =           app.ron_config.volume_input.volume_input_button_hovered_color;
        let hovered_text =      app.ron_config.volume_input.volume_input_button_hovered_text_color;
        let pressed_text =      app.ron_config.volume_input.volume_input_button_pressed_text_color;
        let pressed =           app.ron_config.volume_input.volume_input_button_pressed_color;
        let normal =            app.ron_config.volume_input.volume_input_button_color;
        let normal_text =       app.ron_config.volume_input.volume_input_text_color;
        let border_size =       app.ron_config.volume_input.volume_input_border_size;
        let border_color =  app.ron_config.volume_input.volume_input_border_color;
        let border_radius =     app.ron_config.volume_input.volume_input_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed_text, pressed, normal, normal_text, border_color, border_size, border_radius, normal_gradient: app.ron_config.volume_input.volume_input_button_gradient_color.clone(), hovered_gradient: app.ron_config.volume_input.volume_input_button_hovered_gradient_color.clone(), pressed_gradient: app.ron_config.volume_input.volume_input_button_pressed_gradient_color.clone(), shadow_color: app.ron_config.volume_input.volume_input_button_shadow_color, shadow_x: app.ron_config.volume_input.volume_input_button_shadow_x, shadow_y: app.ron_config.volume_input.volume_input_button_shadow_y, shadow_blur: app.ron_config.volume_input.volume_input_button_shadow_blur })
    }
}



pub fn define_volume_text(text: &str, text_orientation: &TextOrientation) -> String { orient_text(text, text_orientation) }





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use iced::{widget::button, Background, Color};
    use crate::helpers::{color::ColorType, style::TextOrientation};
 
    // ---- define_volume_text ------------------------------------------------
 
    #[test]
    fn volume_text_horizontal_returns_unchanged()
    {
        assert_eq!(define_volume_text("🔊 75%", &TextOrientation::Horizontal), "🔊 75%");
    }
 
    #[test]
    fn volume_text_vertical_inserts_newlines()
    {
        let result = define_volume_text("abc", &TextOrientation::Vertical);
        assert_eq!(result, "a\nb\nc");
    }
 
    #[test]
    fn volume_text_empty_string_both_orientations()
    {
        assert_eq!(define_volume_text("", &TextOrientation::Horizontal), "");
        assert_eq!(define_volume_text("", &TextOrientation::Vertical),   "");
    }
 
    // ---- define_volume_output_style ----------------------------------------
 
    fn make_output_app(muted: bool) -> AppData
    {
        let mut app = AppData::default();
        app.modules_data.volume_data.volume_output_is_muted = muted;
        app.ron_config.volume_output.volume_output_button_color = ColorType::RGB([0, 200, 0]);
        app.ron_config.muted_volume_output.muted_volume_output_button_color = ColorType::RGB([200, 0, 0]);
        app.ron_config.volume_output.volume_output_button_hovered_color = ColorType::RGB([0, 100, 0]);
        app.ron_config.muted_volume_output.muted_volume_output_button_hovered_color = ColorType::RGB([100, 0, 0]);
        app.ron_config.volume_output.volume_output_button_pressed_color = ColorType::RGB([0, 50, 0]);
        app.ron_config.muted_volume_output.muted_volume_output_button_pressed_color = ColorType::RGB([50, 0, 0]);
        app
    }
 
    #[test]
    fn volume_output_active_unmuted_uses_normal_color()
    {
        let app   = make_output_app(false);
        let style = define_volume_output_style(&app, button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 200, 0))));
    }
 
    #[test]
    fn volume_output_active_muted_uses_muted_color()
    {
        let app   = make_output_app(true);
        let style = define_volume_output_style(&app, button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(200, 0, 0))));
    }
 
    #[test]
    fn volume_output_muted_and_unmuted_backgrounds_differ()
    {
        let muted   = define_volume_output_style(&make_output_app(true),  button::Status::Active);
        let unmuted = define_volume_output_style(&make_output_app(false), button::Status::Active);
        assert_ne!(muted.background, unmuted.background);
    }
 
    #[test]
    fn volume_output_hovered_unmuted_uses_hovered_color()
    {
        let app   = make_output_app(false);
        let style = define_volume_output_style(&app, button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 100, 0))));
    }
 
    #[test]
    fn volume_output_hovered_muted_uses_muted_hovered_color()
    {
        let app   = make_output_app(true);
        let style = define_volume_output_style(&app, button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(100, 0, 0))));
    }
 
    #[test]
    fn volume_output_pressed_unmuted_uses_pressed_color()
    {
        let app   = make_output_app(false);
        let style = define_volume_output_style(&app, button::Status::Pressed);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 50, 0))));
    }
 
    // ---- define_volume_input_style -----------------------------------------
 
    fn make_input_app(muted: bool) -> AppData
    {
        let mut app = AppData::default();
        app.modules_data.volume_data.volume_input_is_muted = muted;
        app.ron_config.volume_input.volume_input_button_color = ColorType::RGB([0, 0, 200]);
        app.ron_config.muted_volume_input.muted_volume_input_button_color = ColorType::RGB([200, 0, 200]);
        app.ron_config.volume_input.volume_input_button_hovered_color = ColorType::RGB([0, 0, 100]);
        app.ron_config.muted_volume_input.muted_volume_input_button_hovered_color = ColorType::RGB([100, 0, 100]);
        app.ron_config.volume_input.volume_input_button_pressed_color = ColorType::RGB([0, 0, 50]);
        app.ron_config.muted_volume_input.muted_volume_input_button_pressed_color = ColorType::RGB([50, 0, 50]);
        app
    }
 
    #[test]
    fn volume_input_active_unmuted_uses_normal_color()
    {
        let app   = make_input_app(false);
        let style = define_volume_input_style(&app, button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(0, 0, 200))));
    }
 
    #[test]
    fn volume_input_active_muted_uses_muted_color()
    {
        let app   = make_input_app(true);
        let style = define_volume_input_style(&app, button::Status::Active);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(200, 0, 200))));
    }
 
    #[test]
    fn volume_input_muted_and_unmuted_backgrounds_differ()
    {
        let muted   = define_volume_input_style(&make_input_app(true),  button::Status::Active);
        let unmuted = define_volume_input_style(&make_input_app(false), button::Status::Active);
        assert_ne!(muted.background, unmuted.background);
    }
 
    #[test]
    fn volume_input_hovered_muted_uses_muted_hovered_color()
    {
        let app   = make_input_app(true);
        let style = define_volume_input_style(&app, button::Status::Hovered);
        assert_eq!(style.background, Some(Background::Color(Color::from_rgb8(100, 0, 100))));
    }
}
