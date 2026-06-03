// ============ IMPORTS ============
use iced::{Alignment, Element, Font, Length, Task, Theme, border::Radius, widget::{button, column, container, row, text}};
use iced_layershell::reexport::{Anchor, Layer, NewLayerShellSettings, KeyboardInteractivity};
use serde::{Serialize, Deserialize};





// ============ CRATES ============
use crate::helpers::{color::{ColorType, Gradient}, style::{TextOrientation, orient_text, set_style}};
use crate::ron::{BarConfig, BarPosition};
use crate::modules::tray::MenuItem;
use crate::{AppData, WindowInfo};
use crate::update::Message;





// ============ ENUM/STRUCT, ETC ============
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ContextMenuConfig
{
    pub context_menu_background_color:           ColorType,
    pub context_menu_background_size:            u16,
    pub context_menu_background_border_color:    ColorType,
    pub context_menu_background_border_size:     f32,
    pub context_menu_background_border_radius:   [f32; 4],
    pub context_menu_text_size:                  u32,
    pub context_menu_text_color:                 ColorType,
    pub context_menu_orientation:                TextOrientation,
    pub context_menu_size:                       u32,
    pub context_menu_item_size:                  u32,
    pub context_menu_button_color:               ColorType,
    pub context_menu_button_hovered_color:       ColorType,
    pub context_menu_button_hovered_text_color:  ColorType,
    pub context_menu_button_pressed_text_color:  ColorType,
    pub context_menu_button_pressed_color:       ColorType,
    pub context_menu_border_color:               ColorType,
    pub context_menu_border_size:                f32,
    pub context_menu_border_radius:              [f32; 4],
    pub context_menu_button_gradient_color:         Option<Gradient>,
    pub context_menu_button_hovered_gradient_color: Option<Gradient>,
    pub context_menu_button_pressed_gradient_color: Option<Gradient>
}

impl Default for ContextMenuConfig
{
    fn default() -> Self
    {
        Self
        {
            context_menu_background_color:           ColorType::RGBA([20, 20, 24, 98]),
            context_menu_background_size:            5,
            context_menu_background_border_color:    ColorType::RGB([255, 255, 255]),
            context_menu_background_border_size:     1.0,
            context_menu_background_border_radius:   [3.0, 3.0, 3.0, 3.0],
            context_menu_text_size:                  15,
            context_menu_text_color:                 ColorType::RGB([255, 255, 255]),
            context_menu_orientation:                TextOrientation::Vertical,
            context_menu_size:                       300,
            context_menu_item_size:                  30,
            context_menu_button_color:               ColorType::RGB([45, 40, 55]),
            context_menu_button_hovered_color:       ColorType::RGB([150, 40, 80]),
            context_menu_button_hovered_text_color:  ColorType::RGB([255, 255, 255]),
            context_menu_button_pressed_text_color:  ColorType::RGB([255, 255, 255]),
            context_menu_button_pressed_color:       ColorType::RGB([85, 30, 55]),
            context_menu_border_color:               ColorType::RGB([130, 90, 140]),
            context_menu_border_size:                1.0,
            context_menu_border_radius:              [3.0, 3.0, 3.0, 3.0],
            context_menu_button_gradient_color:         None,
            context_menu_button_hovered_gradient_color: None,
            context_menu_button_pressed_gradient_color: None
        }
    }
}

#[derive(Default, Clone)]
pub struct ContextMenuData
{
    pub context_menu_is_open: bool,
    pub cursor_is_inside_menu: bool,
    pub mouse_position: (i32, i32),
    pub items: Vec<MenuItem>,
    pub default_font: Font,
    pub service: String,
    pub path: String
}





