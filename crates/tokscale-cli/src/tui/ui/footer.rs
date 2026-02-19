use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use super::spinner::{get_phase_message, get_scanner_spans};
use super::widgets::{format_cost, format_tokens};
use crate::tui::app::{App, ClickAction, SortField, Tab};
use crate::tui::data::Source;

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.border))
        .style(Style::default().bg(app.theme.background));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split into 3 rows: sources+sort, help text, status
    let row_constraints = if inner.height >= 3 {
        vec![
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]
    } else if inner.height >= 2 {
        vec![Constraint::Length(1), Constraint::Length(1)]
    } else {
        vec![Constraint::Length(1)]
    };

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(inner);

    render_main_row(frame, app, rows[0]);

    if rows.len() >= 2 {
        render_help_row(frame, app, rows[1]);
    }

    if rows.len() >= 3 {
        render_status_row(frame, app, rows[2]);
    }
}

fn render_main_row(frame: &mut Frame, app: &mut App, area: Rect) {
    let is_very_narrow = app.is_very_narrow();

    // Split into left (sources) and right (sort + totals)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    // Left side: source badges
    render_source_badges(frame, app, chunks[0]);

    // Right side: scroll info | tokens | cost (models)
    let mut right_spans: Vec<Span> = Vec::new();

    // Scroll position indicator for Overview tab
    if app.current_tab == Tab::Overview {
        let total_models = app.data.models.len();
        if total_models > app.max_visible_items && app.max_visible_items > 0 {
            let start = app.scroll_offset + 1;
            let end = (app.scroll_offset + app.max_visible_items).min(total_models);
            if !is_very_narrow {
                right_spans.push(Span::styled(
                    format!("↓ {}-{} of {} ", start, end, total_models),
                    Style::default().fg(app.theme.muted),
                ));
                right_spans.push(Span::styled("| ", Style::default().fg(app.theme.muted)));
            }
        }
    }

    // Total tokens
    let total_tokens = app.data.total_tokens;
    right_spans.push(Span::styled(
        format_tokens(total_tokens),
        Style::default().fg(Color::Cyan),
    ));
    if !is_very_narrow {
        right_spans.push(Span::styled(
            " tokens",
            Style::default().fg(app.theme.muted),
        ));
    }

    right_spans.push(Span::styled(" | ", Style::default().fg(app.theme.muted)));

    // Total cost
    right_spans.push(Span::styled(
        format_cost(app.data.total_cost),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));

    // Model count
    if !is_very_narrow {
        right_spans.push(Span::styled(
            format!(" ({} models)", app.data.models.len()),
            Style::default().fg(app.theme.muted),
        ));
    }

    let right_line = Line::from(right_spans);
    let right_para = Paragraph::new(right_line).alignment(Alignment::Right);
    frame.render_widget(right_para, chunks[1]);
}

fn render_source_badges(frame: &mut Frame, app: &mut App, area: Rect) {
    let mut spans: Vec<Span> = Vec::new();
    let mut x_offset = area.x;
    let is_very_narrow = app.is_very_narrow();

    let enabled_count = app.enabled_sources.len();
    let total_count = Source::all().len();

    let enabled_names: Vec<&str> = Source::all()
        .iter()
        .filter(|s| app.enabled_sources.contains(s))
        .map(|s| s.as_str())
        .collect();

    let source_text = if is_very_narrow {
        format!("[s:{}/{}]", enabled_count, total_count)
    } else if enabled_count == total_count {
        "[s:Sources] All".to_string()
    } else if enabled_count <= 3 {
        format!("[s:Sources] {}", enabled_names.join(", "))
    } else {
        format!("[s:Sources] {} of {}", enabled_count, total_count)
    };

    spans.push(Span::styled(source_text, Style::default().fg(Color::Cyan)));
    spans.push(Span::raw(" "));

    x_offset += 20;

    if !is_very_narrow {
        spans.push(Span::styled("| ", Style::default().fg(app.theme.muted)));

        let sort_buttons = [
            (SortField::Date, "Date"),
            (SortField::Cost, "Cost"),
            (SortField::Tokens, "Tok"),
        ];

        for (field, label) in sort_buttons {
            let is_active = app.sort_field == field;
            let style = if is_active {
                Style::default()
                    .fg(app.theme.foreground)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.muted)
            };

            spans.push(Span::styled(label, style));
            spans.push(Span::raw(" "));

            let btn_width = label.len() as u16;
            app.add_click_area(
                Rect::new(x_offset + 2, area.y, btn_width, 1),
                ClickAction::Sort(field),
            );
            x_offset += btn_width + 1;
        }
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}

