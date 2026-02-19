use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::tui::app::App;
use crate::tui::data::Source;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let modal_width = 40u16.min(area.width.saturating_sub(4));
    let modal_height = 14u16.min(area.height.saturating_sub(4));

    let modal_x = (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = (area.height.saturating_sub(modal_height)) / 2;

    let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

    frame.render_widget(Clear, modal_area);

    let block = Block::default()
        .title(" Select Sources ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.accent))
        .style(Style::default().bg(app.theme.background));

    let inner = block.inner(modal_area);
    frame.render_widget(block, modal_area);

    let sources = Source::all();
    let mut lines: Vec<Line> = Vec::new();

    for (i, source) in sources.iter().enumerate() {
        let is_selected = i == app.source_picker_index;
        let is_enabled = app.enabled_sources.contains(source);

        let checkbox = if is_enabled { "[✓]" } else { "[ ]" };
        let cursor = if is_selected { "▸ " } else { "  " };

        let style = if is_selected {
            Style::default()
                .fg(app.theme.accent)
                .add_modifier(Modifier::BOLD)
        } else if is_enabled {
            Style::default().fg(app.theme.foreground)
        } else {
            Style::default().fg(app.theme.muted)
        };

        let line = Line::from(vec![
            Span::styled(cursor, style),
            Span::styled(checkbox, style),
            Span::styled(" ", Style::default()),
            Span::styled(source.as_str(), style),
        ]);
        lines.push(line);
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("↑↓", Style::default().fg(app.theme.muted)),
        Span::styled(" move  ", Style::default().fg(app.theme.muted)),
        Span::styled("space", Style::default().fg(app.theme.muted)),
        Span::styled(" toggle  ", Style::default().fg(app.theme.muted)),
        Span::styled("enter", Style::default().fg(app.theme.muted)),
        Span::styled(" done", Style::default().fg(app.theme.muted)),
    ]));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}