// ============ FUNCTIONS ============
pub fn create_context_menu(app: &mut AppData) -> Task<Message>
{
    let anchor_position = match app.ron_config.general.bar_position
    {
        BarPosition::Down  => Anchor::Bottom | Anchor::Left,
        BarPosition::Up    => Anchor::Top | Anchor::Left,
        BarPosition::Left  => Anchor::Left | Anchor::Top,
        BarPosition::Right => Anchor::Right | Anchor::Top
    };

    let context_menu_size = get_context_menu_size(&app.context_menu_data, &app.ron_config);

    let (context_menu_pos_x, context_menu_pos_y) = if let Some(forced_values) = app.ron_config.general.force_static_position_context_menu
    {
        forced_values
    }
    else
    {
        smart_popup_position(app.context_menu_data.mouse_position.0, app.context_menu_data.mouse_position.1, app.monitor_size.0 as i32, app.monitor_size.1 as i32, context_menu_size.0 as i32, context_menu_size.1 as i32)
    };

    // create a full-screen transparent backdrop to capture outside clicks
    let backdrop_id = iced::window::Id::unique();
    app.ids.insert(backdrop_id, WindowInfo::ContextMenuBackdrop);

    // create the actual context menu window
    let id = iced::window::Id::unique();
    app.ids.insert(id, WindowInfo::ContextMenu);

    let backdrop_settings = NewLayerShellSettings
    {
        layer: Layer::Overlay,
        size: Some((app.monitor_size.0, app.monitor_size.1)),
        exclusive_zone: Some(0),
        keyboard_interactivity: KeyboardInteractivity::None,
        anchor: Anchor::Top | Anchor::Left,
        margin: Some((0, 0, 0, 0)),
        ..Default::default()
    };

    let menu_settings = NewLayerShellSettings
    {
        layer: Layer::Overlay,
        size: Some((context_menu_size.0, context_menu_size.1)),
        exclusive_zone: Some(0),
        keyboard_interactivity: KeyboardInteractivity::Exclusive,
        anchor: anchor_position,
        margin: Some((context_menu_pos_y, 0, 0, context_menu_pos_x)),
        ..Default::default()
    };

    Task::batch([
        Task::done(Message::NewLayerShell { settings: backdrop_settings, id: backdrop_id }),
        Task::done(Message::NewLayerShell { settings: menu_settings, id })
    ])
}



pub fn context_menu_view<'a>(data: &'a ContextMenuData, ron_config: &'a BarConfig) -> Element<'a, Message>
{
    let button_vec: Vec<Element<'_, Message>> = data.items.iter().map(|item|
    {
        let (text_to_send, (width, heigth)) = match ron_config.context_menu.context_menu_orientation
        {
            TextOrientation::Horizontal =>
            {
                (orient_text(&item.label, &TextOrientation::Vertical), (Length::Fixed(ron_config.context_menu.context_menu_item_size as f32), Length::Fill))
            }
            TextOrientation::Vertical =>
            {
                (item.label.clone(), (Length::Fill, Length::Fixed(ron_config.context_menu.context_menu_item_size as f32)))
            }
        };

        let _ctx_text_color = ron_config.context_menu.context_menu_text_color.to_iced_color();
        let color_to_send    = _ctx_text_color;
        let hovered_text     = ron_config.context_menu.context_menu_button_hovered_text_color;
        let pressed_text     = ron_config.context_menu.context_menu_button_pressed_text_color;
        let normal_text      = ron_config.context_menu.context_menu_text_color;
        let border_color     = ron_config.context_menu.context_menu_border_color;
        let border_size      = ron_config.context_menu.context_menu_border_size;
        let border_radius    = ron_config.context_menu.context_menu_border_radius;
        let normal_background  = crate::helpers::style::match_color_or_gradient(ron_config.context_menu.context_menu_button_gradient_color.as_ref(),         ron_config.context_menu.context_menu_button_color);
        let hovered_background = crate::helpers::style::match_color_or_gradient(ron_config.context_menu.context_menu_button_hovered_gradient_color.as_ref(), ron_config.context_menu.context_menu_button_hovered_color);
        let pressed_background = crate::helpers::style::match_color_or_gradient(ron_config.context_menu.context_menu_button_pressed_gradient_color.as_ref(), ron_config.context_menu.context_menu_button_pressed_color);
        button(text(text_to_send).color(color_to_send).align_y(Alignment::Center).align_y(Alignment::Center).font(data.default_font).size(ron_config.context_menu.context_menu_text_size).width(Length::Fill).height(Length::Fill).center()).width(width).height(heigth).on_press(Message::TrayAction(data.service.to_string(), data.path.to_string(), item.id, item.label.to_string())).style(move |_: &Theme, status: button::Status|
        {
            set_style(crate::UserStyle { status, hovered_text, pressed_text, normal_text, border_color, border_size, border_radius, normal_background, hovered_background, pressed_background, shadow_color: None, shadow_blur: 0., shadow_x: 0., shadow_y: 0. })
        }).into()
    }).collect();

    let row_or_column: Element<Message> = match &ron_config.context_menu.context_menu_orientation
    {
        TextOrientation::Horizontal => row(button_vec).spacing(0).width(Length::Fill).height(Length::Fill).into(),
        TextOrientation::Vertical   => column(button_vec).spacing(0).width(Length::Fill).height(Length::Fill).into()
    };

    container
    (
        row_or_column
    ).padding(ron_config.context_menu.context_menu_background_size).width(Length::Fill).height(Length::Fill).style(move |_: &Theme| context_menu_background_button_style(ron_config)).width(Length::Fill).height(Length::Fill).into()
}



