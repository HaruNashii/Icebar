// ============ IMPORTS ============
use iced::{Alignment, Element, Length, Theme, widget::{button, container, image, mouse_area, row, text}};





// ============ CRATES ============
use crate::{helpers::{string::ellipsize, style::{UserStyle, set_style}}, modules::data::Modules};
use crate::ron::ActionOnClick;
use crate::{AppData, Message};





// ============ FUNCTIONS ============
pub fn view(app: &AppData) -> Element<'_,Message>
{
    // ---------- MODULES ----------
    let left = build_modules(&app.ron_config.left_modules, app);
    let center = build_modules(&app.ron_config.center_modules, app);
    let right = build_modules(&app.ron_config.right_modules, app);

    // ---------- bar ----------
    let bar = row!
    [
        container(left).align_x(iced::alignment::Horizontal::Left).align_y(iced::alignment::Vertical::Top).width(Length::Fill),
        container(center).align_x(iced::alignment::Horizontal::Center).align_y(iced::alignment::Vertical::Top).width(Length::Shrink),
        container(right).align_x(iced::alignment::Horizontal::Right).align_y(iced::alignment::Vertical::Top).width(Length::Fill),
    ].align_y(Alignment::Start);
    bar.into()
}



fn build_modules<'a>(list_of_modules: &'a Vec<Modules>, app: &'a AppData) -> Element<'a, Message> 
{
    let mut children = Vec::new();
    for item in list_of_modules
    {
        let element: Element<_> = match item
        {
            Modules::Tray => row ( app.modules_data.tray_icons.iter().enumerate().map(|(i,(icon,_))| { let content: Element<_> = if let Some(icon) = icon { image(icon.clone()).width(app.ron_config.tray_icon_size).height(app.ron_config.tray_icon_size).into() } else { text("?").into() }; button(content).style(|_: &Theme, status: button::Status| 
            {
                let hovered = app.ron_config.tray_button_hovered_color_rgb;
                let hovered_text = app.ron_config.tray_button_hovered_text_color_rgb;
                let pressed = app.ron_config.tray_button_pressed_color_rgb;
                let normal = app.ron_config.tray_button_color_rgb;
                let normal_text = app.ron_config.tray_button_text_color_rgb;
                let border_size = app.ron_config.tray_border_size;
                let border_color_rgba = app.ron_config.tray_border_color_rgba;
                let border_radius = app.ron_config.tray_border_radius;
                set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
            }).padding(app.ron_config.tray_button_size).on_press(Message::TrayIconClicked(i)).into() })).height(app.ron_config.tray_height).spacing(app.ron_config.tray_spacing).align_y(Alignment::Center).into(),



            Modules::HyprWorkspaces | Modules::SwayWorkspaces => mouse_area ( row(app.modules_data.workspace_data.visible_workspaces.iter().map(|i| 
            {
                let id = *i; // workspace id (i32)

                // ================= TEXT RESOLUTION =================
                let workspace_text = if id == app.modules_data.workspace_data.current_workspace
                {
                    // ---- ACTIVE WORKSPACE ----
                    if let Some(selected) = &app.ron_config.workspace_selected_text 
                    {
                        selected.get((id - 1) as usize).cloned().unwrap_or_else(|| id.to_string())
                    } 
                    else 
                    {
                        id.to_string()
                    }
                } 
                else 
                {
                    // ---- NORMAL WORKSPACE ----
                    app.ron_config.workspace_text.get((id - 1) as usize).cloned().unwrap_or_else(|| id.to_string())
                };

                let padding_y = if let Some(value) = app.ron_config.workspace_different_selected_width && id == app.modules_data.workspace_data.current_workspace
                {
                    value
                }
                else
                {
                    app.ron_config.workspace_width
                };

                button(text(workspace_text.clone()).font(app.default_font).size(app.ron_config.workspace_text_size)).on_press(Message::WorkspaceButtonPressed(*i as usize)).style(move|_: &Theme, status: button::Status| 
                {
                    let hovered = app.ron_config.workspace_button_hovered_color_rgb;
                    let hovered_text = app.ron_config.workspace_button_hovered_text_color_rgb;
                    let pressed = app.ron_config.workspace_button_pressed_color_rgb;
                    let normal = if app.modules_data.workspace_data.current_workspace == *i { app.ron_config.workspace_button_selected_color_rgb } else { app.ron_config.workspace_button_color_rgb };
                    let normal_text = app.ron_config.workspace_button_text_color_rgb;
                    let border_size = app.ron_config.workspace_border_size;
                    let border_color_rgba = app.ron_config.workspace_border_color_rgba;
                    let border_radius = app.ron_config.workspace_border_radius;
                    set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
                }).padding([app.ron_config.workspace_width, padding_y * 2]).into() 
            })).height(app.ron_config.workspace_height).spacing(app.ron_config.workspace_spacing).align_y(Alignment::Center)).on_enter(Message::IsHoveringWorkspace(true)).on_exit(Message::IsHoveringWorkspace(false)).into(),




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

                container(mouse_area(button(text(&*app.modules_data.clock_data.current_time).font(app.default_font).size(app.ron_config.clock_text_size)).style(|_: &Theme, status: button::Status| 
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
                })).on_press(left_click_message).on_right_press(right_click_message)).height(app.ron_config.clock_height).align_y(Alignment::Center).into()
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
                container(mouse_area ( button (text(&*app.modules_data.volume_data.output_volume_level).font(app.default_font).size(app.ron_config.volume_output_text_size)).style(|_: &Theme, status: button::Status| 
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
                })).on_press(left_click_message).on_right_press(right_click_message).on_enter(Message::IsHoveringVolumeOutput(true)).on_exit(Message::IsHoveringVolumeOutput(false))).height(app.ron_config.volume_output_height).align_y(Alignment::Center).into()
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
                container(mouse_area ( button (text(&*app.modules_data.volume_data.input_volume_level).font(app.default_font).size(app.ron_config.volume_input_text_size)).style(|_: &Theme, status: button::Status| 
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
                })).on_press(left_click_message).on_right_press(right_click_message).on_enter(Message::IsHoveringVolumeInput(true)).on_exit(Message::IsHoveringVolumeInput(false))).height(app.ron_config.volume_input_height).align_y(Alignment::Center).into()
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


                let element = container(mouse_area(button(text(text_to_render).wrapping(iced::widget::text::Wrapping::None).font(app.default_font).size(custom_module.text_size)).height(custom_module.height).style(|_, status| 
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
            
                row![element].spacing(app.ron_config.custom_modules_spacing).into()
            }
        };
        children.push(element);
    }
    row(children).spacing(8).align_y(Alignment::Center).into()
}


