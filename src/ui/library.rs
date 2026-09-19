use crate::{
    helpers::format_duration, library::LibraryNode, playlist::RepeatMode, ui::player::render_cover,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph, Widget},
};
use unicode_width::UnicodeWidthStr;

const ACCENT: Color = Color::Rgb(150, 110, 255);
const ACCENT_LIGHT: Color = Color::Rgb(200, 180, 255);
const MUTED: Color = Color::Rgb(100, 100, 115);
const TEXT: Color = Color::Rgb(235, 235, 240);
const BAR: Color = Color::Rgb(55, 55, 65);

use crate::app::App;

pub fn render_library(frame: &mut Frame, area: Rect, app: &mut App) -> Option<Rect> {
    let _ = app;
    let [_, player, library, _] = if area.width >= 100 {
        Layout::horizontal([
            Constraint::Length(1),
            Constraint::Percentage(25),
            Constraint::Percentage(75),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .spacing(1)
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

    let cover = render_player(frame, player, app);
    render_tree(frame, library, app);

    cover
}

fn render_player(frame: &mut Frame, area: Rect, app: &App) -> Option<Rect> {
    let [_, cover, player_info, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Percentage(40),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .flex(Flex::Center)
    .spacing(1)
    .areas(area);

    render_cover(frame, cover, app);
    render_player_info(frame, player_info, app);
    // render_title(frame, title, app);
    // render_artist(frame, artist, app);
    // render_progress(frame, progress, app);

    Some(cover)
}

fn render_player_info(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = Vec::new();

    if let Some(track) = &app.current_track {
        if !track.title.is_empty() && track.title != "Unknown" {
            let title = Line::from(Span::styled(
                app.current_track
                    .as_ref()
                    .map_or("Unknown", |track| track.title.as_str()),
                Style::default().fg(TEXT),
            ));
            lines.push(title);
        }

        // Artist
        if !track.artist.is_empty() && track.artist != "Unknown" {
            let artist = Line::from(Span::styled(
                app.current_track
                    .as_ref()
                    .map_or("Unknown", |track| track.artist.as_str()),
                Style::default().fg(TEXT),
            ));

            lines.push(artist);
        }
    };

    // Time/Controls
    let time = format!(
        "{} / {}",
        format_duration(app.elapsed),
        format_duration(app.duration)
    );

    let mut spans = vec![Span::styled(time, Style::default().fg(ACCENT_LIGHT))];
    let spacing = Span::raw(" ");
    let play_icon = if app.playing { "" } else { "" };

    spans.push(spacing.clone());
    spans.push(Span::styled(play_icon, Style::default().fg(ACCENT)));

    if app.shuffle {
        spans.push(spacing.clone());
        spans.push(Span::styled("", Style::default().fg(ACCENT)));
    }

    match app.repeat {
        RepeatMode::Off => {}

        RepeatMode::One => {
            spans.push(spacing.clone());
            spans.push(Span::styled("¹", Style::default().fg(ACCENT)));
        }

        RepeatMode::All => {
            spans.push(spacing.clone());
            spans.push(Span::styled("", Style::default().fg(ACCENT)));
        }
    }

    let info = Line::from(spans);

    lines.push(info);

    frame.render_widget(
        Paragraph::new(lines)
            .centered()
            .alignment(Alignment::Center),
        area,
    );
}

struct PlayerProgress {
    elapsed: String,
    duration: String,
    playing: bool,
    repeat: RepeatMode,
    shuffle: bool,
}

impl Widget for PlayerProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let y = area.y;

        let play_icon = if self.playing { "" } else { "" };

        // Elementos do texto
        let time = format!("{} / {}", self.elapsed, self.duration);

        // Monta os controles apenas quando existem
        let mut controls = Vec::new();

        controls.push(play_icon);

        if self.shuffle {
            controls.push("");
        }

        match self.repeat {
            RepeatMode::Off => {}
            RepeatMode::One => controls.push("¹"),
            RepeatMode::All => controls.push(""),
        }

        // controls.push("+");
        // controls.push("-");

        // Largura do bloco de tempo
        let time_width = UnicodeWidthStr::width(time.as_str());

        // Largura dos controles
        let controls_width: usize = controls
            .iter()
            .map(|icon| UnicodeWidthStr::width(*icon))
            .sum::<usize>()
            + controls.len().saturating_sub(1) * 2;

        // Espaço entre tempo e controles
        let spacing = 2usize;

        let total_width = time_width + spacing + controls_width;

        if total_width > area.width as usize {
            return;
        }

        // Centraliza o conjunto inteiro
        let start_x = area.x + ((area.width as usize - total_width) / 2) as u16;

        // Tempo
        buf.set_string(start_x, y, &time, Style::default().fg(MUTED));

        // Controles
        let mut cursor = start_x + time_width as u16 + spacing as u16;

        for (index, icon) in controls.iter().enumerate() {
            buf.set_string(cursor, y, *icon, Style::default().fg(MUTED));

            cursor += UnicodeWidthStr::width(*icon) as u16;

            if index + 1 < controls.len() {
                cursor += 2;
            }
        }
    }
}

fn render_progress(frame: &mut Frame, area: Rect, app: &App) {
    let [progress_area] = Layout::vertical([Constraint::Length(1)])
        .flex(Flex::Center)
        .areas(area);

    let widget = PlayerProgress {
        elapsed: format_duration(app.elapsed),
        duration: format_duration(app.duration),
        playing: app.playing,
        repeat: app.repeat,
        shuffle: app.shuffle,
    };

    frame.render_widget(widget, progress_area);
}

fn render_tree(frame: &mut Frame, area: Rect, app: &mut App) {
    let [_, library, _] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .flex(Flex::Center)
    .areas(area);

    let nodes = app.library.visible_nodes();

    let items: Vec<ListItem> = nodes
        .iter()
        .map(|(node, depth)| {
            let indent = "  ".repeat(*depth);

            match node {
                LibraryNode::Directory { name, .. } => ListItem::new(format!("{indent}󰉋 {name}")),

                LibraryNode::Track(track) => {
                    let selected = app.library.state.selected_tracks.contains(&track.path);

                    let prefix = if selected { "󰄲 " } else { "󰄱 " };

                    ListItem::new(format!("{indent}{prefix}󰎈 {}", track.title))
                }
            }
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol("› ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    frame.render_stateful_widget(list, library, &mut app.library.state.list_state);
}
