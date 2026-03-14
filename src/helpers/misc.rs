// ============ IMPORTS ============
use iced::widget::{button, container, mouse_area};
use iced::{Alignment, Element, Theme};
use std::collections::HashSet;





// ============ CRATES ============
use crate::ron::{BarConfig, BarPosition};
use crate::modules::data::Modules;
use crate::update::Message;
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
pub struct ValidatedBarSizeAndMargin
{
    pub bar_size: (u32, u32),
    pub exclusive_zone: i32,
    pub floating_space: (i32, i32, i32, i32)
}





// ============ FUNCTIONS ============
pub fn is_active_module(active_modules: &HashSet<Modules>, module: Modules) -> bool
{
    active_modules.contains(&module)
}



pub fn validade_bar_data(ron_config: &BarConfig) -> ValidatedBarSizeAndMargin
{
    let bar_maximum_size: u32 = 400;
    let bar_size_x = ron_config.bar_size[0].clamp(0, bar_maximum_size);
    let bar_size_y = ron_config.bar_size[1].clamp(0, bar_maximum_size);

    // Here the bar width is always 0 when the bar is Up or Down, because the size showed is of the
    // container in the view.rs, the same occuors with the Left and Right bars but with the height
    match ron_config.bar_position 
    {
        BarPosition::Up => 
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On Top!!!") }
            if ron_config.bar_size[1] > 400 
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar width is greater than the bar size limit, clamping to: {}", bar_maximum_size); 
            };
            ValidatedBarSizeAndMargin
            {
                bar_size: (0, bar_size_y),
                exclusive_zone: bar_size_y as i32 + ron_config.increased_exclusive_bar_zone, 
                floating_space: (ron_config.floating_space, 0, 0 ,0)
            }
        },
        BarPosition::Right =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Right!!!") }
            if ron_config.bar_size[0] > 400 
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar width is greater than the bar size limit, clamping to: {}", bar_maximum_size); }
            ;
            ValidatedBarSizeAndMargin
            {
                bar_size: (bar_size_x, 0),
                exclusive_zone: bar_size_x as i32 + ron_config.increased_exclusive_bar_zone, 
                floating_space: (0, ron_config.floating_space, 0, 0)
            }
        }
        BarPosition::Down =>
        {
            if ron_config.bar_size[1] == 0 { panic!("ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On The Bottom!!!") }
            if ron_config.bar_size[1] > 400 
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar width is greater than the bar size limit, clamping to: {}", bar_maximum_size); 
            };
            ValidatedBarSizeAndMargin
            {
                bar_size:  (0, bar_size_y),
                exclusive_zone: bar_size_y as i32 + ron_config.increased_exclusive_bar_zone,
                floating_space: (0, 0, ron_config.floating_space, 0)
            }
        }
        BarPosition::Left =>
        {
            if ron_config.bar_size[0] == 0 { panic!("ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Left!!!") }
            if ron_config.bar_size[0] > 400 
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar width is greater than the bar size limit, clamping to: {}", bar_maximum_size); 
            };
            ValidatedBarSizeAndMargin
            {
                bar_size: (bar_size_x, 0),
                exclusive_zone: bar_size_x as i32 + ron_config.increased_exclusive_bar_zone,
                floating_space: (0, 0, 0, ron_config.floating_space)
            }
        }
    }
}



pub fn create_button_container_without_hover_message<'a, F>(app: &'a AppData, padding: u16, text_data: (iced::widget::text::Rich<'a, (), Message>, u32), left_click_message: Message, right_click_message: Message, style_func: F) -> Element<'a, Message>
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
            .on_right_press(right_click_message)
        )
        .on_press(left_click_message)
        .style(move |_: &Theme, status: button::Status| 
        {
            style_func(app, status)
        })
    ).align_y(Alignment::Center).padding(padding)
    .into()
}



pub fn create_button_container<'a, F>(app: &'a AppData, padding: u16, text_data: (iced::widget::text::Rich<'a, (), Message>, u32), hover_message: (Message, Message), left_click_message: Message, right_click_message: Message, style_func: F) -> Element<'a, Message>
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
            .on_enter(hover_message.0)
            .on_exit(hover_message.1)
            .on_right_press(right_click_message)
        )
        .on_press(left_click_message)
        .style(move |_: &Theme, status: button::Status| 
        {
            style_func(app, status)
        })
    ).align_y(Alignment::Center).padding(padding)
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
        let mut modules: HashSet<Modules> = HashSet::new();
        modules.insert(Modules::Clock);
        modules.insert(Modules::Network);
        modules.insert(Modules::Tray);
        assert!(is_active_module(&modules, Modules::Clock));
        assert!(is_active_module(&modules, Modules::Network));
        assert!(is_active_module(&modules, Modules::Tray));
    }
 
    #[test]
    fn is_active_module_not_found()
    {
        let mut modules: HashSet<Modules> = HashSet::new();
        modules.insert(Modules::Clock);
        assert!(!is_active_module(&modules, Modules::Network));
    }
 
    #[test]
    fn is_active_module_empty_list()
    {
        assert!(!is_active_module(&HashSet::new(), Modules::Clock));
    }
 
    #[test]
    fn is_active_module_custom_module_matches_by_index()
    {
        let mut modules: HashSet<Modules> = HashSet::new();
        modules.insert(Modules::CustomModule(0));
        modules.insert(Modules::CustomModule(2));
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
 
        let bar_data = validade_bar_data(&config);
        let (_, h) =  bar_data.bar_size;
        let exclusive =  bar_data.exclusive_zone;
        let (top, right, bottom, left) = bar_data.floating_space;

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
 
        let bar_data = validade_bar_data(&config);
        let (top, right, bottom, left) = bar_data.floating_space;
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
 
        let bar_data = validade_bar_data(&config);
        let (w, _) =  bar_data.bar_size;
        let exclusive =  bar_data.exclusive_zone;
        let (top, right, bottom, left) = bar_data.floating_space;
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
 
        let bar_data = validade_bar_data(&config);
        let (w, _) =  bar_data.bar_size;
        let exclusive =  bar_data.exclusive_zone;
        let (_, right, _, _) = bar_data.floating_space;
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
 
        let bar_data = validade_bar_data(&config);
        assert_eq!(bar_data.exclusive_zone, 40); // 30 + 10
    }
 
    #[test]
    #[should_panic(expected = "Bar Heigth Can't Be Zero")]
    fn validate_bar_up_with_zero_height_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Up;
        config.bar_size = [0, 0];
        validade_bar_data(&config);
    }
 
    #[test]
    #[should_panic(expected = "Bar Heigth Can't Be Zero")]
    fn validate_bar_down_with_zero_height_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Down;
        config.bar_size = [0, 0];
        validade_bar_data(&config);
    }
 
    #[test]
    #[should_panic(expected = "Bar Width Can't Be Zero")]
    fn validate_bar_left_with_zero_width_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Left;
        config.bar_size = [0, 0];
        validade_bar_data(&config);
    }
 
    #[test]
    #[should_panic(expected = "Bar Width Can't Be Zero")]
    fn validate_bar_right_with_zero_width_panics()
    {
        let mut config = crate::ron::BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Right;
        config.bar_size = [0, 0];
        validade_bar_data(&config);
    }
}
