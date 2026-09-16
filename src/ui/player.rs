use std::time::Duration;

use crate::{App, playlist::RepeatMode};
use ratatui::{
    Frame,
    buffer::Buffer,
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

pub fn render_player(frame: &mut Frame, area: Rect, app: &App) -> Rect {
    let [player] = Layout::horizontal([Constraint::Length(56)])
        .flex(Flex::Center)
        .areas(area);

    let [cover, title, artist, progress, controls] = Layout::vertical([
        Constraint::Length(12),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .flex(Flex::Center)
    .spacing(1)
    .areas(player);

    render_cover(frame, cover, app);
    render_title(frame, title, app);
    render_artist(frame, artist, app);
    render_progress(frame, progress, app);
    // render_controls(frame, controls, app);

    cover
}

fn render_cover(frame: &mut Frame, area: Rect, app: &App) {
    // COVER

    frame.render_widget(Paragraph::new(""), area);
}

fn render_title(frame: &mut Frame, area: Rect, app: &App) {
    let title = Paragraph::new(Line::from(Span::styled(
        app.current_track.as_ref().unwrap().title.clone(),
        Style::default().fg(TEXT),
    )))
    .alignment(Alignment::Center);

    frame.render_widget(title, area);
}

fn render_artist(frame: &mut Frame, area: Rect, app: &App) {
    let year = app
        .current_track
        .as_ref()
        .unwrap()
        .year
        .clone()
        .map(|year| format!(" / {year}"))
        .unwrap_or_default();
    let text = format!(
        "{} ({}{})",
        app.current_track.as_ref().unwrap().artist.clone(),
        app.current_track.as_ref().unwrap().album.clone(),
        year
    );
    let artist = Paragraph::new(Line::from(Span::styled(text, Style::default().fg(MUTED))))
        .alignment(Alignment::Center);

    frame.render_widget(artist, area);
}

struct PlayerProgress {
    progress: f64,
    elapsed: String,
    duration: String,
    playing: bool,
    repeat: RepeatMode,
    shuffle: bool,
}

impl Widget for PlayerProgress {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 20 || area.height == 0 {
            return;
        }

        let elapsed_width = self.elapsed.len() as u16;
        let duration_width = self.duration.len() as u16;

        let start = elapsed_width + 4;

        let end = area.width.saturating_sub(duration_width + 2);

        if end <= start {
            return;
        }

        let width = end - start;
        let progress = self.progress.clamp(0.0, 1.0);
        let position = ((width.saturating_sub(1)) as f64 * progress).round() as u16;

        // Coordenadas absolutas do widget
        let x = area.x;
        let y = area.y;

        // Play/Pause
        let play_icon = if self.playing { " " } else { " " }; // |> ⤨ ▶ ❚❚||↻ ●   󰏤     󰁗  󰁐  󱦰 󱦱 󰈆
        buf.set_string(x, y, play_icon, Style::default().fg(MUTED));

        // Tempo atual
        buf.set_string(x + 2, y, &self.elapsed, Style::default().fg(MUTED));

        // Barra de progresso
        for offset in 0..width {
            let color = if offset <= position { ACCENT } else { BAR };
            // let line = if offset <= position { "━" } else { "─" };
            let line = "━";

            buf.set_string(x + start + offset, y, line, Style::default().fg(color));
        }

        // Indicador
        // buf.set_string(x + start + position, y, "━", Style::default().fg(ACCENT));

        // Tempo total
        buf.set_string(x + end + 2, y, &self.duration, Style::default().fg(MUTED));

        // Controls
        let mut cursor = x + end + 8;

        let draw_icon = |buf: &mut Buffer, icon: &str, cursor: &mut u16| {
            buf.set_string(*cursor, y, icon, Style::default().fg(MUTED));

            *cursor += icon.chars().count() as u16 + 1;
        };

        if self.shuffle {
            draw_icon(buf, " ", &mut cursor);
        }

        match self.repeat {
            RepeatMode::Off => {}

            RepeatMode::One => {
                draw_icon(buf, " ¹", &mut cursor);
            }

            RepeatMode::All => {
                draw_icon(buf, " ", &mut cursor);
            }
        }
    }
}

fn render_progress(frame: &mut Frame, area: Rect, app: &App) {
    let [progress_area] = Layout::vertical([Constraint::Length(1)])
        .flex(Flex::Center)
        .areas(area);

    let widget = PlayerProgress {
        progress: app.progress(),
        elapsed: format_duration(app.elapsed),
        duration: format_duration(app.duration),
        playing: app.playing,
        repeat: app.repeat,
        shuffle: app.shuffle,
    };

    frame.render_widget(widget, progress_area);
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();

    let minutes = seconds / 60;
    let seconds = seconds % 60;

    format!("{minutes:02}:{seconds:02}")
}

fn render_controls(frame: &mut Frame, area: Rect, app: &App) {
    let play_icon = if app.playing { "❚❚" } else { "▶" };

    let controls = Line::from(vec![
        Span::styled("◀", Style::default().fg(MUTED)),
        Span::raw("   "),
        Span::styled(play_icon, Style::default().fg(ACCENT_LIGHT)),
        Span::raw("   "),
        Span::styled("▶", Style::default().fg(MUTED)),
    ]);

    frame.render_widget(Paragraph::new(controls).alignment(Alignment::Center), area);
}
