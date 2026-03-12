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

    // ---- validade_bar_size_and_margin ---------------------------------------
 
    #[test]
    fn validate_bar_margin_up_applies_to_top()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Up;
        config.bar_size = [0, 40];
        config.floating_space = 8;
        config.increased_exclusive_bar_zone = 0;
 
        let ((_, h), exclusive, (top, right, bottom, left)) = validade_bar_size_and_margin(&config);
        assert_eq!(h, 40);
        assert_eq!(exclusive, 40);
        assert_eq!(top, 8);
        assert_eq!(right, 0);
        assert_eq!(bottom, 0);
        assert_eq!(left, 0);
    }
 
    #[test]
    fn validate_bar_margin_down_applies_to_bottom()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Down;
        config.bar_size = [0, 35];
        config.floating_space = 4;
        config.increased_exclusive_bar_zone = 0;
 
        let (_, _, (top, right, bottom, left)) = validade_bar_size_and_margin(&config);
        assert_eq!(top, 0);
        assert_eq!(bottom, 4);
        assert_eq!(right, 0);
        assert_eq!(left, 0);
    }
 
    #[test]
    fn validate_bar_margin_left_applies_to_left()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Left;
        config.bar_size = [50, 0];
        config.floating_space = 6;
        config.increased_exclusive_bar_zone = 0;
 
        let ((w, _), exclusive, (top, right, bottom, left)) = validade_bar_size_and_margin(&config);
        assert_eq!(w, 50);
        assert_eq!(exclusive, 50);
        assert_eq!(left, 6);
        assert_eq!(top, 0);
        assert_eq!(right, 0);
        assert_eq!(bottom, 0);
    }
 
    #[test]
    fn validate_bar_margin_right_applies_to_right()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Right;
        config.bar_size = [50, 0];
        config.floating_space = 3;
        config.increased_exclusive_bar_zone = 5;
 
        let ((w, _), exclusive, (_, right, _, _)) = validade_bar_size_and_margin(&config);
        assert_eq!(w, 50);
        assert_eq!(exclusive, 55); // 50 + 5
        assert_eq!(right, 3);
    }
 
    #[test]
    fn validate_bar_increased_exclusive_zone_adds_to_exclusive()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Up;
        config.bar_size = [0, 30];
        config.increased_exclusive_bar_zone = 10;
 
        let (_, exclusive, _) = validade_bar_size_and_margin(&config);
        assert_eq!(exclusive, 40); // 30 + 10
    }
 
    #[test]
    #[should_panic(expected = "Bar Heigth Can't Be Zero")]
    fn validate_bar_up_with_zero_height_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Up;
        config.bar_size = [0, 0];
        validade_bar_size_and_margin(&config);
    }
 
    #[test]
    #[should_panic(expected = "Bar Heigth Can't Be Zero")]
    fn validate_bar_down_with_zero_height_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Down;
        config.bar_size = [0, 0];
        validade_bar_size_and_margin(&config);
    }
 
    #[test]
    #[should_panic(expected = "Bar Width Can't Be Zero")]
    fn validate_bar_left_with_zero_width_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Left;
        config.bar_size = [0, 0];
        validade_bar_size_and_margin(&config);
    }
 
    #[test]
    #[should_panic(expected = "Bar Width Can't Be Zero")]
    fn validate_bar_right_with_zero_width_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Right;
        config.bar_size = [0, 0];
        validade_bar_size_and_margin(&config);
    }
}
