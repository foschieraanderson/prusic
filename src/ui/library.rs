use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

const ACCENT: Color = Color::Rgb(150, 110, 255);
const ACCENT_LIGHT: Color = Color::Rgb(200, 180, 255);
const MUTED: Color = Color::Rgb(100, 100, 115);
const TEXT: Color = Color::Rgb(235, 235, 240);
const BAR: Color = Color::Rgb(55, 55, 65);

use crate::app::App;

pub fn render_library(frame: &mut Frame, area: Rect, app: &App) -> Option<Rect> {
    let _ = app;
    let [_, player, library, _] = if area.width >= 100 {
        Layout::horizontal([
            Constraint::Length(1),
            Constraint::Percentage(25),
            Constraint::Percentage(75),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .areas(area)
    } else {
        Layout::vertical([
            Constraint::Length(1),
            Constraint::Percentage(40),
            Constraint::Percentage(60),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .spacing(1)
        .areas(area)
    };

    // let [cover, title, artist, progress, controls] = Layout::vertical([
    //     Constraint::Length(12),
    //     Constraint::Length(1),
    //     Constraint::Length(1),
    //     Constraint::Length(3),
    //     Constraint::Length(3),
    // ])
    // .flex(Flex::Center)
    // .spacing(1)
    // .areas(player);

    frame.render_widget(ratatui::widgets::Paragraph::new("Library"), library);

    Some(player)
}
