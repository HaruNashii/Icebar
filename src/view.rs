// ============ IMPORTS ============
use iced::{Alignment, Color, Element, Length, Theme, widget::{Space, button, column, container, mouse_area, row, text}};





// ============ CRATES ============
use crate::{helpers::{media_buttons::create_media_button, misc::create_button_container, style::{bar_style, orient_text}}, modules::volume::define_volume_text};
use crate::modules::{clock::define_clock_style, custom_modules::{define_custom_module_style, define_custom_module_text}, data::Modules, media_player::{define_button_data, define_media_player_buttons_text, define_media_player_metadata_style, define_media_player_metadata_text}, network::{define_network_style, define_network_text}, tray::{define_tray_icon, define_tray_style}, volume::{define_volume_input_style, define_volume_output_style}, workspaces::{define_workspaces_padding, define_workspaces_style, define_workspaces_text}};
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
                // ---------- build tray buttons ----------
                let children: Vec<Element<_>> = app.modules_data.tray_icons.iter().enumerate().map(|(i, (icon, _))| 
                {
                    let button_content = define_tray_icon(app, icon);
                    button(button_content).style(|_: &Theme, status: button::Status|
                    {
                        define_tray_style(app, status)
                    }).padding(app.ron_config.tray_button_size).on_press(Message::TrayIconClicked(i)).into()
                }).collect();
            
                match axis 
                {
                    Axis::Horizontal => row(children).spacing(app.ron_config.tray_spacing).align_y(Alignment::Center).into(),
                    Axis::Vertical => column(children).spacing(app.ron_config.tray_spacing).align_x(Alignment::Center).into()
                }
            },

            Modules::HyprWorkspaces | Modules::SwayWorkspaces | Modules::NiriWorkspaces =>
            {
                let workspace_buttons = app.modules_data.workspace_data.visible_workspaces.iter().map(|i| 
                {
                    let workspace_text = define_workspaces_text(app, *i);
                    let padding_y = define_workspaces_padding(app, *i);
                    let [r, g, b] = &app.ron_config.workspace_text_color_rgb;
                    let color_to_send = Color::from_rgb8(*r, *g, *b);
                    button
                    (
                        mouse_area
                        (
                            text
                            (
                                orient_text
                                (
                                    &workspace_text,
                                    &app.ron_config.workspace_text_orientation
                                )
                            )
                            .color(color_to_send)
                            .wrapping(iced::widget::text::Wrapping::Word)
                            .font(app.default_font)
                            .size(app.ron_config.workspace_text_size)
                            .center()
                        )
                        .on_enter(Message::IsHoveringWorkspace(true))
                        .on_exit(Message::IsHoveringWorkspace(false))
                    )
                    .padding(padding_y)
                    .style(move |_: &Theme, status: button::Status| 
                    {
                        define_workspaces_style(app, status, i)
                    })
                    .padding([app.ron_config.workspace_width, padding_y * 2])
                    .on_press(Message::WorkspaceButtonPressed(*i as usize))
                    .into()
                 });

                match axis 
                {
                    Axis::Horizontal => row(workspace_buttons).align_y(Alignment::Center).spacing(app.ron_config.workspace_spacing).into(),
                    Axis::Vertical => column(workspace_buttons).align_x(Alignment::Center).spacing(app.ron_config.workspace_spacing).into(),
                }
            }

            Modules::MediaPlayerMetaData => 
            {
                if app.ron_config.dont_show_metadata_if_empty && app.modules_data.media_player_data.metadata.is_empty()
                {
                    continue;
                }

                let formated_metadata = define_media_player_metadata_text(app);
                let left_click_metadata_message: Message = match &app.ron_config.action_on_left_click_media_player_metadata { ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Media Player Custom Action".to_string(), true, false)) };
                let right_click_metadata_message: Message = match &app.ron_config.action_on_right_click_media_player_metadata { ActionOnClick::DefaultAction => Message::Nothing, ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Media Player Custom Action".to_string(), false, false)) };
                let [r, g, b] = &app.ron_config.media_player_metadata_text_color_rgb;
                let color_to_send = Color::from_rgb8(*r, *g, *b);
                let media_player_metadata_container = create_button_container(app, (formated_metadata, color_to_send),  Message::IsHoveringMediaPlayerMetaData(true), Message::IsHoveringMediaPlayerMetaData(false), left_click_metadata_message, right_click_metadata_message, define_media_player_metadata_style);
                
                match axis 
                {
                    Axis::Horizontal => row([media_player_metadata_container]).align_y(Alignment::Center).into(),
                    Axis::Vertical => column([media_player_metadata_container]).align_x(Alignment::Center).into()
                }
            }



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
                let media_buttons: Vec<Element<Message>> = button_data.into_iter().map(|(label, message)| { create_media_button(app, label, message, color_to_send) }).collect();

                match axis 
                {
                    Axis::Horizontal => row(media_buttons).spacing(app.ron_config.media_player_button_spacing).align_y(Alignment::Center).into(),
                    Axis::Vertical => column(media_buttons).spacing(app.ron_config.media_player_button_spacing).align_x(Alignment::Center).into()
                }
            }




            Modules::Clock => 
            {
                let left_click_message: Message = match &app.ron_config.action_on_left_click_clock
                {
                    ActionOnClick::DefaultAction => Message::ToggleAltClock,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Clock Custom Action".to_string(), true, false))
                };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_clock
                {
                    ActionOnClick::DefaultAction => Message::Nothing,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Clock Custom Action".to_string(), false, false))
                };

                let (text_orientation, color_to_send) = if app.is_showing_alt_clock
                {
                    let [r, g, b] = &app.ron_config.alt_clock_text_color_rgb;
                    (&app.ron_config.alt_clock_text_orientation, Color::from_rgb8(*r, *g, *b))
                }
                else
                {
                    let [r, g, b] = &app.ron_config.clock_text_color_rgb;
                    (&app.ron_config.clock_text_orientation, Color::from_rgb8(*r, *g, *b))
                };
                let text_string = orient_text(&app.modules_data.clock_data.current_time, text_orientation);
                let clock_container = create_button_container(app, (text_string, color_to_send), Message::Nothing, Message::Nothing, left_click_message, right_click_message, define_clock_style);

                match axis 
                {
                    Axis::Horizontal => row([clock_container]).align_y(Alignment::Center).into(),
                    Axis::Vertical => column([clock_container]).align_x(Alignment::Center).into()
                }
            }



            Modules::Network => 
            {
                let left_click_message: Message = match &app.ron_config.action_on_left_click_network
                {
                    ActionOnClick::DefaultAction => Message::ToggleAltNetwork,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Network Custom Action".to_string(), true, false))
                };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_network
                {
                    ActionOnClick::DefaultAction => Message::Nothing,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Network Custom Action".to_string(), false, false))
                };

                let color_to_send = if app.is_showing_alt_network_module
                {
                    let [r, g, b] = &app.ron_config.alt_network_text_color_rgb;
                    Color::from_rgb8(*r, *g, *b)
                }
                else
                {
                    let [r, g, b] = &app.ron_config.network_text_color_rgb;
                     Color::from_rgb8(*r, *g, *b)
                };

                let text_to_send = define_network_text(app);
                let network_container = create_button_container(app, (text_to_send, color_to_send), Message::Nothing, Message::Nothing, left_click_message, right_click_message, define_network_style);

                match axis 
                {
                    Axis::Horizontal => row([network_container]).align_y(Alignment::Center).into(),
                    Axis::Vertical => column([network_container]).align_x(Alignment::Center).into()
                }
            }



            Modules::VolumeOutput =>
            {
                let left_click_message: Message = match &app.ron_config.action_on_left_click_volume_output
                {
                    ActionOnClick::DefaultAction => Message::MuteAudioPressedOutput,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Output Custom Action".to_string(), true, false))

                };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_volume_output
                {
                    ActionOnClick::DefaultAction => Message::Nothing,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Output Custom Action".to_string(), false, false))

                };

                let (text_orientation, color_to_send) = if app.volume_output_is_muted
                {
                    let [r, g, b] = &app.ron_config.muted_volume_output_text_color_rgb;
                    (&app.ron_config.muted_volume_output_text_orientation, Color::from_rgb8(*r, *g, *b))
                }
                else
                {
                    let [r, g, b] = &app.ron_config.volume_output_text_color_rgb;
                    (&app.ron_config.volume_output_text_orientation, Color::from_rgb8(*r, *g, *b))
                };

                let text_to_send = define_volume_text(&app.modules_data.volume_data.output_volume_level, text_orientation);
                let volume_output_container = create_button_container(app, (text_to_send, color_to_send), Message::IsHoveringVolumeOutput(true), Message::IsHoveringVolumeOutput(false), left_click_message, right_click_message, define_volume_output_style);

                match axis 
                {
                    Axis::Horizontal => row([volume_output_container]).align_y(Alignment::Center).into(),
                    Axis::Vertical => column([volume_output_container]).align_x(Alignment::Center).into()
                }
            }



            Modules::VolumeInput => 
            {
                let left_click_message: Message = match &app.ron_config.action_on_left_click_volume_input
                {
                    ActionOnClick::DefaultAction => Message::MuteAudioPressedInput,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Input Custom Action".to_string(), true, false))

                };
                let right_click_message: Message = match &app.ron_config.action_on_right_click_volume_input
                {
                    ActionOnClick::DefaultAction => Message::Nothing,
                    ActionOnClick::CustomAction(custom_action) => Message::CreateCustomModuleCommand((None, custom_action.to_vec(), "Volume Input Custom Action".to_string(), false, false))

                };

                let (text_orientation, color_to_send) = if app.volume_input_is_muted
                {
                    let [r, g, b] = &app.ron_config.muted_volume_input_text_color_rgb;
                    (&app.ron_config.muted_volume_input_text_orientation, Color::from_rgb8(*r, *g, *b))
                }
                else
                {
                    let [r, g, b] = &app.ron_config.volume_input_text_color_rgb;
                    (&app.ron_config.volume_output_text_orientation, Color::from_rgb8(*r, *g, *b))
                };

                let text_to_send = define_volume_text(&app.modules_data.volume_data.input_volume_level, text_orientation);
                let volume_input_container = create_button_container(app, (text_to_send, color_to_send), Message::IsHoveringVolumeInput(true), Message::IsHoveringVolumeInput(false), left_click_message, right_click_message, define_volume_input_style);
                
                match axis 
                {
                    Axis::Vertical => column([volume_input_container]).align_x(Alignment::Center).into(),
                    Axis::Horizontal => row([volume_input_container]).align_y(Alignment::Center).into() 
                }
            }



            Modules::CustomModule(borrowed_index) => 
            {
                let index = *borrowed_index;
                let custom_module = &app.ron_config.custom_modules[index];
                let text_to_render = define_custom_module_text(index, custom_module, app);
                let [r, g, b] = &custom_module.text_color_rgb;
                let color_to_send = Color::from_rgb8(*r, *g, *b);

                let element = container
                (
                        button
                        (
                            mouse_area
                            (
                                text(orient_text(&text_to_render, &custom_module.text_orientation))
                                .color(color_to_send)
                                .wrapping(iced::widget::text::Wrapping::Word)
                                .font(app.default_font)
                                .size(custom_module.text_size)
                                .center()
                            )
                            .on_right_press(Message::CreateCustomModuleCommand((Some(index), custom_module.command_to_exec_on_right_click.clone(), custom_module.name.clone(), false, custom_module.use_output_as_text)))
                        )
                        .on_press(Message::CreateCustomModuleCommand((Some(index), custom_module.command_to_exec_on_left_click.clone(), custom_module.name.clone(), true, custom_module.use_output_as_text)))
                        .style(|_, status| {define_custom_module_style(custom_module, status)})  
                ).align_y(Alignment::Center);
                
                match axis 
                {
                    Axis::Horizontal => row![element].align_y(Alignment::Center).into(),
                    Axis::Vertical => column![element].align_x(Alignment::Center).into()
                }
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
                container(start).height(Length::Fill).align_x(iced::alignment::Horizontal::Left).align_y(iced::alignment::Vertical::Center),
                Space::new().width(Length::Fill),
                container(center).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Center),
                Space::new().width(Length::Fill),
                container(end).height(Length::Fill).align_x(iced::alignment::Horizontal::Right).align_y(iced::alignment::Vertical::Center),
                Space::new().width(padding),
            ].width(Length::Fill).height(Length::Fill).into()
        }

        Axis::Vertical => 
        {
            column!
            [
                Space::new().height(padding),
                container(start).height(Length::Fill).align_x(iced::alignment::Horizontal::Center),
                Space::new().height(Length::Fill),
                container(center).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Center),
                Space::new().height(Length::Fill),
                container(end).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Bottom),
                Space::new().height(padding),
            ].width(Length::Fill).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).into()
        }
    }
}
