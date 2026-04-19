// ============ IMPORTS ============
use std::time::Duration;
use iced::event;





// ============ CRATES ============
use crate::helpers::config_watcher::config_file_watcher;
use crate::AppData;
use crate::update::Message;
use crate::ron::ActionOnClick;
use crate::modules::
{
    data::Modules,
    hypr::hypr_event_subscription,
    network::network_subscription,
    plasma::plasma_event_subscription,
    sway::sway_event_subscription,
    niri::niri_event_subscription,
    clock_subscription::clock_subscription,
    power_profile::power_profile_subscription,
    media_player::media_player_subscription,
    tray::{TraySubscription, tray_stream},
    poll_subscriptions::{CpuPollConfig, cpu_subscription, RamPollConfig, ram_subscription, DiskPollConfig, disk_subscription, CpuTempPollConfig, cpu_temp_subscription, NetworkSpeedPollConfig, network_speed_subscription}
};
use crate::volume_mixer::volume_mixer_subscription;





// ============ FUNCTIONS ============
pub fn subscription(app: &AppData) -> iced::Subscription<Message>
{
    let has_mixer_action = |action: &ActionOnClick| matches!(action,
        ActionOnClick::ShowVolumeOutputMixer | ActionOnClick::ShowVolumeInputMixer | ActionOnClick::ShowCalendar
    );

    let needs_cursor = app.modules_data.active_modules.contains(&Modules::Tray)
        || app.ron_config.auto_hide.is_some()
        || app.modules_data.active_modules.contains(&Modules::Clock) && (has_mixer_action(&app.ron_config.clock.action_on_left_click_clock)
        || has_mixer_action(&app.ron_config.clock.action_on_right_click_clock))
        || has_mixer_action(&app.ron_config.volume_output.action_on_left_click_volume_output)
        || has_mixer_action(&app.ron_config.volume_output.action_on_right_click_volume_output)
        || has_mixer_action(&app.ron_config.volume_input.action_on_left_click_volume_input)
        || has_mixer_action(&app.ron_config.volume_input.action_on_right_click_volume_input)
        || app.modules_data.active_modules.contains(&Modules::Network) && (has_mixer_action(&app.ron_config.network.action_on_left_click_network)
        || has_mixer_action(&app.ron_config.network.action_on_right_click_network))
        || app.modules_data.active_modules.contains(&Modules::Cpu) && (has_mixer_action(&app.ron_config.cpu.action_on_left_click_cpu)
        || has_mixer_action(&app.ron_config.cpu.action_on_right_click_cpu))
        || app.modules_data.active_modules.contains(&Modules::CpuTemp) && (has_mixer_action(&app.ron_config.cpu_temp.action_on_left_click_cpu_temp)
        || has_mixer_action(&app.ron_config.cpu_temp.action_on_right_click_cpu_temp))
        || (app.modules_data.active_modules.contains(&Modules::MediaPlayerMetaData) || app.modules_data.active_modules.contains(&Modules::MediaPlayerButtons))
            && (has_mixer_action(&app.ron_config.media_player_metadata.action_on_left_click_media_player_metadata)
            || has_mixer_action(&app.ron_config.media_player_metadata.action_on_right_click_media_player_metadata))
        || app.modules_data.active_modules.contains(&Modules::PowerProfile) && (has_mixer_action(&app.ron_config.power_profile.action_on_left_click_power_profile)
        || has_mixer_action(&app.ron_config.power_profile.action_on_right_click_power_profile));

    let event_reader = match (app.modules_data.active_modules.contains(&Modules::Tray), needs_cursor)
    {
        (true,  true)  => event::listen_with(event_reader_with_tray_cursor),
        (true,  false) => event::listen_with(event_reader_with_tray_no_cursor),
        (false, true)  => event::listen_with(event_reader_without_tray_cursor),
        (false, false) => event::listen_with(event_reader_without_tray_no_cursor)
    };

    let mut subs = vec![event_reader];

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
    if has_continuous_modules
    {
        subs.push(iced::time::every(Duration::from_millis(225)).map(|_| Message::Tick));
    }

    let mut volume_sub_added        = false;
    let mut mixer_sub_added         = false;
    let mut hypr_sub_added          = false;
    let mut sway_sub_added          = false;
    let mut niri_event_sub_added    = false;
    let mut power_profile_sub_added = false;
    let mut media_player_sub_added  = false;

    for module_name in &app.modules_data.active_modules
    {
        match module_name
        {
            Modules::Cpu =>
            {
                let cfg = CpuPollConfig
                {
                    interval_ms: app.ron_config.cpu.cpu_update_interval.max(1),
                    format:      app.ron_config.cpu.cpu_format.clone(),
                    orientation: app.ron_config.cpu.cpu_text_orientation
                };
                subs.push(iced::Subscription::run_with(cfg, cpu_subscription));
            }

            Modules::Ram =>
            {
                let cfg = RamPollConfig
                {
                    interval_ms: app.ron_config.ram.ram_update_interval.max(1),
                    format:      app.ron_config.ram.ram_format.clone(),
                    orientation: app.ron_config.ram.ram_text_orientation
                };
                subs.push(iced::Subscription::run_with(cfg, ram_subscription));
            }

            Modules::Disk =>
            {
                let cfg = DiskPollConfig
                {
                    interval_ms: app.ron_config.disk.disk_update_interval.max(1),
                    format:      app.ron_config.disk.disk_format.clone(),
                    orientation: app.ron_config.disk.disk_text_orientation,
                    mount:       app.ron_config.disk.disk_mount.clone()
                };
                subs.push(iced::Subscription::run_with(cfg, disk_subscription));
            }

            Modules::CpuTemp =>
            {
                let cfg = CpuTempPollConfig
                {
                    interval_ms: app.ron_config.cpu_temp.cpu_temp_update_interval.max(1),
                    format:      app.ron_config.cpu_temp.cpu_temp_format.clone(),
                    orientation: app.ron_config.cpu_temp.cpu_temp_text_orientation
                };
                subs.push(iced::Subscription::run_with(cfg, cpu_temp_subscription));
            }

            Modules::Network =>
            {
                subs.push(network_subscription(app.ron_config.network.network_disconnected_text.clone()));
                subs.push(iced::Subscription::run_with(NetworkSpeedPollConfig, network_speed_subscription));
            }

            Modules::Clock =>
            {
                let granularity = app.ron_config.clock.clock_update_interval.clamp(100, 1000);
                subs.push(clock_subscription(granularity));
            }

            Modules::PowerProfile if !power_profile_sub_added =>
            {
                    subs.push(iced::Subscription::run(power_profile_subscription));
                    power_profile_sub_added = true;
            }

            Modules::NiriWorkspaces | Modules::FocusedWindowNiri if !niri_event_sub_added =>
            {
                    subs.push(iced::Subscription::run(niri_event_subscription));
                    niri_event_sub_added = true;
            }

            Modules::Tray => subs.push(iced::Subscription::run_with(TraySubscription, tray_stream)),

            Modules::PlasmaWorkspaces => subs.push(iced::Subscription::run(plasma_event_subscription)),

            Modules::MediaPlayerMetaData | Modules::MediaPlayerButtons if !media_player_sub_added =>
            {
                    let player = app.ron_config.media_player_metadata.player.clone();
                    let format = app.ron_config.media_player_metadata.media_player_metadata_format.clone();
                    subs.push(iced::Subscription::run_with((player, format), |(p, f)|
                    {
                        media_player_subscription(p.clone(), f.clone())
                    }));
                    media_player_sub_added = true;
            }

            Modules::FocusedWindowHypr | Modules::HyprWorkspaces if !hypr_sub_added =>
            {
                    subs.push(iced::Subscription::run(hypr_event_subscription));
                    hypr_sub_added = true;
            }

            Modules::SwayWorkspaces | Modules::FocusedWindowSway if !sway_sub_added =>
            {
                    subs.push(iced::Subscription::run(sway_event_subscription));
                    sway_sub_added = true;
            }

            Modules::VolumeOutput | Modules::VolumeInput =>
            {
                if !volume_sub_added
                {
                    subs.push(iced::Subscription::run(crate::modules::volume::volume_subscription));
                    volume_sub_added = true;
                }
                if !mixer_sub_added
                {
                    subs.push(iced::Subscription::run(volume_mixer_subscription));
                    mixer_sub_added = true;
                }
            }

            _ => {}
        }
    }

    if let Some(reload_interval) = app.ron_config.general.bar_check_reload_interval_ms
    {
        subs.push(config_file_watcher(reload_interval, app.cli_data.config.clone()));
    }

    if app.ron_config.auto_hide.is_some()
    {
        subs.push(iced::time::every(Duration::from_millis(16)).map(|_| Message::BarVisibilityTick));
    }

    iced::Subscription::batch(subs)
}



