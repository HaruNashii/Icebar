// ============ IMPORTS ============
use chrono::{Datelike, Local, NaiveDate};
use iced::{Alignment, Element, Length, Task, Theme, border::Radius, widget::{button, column, container, row, text, Space}};
use iced_layershell::reexport::{Anchor, Layer, NewLayerShellSettings, KeyboardInteractivity};
use serde::{Deserialize, Serialize};





// ============ CRATES ============
use crate::helpers::{color::{ColorType, Gradient}, style::{UserStyle, set_style}};
use crate::windows::context_menu::smart_popup_position;
use crate::{AppData, WindowInfo};
use crate::ron::BarPosition;
use crate::update::Message;





// ============ ENUM/STRUCT, ETC ============
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CalendarView
{
    #[default] Month,
    Year,
    Decade
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum CalendarNavPosition
{
    #[default] Above,
    Below,
    Left,
    Right
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum FirstWeekDay
{
    #[default] Monday,
    Sunday,
    Saturday
}

#[derive(Default, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum DayClickAction
{
    #[default] HighlightOnly,
    CustomAction(Vec<String>)
}

#[derive(Clone, Debug)]
pub struct CalendarData
{
    pub is_open:           bool,
    pub cursor_inside:     bool,
    pub current_view:      CalendarView,
    pub viewing_month:     NaiveDate,
    pub viewing_year:      i32,
    pub viewing_decade:    i32,
    pub selected_day:      Option<NaiveDate>,
    pub mouse_pos:         (i32, i32)
}

impl Default for CalendarData
{
    fn default() -> Self
    {
        let today = Local::now().date_naive();
        Self
        {
            is_open:        false,
            cursor_inside:  false,
            current_view:   CalendarView::Month,
            viewing_month:  NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap_or(today),
            viewing_year:   today.year(),
            viewing_decade: (today.year() / 10) * 10,
            selected_day:   None,
            mouse_pos:      (0, 0)
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct CalendarButtonStyle
{
    pub color:                  ColorType,
    pub hovered_color:          ColorType,
    pub pressed_color:          ColorType,
    pub text_color:             ColorType,
    pub hovered_text_color:     ColorType,
    pub pressed_text_color:     ColorType,
    pub border_color:           ColorType,
    pub border_size:            f32,
    pub border_radius:          [f32; 4],
    pub gradient_color:         Option<Gradient>,
    pub hovered_gradient_color: Option<Gradient>,
    pub pressed_gradient_color: Option<Gradient>
}

impl Default for CalendarButtonStyle
{
    fn default() -> Self
    {
        Self
        {
            color:               ColorType::RGB([45, 40, 55]),
            hovered_color:       ColorType::RGB([80, 60, 100]),
            pressed_color:       ColorType::RGB([55, 35, 75]),
            text_color:          ColorType::RGB([220, 220, 235]),
            hovered_text_color:  ColorType::RGB([255, 255, 255]),
            pressed_text_color:  ColorType::RGB([255, 255, 255]),
            border_color:        ColorType::RGB([80, 70, 100]),
            border_size:         1.0,
            border_radius:       [4., 4., 4., 4.],
            gradient_color:         None,
            hovered_gradient_color: None,
            pressed_gradient_color: None
        }
    }
}

impl CalendarButtonStyle
{
    fn to_user_style(&self, status: button::Status) -> iced::widget::button::Style
    {
        set_style(UserStyle
        {
            status,
            normal_text:    self.text_color,
            hovered_text:   self.hovered_text_color,
            pressed_text:   self.pressed_text_color,
            border_color:   self.border_color,
            border_size:    self.border_size,
            border_radius:  self.border_radius,
            normal_background:  crate::helpers::style::match_color_or_gradient(self.gradient_color.as_ref(),         self.color),
            hovered_background: crate::helpers::style::match_color_or_gradient(self.hovered_gradient_color.as_ref(), self.hovered_color),
            pressed_background: crate::helpers::style::match_color_or_gradient(self.pressed_gradient_color.as_ref(), self.pressed_color),
            shadow_color:      None,
            shadow_x:          0.,
            shadow_y:          0.,
            shadow_blur:       0.
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct CalendarWindowConfig
{
    pub calendar_window_size:               [u32; 2],
    pub calendar_background_color:          ColorType,
    pub calendar_background_border_color:   ColorType,
    pub calendar_background_border_size:    f32,
    pub calendar_background_border_radius:  [f32; 4],
    pub calendar_padding:                   u16,
    pub calendar_nav_spacing_y:             u32,
    pub calendar_nav_position:              CalendarNavPosition,
    pub calendar_nav_grid_spacing:          u16,
    pub calendar_nav_spacing:               u16,
    pub calendar_nav_button_height:         u32,
    pub calendar_nav_button_width:          u32,
    pub calendar_nav_text_size:             u32,
    pub calendar_nav_button_style:          CalendarButtonStyle,
    pub calendar_nav_active_button_style:   CalendarButtonStyle,
    pub calendar_prev_label:                String,
    pub calendar_next_label:                String,
    pub calendar_arrow_button_height:       u32,
    pub calendar_arrow_button_width:        u32,
    pub calendar_arrow_text_size:           u32,
    pub calendar_arrow_button_style:        CalendarButtonStyle,
    pub calendar_total_day_cells:           i32,
    pub calendar_month_spacing_y:           u32,
    pub calendar_weekday_labels:            Vec<String>,
    pub calendar_show_weekday_header:       bool,
    pub calendar_weekday_header_text_size:  u32,
    pub calendar_weekday_header_text_color: ColorType,
    pub calendar_show_week_numbers:         bool,
    pub calendar_week_number_text_size:     u32,
    pub calendar_week_number_text_color:    ColorType,
    pub calendar_first_week_day:            FirstWeekDay,
    pub calendar_day_cell_width:            u32,
    pub calendar_day_cell_height:           u32,
    pub calendar_grid_spacing:              u16,
    pub calendar_day_text_size:             u32,
    pub calendar_day_button_style:          CalendarButtonStyle,
    pub calendar_today_button_style:        CalendarButtonStyle,
    pub calendar_selected_day_button_style: CalendarButtonStyle,
    pub calendar_show_overflow_days_prev_month:        bool,
    pub calendar_show_overflow_days_next_month:        bool,
    pub calendar_overflow_day_button_style: CalendarButtonStyle,
    pub calendar_day_click_action:          DayClickAction,
    pub calendar_year_spacing_y:            u32,
    pub calendar_month_cell_width:          u32,
    pub calendar_month_cell_height:         u32,
    pub calendar_year_grid_columns:         u32,
    pub calendar_year_grid_spacing:         u16,
    pub calendar_month_text_size:           u32,
    pub calendar_month_labels:              Vec<String>,
    pub calendar_month_button_style:        CalendarButtonStyle,
    pub calendar_current_month_button_style: CalendarButtonStyle,
    pub calendar_decade_spacing_y:          u32,
    pub calendar_year_cell_width:           u32,
    pub calendar_year_cell_height:          u32,
    pub calendar_decade_grid_columns:       u32,
    pub calendar_decade_grid_spacing:       u16,
    pub calendar_year_text_size:            u32,
    pub calendar_year_button_style:         CalendarButtonStyle,
    pub calendar_current_year_button_style: CalendarButtonStyle
}

impl Default for CalendarWindowConfig
{
    fn default() -> Self
    {
        Self
        {
            calendar_window_size:               [340, 310],
            calendar_background_color:          ColorType::RGBA([20, 20, 28, 97]),
            calendar_background_border_color:   ColorType::RGB([100, 80, 130]),
            calendar_background_border_size:    1.0,
            calendar_background_border_radius:  [6., 6., 6., 6.],
            calendar_padding:                   8,
            calendar_show_overflow_days_prev_month: true,
            calendar_show_overflow_days_next_month: true,
            calendar_nav_spacing_y:             0,
            calendar_nav_position:              CalendarNavPosition::Above,
            calendar_nav_grid_spacing:          6,
            calendar_nav_spacing:               4,
            calendar_nav_button_height:         28,
            calendar_nav_button_width:          80,
            calendar_nav_text_size:             14,
            calendar_nav_button_style:          CalendarButtonStyle
            {
                color:              ColorType::RGB([45, 40, 55]),
                hovered_color:      ColorType::RGB([90, 60, 115]),
                pressed_color:      ColorType::RGB([65, 40, 85]),
                text_color:         ColorType::RGB([200, 190, 220]),
                hovered_text_color: ColorType::RGB([255, 255, 255]),
                pressed_text_color: ColorType::RGB([255, 255, 255]),
                border_color:       ColorType::RGB([80, 65, 100]),
                border_size:        1.0,
                border_radius:      [4., 4., 4., 4.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            calendar_nav_active_button_style:   CalendarButtonStyle
            {
                color:              ColorType::RGB([110, 60, 150]),
                hovered_color:      ColorType::RGB([130, 80, 170]),
                pressed_color:      ColorType::RGB([90, 45, 125]),
                text_color:         ColorType::RGB([255, 255, 255]),
                hovered_text_color: ColorType::RGB([255, 255, 255]),
                pressed_text_color: ColorType::RGB([235, 235, 255]),
                border_color:       ColorType::RGB([160, 100, 200]),
                border_size:        1.0,
                border_radius:      [4., 4., 4., 4.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            calendar_month_spacing_y:           0,
            calendar_year_spacing_y:            0,
            calendar_decade_spacing_y:          0,
            calendar_total_day_cells:           35,
            calendar_prev_label:                "‹".to_string(),
            calendar_next_label:                "›".to_string(),
            calendar_arrow_button_height:       28,
            calendar_arrow_button_width:        28,
            calendar_arrow_text_size:           18,
            calendar_arrow_button_style:        CalendarButtonStyle::default(),
            calendar_weekday_labels:            vec![
                "Mo".to_string(), "Tu".to_string(), "We".to_string(),
                "Th".to_string(), "Fr".to_string(), "Sa".to_string(), "Su".to_string(),
            ],
            calendar_show_weekday_header:       true,
            calendar_weekday_header_text_size:  12,
            calendar_weekday_header_text_color: ColorType::RGB([160, 145, 190]),
            calendar_show_week_numbers:         false,
            calendar_week_number_text_size:     11,
            calendar_week_number_text_color:    ColorType::RGB([110, 100, 140]),
            calendar_first_week_day:            FirstWeekDay::Monday,
            calendar_day_cell_width:            34,
            calendar_day_cell_height:           28,
            calendar_grid_spacing:              2,
            calendar_day_text_size:             13,
            calendar_day_button_style:          CalendarButtonStyle::default(),
            calendar_today_button_style:        CalendarButtonStyle
            {
                color:              ColorType::RGB([70, 50, 100]),
                hovered_color:      ColorType::RGB([100, 75, 140]),
                pressed_color:      ColorType::RGB([55, 40, 80]),
                text_color:         ColorType::RGB([220, 200, 255]),
                hovered_text_color: ColorType::RGB([255, 255, 255]),
                pressed_text_color: ColorType::RGB([255, 255, 255]),
                border_color:       ColorType::RGB([140, 100, 200]),
                border_size:        1.5,
                border_radius:      [4., 4., 4., 4.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            calendar_selected_day_button_style: CalendarButtonStyle
            {
                color:              ColorType::RGB([130, 60, 180]),
                hovered_color:      ColorType::RGB([155, 80, 205]),
                pressed_color:      ColorType::RGB([110, 45, 155]),
                text_color:         ColorType::RGB([255, 255, 255]),
                hovered_text_color: ColorType::RGB([255, 255, 255]),
                pressed_text_color: ColorType::RGB([235, 220, 255]),
                border_color:       ColorType::RGB([200, 140, 255]),
                border_size:        1.5,
                border_radius:      [4., 4., 4., 4.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            calendar_overflow_day_button_style: CalendarButtonStyle
            {
                color:              ColorType::RGBA([35, 30, 45, 60]),
                hovered_color:      ColorType::RGBA([55, 45, 70, 80]),
                pressed_color:      ColorType::RGBA([30, 25, 40, 60]),
                text_color:         ColorType::RGB([100, 90, 120]),
                hovered_text_color: ColorType::RGB([140, 130, 160]),
                pressed_text_color: ColorType::RGB([120, 110, 140]),
                border_color:       ColorType::RGBA([70, 60, 90, 40]),
                border_size:        0.5,
                border_radius:      [4., 4., 4., 4.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            calendar_day_click_action:          DayClickAction::HighlightOnly,
            calendar_month_cell_width:          70,
            calendar_month_cell_height:         38,
            calendar_year_grid_columns:         3,
            calendar_year_grid_spacing:         4,
            calendar_month_text_size:           13,
            calendar_month_labels:              vec![
                "Jan".to_string(), "Feb".to_string(), "Mar".to_string(),
                "Apr".to_string(), "May".to_string(), "Jun".to_string(),
                "Jul".to_string(), "Aug".to_string(), "Sep".to_string(),
                "Oct".to_string(), "Nov".to_string(), "Dec".to_string(),
            ],
            calendar_month_button_style:        CalendarButtonStyle::default(),
            calendar_current_month_button_style: CalendarButtonStyle
            {
                color:              ColorType::RGB([110, 60, 150]),
                hovered_color:      ColorType::RGB([140, 85, 185]),
                pressed_color:      ColorType::RGB([90, 45, 120]),
                text_color:         ColorType::RGB([255, 255, 255]),
                hovered_text_color: ColorType::RGB([255, 255, 255]),
                pressed_text_color: ColorType::RGB([235, 220, 255]),
                border_color:       ColorType::RGB([180, 120, 230]),
                border_size:        1.5,
                border_radius:      [4., 4., 4., 4.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            },
            calendar_year_cell_width:           60,
            calendar_year_cell_height:          38,
            calendar_decade_grid_columns:       3,
            calendar_decade_grid_spacing:       4,
            calendar_year_text_size:            13,
            calendar_year_button_style:         CalendarButtonStyle::default(),
            calendar_current_year_button_style: CalendarButtonStyle
            {
                color:              ColorType::RGB([110, 60, 150]),
                hovered_color:      ColorType::RGB([140, 85, 185]),
                pressed_color:      ColorType::RGB([90, 45, 120]),
                text_color:         ColorType::RGB([255, 255, 255]),
                hovered_text_color: ColorType::RGB([255, 255, 255]),
                pressed_text_color: ColorType::RGB([235, 220, 255]),
                border_color:       ColorType::RGB([180, 120, 230]),
                border_size:        1.5,
                border_radius:      [4., 4., 4., 4.],
                gradient_color:         None,
                hovered_gradient_color: None,
                pressed_gradient_color: None
            }
        }
    }
}





// ============ FUNCTIONS ============
pub fn create_calendar_window(app: &mut AppData) -> Task<Message>
{
    let cfg  = &app.ron_config.calendar_window;
    let [w, h] = cfg.calendar_window_size;

    let anchor = match app.ron_config.general.bar_position
    {
        BarPosition::Down  => Anchor::Bottom | Anchor::Left,
        BarPosition::Up    => Anchor::Top    | Anchor::Left,
        BarPosition::Left  => Anchor::Left   | Anchor::Top,
        BarPosition::Right => Anchor::Right  | Anchor::Top
    };

    let (mx, my) = app.modules_data.calendar_data.mouse_pos;
    let (pos_x, pos_y) = smart_popup_position(mx, my, app.monitor_size.0 as i32, app.monitor_size.1 as i32, w as i32, h as i32);
    // backdrop
    let backdrop_id = iced::window::Id::unique();
    app.ids.insert(backdrop_id, WindowInfo::ContextMenuBackdrop);

    // calendar window
    let id = iced::window::Id::unique();
    app.ids.insert(id, WindowInfo::Calendar);

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

    let cal_settings = NewLayerShellSettings
    {
        layer:                   Layer::Overlay,
        size:                    Some((w, h)),
        exclusive_zone:          Some(0),
        keyboard_interactivity:  KeyboardInteractivity::Exclusive,
        anchor,
        margin:                  Some((pos_y, 0, 0, pos_x)),
        ..Default::default()
    };

    Task::batch([
        Task::done(Message::NewLayerShell { settings: backdrop_settings, id: backdrop_id }),
        Task::done(Message::NewLayerShell { settings: cal_settings, id })
    ])
}



pub fn calendar_view<'a>(app: &'a AppData) -> Element<'a, Message>
{
    let cfg  = &app.ron_config.calendar_window;
    let data = &app.modules_data.calendar_data;
    let nav_bar  = build_nav_bar(cfg, data);
    let grid     = build_grid(cfg, data);
    let spacing = cfg.calendar_nav_grid_spacing;

    let inner: Element<'_, Message> = match cfg.calendar_nav_position
    {
        CalendarNavPosition::Above => column![nav_bar, Space::new().height(spacing as f32), grid].align_x(Alignment::Center).into(),
        CalendarNavPosition::Below => column![grid, Space::new().height(spacing as f32), nav_bar].align_x(Alignment::Center).into(),
        CalendarNavPosition::Left => row![nav_bar, Space::new().width(spacing as f32), grid].align_y(Alignment::Center).into(),
        CalendarNavPosition::Right => row![grid, Space::new().width(spacing as f32), nav_bar].align_y(Alignment::Center).into()
    };

    container(inner)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .padding(cfg.calendar_padding)
        .style(move |_: &Theme|
        {
            iced::widget::container::Style
            {
                background:  Some(iced::Background::Color(cfg.calendar_background_color.to_iced_color())),
                border:      iced::Border
                {
                    color:   cfg.calendar_background_border_color.to_iced_color(),
                    width:   cfg.calendar_background_border_size,
                    radius:  Radius { top_left: cfg.calendar_background_border_radius[0], top_right: cfg.calendar_background_border_radius[1], bottom_left: cfg.calendar_background_border_radius[2], bottom_right: cfg.calendar_background_border_radius[3] }
                },
                ..Default::default()
            }
        })
        .into()
}



fn build_nav_bar<'a>(cfg: &'a CalendarWindowConfig, data: &'a CalendarData) -> Element<'a, Message>
{
    let spacing = cfg.calendar_nav_spacing;
    let spacing_y = cfg.calendar_nav_spacing_y;
    let prev_btn = make_arrow_btn(cfg, Message::CalendarPrev, true);
    let next_btn = make_arrow_btn(cfg, Message::CalendarNext, false);
    let month_btn  = make_nav_tab(cfg, data, "Month",  CalendarView::Month,  Message::CalendarSetView(CalendarView::Month));
    let year_btn   = make_nav_tab(cfg, data, "Year",   CalendarView::Year,   Message::CalendarSetView(CalendarView::Year));
    let decade_btn = make_nav_tab(cfg, data, "Decade", CalendarView::Decade, Message::CalendarSetView(CalendarView::Decade));
    let tabs: Element<'_, Message> = row![month_btn, year_btn, decade_btn].spacing(spacing as f32).align_y(Alignment::Center).into();

    column!
    [
        Space::new().height(spacing_y),
        row![prev_btn, Space::new().width(spacing as f32), tabs, Space::new().width(spacing as f32), next_btn].align_y(Alignment::Center)
    ].into()
}



fn make_nav_tab<'a>(cfg: &'a CalendarWindowConfig, data: &'a CalendarData, label: &'a str, view: CalendarView, msg: Message) -> Element<'a, Message>
{
    let is_active  = data.current_view == view;
    let style_cfg  = if is_active { &cfg.calendar_nav_active_button_style } else { &cfg.calendar_nav_button_style };
    let style_cfg2 = style_cfg.clone();

    button(
        text(label)
            .size(cfg.calendar_nav_text_size)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
    )
    .width(cfg.calendar_nav_button_width)
    .height(cfg.calendar_nav_button_height)
    .on_press(msg)
    .style(move |_: &Theme, status| style_cfg2.to_user_style(status))
    .into()
}



fn make_arrow_btn<'a>(cfg: &'a CalendarWindowConfig, msg: Message, is_prev: bool) -> Element<'a, Message>
{
    let label = if is_prev { cfg.calendar_prev_label.as_str() } else { cfg.calendar_next_label.as_str() };
    let style  = cfg.calendar_arrow_button_style.clone();

    button(
        text(label)
            .size(cfg.calendar_arrow_text_size)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
    )
    .width(cfg.calendar_arrow_button_width)
    .height(cfg.calendar_arrow_button_height)
    .on_press(msg)
    .style(move |_: &Theme, status| style.to_user_style(status))
    .into()
}



fn build_grid<'a>(cfg: &'a CalendarWindowConfig, data: &'a CalendarData) -> Element<'a, Message>
{
    match data.current_view
    {
        CalendarView::Month  => build_month_grid(cfg, data),
        CalendarView::Year   => build_year_grid(cfg, data),
        CalendarView::Decade => build_decade_grid(cfg, data)
    }
}



fn build_month_grid<'a>(cfg: &'a CalendarWindowConfig, data: &'a CalendarData) -> Element<'a, Message>
{
    let today         = Local::now().date_naive();
    let first_of_view = data.viewing_month;
    let first_wd      = first_weekday_offset(first_of_view, &cfg.calendar_first_week_day);
    let days_in_month = days_in_month(first_of_view.year(), first_of_view.month());
    let spacing       = cfg.calendar_grid_spacing;

    let mut rows: Vec<Element<'_, Message>> = Vec::new();

    if cfg.calendar_show_weekday_header
    {
        let ordered_labels = ordered_weekday_labels(cfg);
        let mut header_cells: Vec<Element<'_, Message>> = Vec::new();

        if cfg.calendar_show_week_numbers
        {
            let wk_color = cfg.calendar_week_number_text_color;
            header_cells.push(
                container(text("Wk").size(cfg.calendar_week_number_text_size).color(wk_color.to_iced_color()))
                    .width(cfg.calendar_day_cell_width).height(cfg.calendar_day_cell_height)
                    .align_x(Alignment::Center).align_y(Alignment::Center)
                    .into()
            );
        }

        for lbl in ordered_labels
        {
            let hdr_color = cfg.calendar_weekday_header_text_color;
            header_cells.push(
                container(text(lbl).size(cfg.calendar_weekday_header_text_size).color(hdr_color.to_iced_color()))
                    .width(cfg.calendar_day_cell_width).height(cfg.calendar_day_cell_height)
                    .align_x(Alignment::Center).align_y(Alignment::Center)
                    .into()
            );
        }
        rows.push(row(header_cells).spacing(spacing as f32).into());
    }

    let mut cell_index: i32 = 0;
    let mut week_row_cells: Vec<Element<'_, Message>> = Vec::new();

    let prev_month_days = prev_month_day_count(first_of_view.year(), first_of_view.month());

    while cell_index < cfg.calendar_total_day_cells
    {
        if cell_index % 7 == 0
        {
            if !week_row_cells.is_empty()
            {
                rows.push(row(std::mem::take(&mut week_row_cells)).spacing(spacing as f32).into());
            }

            if cfg.calendar_show_week_numbers
            {
                let day_offset = cell_index - first_wd;
                let wk_date = if day_offset < 0
                {
                    let prev_day = prev_month_days as i32 + day_offset + 1;
                    let prev_m   = if first_of_view.month() == 1 { 12 } else { first_of_view.month() - 1 };
                    let prev_y   = if first_of_view.month() == 1 { first_of_view.year() - 1 } else { first_of_view.year() };
                    NaiveDate::from_ymd_opt(prev_y, prev_m, prev_day.max(1) as u32).unwrap_or(first_of_view)
                }
                else if day_offset < days_in_month as i32
                {
                    NaiveDate::from_ymd_opt(first_of_view.year(), first_of_view.month(), (day_offset + 1) as u32).unwrap_or(first_of_view)
                }
                else
                {
                    let overflow = day_offset - days_in_month as i32 + 1;
                    let next_m   = if first_of_view.month() == 12 { 1 } else { first_of_view.month() + 1 };
                    let next_y   = if first_of_view.month() == 12 { first_of_view.year() + 1 } else { first_of_view.year() };
                    NaiveDate::from_ymd_opt(next_y, next_m, overflow as u32).unwrap_or(first_of_view)
                };

                let wk_num    = wk_date.iso_week().week();
                let wk_color2 = cfg.calendar_week_number_text_color;
                week_row_cells.push(
                    container(text(format!("{wk_num}")).size(cfg.calendar_week_number_text_size).color(wk_color2.to_iced_color()))
                        .width(cfg.calendar_day_cell_width).height(cfg.calendar_day_cell_height)
                        .align_x(Alignment::Center).align_y(Alignment::Center)
                        .into()
                );
            }
        }

        let day_offset = cell_index - first_wd;

        let cell: Element<'_, Message> = if day_offset < 0
        {
            if cfg.calendar_show_overflow_days_prev_month
            {
                let d     = prev_month_days as i32 + day_offset + 1;
                let label = format!("{d}");
                let style = cfg.calendar_overflow_day_button_style.clone();
                make_day_cell(cfg, &label, None, style, Message::Nothing)
            }
            else
            {
                Space::new().width(cfg.calendar_day_cell_width).height(cfg.calendar_day_cell_height).into()
            }
        }
        else if day_offset < days_in_month as i32
        {
            let day_num  = (day_offset + 1) as u32;
            let this_day = NaiveDate::from_ymd_opt(first_of_view.year(), first_of_view.month(), day_num);
            let label    = format!("{day_num}");

            let is_today    = this_day.map(|d| d == today).unwrap_or(false);
            let is_selected = data.selected_day == this_day;

            let style = if is_selected
            {
                cfg.calendar_selected_day_button_style.clone()
            }
            else if is_today
            {
                cfg.calendar_today_button_style.clone()
            }
            else
            {
                cfg.calendar_day_button_style.clone()
            };

            let msg = Message::CalendarDaySelected(day_num);
            make_day_cell(cfg, &label, this_day, style, msg)
        }
        else
        {
            if cfg.calendar_show_overflow_days_next_month
            {
                let d     = day_offset - days_in_month as i32 + 1;
                let label = format!("{d}");
                let style = cfg.calendar_overflow_day_button_style.clone();
                make_day_cell(cfg, &label, None, style, Message::Nothing)
            }
            else
            {
                Space::new().width(cfg.calendar_day_cell_width).height(cfg.calendar_day_cell_height).into()
            }
        };

        week_row_cells.push(cell);
        cell_index += 1;
    }

    if !week_row_cells.is_empty()
    {
        rows.push(row(week_row_cells).spacing(spacing as f32).into());
    }

    column!
    [
        Space::new().height(cfg.calendar_month_spacing_y),
        column(rows).spacing(spacing as f32).align_x(Alignment::Center)
    ].into()
}



fn make_day_cell<'a>(cfg: &CalendarWindowConfig, label: &str, _date: Option<NaiveDate>, style: CalendarButtonStyle, msg: Message) -> Element<'a, Message>
{
    let owned = label.to_string();
    button
    (
        text(owned)
            .size(cfg.calendar_day_text_size)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
    )
    .width(cfg.calendar_day_cell_width)
    .height(cfg.calendar_day_cell_height)
    .on_press(msg)
    .style(move |_: &Theme, status| style.to_user_style(status))
    .into()
}



fn build_year_grid<'a>(cfg: &'a CalendarWindowConfig, data: &'a CalendarData) -> Element<'a, Message>
{
    let today_month  = Local::now().date_naive().month() as usize;
    let today_year   = Local::now().date_naive().year();
    let cols         = cfg.calendar_year_grid_columns.max(1) as usize;
    let spacing      = cfg.calendar_year_grid_spacing;

    let labels = if cfg.calendar_month_labels.len() >= 12
    {
        cfg.calendar_month_labels.clone()
    }
    else
    {
        vec!["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"]
            .into_iter().map(String::from).collect()
    };

    let mut rows: Vec<Element<'_, Message>> = Vec::new();
    let mut row_cells: Vec<Element<'_, Message>> = Vec::new();

    for (i, lbl) in labels.iter().enumerate().take(12)
    {
        let month   = (i + 1) as u32;
        let is_curr = data.viewing_year == today_year && month == today_month as u32;
        let style   = if is_curr { cfg.calendar_current_month_button_style.clone() } else { cfg.calendar_month_button_style.clone() };
        let msg     = Message::CalendarMonthSelected(month);
        let owned   = lbl.clone();

        row_cells.push
        (
            button
            (
                text(owned)
                    .size(cfg.calendar_month_text_size)
                    .width(Length::Fill).height(Length::Fill)
                    .align_x(Alignment::Center).align_y(Alignment::Center)
            )
            .width(cfg.calendar_month_cell_width)
            .height(cfg.calendar_month_cell_height)
            .on_press(msg)
            .style(move |_: &Theme, status| style.to_user_style(status))
            .into()
        );

        if row_cells.len() == cols
        {
            rows.push(row(std::mem::take(&mut row_cells)).spacing(spacing as f32).into());
        }
    }
    if !row_cells.is_empty()
    {
        rows.push(row(row_cells).spacing(spacing as f32).into());
    }

    column!
    [
        Space::new().height(cfg.calendar_year_spacing_y),
        column(rows).spacing(spacing as f32).align_x(Alignment::Center)
    ].into()
}



fn build_decade_grid<'a>(cfg: &'a CalendarWindowConfig, data: &'a CalendarData) -> Element<'a, Message>
{
    let today_year = Local::now().date_naive().year();
    let cols       = cfg.calendar_decade_grid_columns.max(1) as usize;
    let spacing    = cfg.calendar_decade_grid_spacing;

    let mut rows: Vec<Element<'_, Message>> = Vec::new();
    let mut row_cells: Vec<Element<'_, Message>> = Vec::new();

    let start = data.viewing_decade - 1;
    for i in 0..12i32
    {
        let year    = start + i;
        let is_curr = year == today_year;
        let style   = if is_curr { cfg.calendar_current_year_button_style.clone() } else { cfg.calendar_year_button_style.clone() };
        let msg     = Message::CalendarYearSelected(year);

        row_cells.push
        (
            button
            (
                text(format!("{year}"))
                    .size(cfg.calendar_year_text_size)
                    .width(Length::Fill).height(Length::Fill)
                    .align_x(Alignment::Center).align_y(Alignment::Center)
            )
            .width(cfg.calendar_year_cell_width)
            .height(cfg.calendar_year_cell_height)
            .on_press(msg)
            .style(move |_: &Theme, status| style.to_user_style(status))
            .into()
        );

        if row_cells.len() == cols
        {
            rows.push(row(std::mem::take(&mut row_cells)).spacing(spacing as f32).into());
        }
    }
    if !row_cells.is_empty()
    {
        rows.push(row(row_cells).spacing(spacing as f32).into());
    }

    column!
    [
        Space::new().height(cfg.calendar_decade_spacing_y),
        column(rows).spacing(spacing as f32).align_x(Alignment::Center)
    ].into()
}



fn first_weekday_offset(first: NaiveDate, fwd: &FirstWeekDay) -> i32
{
    use chrono::Weekday;
    let iso_wd = first.weekday();
    let col0_iso = match fwd
    {
        FirstWeekDay::Monday   => Weekday::Mon,
        FirstWeekDay::Sunday   => Weekday::Sun,
        FirstWeekDay::Saturday => Weekday::Sat
    };
    let iso_num = |wd: Weekday| -> i32 { wd.num_days_from_monday() as i32 };
    (iso_num(iso_wd) - iso_num(col0_iso)).rem_euclid(7)
}



fn ordered_weekday_labels(cfg: &CalendarWindowConfig) -> Vec<String>
{
    let base: Vec<String> = if cfg.calendar_weekday_labels.len() >= 7
    {
        cfg.calendar_weekday_labels[..7].to_vec()
    }
    else
    {
        vec!["Mo","Tu","We","Th","Fr","Sa","Su"].into_iter().map(String::from).collect()
    };

    let rotate_by = match cfg.calendar_first_week_day
    {
        FirstWeekDay::Monday   => 0,
        FirstWeekDay::Sunday   => 1,
        FirstWeekDay::Saturday => 2
    };
    let mut out = base.clone();
    out.rotate_right(rotate_by);
    out
}



fn days_in_month(year: i32, month: u32) -> u32
{
    let next_m = if month == 12 { 1 } else { month + 1 };
    let next_y = if month == 12 { year + 1 } else { year };
    NaiveDate::from_ymd_opt(next_y, next_m, 1)
        .and_then(|d| d.pred_opt())
        .map(|d| d.day())
        .unwrap_or(30)
}



fn prev_month_day_count(year: i32, month: u32) -> u32
{
    let (py, pm) = if month == 1 { (year - 1, 12u32) } else { (year, month - 1) };
    days_in_month(py, pm)
}





// ============ TESTS ============
#[cfg(test)]
mod tests
{
    use super::*;
    use chrono::NaiveDate;

    fn jan_1_2024() -> NaiveDate { NaiveDate::from_ymd_opt(2024, 1, 1).unwrap() }
    fn mar_1_2024() -> NaiveDate { NaiveDate::from_ymd_opt(2024, 3, 1).unwrap() }
    fn feb_1_2024() -> NaiveDate { NaiveDate::from_ymd_opt(2024, 2, 1).unwrap() }
    fn dec_1_2023() -> NaiveDate { NaiveDate::from_ymd_opt(2023, 12, 1).unwrap() }
    fn feb_1_2023() -> NaiveDate { NaiveDate::from_ymd_opt(2023, 2, 1).unwrap() }


    #[test]
    fn days_in_month_january_is_31()
    {
        assert_eq!(days_in_month(2024, 1), 31);
    }

    #[test]
    fn days_in_month_april_is_30()
    {
        assert_eq!(days_in_month(2024, 4), 30);
    }

    #[test]
    fn days_in_month_june_is_30()
    {
        assert_eq!(days_in_month(2024, 6), 30);
    }

    #[test]
    fn days_in_month_september_is_30()
    {
        assert_eq!(days_in_month(2024, 9), 30);
    }

    #[test]
    fn days_in_month_november_is_30()
    {
        assert_eq!(days_in_month(2024, 11), 30);
    }

    #[test]
    fn days_in_month_december_is_31()
    {
        assert_eq!(days_in_month(2024, 12), 31);
    }

    #[test]
    fn days_in_month_february_leap_year_is_29()
    {
        assert_eq!(days_in_month(2024, 2), 29);
    }

    #[test]
    fn days_in_month_february_non_leap_year_is_28()
    {
        assert_eq!(days_in_month(2023, 2), 28);
    }

    #[test]
    fn days_in_month_february_century_non_leap_is_28()
    {
        assert_eq!(days_in_month(1900, 2), 28);
    }

    #[test]
    fn days_in_month_february_400_year_leap_is_29()
    {
        assert_eq!(days_in_month(2000, 2), 29);
    }

    #[test]
    fn days_in_month_december_rolls_into_next_year_correctly()
    {
        assert_eq!(days_in_month(2023, 12), 31);
    }

    #[test]
    fn days_in_month_all_31_day_months()
    {
        for m in [1u32, 3, 5, 7, 8, 10, 12]
        {
            assert_eq!(days_in_month(2024, m), 31, "month {m} should have 31 days");
        }
    }

    #[test]
    fn days_in_month_all_30_day_months()
    {
        for m in [4u32, 6, 9, 11]
        {
            assert_eq!(days_in_month(2024, m), 30, "month {m} should have 30 days");
        }
    }


    #[test]
    fn prev_month_day_count_january_wraps_to_december()
    {
        assert_eq!(prev_month_day_count(2024, 1), 31);
    }

    #[test]
    fn prev_month_day_count_march_returns_feb_days_leap()
    {
        assert_eq!(prev_month_day_count(2024, 3), 29);
    }

    #[test]
    fn prev_month_day_count_march_returns_feb_days_non_leap()
    {
        assert_eq!(prev_month_day_count(2023, 3), 28);
    }

    #[test]
    fn prev_month_day_count_may_returns_april_30()
    {
        assert_eq!(prev_month_day_count(2024, 5), 30);
    }

    #[test]
    fn prev_month_day_count_june_returns_may_31()
    {
        assert_eq!(prev_month_day_count(2024, 6), 31);
    }

    #[test]
    fn prev_month_day_count_december_returns_november_30()
    {
        assert_eq!(prev_month_day_count(2024, 12), 30);
    }


    #[test]
    fn first_weekday_offset_monday_start_jan_2024_is_0()
    {
        let d = jan_1_2024();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 0);
    }

    #[test]
    fn first_weekday_offset_sunday_start_jan_2024()
    {
        let d = jan_1_2024();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Sunday), 1);
    }

    #[test]
    fn first_weekday_offset_saturday_start_jan_2024()
    {
        let d = jan_1_2024();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Saturday), 2);
    }

    #[test]
    fn first_weekday_offset_monday_start_mar_2024()
    {
        let d = mar_1_2024();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 4);
    }

    #[test]
    fn first_weekday_offset_sunday_start_mar_2024()
    {
        let d = mar_1_2024();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Sunday), 5);
    }

    #[test]
    fn first_weekday_offset_saturday_start_mar_2024()
    {
        let d = mar_1_2024();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Saturday), 6);
    }

    #[test]
    fn first_weekday_offset_result_always_in_0_to_6()
    {
        let dates = [
            jan_1_2024(), feb_1_2024(), mar_1_2024(), dec_1_2023(), feb_1_2023(),
            NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
            NaiveDate::from_ymd_opt(2020, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        ];
        let fwds = [FirstWeekDay::Monday, FirstWeekDay::Sunday, FirstWeekDay::Saturday];
        for d in dates
        {
            for fwd in &fwds
            {
                let off = first_weekday_offset(d, fwd);
                assert!((0..7).contains(&off), "offset {off} out of range for {d} / {fwd:?}");
            }
        }
    }

    #[test]
    fn first_weekday_offset_monday_start_sunday_month_start_gives_6()
    {
        let d = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 6);
    }

    #[test]
    fn first_weekday_offset_monday_start_saturday_month_start()
    {
        let d = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 5);
    }

    #[test]
    fn first_weekday_offset_monday_start_tuesday_month_start()
    {
        let d = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 1);
    }

    #[test]
    fn first_weekday_offset_monday_start_wednesday_month_start()
    {
        let d = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 2);
    }

    #[test]
    fn first_weekday_offset_monday_start_thursday_month_start()
    {
        let d = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 3);
    }

    #[test]
    fn first_weekday_offset_monday_start_friday_month_start()
    {
        let d = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 4);
    }


    #[test]
    fn ordered_weekday_labels_monday_start_first_is_mo()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Monday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels[0], "Mo");
    }

    #[test]
    fn ordered_weekday_labels_monday_start_last_is_su()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Monday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels[6], "Su");
    }

    #[test]
    fn ordered_weekday_labels_sunday_start_first_is_su()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Sunday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels[0], "Su");
    }

    #[test]
    fn ordered_weekday_labels_sunday_start_last_is_sa()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Sunday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels[6], "Sa");
    }

    #[test]
    fn ordered_weekday_labels_saturday_start_first_is_sa()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Saturday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels[0], "Sa");
    }

    #[test]
    fn ordered_weekday_labels_saturday_start_last_is_fr()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Saturday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels[6], "Fr");
    }

    #[test]
    fn ordered_weekday_labels_always_returns_7_items()
    {
        for fwd in [FirstWeekDay::Monday, FirstWeekDay::Sunday, FirstWeekDay::Saturday]
        {
            let mut cfg = CalendarWindowConfig::default();
            cfg.calendar_first_week_day = fwd;
            assert_eq!(ordered_weekday_labels(&cfg).len(), 7);
        }
    }

    #[test]
    fn ordered_weekday_labels_contains_all_days_exactly_once()
    {
        for fwd in [FirstWeekDay::Monday, FirstWeekDay::Sunday, FirstWeekDay::Saturday]
        {
            let mut cfg = CalendarWindowConfig::default();
            cfg.calendar_first_week_day = fwd;
            let labels = ordered_weekday_labels(&cfg);
            let expected = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
            for day in expected
            {
                let count = labels.iter().filter(|l| l.as_str() == day).count();
                assert_eq!(count, 1, "day {day} should appear exactly once");
            }
        }
    }

    #[test]
    fn ordered_weekday_labels_monday_full_sequence()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Monday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels, vec!["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"]);
    }

    #[test]
    fn ordered_weekday_labels_sunday_full_sequence()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Sunday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels, vec!["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]);
    }

    #[test]
    fn ordered_weekday_labels_saturday_full_sequence()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Saturday;
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels, vec!["Sa", "Su", "Mo", "Tu", "We", "Th", "Fr"]);
    }

    #[test]
    fn ordered_weekday_labels_uses_custom_labels_when_provided()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_first_week_day = FirstWeekDay::Monday;
        cfg.calendar_weekday_labels = vec![
            "Lun".to_string(), "Mar".to_string(), "Mié".to_string(),
            "Jue".to_string(), "Vie".to_string(), "Sáb".to_string(), "Dom".to_string(),
        ];
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels[0], "Lun");
        assert_eq!(labels[6], "Dom");
    }

    #[test]
    fn ordered_weekday_labels_falls_back_to_defaults_when_too_few_custom()
    {
        let mut cfg = CalendarWindowConfig::default();
        cfg.calendar_weekday_labels = vec!["Mo".to_string(), "Tu".to_string()];
        let labels = ordered_weekday_labels(&cfg);
        assert_eq!(labels.len(), 7);
    }


    #[test]
    fn calendar_data_default_is_closed()
    {
        let data = CalendarData::default();
        assert!(!data.is_open);
    }

    #[test]
    fn calendar_data_default_cursor_not_inside()
    {
        let data = CalendarData::default();
        assert!(!data.cursor_inside);
    }

    #[test]
    fn calendar_data_default_view_is_month()
    {
        let data = CalendarData::default();
        assert_eq!(data.current_view, CalendarView::Month);
    }

    #[test]
    fn calendar_data_default_viewing_month_is_first_day_of_month()
    {
        let data = CalendarData::default();
        assert_eq!(data.viewing_month.day(), 1);
    }

    #[test]
    fn calendar_data_default_selected_day_is_none()
    {
        let data = CalendarData::default();
        assert!(data.selected_day.is_none());
    }

    #[test]
    fn calendar_data_default_mouse_pos_is_origin()
    {
        let data = CalendarData::default();
        assert_eq!(data.mouse_pos, (0, 0));
    }

    #[test]
    fn calendar_data_default_viewing_year_matches_current_year()
    {
        let data = CalendarData::default();
        let today_year = Local::now().date_naive().year();
        assert_eq!(data.viewing_year, today_year);
    }

    #[test]
    fn calendar_data_default_viewing_decade_is_decade_start()
    {
        let data = CalendarData::default();
        let today_year = Local::now().date_naive().year();
        let expected_decade = (today_year / 10) * 10;
        assert_eq!(data.viewing_decade, expected_decade);
        assert_eq!(data.viewing_decade % 10, 0);
    }

    #[test]
    fn calendar_data_default_viewing_month_year_matches_current()
    {
        let data = CalendarData::default();
        let today = Local::now().date_naive();
        assert_eq!(data.viewing_month.year(), today.year());
        assert_eq!(data.viewing_month.month(), today.month());
    }


    #[test]
    fn calendar_window_config_default_window_size_is_non_zero()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_window_size[0] > 0);
        assert!(cfg.calendar_window_size[1] > 0);
    }

    #[test]
    fn calendar_window_config_default_total_day_cells_is_35_or_42()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_total_day_cells == 35 || cfg.calendar_total_day_cells == 42);
    }

    #[test]
    fn calendar_window_config_default_nav_position_is_above()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_nav_position, CalendarNavPosition::Above);
    }

    #[test]
    fn calendar_window_config_default_first_week_day_is_monday()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_first_week_day, FirstWeekDay::Monday);
    }

    #[test]
    fn calendar_window_config_default_weekday_labels_has_7_entries()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_weekday_labels.len(), 7);
    }

    #[test]
    fn calendar_window_config_default_month_labels_has_12_entries()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_month_labels.len(), 12);
    }

    #[test]
    fn calendar_window_config_default_prev_label_is_not_empty()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(!cfg.calendar_prev_label.is_empty());
    }

    #[test]
    fn calendar_window_config_default_next_label_is_not_empty()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(!cfg.calendar_next_label.is_empty());
    }

    #[test]
    fn calendar_window_config_default_day_click_action_is_highlight_only()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_day_click_action, DayClickAction::HighlightOnly);
    }

    #[test]
    fn calendar_window_config_default_show_weekday_header_is_true()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_show_weekday_header);
    }

    #[test]
    fn calendar_window_config_default_show_overflow_days_prev_is_true()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_show_overflow_days_prev_month);
    }

    #[test]
    fn calendar_window_config_default_show_overflow_days_next_is_true()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_show_overflow_days_next_month);
    }

    #[test]
    fn calendar_window_config_default_show_week_numbers_is_false()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(!cfg.calendar_show_week_numbers);
    }

    #[test]
    fn calendar_window_config_default_padding_is_nonzero()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_padding > 0);
    }

    #[test]
    fn calendar_window_config_default_day_cell_dimensions_nonzero()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_day_cell_width > 0);
        assert!(cfg.calendar_day_cell_height > 0);
    }

    #[test]
    fn calendar_window_config_default_nav_button_dimensions_nonzero()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_nav_button_width > 0);
        assert!(cfg.calendar_nav_button_height > 0);
    }

    #[test]
    fn calendar_window_config_default_arrow_button_dimensions_nonzero()
    {
        let cfg = CalendarWindowConfig::default();
        assert!(cfg.calendar_arrow_button_width > 0);
        assert!(cfg.calendar_arrow_button_height > 0);
    }

    #[test]
    fn calendar_window_config_default_year_grid_columns_is_3()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_year_grid_columns, 3);
    }

    #[test]
    fn calendar_window_config_default_decade_grid_columns_is_3()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_decade_grid_columns, 3);
    }


    #[test]
    fn calendar_button_style_default_border_size_is_positive()
    {
        let style = CalendarButtonStyle::default();
        assert!(style.border_size > 0.0);
    }

    #[test]
    fn calendar_button_style_default_border_radius_has_4_values()
    {
        let style = CalendarButtonStyle::default();
        assert_eq!(style.border_radius.len(), 4);
    }

    #[test]
    fn calendar_button_style_default_border_radius_all_equal()
    {
        let style = CalendarButtonStyle::default();
        assert_eq!(style.border_radius[0], style.border_radius[1]);
        assert_eq!(style.border_radius[1], style.border_radius[2]);
        assert_eq!(style.border_radius[2], style.border_radius[3]);
    }


    #[test]
    fn calendar_view_default_is_month()
    {
        assert_eq!(CalendarView::default(), CalendarView::Month);
    }

    #[test]
    fn calendar_view_variants_are_distinct()
    {
        assert_ne!(CalendarView::Month, CalendarView::Year);
        assert_ne!(CalendarView::Year, CalendarView::Decade);
        assert_ne!(CalendarView::Month, CalendarView::Decade);
    }

    #[test]
    fn calendar_view_clone_equals_original()
    {
        let v = CalendarView::Year;
        assert_eq!(v, v.clone());
    }


    #[test]
    fn calendar_nav_position_default_is_above()
    {
        assert_eq!(CalendarNavPosition::default(), CalendarNavPosition::Above);
    }

    #[test]
    fn calendar_nav_position_variants_are_distinct()
    {
        assert_ne!(CalendarNavPosition::Above, CalendarNavPosition::Below);
        assert_ne!(CalendarNavPosition::Left,  CalendarNavPosition::Right);
        assert_ne!(CalendarNavPosition::Above, CalendarNavPosition::Left);
    }


    #[test]
    fn first_week_day_default_is_monday()
    {
        assert_eq!(FirstWeekDay::default(), FirstWeekDay::Monday);
    }

    #[test]
    fn first_week_day_variants_are_distinct()
    {
        assert_ne!(FirstWeekDay::Monday,   FirstWeekDay::Sunday);
        assert_ne!(FirstWeekDay::Sunday,   FirstWeekDay::Saturday);
        assert_ne!(FirstWeekDay::Monday,   FirstWeekDay::Saturday);
    }


    #[test]
    fn day_click_action_default_is_highlight_only()
    {
        assert_eq!(DayClickAction::default(), DayClickAction::HighlightOnly);
    }

    #[test]
    fn day_click_action_custom_action_stores_commands()
    {
        let action = DayClickAction::CustomAction(vec!["notify-send".to_string(), "{date}".to_string()]);
        match action
        {
            DayClickAction::CustomAction(cmds) =>
            {
                assert_eq!(cmds[0], "notify-send");
                assert_eq!(cmds[1], "{date}");
            }
            _ => panic!("expected CustomAction")
        }
    }

    #[test]
    fn day_click_action_highlight_only_equals_itself()
    {
        assert_eq!(DayClickAction::HighlightOnly, DayClickAction::HighlightOnly);
    }

    #[test]
    fn day_click_action_custom_action_not_equal_to_highlight_only()
    {
        let action = DayClickAction::CustomAction(vec![]);
        assert_ne!(action, DayClickAction::HighlightOnly);
    }


    #[test]
    fn decade_grid_starts_one_before_decade()
    {
        let data = CalendarData { viewing_decade: 2020, ..Default::default() };
        let start = data.viewing_decade - 1;
        assert_eq!(start, 2019);
    }

    #[test]
    fn decade_grid_covers_12_years()
    {
        let data = CalendarData { viewing_decade: 2020, ..Default::default() };
        let start = data.viewing_decade - 1;
        let years: Vec<i32> = (0..12).map(|i| start + i).collect();
        assert_eq!(years.len(), 12);
        assert_eq!(years.first(), Some(&2019));
        assert_eq!(years.last(), Some(&2030));
    }

    #[test]
    fn decade_grid_includes_today_year_for_current_decade()
    {
        let today_year = Local::now().date_naive().year();
        let decade = (today_year / 10) * 10;
        let data = CalendarData { viewing_decade: decade, ..Default::default() };
        let start = data.viewing_decade - 1;
        let years: Vec<i32> = (0..12).map(|i| start + i).collect();
        assert!(years.contains(&today_year));
    }


    #[test]
    fn prev_overflow_day_label_for_jan_2024_offset_minus_1()
    {
        let prev_days = prev_month_day_count(2024, 1);
        assert_eq!(prev_days, 31);
        let d = prev_days as i32 + (-1) + 1;
        assert_eq!(d, 31);
    }

    #[test]
    fn prev_overflow_day_label_for_jan_2024_offset_minus_2()
    {
        let prev_days = prev_month_day_count(2024, 1);
        let d = prev_days as i32 + (-2) + 1;
        assert_eq!(d, 30);
    }

    #[test]
    fn next_overflow_day_label_first_day_after_month()
    {
        let dim = days_in_month(2024, 1);
        let day_offset = dim as i32;
        let d = day_offset - dim as i32 + 1;
        assert_eq!(d, 1);
    }

    #[test]
    fn next_overflow_day_label_second_day_after_month()
    {
        let dim = days_in_month(2024, 1);
        let day_offset = dim as i32 + 1;
        let d = day_offset - dim as i32 + 1;
        assert_eq!(d, 2);
    }


    #[test]
    fn month_labels_default_january_is_jan()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_month_labels[0], "Jan");
    }

    #[test]
    fn month_labels_default_december_is_dec()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_month_labels[11], "Dec");
    }

    #[test]
    fn month_labels_default_june_is_jun()
    {
        let cfg = CalendarWindowConfig::default();
        assert_eq!(cfg.calendar_month_labels[5], "Jun");
    }


    #[test]
    fn first_weekday_offset_sunday_month_start_monday_grid_gives_6()
    {
        let d = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 6);
    }

    #[test]
    fn first_weekday_offset_wednesday_month_start_monday_grid()
    {
        let d = NaiveDate::from_ymd_opt(2023, 3, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Monday), 2);
    }

    #[test]
    fn first_weekday_offset_friday_month_start_sunday_grid()
    {
        let d = NaiveDate::from_ymd_opt(2024, 11, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Sunday), 5);
    }

    #[test]
    fn first_weekday_offset_saturday_month_start_saturday_grid_gives_0()
    {
        let d = NaiveDate::from_ymd_opt(2023, 4, 1).unwrap();
        assert_eq!(first_weekday_offset(d, &FirstWeekDay::Saturday), 0);
    }


    #[test]
    fn day_click_action_highlight_only_serializes_and_deserializes()
    {
        let original = DayClickAction::HighlightOnly;
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: DayClickAction = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn day_click_action_custom_action_serializes_and_deserializes()
    {
        let original = DayClickAction::CustomAction(vec!["echo".to_string(), "{date}".to_string()]);
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: DayClickAction = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn calendar_view_serializes_and_deserializes_month()
    {
        let original = CalendarView::Month;
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: CalendarView = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn calendar_view_serializes_and_deserializes_year()
    {
        let original = CalendarView::Year;
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: CalendarView = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn calendar_view_serializes_and_deserializes_decade()
    {
        let original = CalendarView::Decade;
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: CalendarView = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn first_week_day_serializes_and_deserializes_monday()
    {
        let original = FirstWeekDay::Monday;
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: FirstWeekDay = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn first_week_day_serializes_and_deserializes_sunday()
    {
        let original = FirstWeekDay::Sunday;
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: FirstWeekDay = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn first_week_day_serializes_and_deserializes_saturday()
    {
        let original = FirstWeekDay::Saturday;
        let serialized = ron::to_string(&original).expect("serialize failed");
        let deserialized: FirstWeekDay = ron::from_str(&serialized).expect("deserialize failed");
        assert_eq!(original, deserialized);
    }

    #[test]
    fn calendar_nav_position_serializes_and_deserializes_all_variants()
    {
        for variant in [CalendarNavPosition::Above, CalendarNavPosition::Below, CalendarNavPosition::Left, CalendarNavPosition::Right]
        {
            let serialized = ron::to_string(&variant).expect("serialize failed");
            let deserialized: CalendarNavPosition = ron::from_str(&serialized).expect("deserialize failed");
            assert_eq!(variant, deserialized);
        }
    }
}
