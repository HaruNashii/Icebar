// ============ IMPORTS ============
use libpulse_binding::{callbacks::ListResult, context::{Context, FlagSet as ContextFlagSet, introspect::Introspector, subscribe::{Facility, InterestMaskSet}}, mainloop::threaded::Mainloop, volume::Volume};
use std::{pin::Pin, sync::{Arc, Mutex}};
use iced::widget::button;
 
 


 
 
// ============ CRATES ============
use crate::helpers::{style::{TextOrientation, UserStyle, orient_text, set_style}};
use crate::update::Message;
use crate::AppData;
 
 
 
 


// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct VolumeData
{
    pub output_volume_level: String,
    pub input_volume_level:  String,
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
 
 
 
pub fn volume(volume_modifier: VolumeAction)
{
    use std::process::Command;
    match volume_modifier
    {
        VolumeAction::IncreaseOutput(v) => { let _ = Command::new("wpctl").args(["set-volume", "@DEFAULT_SINK@",   &format!("{}%+", v)]).output(); }
        VolumeAction::DecreaseOutput(v) => { let _ = Command::new("wpctl").args(["set-volume", "@DEFAULT_SINK@",   &format!("{}%-", v)]).output(); }
        VolumeAction::MuteOutput        => { let _ = Command::new("wpctl").args(["set-mute",   "@DEFAULT_SINK@",   "toggle"           ]).output(); }
        VolumeAction::IncreaseInput(v)  => { let _ = Command::new("wpctl").args(["set-volume", "@DEFAULT_SOURCE@", &format!("{}%+", v)]).output(); }
        VolumeAction::DecreaseInput(v)  => { let _ = Command::new("wpctl").args(["set-volume", "@DEFAULT_SOURCE@", &format!("{}%-", v)]).output(); }
        VolumeAction::MuteInput         => { let _ = Command::new("wpctl").args(["set-mute",   "@DEFAULT_SOURCE@", "toggle"           ]).output(); }
    };
}
 


pub fn define_volume_output_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    if app.volume_output_is_muted
    {
        let hovered =           app.ron_config.muted_volume_output_button_hovered_color_rgb;
        let hovered_text =      app.ron_config.muted_volume_output_button_hovered_text_color_rgb;
        let pressed =           app.ron_config.muted_volume_output_button_pressed_color_rgb;
        let normal =            app.ron_config.muted_volume_output_button_color_rgb;
        let normal_text =       app.ron_config.muted_volume_output_text_color_rgb;
        let border_size =       app.ron_config.muted_volume_output_border_size;
        let border_color_rgb =  app.ron_config.muted_volume_output_border_color_rgb;
        let border_radius =     app.ron_config.muted_volume_output_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgb, border_size, border_radius} )
    }
    else
    {
        let hovered =           app.ron_config.volume_output_button_hovered_color_rgb;
        let hovered_text =      app.ron_config.volume_output_button_hovered_text_color_rgb;
        let pressed =           app.ron_config.volume_output_button_pressed_color_rgb;
        let normal =            app.ron_config.volume_output_button_color_rgb;
        let normal_text =       app.ron_config.volume_output_text_color_rgb;
        let border_size =       app.ron_config.volume_output_border_size;
        let border_color_rgb =  app.ron_config.volume_output_border_color_rgb;
        let border_radius =     app.ron_config.volume_output_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgb, border_size, border_radius} )
    }
}



pub fn define_volume_input_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    if app.volume_input_is_muted
    {
        let hovered =              app.ron_config.muted_volume_input_button_hovered_color_rgb;
        let hovered_text =         app.ron_config.muted_volume_input_button_hovered_text_color_rgb;
        let pressed =              app.ron_config.muted_volume_input_button_pressed_color_rgb;
        let normal =               app.ron_config.muted_volume_input_button_color_rgb;
        let normal_text =          app.ron_config.muted_volume_input_text_color_rgb;
        let border_size =          app.ron_config.muted_volume_input_border_size;
        let border_color_rgb =     app.ron_config.muted_volume_input_border_color_rgb;
        let border_radius =        app.ron_config.muted_volume_input_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgb, border_size, border_radius} )
    }
    else
    {
        let hovered =           app.ron_config.volume_input_button_hovered_color_rgb;
        let hovered_text =      app.ron_config.volume_input_button_hovered_text_color_rgb;
        let pressed =           app.ron_config.volume_input_button_pressed_color_rgb;
        let normal =            app.ron_config.volume_input_button_color_rgb;
        let normal_text =       app.ron_config.volume_input_text_color_rgb;
        let border_size =       app.ron_config.volume_input_border_size;
        let border_color_rgb =  app.ron_config.volume_input_border_color_rgb;
        let border_radius =     app.ron_config.volume_input_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgb, border_size, border_radius} )
    }
}



pub fn define_volume_text(text: &str, text_orientation: &TextOrientation) -> String { orient_text(text, text_orientation) }





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::AppData;
    use iced::{Background, Color};
    use iced::widget::button;
    use crate::helpers::style::TextOrientation;
 
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
        app.volume_output_is_muted = muted;
        app.ron_config.volume_output_button_color_rgb         = [0,   200, 0];
        app.ron_config.muted_volume_output_button_color_rgb   = [200, 0,   0];
        app.ron_config.volume_output_button_hovered_color_rgb = [0,   100, 0];
        app.ron_config.muted_volume_output_button_hovered_color_rgb = [100, 0, 0];
        app.ron_config.volume_output_button_pressed_color_rgb = [0,   50,  0];
        app.ron_config.muted_volume_output_button_pressed_color_rgb = [50,  0, 0];
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
        app.volume_input_is_muted = muted;
        app.ron_config.volume_input_button_color_rgb         = [0,   0, 200];
        app.ron_config.muted_volume_input_button_color_rgb   = [200, 0, 200];
        app.ron_config.volume_input_button_hovered_color_rgb = [0,   0, 100];
        app.ron_config.muted_volume_input_button_hovered_color_rgb = [100, 0, 100];
        app.ron_config.volume_input_button_pressed_color_rgb = [0,   0, 50];
        app.ron_config.muted_volume_input_button_pressed_color_rgb = [50,  0, 50];
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
