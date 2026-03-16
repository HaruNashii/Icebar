// ============ IMPORTS ============
use iced::{Alignment, Color, Element, Length, Theme, widget::{Space, button, column, container, mouse_area, row}};





// ============ CRATES ============
use crate::{helpers::{misc::{create_button_container, create_button_container_without_hover_message}, string::{convert_text_to_rich_text, convert_text_to_rich_text_ellipsized, ellipsize}, style::{apply_separator, bar_style, orient_text}}, modules::{cpu::define_cpu_text, cpu_temp::{define_cpu_temp_style, define_cpu_temp_text}, focused_window::{define_focused_window_style, define_focused_window_text}, ram::{define_ram_style, define_ram_text}, volume::define_volume_text}};
use crate::modules::{cpu::define_cpu_style, clock::define_clock_style, custom_modules::{define_custom_module_style, define_custom_module_text}, data::Modules, media_player::{create_media_button, define_button_data, define_media_player_buttons_text, define_media_player_metadata_style, define_media_player_metadata_text}, network::{define_network_style, define_network_text}, tray::{define_tray_icon, define_tray_style}, volume::{define_volume_input_style, define_volume_output_style}, workspaces::{define_workspaces_padding, define_workspaces_style, define_workspaces_text}};
use crate::ron::{ActionOnClick, BarPosition};
use crate::update::Message;
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone, Copy)]
pub enum Axis 
{
    #[default] Horizontal,
    Vertical,
}





// ============ FUNCTIONS ============
pub fn view(app: &AppData) -> Element<'_, Message>
{
    let axis = match app.ron_config.bar_position 
    {
        BarPosition::Left | BarPosition::Right => Axis::Vertical,
        _ => Axis::Horizontal,
    };

    let start  = build_modules(&app.ron_config.left_modules, app, axis);
    let center = build_modules(&app.ron_config.center_modules, app, axis);
    let end    = build_modules(&app.ron_config.right_modules, app, axis);

    let content = axis_layout(app.ron_config.bar_side_spaces_size, axis, start, center, end);


    let fixed_bar_size_y = if app.ron_config.bar_size[1] == 0 { app.monitor_size.1 } else { app.ron_config.bar_size[1] };
    let fixed_bar_size_x = if app.ron_config.bar_size[0] == 0 { app.monitor_size.0 } else { app.ron_config.bar_size[0] };
    container
    (
        container(content).height(Length::Fixed(fixed_bar_size_y as f32)).width(Length::Fixed(fixed_bar_size_x as f32)).style(bar_style(app))
    ).width(Length::Fill).height(Length::Fill).center(Length::Fill).into()
}



