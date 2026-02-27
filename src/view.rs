// ============ IMPORTS ============
use iced::{Alignment, Element, Length, Theme, widget::{button, column, container, image, mouse_area, row, text}};





// ============ CRATES ============
use crate::{helpers::{string::ellipsize, style::{UserStyle, orient_text, set_style}}, modules::data::Modules, ron::BarPosition};
use crate::ron::ActionOnClick;
use crate::{AppData, Message};





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

    axis_layout(axis, start, center, end)
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
                let children: Vec<Element<_>> =
                    app.modules_data.tray_icons.iter().enumerate().map(|(i, (icon, _))| 
                    {
                        let content: Element<_> = if let Some(icon) = icon 
                        { image(icon.clone()).width(app.ron_config.tray_icon_size).height(app.ron_config.tray_icon_size).into() } 
                        else
                        { text("?").into() };
            
                        button(content) .style(|_: &Theme, status: button::Status|
                        {
                            let hovered = app.ron_config.tray_button_hovered_color_rgb;
                            let hovered_text = app.ron_config.tray_button_hovered_text_color_rgb;
                            let pressed = app.ron_config.tray_button_pressed_color_rgb;
                            let normal = app.ron_config.tray_button_color_rgb;
                            let normal_text = app.ron_config.tray_button_text_color_rgb;
                            let border_size = app.ron_config.tray_border_size;
                            let border_color_rgba = app.ron_config.tray_border_color_rgba;
                            let border_radius = app.ron_config.tray_border_radius;
                            set_style(UserStyle {status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius})
                        }).padding(app.ron_config.tray_button_size).on_press(Message::TrayIconClicked(i)).into()
                    }).collect();
            
                match axis 
                {
                    Axis::Horizontal => row(children).width(Length::Fill).align_y(Alignment::Center).into(),
                    Axis::Vertical => column(children).width(Length::Shrink).align_x(Alignment::Center).into()
                }
            },



            Modules::HyprWorkspaces | Modules::SwayWorkspaces =>
            {
                let workspace_buttons = app.modules_data.workspace_data.visible_workspaces.iter().map(|i| 
                {
                    let id = *i;

                    // ================= TEXT RESOLUTION =================
                    let workspace_text = if id == app.modules_data.workspace_data.current_workspace 
                    {
                        if let Some(selected) = &app.ron_config.workspace_selected_text 
                        { selected.get((id - 1) as usize).cloned().unwrap_or_else(|| id.to_string()) } 
                        else 
                        { id.to_string() }
                    } 
                    else 
                    { 
                        app.ron_config.workspace_text.get((id - 1) as usize).cloned().unwrap_or_else(|| id.to_string()) 
                    };

                    let padding_y = if let Some(value) = app.ron_config.workspace_different_selected_width && id == app.modules_data.workspace_data.current_workspace
                    { value } 
                    else 
                    { app.ron_config.workspace_width };

                    button(text(orient_text(&workspace_text.clone(), &app.ron_config.workspace_text_orientation)).wrapping(iced::widget::text::Wrapping::Word).font(app.default_font).size(app.ron_config.workspace_text_size).center()).padding(padding_y).on_press(Message::WorkspaceButtonPressed(*i as usize)).style(move |_: &Theme, status: button::Status| 
                    {
                        let hovered = app.ron_config.workspace_button_hovered_color_rgb;
                        let hovered_text = app.ron_config.workspace_button_hovered_text_color_rgb;
                        let pressed = app.ron_config.workspace_button_pressed_color_rgb;
                        let normal = if app.modules_data.workspace_data.current_workspace == *i 
                        { app.ron_config.workspace_button_selected_color_rgb }
                        else 
                        { app.ron_config.workspace_button_color_rgb };
                        let normal_text = app.ron_config.workspace_button_text_color_rgb;
                        let border_size = app.ron_config.workspace_border_size;
                        let border_color_rgba = app.ron_config.workspace_border_color_rgba;
                        let border_radius = app.ron_config.workspace_border_radius;
                        set_style(UserStyle {status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius})
                     }).padding([app.ron_config.workspace_width, padding_y * 2]).into()
                 });

                match axis 
                {
                    Axis::Horizontal => row(workspace_buttons).width(Length::Fill).align_y(Alignment::Center).into(),
                    Axis::Vertical => column(workspace_buttons).width(Length::Shrink).align_x(Alignment::Center).into(),
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

                let clock_container: Element<'a, Message> = container(mouse_area(button(text(orient_text(&app.modules_data.clock_data.current_time, &app.ron_config.clock_text_orientation)).wrapping(iced::widget::text::Wrapping::Word).font(app.default_font).size(app.ron_config.clock_text_size).center()).style(|_: &Theme, status: button::Status| 
                {
                    let hovered = app.ron_config.clock_button_hovered_color_rgb;
                    let hovered_text = app.ron_config.clock_button_hovered_text_color_rgb;
                    let pressed = app.ron_config.clock_button_pressed_color_rgb;
                    let normal = app.ron_config.clock_button_color_rgb;
                    let normal_text = app.ron_config.clock_button_text_color_rgb;
                    let border_size = app.ron_config.clock_border_size;
                    let border_color_rgba = app.ron_config.clock_border_color_rgba;
                    let border_radius = app.ron_config.clock_border_radius;
                    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
                })).on_press(left_click_message).on_right_press(right_click_message)).align_y(Alignment::Center).into();

                match axis 
                {
                    Axis::Vertical => column([clock_container]).width(Length::Fill).align_x(Alignment::Center).into(),
                    Axis::Horizontal => row([clock_container]).width(Length::Shrink).align_y(Alignment::Center).into() 
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

                let volume_output_container = container(mouse_area ( button (text(orient_text(&app.modules_data.volume_data.output_volume_level, &app.ron_config.volume_output_text_orientation)).wrapping(iced::widget::text::Wrapping::Word).font(app.default_font).size(app.ron_config.volume_output_text_size).center()).style(|_: &Theme, status: button::Status| 
                {
                    let hovered = app.ron_config.volume_output_button_hovered_color_rgb;
                    let hovered_text = app.ron_config.volume_output_button_hovered_text_color_rgb;
                    let pressed = app.ron_config.volume_output_button_pressed_color_rgb;
                    let normal = app.ron_config.volume_output_button_color_rgb;
                    let normal_text = app.ron_config.volume_output_button_text_color_rgb;
                    let border_size = app.ron_config.volume_output_border_size;
                    let border_color_rgba = app.ron_config.volume_output_border_color_rgba;
                    let border_radius = app.ron_config.volume_output_border_radius;
                    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
                })).on_press(left_click_message).on_right_press(right_click_message).on_enter(Message::IsHoveringVolumeOutput(true)).on_exit(Message::IsHoveringVolumeOutput(false))).align_y(Alignment::Center).into();

                match axis 
                {
                    Axis::Vertical => column([volume_output_container]).width(Length::Fill).align_x(Alignment::Center).into(),
                    Axis::Horizontal => row([volume_output_container]).width(Length::Shrink).align_y(Alignment::Center).into() 
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
                let volume_input_container = container(mouse_area ( button (text(orient_text(&app.modules_data.volume_data.input_volume_level, &app.ron_config.volume_input_text_orientation)).wrapping(iced::widget::text::Wrapping::Word).font(app.default_font).size(app.ron_config.volume_input_text_size).center()).style(|_: &Theme, status: button::Status| 
                {
                    let hovered = app.ron_config.volume_input_button_hovered_color_rgb;
                    let hovered_text = app.ron_config.volume_input_button_hovered_text_color_rgb;
                    let pressed = app.ron_config.volume_input_button_pressed_color_rgb;
                    let normal = app.ron_config.volume_input_button_color_rgb;
                    let normal_text = app.ron_config.volume_input_button_text_color_rgb;
                    let border_size = app.ron_config.volume_input_border_size;
                    let border_color_rgba = app.ron_config.volume_input_border_color_rgba;
                    let border_radius = app.ron_config.volume_input_border_radius;
                    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
                })).on_press(left_click_message).on_right_press(right_click_message).on_enter(Message::IsHoveringVolumeInput(true)).on_exit(Message::IsHoveringVolumeInput(false))).align_y(Alignment::Center).into();
                
                match axis 
                {
                    Axis::Vertical => column([volume_input_container]).width(Length::Fill).align_x(Alignment::Center).into(),
                    Axis::Horizontal => row([volume_input_container]).width(Length::Shrink).align_y(Alignment::Center).into() 
                }
            }



            Modules::CustomModule(borrowed_index) => 
            {
                let index = *borrowed_index;
                let custom_module = &app.ron_config.custom_modules[index];

                // TEXT RESOLUTION // COMMAND_OUTPUT
                let text_to_render = if custom_module.use_output_as_text && !custom_module.all_output_as_text_format.is_empty()
                {
                    let output_text = app.cached_command_outputs.get(index).map(String::as_str).unwrap_or("");
                    let output_text = ellipsize(output_text, custom_module.output_text_limit_len);
                    custom_module.all_output_as_text_format.replace("{text}", &custom_module.text).replace("{output}", &output_text).replace('\n', "")
                }
                // CONTINOUS_OUTPUT
                else if custom_module.use_continous_output_as_text && !custom_module.all_output_as_text_format.is_empty() && !&app.cached_continuous_outputs.is_empty() && (app.cached_continuous_outputs.len() - 1) >= index
                {
                    let output_text = ellipsize(&app.cached_continuous_outputs[index], custom_module.output_text_limit_len);
                    custom_module.all_output_as_text_format.replace("{text}", &custom_module.text).replace("{continous_output}", &output_text).replace('\n', "")
                }
                // NO OUTPUT JUST TEXT
                else 
                {
                    custom_module.text.clone()
                };

                let element = container(mouse_area(button(text(orient_text(&text_to_render, &custom_module.text_orientation)).wrapping(iced::widget::text::Wrapping::Word).font(app.default_font).size(custom_module.text_size).center()).style(|_, status| 
                {
                    let hovered = custom_module.button_hovered_color_rgb; 
                    let hovered_text = custom_module.button_hovered_text_color_rgb; 
                    let pressed = custom_module.button_pressed_color_rgb; 
                    let normal = custom_module.button_color_rgb; 
                    let normal_text = custom_module.button_text_color_rgb; 
                    let border_size = custom_module.border_size; 
                    let border_color_rgba = custom_module.border_color_rgba; 
                    let border_radius = custom_module.border_radius;
                    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
                })).on_press(Message::CreateCustomModuleCommand((Some(index), custom_module.command_to_exec_on_left_click.clone(), custom_module.name.clone(), true, custom_module.use_output_as_text,))).on_right_press(Message::CreateCustomModuleCommand((Some(index), custom_module.command_to_exec_on_right_click.clone(), custom_module.name.clone(), false, custom_module.use_output_as_text)))).align_y(Alignment::Center);
                
                match axis 
                {
                    Axis::Vertical => column![element].width(Length::Fill).align_x(Alignment::Center).into(),
                    Axis::Horizontal => row![element].width(Length::Shrink).align_y(Alignment::Center).into() 
                }
            }
        };
        children.push(element);
    }

    match axis 
    {
        Axis::Vertical => column(children).width(Length::Fill).align_x(Alignment::Center).into(),
        Axis::Horizontal => row(children).width(Length::Shrink).align_y(Alignment::Center).into() 
    }
}



