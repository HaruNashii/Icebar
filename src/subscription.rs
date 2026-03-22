// ============ IMPORTS ============
use iced::{event, mouse, time};
use std::time::Duration;





// ============ CRATES ============
use crate::{helpers::config_watcher::config_file_watcher, modules::{data::Modules, hypr::hypr_event_subscription, network::network_subscription, sway::sway_event_subscription, tray::{TraySubscription, tray_stream}, volume::volume_subscription}};
use crate::update::Message;
use crate::AppData;





// ============ FUNCTIONS ============
pub fn subscription(app: &AppData) -> iced::Subscription<Message>
{
    let event_reader = if app.modules_data.active_modules.contains(&Modules::Tray)
    {
        event::listen_with(event_reader_with_tray)
    }
    else
    {
        event::listen_with(event_reader_without_tray)
    };


    let mut subs = vec!
    [
        event_reader,
    ];

    let has_continuous_modules = app.modules_data.active_modules.iter().any(|m| 
    {
        if let Modules::CustomModule(i) = m 
        {
            app.ron_config.custom_module.custom_modules.get(*i).is_some_and(|m| !m.continous_command.is_empty())
        } 
        else 
        {
            false 
        }
    });
    if has_continuous_modules { subs.push(time::every(Duration::from_millis(225)).map(|_| Message::Tick)); }

    let mut volume_sub_added = false;
    let mut hypr_sub_added = false;
    let mut sway_sub_added = false;
    let mut media_player_sub_added = false;
    for module_name in &app.modules_data.active_modules
    {
        match module_name
        {
            Modules::Disk =>                subs.push(time::every(Duration::from_millis(app.ron_config.disk.disk_update_interval)).map(|_| Message::UpdateDisk)),
            Modules::Tray =>                subs.push(iced::Subscription::run_with(TraySubscription, tray_stream)),
            Modules::Cpu =>                 subs.push(time::every(Duration::from_millis(app.ron_config.cpu.cpu_update_interval)).map(|_| Message::UpdateCpu)),
            Modules::CpuTemp =>             subs.push(time::every(Duration::from_millis(app.ron_config.cpu_temp.cpu_temp_update_interval)).map(|_| Message::UpdateCpuTemp)),
            Modules::Ram =>                 subs.push(time::every(Duration::from_millis(app.ron_config.ram.ram_update_interval)).map(|_| Message::UpdateRam)),
            Modules::FocusedWindowNiri =>   subs.push(time::every(Duration::from_millis(app.ron_config.focused_window.focused_window_update_interval)).map(|_| Message::UpdateFocusedWindowNiri)),
            Modules::Clock =>               subs.push(time::every(Duration::from_millis(app.ron_config.clock.clock_update_interval)).map(|_| Message::UpdateClock)),
            Modules::NiriWorkspaces =>      subs.push(time::every(Duration::from_millis(app.ron_config.workspace.niri_workspaces_update_interval)).map(|_| Message::UpdateNiriWorkspaces)),
            Modules::MediaPlayerMetaData | Modules::MediaPlayerButtons => 
            {
                if !media_player_sub_added 
                {
                    subs.push(time::every(Duration::from_millis(app.ron_config.media_player_metadata.media_player_metadata_update_interval)).map(|_| Message::UpdateMediaPlayerMetadata));
                    media_player_sub_added = true;
                };
            },
            Modules::Network =>             
            {
                subs.push(network_subscription(app.ron_config.network.network_disconnected_text.clone()));
                subs.push(time::every(Duration::from_secs(1)).map(|_| Message::UpdateNetworkSpeed));
            },
            Modules::FocusedWindowHypr | Modules::HyprWorkspaces =>
            {
                if !hypr_sub_added
                {
                    subs.push(iced::Subscription::run(hypr_event_subscription));
                    hypr_sub_added = true;
                }
            }
            Modules::SwayWorkspaces | Modules::FocusedWindowSway =>
            {
                if !sway_sub_added
                {
                    subs.push(iced::Subscription::run(sway_event_subscription));
                    sway_sub_added = true;
                }
            }
            Modules::VolumeOutput | Modules::VolumeInput =>
            {
                if !volume_sub_added
                {
                    subs.push(
                        iced::Subscription::run(volume_subscription)
                    );
                    volume_sub_added = true;
                }
            }
            _=> {},
        }
    }

    if let Some(reload_interval) = app.ron_config.general.bar_check_reload_interval_ms 
    {
        subs.push(config_file_watcher(reload_interval));
    };

    iced::Subscription::batch(subs)
}



fn event_reader_with_tray(event: iced::Event, _status: iced::event::Status, _id: iced::window::Id) -> Option<Message>
{
    match event 
    {
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape), .. }) => { Some(Message::CloseContextMenu) }
        iced::Event::Mouse(mouse::Event::ButtonPressed(_)) => { Some(Message::MouseButtonClicked) }
        iced::Event::Mouse(mouse::Event::WheelScrolled { delta, .. }) => Some(Message::MouseWheelScrolled(delta)),
        iced::Event::Mouse(mouse::Event::CursorMoved { position })    => Some(Message::CursorMoved(position)),
        _ => None
    }
}



fn event_reader_without_tray(event: iced::Event, _status: iced::event::Status, _id: iced::window::Id) -> Option<Message>
{
    match event 
    {
        iced::Event::Mouse(mouse::Event::WheelScrolled { delta, .. }) => Some(Message::MouseWheelScrolled(delta)),
        _ => None
    }
}