fn build_modules<'a>(list_of_modules: &'a Vec<Modules>, app: &'a AppData, axis: Axis) -> Element<'a, Message> 
{
    let mut children = Vec::new();
    for item in list_of_modules
    {
        let element: Element<_> = match item
        {   
            Modules::Tray =>
            {
                let children: Vec<Element<_>> = app.modules_data.tray_icons.iter().enumerate().map(|(i, (icon, _))|
                {
                    let button_content = define_tray_icon(app, icon);
                    button(button_content)
                        .style(|_: &Theme, status: button::Status| define_tray_style(app, status))
                        .padding(app.ron_config.tray_button_size)
                        .on_press(Message::TrayIconClicked(i))
                        .into()
                }).collect();
             
                let inner: Element<_> = match axis
                {
                    Axis::Horizontal => row(children).spacing(app.ron_config.tray_spacing).align_y(Alignment::Center).into(),
                    Axis::Vertical   => column(children).spacing(app.ron_config.tray_spacing).align_x(Alignment::Center).into(),
                };
             
                apply_separator(
                    inner,
                    app.ron_config.tray_side_separator,
                    Color::from_rgb8(app.ron_config.tray_side_separator_color[0], app.ron_config.tray_side_separator_color[1], app.ron_config.tray_side_separator_color[2]),
                    app.ron_config.tray_side_separator_width,
                    app.ron_config.tray_side_separator_height,
                )
            },
             
             
            // ── HyprWorkspaces / SwayWorkspaces / NiriWorkspaces ─────────────
            Modules::HyprWorkspaces | Modules::SwayWorkspaces | Modules::NiriWorkspaces =>
            {
                let workspace_buttons = app.modules_data.workspace_data.visible_workspaces.iter().map(|i|
                {
                    let non_color_workspace_text = define_workspaces_text(app, *i);
                    let padding_y = define_workspaces_padding(app, *i);
                    let color_to_send = if *i == app.modules_data.workspace_data.current_workspace
                    {
                        let [r, g, b] = &app.ron_config.workspace_selected_text_color_rgb;
                        Color::from_rgb8(*r, *g, *b)
                    }
                    else
                    {
                        let [r, g, b] = &app.ron_config.workspace_text_color_rgb;
                        Color::from_rgb8(*r, *g, *b)
                    };
                    let workspace_text = convert_text_to_rich_text(&non_color_workspace_text, Some(color_to_send));
                    button(
                        workspace_text
                            .wrapping(iced::widget::text::Wrapping::Word)
                            .font(app.default_font)
                            .size(app.ron_config.workspace_text_size)
                            .center()
                    )
                    .padding(padding_y)
                    .style(move |_: &Theme, status: button::Status| define_workspaces_style(app, status, i))
                    .padding([app.ron_config.workspace_width, padding_y * 2])
                    .on_press(Message::WorkspaceButtonPressed(*i as usize))
                    .into()
                });
             
                let inner: Element<_> = match axis
                {
                    Axis::Horizontal => mouse_area(row(workspace_buttons).align_y(Alignment::Center).spacing(app.ron_config.workspace_spacing))
                        .on_enter(Message::IsHoveringWorkspace(true))
                        .on_exit(Message::IsHoveringWorkspace(false))
                        .into(),
                    Axis::Vertical => mouse_area(column(workspace_buttons).align_x(Alignment::Center).spacing(app.ron_config.workspace_spacing))
                        .on_enter(Message::IsHoveringWorkspace(true))
                        .on_exit(Message::IsHoveringWorkspace(false))
                        .into(),
                };
             
                apply_separator
                (
                    inner,
                    app.ron_config.workspace_side_separator,
                    Color::from_rgb8(app.ron_config.workspace_side_separator_color[0], app.ron_config.workspace_side_separator_color[1], app.ron_config.workspace_side_separator_color[2]),
                    app.ron_config.workspace_side_separator_width,
                    app.ron_config.workspace_side_separator_height,
                )
            },
             
             
            // ── MediaPlayerMetaData ──────────────────────────────────────────
            Modules::MediaPlayerMetaData =>
            {
                if app.ron_config.dont_show_metadata_if_empty && app.modules_data.media_player_data.metadata.is_empty()
                {
                    continue;
                }
             
                let text_to_send = define_media_player_metadata_text(app);
                let left_click_metadata_message: Message  = match &app.ron_config.action_on_left_click_media_player_metadata  { ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Media Player Custom Action".to_string(), true, false)) };
                let right_click_metadata_message: Message = match &app.ron_config.action_on_right_click_media_player_metadata { ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Media Player Custom Action".to_string(), false, false)) };
                let [r, g, b] = &app.ron_config.media_player_metadata_text_color_rgb;
                let color_to_send = Color::from_rgb8(*r, *g, *b);
                let colored_formated_metadata = convert_text_to_rich_text_ellipsized::<Message>(&text_to_send, Some(color_to_send), &app.ron_config.ellipsis_text, app.ron_config.media_player_metadata_text_limit_len);
                let inner = create_button_container(app, app.ron_config.media_player_metadata_padding, (colored_formated_metadata, app.ron_config.media_player_metadata_text_size), (Message::IsHoveringMediaPlayerMetaData(true), Message::IsHoveringMediaPlayerMetaData(false)), left_click_metadata_message, right_click_metadata_message, define_media_player_metadata_style);
             
                apply_separator
                (
                    match axis
                    {
                        Axis::Horizontal => row([inner]).align_y(Alignment::Center).into(),
                        Axis::Vertical   => column([inner]).align_x(Alignment::Center).into(),
                    },
                    app.ron_config.media_player_metadata_side_separator,
                    Color::from_rgb8(app.ron_config.media_player_metadata_side_separator_color[0], app.ron_config.media_player_metadata_side_separator_color[1], app.ron_config.media_player_metadata_side_separator_color[2]),
                    app.ron_config.media_player_metadata_side_separator_width,
                    app.ron_config.media_player_metadata_side_separator_height,
                )
            },
             
             
            // ── MediaPlayerButtons ───────────────────────────────────────────
            Modules::MediaPlayerButtons =>
            {
                if app.ron_config.dont_show_metadata_if_empty && app.modules_data.media_player_data.metadata.is_empty()
                {
                    continue;
                }
             
                let (previous_text, play_pause_text, next_text) = define_media_player_buttons_text(app);
                let button_data = define_button_data(previous_text, play_pause_text, next_text);
                let [r, g, b] = &app.ron_config.media_player_metadata_text_color_rgb;
                let color_to_send = Color::from_rgb8(*r, *g, *b);
                let padding = app.ron_config.media_player_button_padding;
                let media_buttons: Vec<Element<Message>> = button_data.into_iter().map(|(label, message)| { create_media_button(app, padding, label, message, color_to_send) }).collect();
             
                let inner: Element<_> = match axis
                {
                    Axis::Horizontal => row(media_buttons).spacing(app.ron_config.media_player_button_spacing).align_y(Alignment::Center).into(),
                    Axis::Vertical   => column(media_buttons).spacing(app.ron_config.media_player_button_spacing).align_x(Alignment::Center).into(),
                };
             
                apply_separator
                (
                    inner,
                    app.ron_config.media_player_buttons_side_separator,
                    Color::from_rgb8(app.ron_config.media_player_buttons_side_separator_color[0], app.ron_config.media_player_buttons_side_separator_color[1], app.ron_config.media_player_buttons_side_separator_color[2]),
                    app.ron_config.media_player_buttons_side_separator_width,
                    app.ron_config.media_player_buttons_side_separator_height,
                )
            },
             
             
            // ── FocusedWindow ────────────────────────────────────────────────
            Modules::FocusedWindowHypr | Modules::FocusedWindowNiri | Modules::FocusedWindowSway =>
            {
                let text_to_ellipsize = &define_focused_window_text(app);
                let text_to_send = ellipsize(&app.ron_config.ellipsis_text, text_to_ellipsize, app.ron_config.focused_window_text_limit_len);
                if app.ron_config.dont_show_focused_window_if_empty && text_to_send.is_empty() { continue; };
                let text_data =
                (
                    convert_text_to_rich_text_ellipsized(
                        &text_to_send,
                        Some(Color::from_rgb8(app.ron_config.focused_window_text_color_rgb[0], app.ron_config.focused_window_text_color_rgb[1], app.ron_config.focused_window_text_color_rgb[2])),
                        &app.ron_config.ellipsis_text,
                        app.ron_config.focused_window_text_limit_len,
                    ),
                    app.ron_config.focused_window_text_size,
                );
                let inner = create_button_container_without_hover_message(app, app.ron_config.focused_window_padding, text_data, Message::Nothing, Message::Nothing, define_focused_window_style);
             
                apply_separator
                (
                    inner,
                    app.ron_config.focused_window_side_separator,
                    Color::from_rgb8(app.ron_config.focused_window_side_separator_color[0], app.ron_config.focused_window_side_separator_color[1], app.ron_config.focused_window_side_separator_color[2]),
                    app.ron_config.focused_window_side_separator_width,
                    app.ron_config.focused_window_side_separator_height,
                )
            },
             
             
            // ── Ram ──────────────────────────────────────────────────────────
            Modules::Ram =>
            {
                let text_data = (convert_text_to_rich_text::<Message>(&define_ram_text(app), Some(Color::from_rgb8(app.ron_config.ram_text_color_rgb[0], app.ron_config.ram_text_color_rgb[1], app.ron_config.ram_text_color_rgb[2]))), app.ron_config.ram_text_size);
                let inner = create_button_container_without_hover_message(app, app.ron_config.ram_padding, text_data, Message::Nothing, Message::Nothing, define_ram_style);
             
                apply_separator
                (
                    inner,
                    app.ron_config.ram_side_separator,
                    Color::from_rgb8(app.ron_config.ram_side_separator_color[0], app.ron_config.ram_side_separator_color[1], app.ron_config.ram_side_separator_color[2]),
                    app.ron_config.ram_side_separator_width,
                    app.ron_config.ram_side_separator_height,
                )
            },
             
             
            // ── Cpu ──────────────────────────────────────────────────────────
            Modules::Cpu =>
            {
                let text_to_send = define_cpu_text(app);
                let left_click_metadata_message: Message  = match &app.ron_config.action_on_left_click_cpu  { ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Cpu Custom Action".to_string(), true, false)) };
                let right_click_metadata_message: Message = match &app.ron_config.action_on_right_click_cpu { ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Cpu Custom Action".to_string(), false, false)) };
                let [r, g, b] = &app.ron_config.cpu_text_color_rgb;
                let color_to_send = Color::from_rgb8(*r, *g, *b);
                let colored_formated_metadata = convert_text_to_rich_text::<Message>(&text_to_send, Some(color_to_send));
                let inner = create_button_container_without_hover_message(app, app.ron_config.cpu_padding, (colored_formated_metadata, app.ron_config.cpu_text_size), left_click_metadata_message, right_click_metadata_message, define_cpu_style);
             
                apply_separator
                (
                    match axis
                    {
                        Axis::Horizontal => row([inner]).align_y(Alignment::Center).into(),
                        Axis::Vertical   => column([inner]).align_x(Alignment::Center).into(),
                    },
                    app.ron_config.cpu_side_separator,
                    Color::from_rgb8(app.ron_config.cpu_side_separator_color[0], app.ron_config.cpu_side_separator_color[1], app.ron_config.cpu_side_separator_color[2]),
                    app.ron_config.cpu_side_separator_width,
                    app.ron_config.cpu_side_separator_height,
                )
            },
             
             
            // ── CpuTemp ──────────────────────────────────────────────────────
            Modules::CpuTemp =>
            {
                let left_click_metadata_message: Message  = match &app.ron_config.action_on_left_click_cpu_temp  { ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Cpu Temp Custom Action".to_string(), true, false)) };
                let right_click_metadata_message: Message = match &app.ron_config.action_on_right_click_cpu_temp { ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Cpu Temp Custom Action".to_string(), false, false)) };
                let text_to_send = define_cpu_temp_text(app);
                let [r, g, b] = &app.ron_config.cpu_temp_text_color_rgb;
                let color_to_send = Color::from_rgb8(*r, *g, *b);
                let colored_cpu_temp = convert_text_to_rich_text::<Message>(&text_to_send, Some(color_to_send));
                let inner = create_button_container_without_hover_message(app, app.ron_config.cpu_temp_padding, (colored_cpu_temp, app.ron_config.cpu_temp_text_size), left_click_metadata_message, right_click_metadata_message, define_cpu_temp_style);
             
                apply_separator
                (
                    match axis
                    {
                        Axis::Horizontal => row([inner]).align_y(Alignment::Center).into(),
                        Axis::Vertical   => column([inner]).align_x(Alignment::Center).into(),
                    },
                    app.ron_config.cpu_temp_side_separator,
                    Color::from_rgb8(app.ron_config.cpu_temp_side_separator_color[0], app.ron_config.cpu_temp_side_separator_color[1], app.ron_config.cpu_temp_side_separator_color[2]),
                    app.ron_config.cpu_temp_side_separator_width,
                    app.ron_config.cpu_temp_side_separator_height,
                )
            },
             
             
            // ── Network ──────────────────────────────────────────────────────
            Modules::Network =>
            {
                let left_click_message: Message  = match &app.ron_config.action_on_left_click_network  { ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::ToggleAltNetwork, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Network Custom Action".to_string(), true, false)) };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_network { ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Network Custom Action".to_string(), false, false)) };
             
                let (color_to_send, text_size, padding, side_separator, side_separator_color, side_separator_width, side_separator_height) = if app.is_showing_alt_network_module
                {
                    let [sr, sg, sb] = &app.ron_config.alt_network_side_separator_color;
                    let [r, g, b] = &app.ron_config.alt_network_text_color_rgb;
                    (Color::from_rgb8(*r, *g, *b), app.ron_config.alt_network_text_size, app.ron_config.alt_network_padding, app.ron_config.alt_network_side_separator, Color::from_rgb8(*sr, *sg, *sb), app.ron_config.alt_network_side_separator_width, app.ron_config.alt_network_side_separator_height)
                }
                else
                {
                    let [sr, sg, sb] = &app.ron_config.network_side_separator_color;
                    let [r, g, b] = &app.ron_config.network_text_color_rgb;
                    (Color::from_rgb8(*r, *g, *b), app.ron_config.network_text_size, app.ron_config.network_padding, app.ron_config.network_side_separator, Color::from_rgb8(*sr, *sg, *sb), app.ron_config.network_side_separator_width, app.ron_config.network_side_separator_height)
                };
             
                let text_to_send = define_network_text(app);
                let colored_network_string = convert_text_to_rich_text::<Message>(&text_to_send, Some(color_to_send));
                let inner = create_button_container_without_hover_message(app, padding, (colored_network_string, text_size), left_click_message, right_click_message, define_network_style);
             
                apply_separator
                (
                    match axis
                    {
                        Axis::Horizontal => row([inner]).align_y(Alignment::Center).into(),
                        Axis::Vertical   => column([inner]).align_x(Alignment::Center).into(),
                    },
                    side_separator,
                    side_separator_color,
                    side_separator_width,
                    side_separator_height
                )
            },
             
             
            // ── VolumeOutput ─────────────────────────────────────────────────
            Modules::VolumeOutput =>
            {
                let left_click_message: Message  = match &app.ron_config.action_on_left_click_volume_output  { ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::MuteAudioPressedOutput, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Output Custom Action".to_string(), true, false)) };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_volume_output { ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Output Custom Action".to_string(), false, false)) };
             
                let (text_orientation, color_to_send, text_size, padding, side_separator, side_separator_color, side_separator_width, side_separator_height) = if app.volume_output_is_muted
                {
                    let [sr, sg, sb] = &app.ron_config.muted_volume_output_side_separator_color;
                    let [r, g, b] = &app.ron_config.muted_volume_output_text_color_rgb;
                    (
                        &app.ron_config.muted_volume_output_text_orientation, Color::from_rgb8(*r, *g, *b), &app.ron_config.muted_volume_output_text_size, app.ron_config.muted_volume_output_padding,
                        &app.ron_config.muted_volume_output_side_separator, Color::from_rgb8(*sr, *sg, *sb), &app.ron_config.muted_volume_output_side_separator_width, &app.ron_config.muted_volume_output_side_separator_height
                    )
                }
                else
                {
                    let [sr, sg, sb] = &app.ron_config.volume_output_side_separator_color;
                    let [r, g, b] = &app.ron_config.volume_output_text_color_rgb;
                    (
                        &app.ron_config.volume_output_text_orientation, Color::from_rgb8(*r, *g, *b), &app.ron_config.volume_output_text_size, app.ron_config.volume_output_padding,
                        &app.ron_config.volume_output_side_separator, Color::from_rgb8(*sr, *sg, *sb), &app.ron_config.volume_output_side_separator_width, &app.ron_config.volume_output_side_separator_height
                    )
                };
             
                let text_to_send = define_volume_text(&app.modules_data.volume_data.output_volume_level, text_orientation);
                let colored_volume_output_string = convert_text_to_rich_text::<Message>(&text_to_send, Some(color_to_send));
                let inner = create_button_container(app, padding, (colored_volume_output_string, *text_size), (Message::IsHoveringVolumeOutput(true), Message::IsHoveringVolumeOutput(false)), left_click_message, right_click_message, define_volume_output_style);
             
                apply_separator
                (
                    match axis
                    {
                        Axis::Horizontal => row([inner]).align_y(Alignment::Center).into(),
                        Axis::Vertical   => column([inner]).align_x(Alignment::Center).into(),
                    },
                    *side_separator,
                    side_separator_color,
                    *side_separator_width,
                    *side_separator_height
                )
            },
             
             
            // ── VolumeInput ──────────────────────────────────────────────────
            Modules::VolumeInput =>
            {
                let left_click_message: Message  = match &app.ron_config.action_on_left_click_volume_input  { ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::MuteAudioPressedInput, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Input Custom Action".to_string(), true, false)) };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_volume_input { ActionOnClick::Nothing => Message::Nothing, ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones, ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Input Custom Action".to_string(), false, false)) };
             
                let (text_orientation, color_to_send, text_size, padding, side_separator, side_separator_color, side_separator_width, side_separator_height) = if app.volume_input_is_muted
                {
                    let [sr, sg, sb] = &app.ron_config.muted_volume_input_side_separator_color;
                    let [r, g, b] = &app.ron_config.muted_volume_input_text_color_rgb;
                    (
                        &app.ron_config.muted_volume_input_text_orientation, Color::from_rgb8(*r, *g, *b), &app.ron_config.muted_volume_input_text_size, app.ron_config.muted_volume_input_padding,
                        &app.ron_config.muted_volume_input_side_separator, Color::from_rgb8(*sr, *sg, *sb), &app.ron_config.muted_volume_input_side_separator_width, &app.ron_config.muted_volume_input_side_separator_height
                    )
                }
                else
                {
                    let [sr, sg, sb] = &app.ron_config.volume_input_side_separator_color;
                    let [r, g, b] = &app.ron_config.volume_input_text_color_rgb;
                    (
                        &app.ron_config.volume_input_text_orientation, Color::from_rgb8(*r, *g, *b), &app.ron_config.volume_input_text_size, app.ron_config.volume_input_padding,
                        &app.ron_config.volume_input_side_separator, Color::from_rgb8(*sr, *sg, *sb), &app.ron_config.volume_input_side_separator_width, &app.ron_config.volume_input_side_separator_height
                    )
                };
             
                let text_to_send = define_volume_text(&app.modules_data.volume_data.input_volume_level, text_orientation);
                let colored_volume_input_string = convert_text_to_rich_text::<Message>(&text_to_send, Some(color_to_send));
                let inner = create_button_container(app, padding, (colored_volume_input_string, *text_size), (Message::IsHoveringVolumeInput(true), Message::IsHoveringVolumeInput(false)), left_click_message, right_click_message, define_volume_input_style);
             
                apply_separator
                (
                    match axis
                    {
                        Axis::Horizontal => row([inner]).align_y(Alignment::Center).into(),
                        Axis::Vertical   => column([inner]).align_x(Alignment::Center).into(),
                    },
                    *side_separator,
                    side_separator_color,
                    *side_separator_width,
                    *side_separator_height
                )
            },


            // ── Clock ──────────────────────────────────────────────────
            Modules::Clock => 
            {
                let left_click_message: Message = match &app.ron_config.action_on_left_click_clock
                {
                    ActionOnClick::Nothing => Message::Nothing,
                    ActionOnClick::DefaultAction => Message::ToggleAltClock,
                    ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones,
                    ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Clock Custom Action".to_string(), true, false))
                };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_clock
                {
                    ActionOnClick::Nothing => Message::Nothing,
                    ActionOnClick::DefaultAction => Message::ToggleAltClock,
                    ActionOnClick::ToggleAltClockAndCycleClockTimezones => Message::ToggleAltClockAndCycleClockTimeZones,
                    ActionOnClick::CycleClockTimezones => Message::CycleClockTimeZones,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Clock Custom Action".to_string(), false, false))
                };

                let (text_orientation, color_to_send, text_size, padding, separator_flags, separator_color, separator_width, separator_height) = if app.is_showing_alt_clock
                {
                    let [r, g, b] = &app.ron_config.alt_clock_text_color_rgb;
                    let [separator_r, separator_g, separator_b] = app.ron_config.alt_clock_side_separator_color;
                    (
                        &app.ron_config.alt_clock_text_orientation, 
                        Color::from_rgb8(*r, *g, *b), 
                        app.ron_config.alt_clock_text_size, 
                        app.ron_config.alt_clock_padding, 

                        app.ron_config.alt_clock_side_separator,
                        Color::from_rgb8(separator_r, separator_g, separator_b),
                        app.ron_config.alt_clock_side_separator_width,
                        app.ron_config.alt_clock_side_separator_height
                    )
                }
                else
                {
                    let [r, g, b] = &app.ron_config.clock_text_color_rgb;
                    let [separator_r, separator_g,  separator_b] = app.ron_config.clock_side_separator_color;
                    (
                        &app.ron_config.clock_text_orientation, 
                        Color::from_rgb8(*r, *g, *b), 
                        app.ron_config.clock_text_size, 
                        app.ron_config.clock_padding, 

                        app.ron_config.clock_side_separator,
                        Color::from_rgb8(separator_r, separator_g, separator_b),
                        app.ron_config.clock_side_separator_width,
                        app.ron_config.clock_side_separator_height
                    )
                };
                let text_string = orient_text(&app.modules_data.clock_data.current_time, text_orientation);
                let colored_clock_string = convert_text_to_rich_text::<Message>(&text_string, Some(color_to_send));
                let inner = create_button_container_without_hover_message(app, padding, (colored_clock_string, text_size), left_click_message, right_click_message, define_clock_style);

                apply_separator
                (
                    match axis
                    {
                        Axis::Horizontal => row([inner]).align_y(Alignment::Center).into(),
                        Axis::Vertical   => column([inner]).align_x(Alignment::Center).into(),
                    },
                    separator_flags,
                    separator_color,
                    separator_width,
                    separator_height
                )
            }



            // ── CustomModule ──────────────────────────────────────────────────
            Modules::CustomModule(borrowed_index) => 
            {
                let index = *borrowed_index;
                let custom_module = &app.ron_config.custom_modules[index];
                let text_to_render = define_custom_module_text(index, custom_module, app);
                if custom_module.dont_show_if_any_output_is_empty && text_to_render.is_empty()
                {
                    continue
                };
                let [r, g, b] = &custom_module.text_color_rgb;
                let color_to_send = Color::from_rgb8(*r, *g, *b);
                let text_to_send = orient_text(&text_to_render, &custom_module.text_orientation);
                let colored_custom_module_string = convert_text_to_rich_text::<Message>(&text_to_send, Some(color_to_send));

                let element = container
                (
                        button
                        (
                            mouse_area
                            (
                                colored_custom_module_string
                                .align_y(Alignment::Center)
                                .wrapping(iced::widget::text::Wrapping::Word)
                                .font(app.default_font)
                                .size(custom_module.text_size)
                                .center()
                            )
                            .on_right_press(Message::CreateCustomModuleCommand((Some(index), custom_module.command_to_exec_on_right_click.clone(), custom_module.name.clone(), false, custom_module.use_output_as_text)))
                        )
                        .on_press(Message::CreateCustomModuleCommand((Some(index), custom_module.command_to_exec_on_left_click.clone(), custom_module.name.clone(), true, custom_module.use_output_as_text)))
                        .style(|_, status| {define_custom_module_style(custom_module, status)})  
                ).padding(custom_module.padding).align_y(Alignment::Center);
                
                apply_separator
                (
                    match axis 
                    {
                        Axis::Horizontal => row![element].align_y(Alignment::Center).into(),
                        Axis::Vertical => column![element].align_x(Alignment::Center).into()
                    },
                    custom_module.side_separator,
                    Color::from_rgb8(custom_module.separator_color[0], custom_module.separator_color[1], custom_module.separator_color[2]), 
                    custom_module.separator_width,
                    custom_module.separator_height,
                )
            }
        };
        children.push(element);
    }

    match axis 
    {
        Axis::Horizontal => row(children).align_y(Alignment::Center).spacing(app.ron_config.spacing_between_all_modules).into(),
        Axis::Vertical => column(children).align_x(Alignment::Center).spacing(app.ron_config.spacing_between_all_modules).into(),
    }
}



fn axis_layout<'a>(padding: u32, axis: Axis, start: Element<'a, Message>, center: Element<'a, Message>, end: Element<'a, Message>) -> Element<'a, Message> 
{
    match axis 
    {
        Axis::Horizontal => 
        {
            row!
            [
                Space::new().width(padding),
                container(start).width(Length::Fill).height(Length::Fill).align_x(iced::alignment::Horizontal::Left).align_y(iced::alignment::Vertical::Center),
                container(center).width(Length::Shrink).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Center),
                container(end).width(Length::Fill).height(Length::Fill).align_x(iced::alignment::Horizontal::Right).align_y(iced::alignment::Vertical::Center),
                Space::new().width(padding),
            ].width(Length::Fill).height(Length::Fill).into()
        }

        Axis::Vertical => 
        {
            column!
            [
                Space::new().height(padding),
                container(start).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Top),
                container(center).height(Length::Shrink).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Center),
                container(end).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Bottom),
                Space::new().height(padding),
            ].width(Length::Fill).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).into()
        }
    }
}