fn event_reader_with_tray_cursor(event: iced::Event, _status: iced::event::Status, _id: iced::window::Id) -> Option<Message>
{
    match event
    {
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape), .. }) => Some(Message::CloseContextMenuAndCalendar),
        iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => Some(Message::MouseButtonClicked),
        iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta, .. }) => Some(Message::MouseWheelScrolled(delta)),
        iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => Some(Message::CursorMoved(position)),
        _ => None
    }
}



fn event_reader_with_tray_no_cursor(event: iced::Event, _status: iced::event::Status, _id: iced::window::Id) -> Option<Message>
{
    match event
    {
        iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape), .. }) => Some(Message::CloseContextMenuAndCalendar),
        iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => Some(Message::MouseButtonClicked),
        iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta, .. }) => Some(Message::MouseWheelScrolled(delta)),
        _ => None
    }
}



fn event_reader_without_tray_cursor(event: iced::Event, _status: iced::event::Status, _id: iced::window::Id) -> Option<Message>
{
    match event
    {
        iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta, .. }) => Some(Message::MouseWheelScrolled(delta)),
        iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => Some(Message::MouseButtonClicked),
        iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => Some(Message::CursorMoved(position)),
        _ => None
    }
}



fn event_reader_without_tray_no_cursor(event: iced::Event, _status: iced::event::Status, _id: iced::window::Id) -> Option<Message>
{
    match event
    {
        iced::Event::Mouse(iced::mouse::Event::WheelScrolled { delta, .. }) => Some(Message::MouseWheelScrolled(delta)),
        iced::Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left)) => Some(Message::MouseButtonClicked),
        _ => None
    }
}
