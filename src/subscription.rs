// ============ IMPORTS ============
use iced::{event, mouse, time};
use std::time::Duration;





// ============ CRATES ============
use crate::modules::{data::Modules, network::{network_subscription}, tray::{TraySubscription, tray_stream}};
use crate::helpers::misc::is_active_module;
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
        time::every(Duration::from_millis(150)).map(|_| Message::Tick),
        event_reader,
    ];


    if is_active_module(&app.modules_data.active_modules, Modules::Tray) 
    {
        subs.push(iced::Subscription::run_with(TraySubscription, tray_stream));
    };
    if is_active_module(&app.modules_data.active_modules, Modules::Network) 
    {
        subs.push(network_subscription(app.ron_config.network_disconnected_text.clone()));
    };

    iced::Subscription::batch(subs)
}
