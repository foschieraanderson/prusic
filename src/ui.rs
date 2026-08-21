use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
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

    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(36), Constraint::Min(0)])
        .split(layout[1]);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(6)])
        .split(content[1]);

    render_header(frame, layout[0]);

    render_cover(frame, content[0], app);

    render_library(frame, right[0], app);

    //render_song_info(frame, right[1], app);

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

fn render_cover(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let Some(song) = app.selected_song() else {
        return;
    };

    let Some(cover) = &song.cover else {
        let paragraph = Paragraph::new("No cover")
            .block(Block::default().title(" Album Art ").borders(Borders::ALL));

        frame.render_widget(paragraph, area);

        return;
    };

    let Ok(image) = crate::cover::decode(&cover.data) else {
        let paragraph = Paragraph::new("Invalid cover")
            .block(Block::default().title(" Album Art ").borders(Borders::ALL));

        frame.render_widget(paragraph, area);

        return;
    };

    let width = area.width.saturating_sub(2) as u32;

    let height = area.height.saturating_sub(2) as u32 * 2;

    let lines = crate::cover::to_ascii(&image, width, height);

    let lines = lines.into_iter().map(Line::from).collect::<Vec<_>>();

    let paragraph =
        Paragraph::new(lines).block(Block::default().title(" Album Art ").borders(Borders::ALL));

    frame.render_widget(paragraph, area);
}
