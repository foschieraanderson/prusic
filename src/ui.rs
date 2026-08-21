use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let paragrapth = Paragraph::new("Rust Music Player").block(
        Block::bordered()
            .title(format!(" Counter: {}", app.counter))
            .borders(Borders::ALL),
    );

    frame.render_widget(paragrapth, frame.area());

    let _ = app;
}