fn context_menu_background_button_style(ron_config: &BarConfig) -> iced::widget::container::Style
{
    let mut background_style = container::Style::default();
    let bgc = ron_config.context_menu.context_menu_background_color.to_iced_color();
    let bgr = ron_config.context_menu.context_menu_background_border_radius;
    background_style.background = Some(iced::Background::Color(bgc));
    background_style.border.color = ron_config.context_menu.context_menu_background_border_color.to_iced_color();
    background_style.border.width = ron_config.context_menu.context_menu_background_border_size;
    background_style.border.radius = Radius { top_left: bgr[0], top_right: bgr[1], bottom_left: bgr[2], bottom_right: bgr[3] };
    background_style
}



pub fn smart_popup_position(cursor_x: i32, cursor_y: i32, screen_w: i32, screen_h: i32, popup_w: i32, popup_h: i32) -> (i32, i32)
{
    let mut x = cursor_x - popup_w / 2;
    let mut y = cursor_y - popup_h / 2;
    let max_x = (screen_w - popup_w).max(0);
    let max_y = (screen_h - popup_h).max(0);
    x = x.clamp(0, max_x);
    y = y.clamp(0, max_y);
    (x, y)
}



pub fn get_context_menu_size(data: &ContextMenuData, ron_config: &BarConfig) -> (u32, u32)
{
    let item_count = data.items.len() as u32;
    let menu_item_size = ron_config.context_menu.context_menu_item_size;
    let context_size = ron_config.context_menu.context_menu_size;
    let context_background_size = ron_config.context_menu.context_menu_background_size as u32;
    match ron_config.context_menu.context_menu_orientation
    {
        TextOrientation::Horizontal =>
        (
            (item_count * menu_item_size) + context_background_size * 2,
            context_size + context_background_size * 2
        ),
        TextOrientation::Vertical =>
        (
            context_size + context_background_size * 2,
            (item_count * menu_item_size) + context_background_size * 2
        )
    }
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn popup_position_centered_when_space_available()
    {
        let (x, y) = smart_popup_position(500, 400, 1920, 1080, 200, 100);
        assert_eq!(x, 400);
        assert_eq!(y, 350);
    }

    #[test]
    fn popup_position_clamps_to_left_edge()
    {
        let (x, _) = smart_popup_position(0, 500, 1920, 1080, 200, 100);
        assert_eq!(x, 0);
    }

    #[test]
    fn popup_position_clamps_to_top_edge()
    {
        let (_, y) = smart_popup_position(500, 0, 1920, 1080, 200, 100);
        assert_eq!(y, 0);
    }

    #[test]
    fn popup_position_clamps_to_right_edge()
    {
        let (x, _) = smart_popup_position(1919, 500, 1920, 1080, 200, 100);
        assert_eq!(x, 1720);
    }

    #[test]
    fn popup_position_clamps_to_bottom_edge()
    {
        let (_, y) = smart_popup_position(500, 1079, 1920, 1080, 200, 100);
        assert_eq!(y, 980);
    }

    #[test]
    fn popup_position_popup_larger_than_screen_clamps_to_zero()
    {
        let (x, y) = smart_popup_position(100, 100, 100, 100, 300, 300);
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn popup_position_cursor_at_center_of_screen()
    {
        let (x, y) = smart_popup_position(960, 540, 1920, 1080, 100, 50);
        assert_eq!(x, 910);
        assert_eq!(y, 515);
    }

    #[test]
    fn popup_position_exact_fit_on_screen_no_clamping()
    {
        let (x, y) = smart_popup_position(100, 100, 1920, 1080, 200, 200);
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn popup_position_zero_size_popup_returns_cursor()
    {
        let (x, y) = smart_popup_position(400, 300, 1920, 1080, 0, 0);
        assert_eq!(x, 400);
        assert_eq!(y, 300);
    }

    #[test]
    fn popup_position_x_and_y_clamp_independently()
    {
        let (x, y) = smart_popup_position(5, 500, 1920, 1080, 200, 100);
        assert_eq!(x, 0);
        assert_eq!(y, 450);
    }

    #[test]
    fn popup_position_cursor_at_bottom_right_corner()
    {
        let (x, y) = smart_popup_position(1919, 1079, 1920, 1080, 200, 100);
        assert_eq!(x, 1720);
        assert_eq!(y, 980);
    }

    #[test]
    fn popup_position_zero_screen_size_returns_zero()
    {
        let (x, y) = smart_popup_position(100, 100, 0, 0, 50, 50);
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    fn make_config(orientation: TextOrientation, size: u32, item_size: u32, bg_size: u16) -> crate::ron::BarConfig
    {
        let mut config = crate::ron::BarConfig::default();
        config.context_menu.context_menu_orientation = orientation;
        config.context_menu.context_menu_size = size;
        config.context_menu.context_menu_item_size = item_size;
        config.context_menu.context_menu_background_size = bg_size;
        config
    }

    fn make_items(count: usize) -> Vec<MenuItem>
    {
        (0..count).map(|i| MenuItem { id: i as i32, label: format!("Item {i}"), _visible: true }).collect()
    }

    #[test]
    fn context_menu_size_single_item_vertical()
    {
        let config = make_config(TextOrientation::Vertical, 150, 40, 0);
        let data = ContextMenuData { items: make_items(1), ..Default::default() };
        let (w, h) = get_context_menu_size(&data, &config);
        assert_eq!(w, 150);
        assert_eq!(h, 40);
    }

    #[test]
    fn context_menu_size_single_item_horizontal()
    {
        let config = make_config(TextOrientation::Horizontal, 60, 50, 0);
        let data = ContextMenuData { items: make_items(1), ..Default::default() };
        let (w, h) = get_context_menu_size(&data, &config);
        assert_eq!(w, 50);
        assert_eq!(h, 60);
    }

    #[test]
    fn context_menu_size_background_padding_applies_to_both_axes_vertical()
    {
        let config = make_config(TextOrientation::Vertical, 100, 30, 10);
        let data = ContextMenuData { items: make_items(0), ..Default::default() };
        let (w, h) = get_context_menu_size(&data, &config);
        assert_eq!(w, 120);
        assert_eq!(h, 20);
    }

    #[test]
    fn context_menu_size_background_padding_applies_to_both_axes_horizontal()
    {
        let config = make_config(TextOrientation::Horizontal, 100, 30, 10);
        let data = ContextMenuData { items: make_items(0), ..Default::default() };
        let (w, h) = get_context_menu_size(&data, &config);
        assert_eq!(w, 20);
        assert_eq!(h, 120);
    }

    #[test]
    fn context_menu_size_many_items_vertical_grows_height()
    {
        let config = make_config(TextOrientation::Vertical, 200, 25, 0);
        let data = ContextMenuData { items: make_items(10), ..Default::default() };
        let (w, h) = get_context_menu_size(&data, &config);
        assert_eq!(w, 200);
        assert_eq!(h, 250);
    }

    #[test]
    fn context_menu_size_many_items_horizontal_grows_width()
    {
        let config = make_config(TextOrientation::Horizontal, 50, 25, 0);
        let data = ContextMenuData { items: make_items(10), ..Default::default() };
        let (w, h) = get_context_menu_size(&data, &config);
        assert_eq!(w, 250);
        assert_eq!(h, 50);
    }

    #[test]
    fn context_menu_size_vertical_and_horizontal_swap_axes()
    {
        let data = ContextMenuData { items: make_items(3), ..Default::default() };

        let config_v = make_config(TextOrientation::Vertical,   200, 40, 5);
        let config_h = make_config(TextOrientation::Horizontal, 200, 40, 5);

        let (wv, hv) = get_context_menu_size(&data, &config_v);
        let (wh, hh) = get_context_menu_size(&data, &config_h);

        assert_eq!(wv, hh);
        assert_eq!(hv, wh);
    }
}
