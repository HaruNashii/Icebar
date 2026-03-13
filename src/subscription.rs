// ============ IMPORTS ============
use iced::{event, mouse, time};
use std::time::Duration;





// ============ CRATES ============
use crate::{helpers::config_watcher::config_file_watcher, modules::{data::Modules, network::network_subscription, tray::{TraySubscription, tray_stream}}};
use crate::update::Message;
use crate::AppData;





// ============ FUNCTIONS ============
pub fn subscription(app: &AppData) -> iced::Subscription<Message>
{
    let event_reader = event::listen_with(|event, _status, _id| 
    {
        match event 
        {
            iced::Event::Mouse(mouse::Event::WheelScrolled {delta, ..} ) => { Some(Message::MouseWheelScrolled(delta)) }
            iced::Event::Mouse(mouse::Event::CursorMoved { position }) => { Some(Message::CursorMoved(position)) }
            _=> None
        }
    });


    let mut subs = vec!
    [
        time::every(Duration::from_millis(225)).map(|_| Message::Tick),
        event_reader,
    ];


    for module_name in &app.modules_data.active_modules
    {
        match module_name
        {
            Modules::Cpu => subs.push(time::every(Duration::from_millis(app.ron_config.cpu_update_interval)).map(|_| Message::UpdateCpu)),
            Modules::CpuTemp => subs.push(time::every(Duration::from_millis(app.ron_config.cpu_temp_update_interval)).map(|_| Message::UpdateCpuTemp)),
            Modules::Ram => subs.push(time::every(Duration::from_millis(app.ron_config.ram_update_interval)).map(|_| Message::UpdateRam)),
            Modules::FocusedWindowNiri => subs.push(time::every(Duration::from_millis(app.ron_config.focused_window_update_interval)).map(|_| Message::UpdateFocusedWindowNiri)),
            Modules::FocusedWindowSway => subs.push(time::every(Duration::from_millis(app.ron_config.focused_window_update_interval)).map(|_| Message::UpdateFocusedWindowSway)),
            Modules::FocusedWindowHypr => subs.push(time::every(Duration::from_millis(app.ron_config.focused_window_update_interval)).map(|_| Message::UpdateFocusedWindowHypr)),
            Modules::Tray => subs.push(iced::Subscription::run_with(TraySubscription, tray_stream)),
            Modules::Network => subs.push(network_subscription(app.ron_config.network_disconnected_text.clone())),
            _=> {},
        }
    }

    if let Some(reload_interval) = app.ron_config.bar_check_reload_interval_ms 
    {
        subs.push(config_file_watcher(reload_interval));
    };

    iced::Subscription::batch(subs)
}
