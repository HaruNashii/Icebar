// ============ IMPORTS ============
use iced::{Alignment, Element, Theme};
use iced::widget::{button, container, mouse_area};





// ============ CRATES ============
use crate::ron::{BarConfig, BarPosition};
use crate::modules::data::Modules;
use crate::update::Message;
use crate::AppData;





// ============ FUNCTIONS ============
pub fn is_active_module(active_modules: &Vec<Modules>, module: Modules) -> bool
{
    for item in active_modules 
    {
        if *item == module 
        {
            return true;
        }
    }
    false
}



pub fn validade_bar_size_and_margin(ron_config: &BarConfig) -> ((u32, u32), u32, (i32, i32, i32, i32))
{
    match ron_config.bar_position 
    {
        BarPosition::Up => 
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On Top!!!") }
            ((0, ron_config.bar_size[1]), ron_config.bar_size[1] + ron_config.increased_exclusive_bar_zone, (ron_config.floating_space, 0, 0 ,0))
        },
        BarPosition::Right =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Right!!!") }
            ((ron_config.bar_size[0], 0), ron_config.bar_size[0] + ron_config.increased_exclusive_bar_zone, (0, ron_config.floating_space, 0, 0))
        }
        BarPosition::Down =>
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On The Bottom!!!") }
            ((0, ron_config.bar_size[1]), ron_config.bar_size[1] + ron_config.increased_exclusive_bar_zone, (0, 0, ron_config.floating_space, 0))
        }
        BarPosition::Left =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Left!!!") }
            ((ron_config.bar_size[0], 0), ron_config.bar_size[0] + ron_config.increased_exclusive_bar_zone, (0, 0, 0, ron_config.floating_space))
        }
    }
}



pub fn create_button_container<'a, F>(app: &'a AppData, text_data: (iced::widget::text::Rich<'a, (), Message>, u32), on_enter_message: Message, on_exit_message: Message, left_click_message: Message, right_click_message: Message, style_func: F) -> Element<'a, Message>
where F: Fn(&AppData, button::Status) -> button::Style + 'a,
{
    container
    (
        button
        (
            mouse_area
            (
                text_data.0
                .wrapping(iced::widget::text::Wrapping::Word)
                .font(app.default_font)
                .size(text_data.1)
                .center()
            )
            .on_enter(on_enter_message)
            .on_exit(on_exit_message)
            .on_right_press(right_click_message)
        )
        .on_press(left_click_message)
        .style(move |_: &Theme, status: button::Status| 
        {
            style_func(app, status)
        })
    ).align_y(Alignment::Center)
    .into()
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::modules::data::Modules;
 
    // ---- is_active_module ---------------------------------------------------
 
    #[test]
    fn is_active_module_found()
    {
        let modules = vec![Modules::Clock, Modules::Network, Modules::Tray];
        assert!(is_active_module(&modules, Modules::Clock));
        assert!(is_active_module(&modules, Modules::Network));
        assert!(is_active_module(&modules, Modules::Tray));
    }
 
    #[test]
    fn is_active_module_not_found()
    {
        let modules = vec![Modules::Clock];
        assert!(!is_active_module(&modules, Modules::Network));
    }
 
    #[test]
    fn is_active_module_empty_list()
    {
        assert!(!is_active_module(&vec![], Modules::Clock));
    }
 
    #[test]
    fn is_active_module_custom_module_matches_by_index()
    {
        let modules = vec![Modules::CustomModule(0), Modules::CustomModule(2)];
        assert!(is_active_module(&modules, Modules::CustomModule(0)));
        assert!(is_active_module(&modules, Modules::CustomModule(2)));
        assert!(!is_active_module(&modules, Modules::CustomModule(1)));
    }
}
