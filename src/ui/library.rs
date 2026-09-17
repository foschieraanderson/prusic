use ratatui::{Frame, layout::Rect};

use crate::app::App;

pub fn render_library(frame: &mut Frame, area: Rect, app: &App) {
    let _ = app;

    frame.render_widget(ratatui::widgets::Paragraph::new("Library"), area);
}