fn axis_layout<'a>(axis: Axis, start: Element<'a, Message>, center: Element<'a, Message>, end: Element<'a, Message>) -> Element<'a, Message> 
{
    match axis 
    {
        Axis::Horizontal => 
        {
            row!
            [
                container(start).width(Length::Fill).align_x(iced::alignment::Horizontal::Left).align_y(iced::alignment::Vertical::Center),
                container(center).width(Length::Fill).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Center),
                container(end).width(Length::Fill).align_x(iced::alignment::Horizontal::Right).align_y(iced::alignment::Vertical::Center),
            ].width(Length::Fill).height(Length::Fill).align_y(iced::alignment::Vertical::Center).into()
        }

        Axis::Vertical => 
        {
            column!
            [
                container(start).height(Length::Shrink).align_y(iced::alignment::Vertical::Top).align_x(iced::alignment::Horizontal::Center),
                container(center).height(Length::Fill).align_y(iced::alignment::Vertical::Center).align_x(iced::alignment::Horizontal::Center),
                container(end).height(Length::Shrink).align_y(iced::alignment::Vertical::Bottom).align_x(iced::alignment::Horizontal::Center),
            ].width(Length::Fill).height(Length::Fill).align_x(iced::alignment::Horizontal::Center).into()
        }
    }
}
