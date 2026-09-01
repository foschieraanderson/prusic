mod config;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::Paragraph;

use crate::config::load_config;

fn main() -> Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    })
}

fn render(frame: &mut Frame) {
    let area = frame.area();

    // ┌──────────────────────────────────────┐
    // │                                      │
    // │              CONTENT                 │
    // │                                      │
    // ├──────────────────────────────────────┤
    // │  [q] Sair  [↑↓] Navegar  [Enter] OK │
    // └──────────────────────────────────────┘
    let [content_area, footer_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

    render_content(frame, content_area);
    render_footer(frame, footer_area);
}

fn render_content(frame: &mut Frame, area: Rect) {
    // Define o tamanho do conteúdo.
    // O espaço restante é distribuído para os lados.
    let [content] = Layout::horizontal([Constraint::Fill(1)])
        .flex(Flex::Center)
        .spacing(2)
        .margin(1 / 2)
        .areas(area);

    // Centraliza também verticalmente.
    let [content] = Layout::vertical([Constraint::Fill(1)])
        .flex(Flex::Center)
        .spacing(2)
        .margin(1)
        .areas(content);

    if let Ok(config) = load_config() {
        let widget = Paragraph::new(format!(
            "Volume: {}\nSpectrum: {}\nColor: {}\n",
            config.player.volume, config.spectrum.enabled, config.appearance.color
        ))
        .on_black();
        frame.render_widget(widget, content);
    };
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("[q] Sair   [↑↓] Navegar   [Enter] Selecionar").centered();
    frame.render_widget(footer, area);
}
