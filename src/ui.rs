use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(frame, layout[0]);
    render_library(frame, layout[1], app);
    render_footer(frame, layout[2]);
}

fn render_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    let header = Paragraph::new("Rust Music Player").block(
        Block::default()
            .title(" Music Player ")
            .borders(Borders::ALL),
    );

    frame.render_widget(header, area);
}

fn render_library(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let items: Vec<ListItem> = app
        .songs
        .iter()
        .map(|song| {
            let text = format!("{} - {}", song.title, song.artist);

            ListItem::new(text)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" Library ").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut state = ratatui::widgets::ListState::default();

    state.select(Some(app.selected));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let footer =
        Paragraph::new("↑↓ Navigate    q Quit").block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
