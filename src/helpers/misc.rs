// ============ IMPORTS ============
use iced_layershell::reexport::Anchor;
use iced::widget::{button, container, mouse_area};
use iced::{Alignment, Element, Theme};
use std::collections::HashSet;





// ============ CRATES ============
use crate::ron::{BarPosition};
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
pub fn define_bar_anchor_position(bar_position: &BarPosition) -> Anchor
{
    match bar_position
    {
        BarPosition::Up => Anchor::Top | Anchor::Left | Anchor::Right,
        BarPosition::Down => Anchor::Bottom | Anchor::Left | Anchor::Right,
        BarPosition::Left => Anchor::Left | Anchor::Top | Anchor::Bottom,
        BarPosition::Right => Anchor::Right | Anchor::Top | Anchor::Bottom,
    }
}



pub fn is_active_module(active_modules: &HashSet<Modules>, module: Modules) -> bool
{
    active_modules.contains(&module)
}



pub fn validate_bar_data(app: &mut AppData) -> ValidatedBarSizeAndMargin
{
    let ron_config = &mut app.ron_config;
    let bar_maximum_size: u32 = 400;
    let mut bar_size_x = ron_config.bar_size[0].clamp(0, bar_maximum_size);
    let mut bar_size_y = ron_config.bar_size[1].clamp(0, bar_maximum_size);
    let maximum_exclusive_bar_zone_size: u32 = 425;
    let exclusive_zone_y = (bar_size_y as i32 + ron_config.increased_exclusive_bar_zone).clamp(0, maximum_exclusive_bar_zone_size as i32);
    let exclusive_zone_x = (bar_size_x as i32 + ron_config.increased_exclusive_bar_zone).clamp(0, maximum_exclusive_bar_zone_size as i32);
    
    // Here the bar width is always 0 when the bar is Up or Down, because the size showed is of the
    // container in the view.rs, the same occuors with the Left and Right bars but with the height
    match ron_config.bar_position 
    {
        BarPosition::Up => 
        {
            if (bar_size_y as i32 + ron_config.increased_exclusive_bar_zone) > maximum_exclusive_bar_zone_size as i32
            {
                println!("\n=== BAR EXCLUSIVE ZONE VALIDATION ===");
                println!("Warning!!!: Bar exclusive zone is greater than the bar size limit, clamping to: {}", maximum_exclusive_bar_zone_size); 
            };
            if ron_config.bar_size[1] == 0 
            { 
                bar_size_y = 35;
                ron_config.bar_size[1] = 35;
                let warning_msg = "Warning!!!: Bar Heigth Can't Be Zero, When The Bar Is On The Top!!!\nUsing default size 35".to_string();
                eprintln!("{warning_msg}");
                app.warning_err = warning_msg;
                app.config_parsed_failed = true;
            }
            if ron_config.bar_size[1] > bar_maximum_size
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar height is greater than the bar size limit, clamping to: {}", bar_maximum_size); 
            };
            ValidatedBarSizeAndMargin
            {
                bar_size: (0, bar_size_y),
                exclusive_zone: exclusive_zone_y,
                floating_space: (ron_config.floating_space, 0, 0, 0)
            }
        },
        BarPosition::Right =>
        {
            if (bar_size_x as i32 + ron_config.increased_exclusive_bar_zone) > maximum_exclusive_bar_zone_size as i32
            {
                println!("\n=== BAR EXCLUSIVE ZONE VALIDATION ===");
                println!("Warning!!!: Bar exclusive zone is greater than the bar size limit, clamping to: {}", maximum_exclusive_bar_zone_size); 
            }
            if ron_config.bar_size[0] == 0 
            { 
                bar_size_x = 35;
                ron_config.bar_size[0] = 35;
                let warning_msg = "ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Right!!!\nUsing default size 35".to_string();
                eprintln!("{warning_msg}");
                app.warning_err = warning_msg;
                app.config_parsed_failed = true;
            }
            if ron_config.bar_size[0] > 400 
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar width is greater than the bar size limit, clamping to: {}", bar_maximum_size); }
            ;
            ValidatedBarSizeAndMargin
            {
                bar_size: (bar_size_x, 0),
                exclusive_zone: exclusive_zone_x,
                floating_space: (0, ron_config.floating_space, 0, 0)
            }
        }
        BarPosition::Down =>
        {
            if (bar_size_y as i32 + ron_config.increased_exclusive_bar_zone) > maximum_exclusive_bar_zone_size as i32
            {
                println!("\n=== BAR EXCLUSIVE ZONE VALIDATION ===");
                println!("Warning!!!: Bar exclusive zone is greater than the bar size limit, clamping to: {}", maximum_exclusive_bar_zone_size); 
            };
            if ron_config.bar_size[1] == 0 
            { 
                bar_size_y = 35;
                ron_config.bar_size[1] = 35;
                let warning_msg = "ERROR!!!: Bar Heigth Can't Be Zero, When The Bar Is On The Bottom!!!\nUsing default size 35".to_string();
                eprintln!("{warning_msg}");
                app.warning_err = warning_msg;
                app.config_parsed_failed = true;
            }
            if ron_config.bar_size[1] > 400 
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar height is greater than the bar size limit, clamping to: {}", bar_maximum_size); 
            };
            ValidatedBarSizeAndMargin
            {
                bar_size:  (0, bar_size_y),
                exclusive_zone: exclusive_zone_y,
                floating_space: (0, 0, ron_config.floating_space, 0)
            }
        }
        BarPosition::Left =>
        {
            if (bar_size_x as i32 + ron_config.increased_exclusive_bar_zone) > maximum_exclusive_bar_zone_size as i32
            {
                println!("\n=== BAR EXCLUSIVE ZONE VALIDATION ===");
                println!("Warning!!!: Bar exclusive zone is greater than the bar size limit, clamping to: {}", maximum_exclusive_bar_zone_size); 
            }
            if ron_config.bar_size[0] == 0 
            { 
                bar_size_x = 35;
                ron_config.bar_size[0] = 35;
                let warning_msg = "ERROR!!!: Bar Width Can't Be Zero, When The Bar Is On The Left!!!\nUsing default size 35".to_string();
                eprintln!("{warning_msg}");
                app.warning_err = warning_msg;
                app.config_parsed_failed = true;
            }
            if ron_config.bar_size[0] > 400 
            { 
                println!("\n=== BAR SIZE VALIDATION ===");
                println!("Warning!!!: Bar width is greater than the bar size limit, clamping to: {}", bar_maximum_size); 
            };
            ValidatedBarSizeAndMargin
            {
                bar_size: (bar_size_x, 0),
                exclusive_zone: exclusive_zone_x,
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
    ).align_x(Alignment::Center).align_y(Alignment::Center).padding(padding)
    .into()
}



