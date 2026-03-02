// ============ IMPORTS ============
use iced::widget::button;
use chrono::Local;





// ============ CRATES ============
use crate::helpers::style::{UserStyle, set_style};
use crate::AppData;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone)]
pub struct ClockData
{
    pub current_time: String
}





// ============ FUNCTIONS ============
pub fn get_current_time(time_format: &str) -> String { Local::now().format(time_format).to_string() }



pub fn define_clock_style(app: &AppData, status: button::Status) -> iced::widget::button::Style
{
    if app.is_showing_alt_clock
    {
        let hovered =           app.ron_config.alt_clock_button_hovered_color_rgb;
        let hovered_text =      app.ron_config.alt_clock_button_hovered_text_color_rgb;
        let pressed =           app.ron_config.alt_clock_button_pressed_color_rgb;
        let normal =            app.ron_config.alt_clock_button_color_rgb;
        let normal_text =       app.ron_config.alt_clock_button_text_color_rgb;
        let border_size =           app.ron_config.alt_clock_border_size;
        let border_color_rgba = app.ron_config.alt_clock_border_color_rgba;
        let border_radius =    app.ron_config.alt_clock_border_radius;
        set_style(UserStyle { status, hovered, hovered_text, pressed, normal, normal_text, border_color_rgba, border_size, border_radius} )
    }
    else
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
    }

}
