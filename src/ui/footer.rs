use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
const ACCENT_LIGHT: Color = Color::Rgb(200, 180, 255);
const MUTED: Color = Color::Rgb(100, 100, 115);
pub fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Line::from(vec![
        Span::styled("[p]", Style::default().fg(ACCENT_LIGHT)),
        Span::styled("      ", Style::default().fg(MUTED)),
        // Span::styled("[←→]", Style::default().fg(ACCENT_LIGHT)),
        // Span::styled(" Seek   ", Style::default().fg(MUTED)),
        Span::styled("[r]", Style::default().fg(ACCENT_LIGHT)),
        Span::styled("    ", Style::default().fg(MUTED)),
        Span::styled("[s]", Style::default().fg(ACCENT_LIGHT)),
        Span::styled("    ", Style::default().fg(MUTED)),
        Span::styled("[b]", Style::default().fg(ACCENT_LIGHT)),
        Span::styled("    ", Style::default().fg(MUTED)),
        Span::styled("[n]", Style::default().fg(ACCENT_LIGHT)),
        Span::styled("    ", Style::default().fg(MUTED)),
        Span::styled("[q]", Style::default().fg(ACCENT_LIGHT)),
        Span::styled(" 󰈆 ", Style::default().fg(MUTED)),
    ]);

    frame.render_widget(Paragraph::new(footer).centered(), area);
}