fn render_help_row(frame: &mut Frame, app: &App, area: Rect) {
    let is_very_narrow = app.is_very_narrow();

    let spans = if is_very_narrow {
        vec![
            Span::styled("↑↓", Style::default().fg(app.theme.muted)),
            Span::styled("·", Style::default().fg(app.theme.muted)),
            Span::styled("←→", Style::default().fg(app.theme.muted)),
            Span::styled("·", Style::default().fg(app.theme.muted)),
            Span::styled("[s]", Style::default().fg(Color::Cyan)),
            Span::styled("·", Style::default().fg(app.theme.muted)),
            Span::styled("[p]", Style::default().fg(Color::Magenta)),
            Span::styled("·", Style::default().fg(app.theme.muted)),
            Span::styled("[r]", Style::default().fg(Color::Yellow)),
            Span::styled("·", Style::default().fg(app.theme.muted)),
            Span::styled("q", Style::default().fg(app.theme.muted)),
        ]
    } else {
        vec![
            Span::styled(
                "↑↓ scroll • ←→/tab view • ",
                Style::default().fg(app.theme.muted),
            ),
            Span::styled("[s:sources]", Style::default().fg(Color::Cyan)),
            Span::styled(" • ", Style::default().fg(app.theme.muted)),
            Span::styled(
                format!("[p:{}]", app.theme.name.as_str()),
                Style::default().fg(Color::Magenta),
            ),
            Span::styled(" ", Style::default()),
            Span::styled(
                if app.auto_refresh {
                    format!("[R:auto {}s]", app.auto_refresh_interval.as_secs())
                } else {
                    "[R:auto off]".to_string()
                },
                Style::default().fg(if app.auto_refresh {
                    Color::Green
                } else {
                    app.theme.muted
                }),
            ),
            Span::styled(" • ", Style::default().fg(app.theme.muted)),
            Span::styled("[r:refresh]", Style::default().fg(Color::Yellow)),
            Span::styled(" • e export • q quit", Style::default().fg(app.theme.muted)),
        ]
    };

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}

fn render_status_row(frame: &mut Frame, app: &App, area: Rect) {
    let mut spans: Vec<Span> = Vec::new();

    if app.data.loading || app.background_loading {
        let scanner_spans = get_scanner_spans(app.spinner_frame);
        spans.extend(scanner_spans);
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            get_phase_message("parsing-sources"),
            Style::default().fg(app.theme.muted),
        ));
    } else if let Some(ref msg) = app.status_message {
        spans.push(Span::styled(
            msg.clone(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        let elapsed = app.last_refresh.elapsed();
        let ago = if elapsed.as_secs() < 60 {
            format!("{}s ago", elapsed.as_secs())
        } else if elapsed.as_secs() < 3600 {
            format!("{}m ago", elapsed.as_secs() / 60)
        } else {
            format!("{}h ago", elapsed.as_secs() / 3600)
        };
        spans.push(Span::styled(
            format!("Last updated: {}", ago),
            Style::default().fg(app.theme.muted),
        ));

        if app.auto_refresh {
            spans.push(Span::styled(
                format!(" • Auto: {}s", app.auto_refresh_interval.as_secs()),
                Style::default().fg(app.theme.muted),
            ));
        }
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
