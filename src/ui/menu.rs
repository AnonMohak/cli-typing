use ratatui::{
    Frame,
    style::{Modifier, Style},
    text::Line,
    widgets::Paragraph,
};

use super::{column, theme};
use crate::app::{MODES, mode_label};

pub fn draw(frame: &mut Frame, selected: usize) {
    let mut lines = vec![
        Line::styled("mode", Style::new().fg(theme::SUB)),
        Line::default(),
    ];
    for (i, &mode) in MODES.iter().enumerate() {
        let line = if i == selected {
            Line::styled(
                format!("> {}", mode_label(mode)),
                Style::new().fg(theme::MAIN).add_modifier(Modifier::BOLD),
            )
        } else {
            Line::styled(
                format!("  {}", mode_label(mode)),
                Style::new().fg(theme::TEXT),
            )
        };
        lines.push(line);
    }
    lines.push(Line::default());
    lines.push(Line::styled(
        "↑↓ select · enter start · esc back",
        Style::new().fg(theme::SUB),
    ));
    let area = column(frame.area(), lines.len() as u16);
    frame.render_widget(Paragraph::new(lines).centered(), area);
}