pub fn create_button_container<'a, F>(app: &'a AppData, padding: u16, text_data: (iced::widget::text::Rich<'a, (), Message>, u32), hover_message: (Message, Message), left_click_message: Message, right_click_message: Message, style_func: F) -> Element<'a, Message>
where F: Fn(&AppData, button::Status) -> button::Style + 'a,
{
    container
    (
        mouse_area
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
        )
        .on_enter(hover_message.0)
        .on_exit(hover_message.1)
    ).align_x(Alignment::Center).align_y(Alignment::Center).padding(padding)
    .into()
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use crate::modules::data::Modules;
    use crate::AppData;
    use crate::ron::BarConfig;

    fn make_app(config: BarConfig) -> AppData
    {
        AppData { ron_config: config, ..Default::default() }
    }

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

    // ---- validate_bar_data --------------------------------------------------

    #[test]
    fn validate_bar_margin_up_applies_to_top()
    {
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Up;
        config.bar_size = [0, 40];
        config.floating_space = 8;
        config.increased_exclusive_bar_zone = 0;

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);
        let (_, h) = bar_data.bar_size;
        let exclusive = bar_data.exclusive_zone;
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
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Down;
        config.bar_size = [0, 35];
        config.floating_space = 4;
        config.increased_exclusive_bar_zone = 0;

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);
        let (top, right, bottom, left) = bar_data.floating_space;
        assert_eq!(top, 0);
        assert_eq!(bottom, 4);
        assert_eq!(right, 0);
        assert_eq!(left, 0);
    }

    #[test]
    fn validate_bar_margin_left_applies_to_left()
    {
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Left;
        config.bar_size = [50, 0];
        config.floating_space = 6;
        config.increased_exclusive_bar_zone = 0;

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);
        let (w, _) = bar_data.bar_size;
        let exclusive = bar_data.exclusive_zone;
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
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Right;
        config.bar_size = [50, 0];
        config.floating_space = 3;
        config.increased_exclusive_bar_zone = 5;

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);
        let (w, _) = bar_data.bar_size;
        let exclusive = bar_data.exclusive_zone;
        let (_, right, _, _) = bar_data.floating_space;
        assert_eq!(w, 50);
        assert_eq!(exclusive, 55);
        assert_eq!(right, 3);
    }

    #[test]
    fn validate_bar_increased_exclusive_zone_adds_to_exclusive()
    {
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Up;
        config.bar_size = [0, 30];
        config.increased_exclusive_bar_zone = 10;

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);
        assert_eq!(bar_data.exclusive_zone, 40);
    }

    #[test]
    fn validate_bar_up_with_zero_height_uses_default_and_sets_warning()
    {
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Up;
        config.bar_size = [0, 0];

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);

        assert_eq!(bar_data.bar_size.1, 35);
        assert!(app.config_parsed_failed);
        assert!(!app.warning_err.is_empty());
    }

    #[test]
    fn validate_bar_down_with_zero_height_uses_default_and_sets_warning()
    {
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Down;
        config.bar_size = [0, 0];

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);

        assert_eq!(bar_data.bar_size.1, 35);
        assert!(app.config_parsed_failed);
        assert!(!app.warning_err.is_empty());
    }

    #[test]
    fn validate_bar_left_with_zero_width_uses_default_and_sets_warning()
    {
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Left;
        config.bar_size = [0, 0];

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);

        assert_eq!(bar_data.bar_size.0, 35);
        assert!(app.config_parsed_failed);
        assert!(!app.warning_err.is_empty());
    }

    #[test]
    fn validate_bar_right_with_zero_width_uses_default_and_sets_warning()
    {
        let mut config = BarConfig::default();
        config.bar_position = crate::ron::BarPosition::Right;
        config.bar_size = [0, 0];

        let mut app = make_app(config);
        let bar_data = validate_bar_data(&mut app);

        assert_eq!(bar_data.bar_size.0, 35);
        assert!(app.config_parsed_failed);
        assert!(!app.warning_err.is_empty());
    }
}
