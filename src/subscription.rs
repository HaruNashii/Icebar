// ============ IMPORTS ============
use iced::{event, mouse, time};
use std::time::Duration;





// ============ CRATES ============
use crate::{AppData, Message, helpers::misc::is_active_module, modules::{data::Modules, tray::{TraySubscription, tray_stream}}};





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

    iced::Subscription::batch(subs)
}
