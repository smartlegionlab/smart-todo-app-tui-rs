use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{
    APP_AUTHOR, APP_LICENSE, APP_NAME, APP_REPO_URL, APP_SITE_URL, APP_VERSION, App,
    MAX_TITLE_CHARS, Mode,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);
    render_list(f, app, chunks[1]);
    render_footer(f, app, chunks[2]);

    match app.mode {
        Mode::Adding | Mode::Editing => render_input_modal(f, app),
        Mode::About => render_about_modal(f, app),
        Mode::Normal => {}
    }
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let (done, total) = app.counts();
    let title = format!(" Smart TODO TUI — done {done}/{total} ");
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title)
        .title_alignment(Alignment::Center);
    f.render_widget(block, area);
}

fn render_list(f: &mut Frame, app: &App, area: Rect) {
    let inner_width = area.width.saturating_sub(2) as usize;
    let highlight_width = 2;
    let available = inner_width.saturating_sub(highlight_width);

    let items: Vec<ListItem> = app
        .todos
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let mark = if t.done { "[x]" } else { "[ ]" };
            let base_style = if t.done {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default().fg(Color::White)
            };

            let pending = app.pending_delete == Some(i);
            let title_style = if pending {
                base_style.fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                base_style
            };

            let suffix = if pending { "  (press d again)" } else { "" };
            let prefix = format!("{mark} ");

            let prefix_len = prefix.chars().count();
            let suffix_len = suffix.chars().count();
            let title_space = available.saturating_sub(prefix_len + suffix_len);

            let title_display = truncate_with_ellipsis(&t.title, title_space);

            ListItem::new(Line::from(vec![
                Span::styled(
                    prefix,
                    Style::default().fg(if t.done { Color::Green } else { Color::Yellow }),
                ),
                Span::styled(title_display, title_style),
                Span::styled(suffix, Style::default().fg(Color::Red)),
            ]))
        })
        .collect();

    let title = if items.is_empty() {
        " Tasks (press 'a' to add) "
    } else {
        " Tasks "
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut state = ListState::default();
    if !app.todos.is_empty() {
        state.select(Some(app.selected));
    }
    f.render_stateful_widget(list, area, &mut state);
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let (text, color) = if app.pending_delete.is_some() {
        (
            "Press 'd' again to confirm delete - any other key to cancel".to_string(),
            Color::Red,
        )
    } else if let Some(status) = &app.status {
        (status.clone(), Color::Red)
    } else {
        (
            "j/k: select | Space: done | a: add | e: edit | d: delete | i: about | q: quit"
                .to_string(),
            Color::Gray,
        )
    };

    let inner_width = area.width.saturating_sub(2) as usize;
    let text_display = truncate_with_ellipsis(&text, inner_width);

    let p = Paragraph::new(text_display)
        .alignment(Alignment::Center)
        .style(Style::default().fg(color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if app.pending_delete.is_some() {
                    Color::Red
                } else {
                    Color::Cyan
                })),
        );
    f.render_widget(p, area);
}

fn render_input_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(80, 20, f.area());
    f.render_widget(Clear, area);

    let current_len = app.input.chars().count();
    let counter = format!(" {current_len}/{MAX_TITLE_CHARS} ");

    let counter_style = if current_len >= MAX_TITLE_CHARS {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else if current_len >= MAX_TITLE_CHARS - 20 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let title_text = match app.mode {
        Mode::Adding => " New task ",
        Mode::Editing => " Edit task ",
        _ => "",
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title(title_text)
        .title_top(Line::from(Span::styled(counter, counter_style)).alignment(Alignment::Right));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height == 0 || inner.width == 0 {
        return;
    }

    let hint_height = if inner.height > 1 { 1 } else { 0 };
    let input_height = inner.height.saturating_sub(hint_height);

    let input_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: input_height,
    };

    render_input_line(f, app, input_area);

    if hint_height > 0 {
        let hint_area = Rect {
            x: inner.x,
            y: inner.y + inner.height - 1,
            width: inner.width,
            height: 1,
        };
        let hint = Paragraph::new("Enter: save - Esc: cancel")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Right);
        f.render_widget(hint, hint_area);
    }
}

fn render_input_line(f: &mut Frame, app: &App, area: Rect) {
    let chars: Vec<char> = app.input.chars().collect();
    let cursor = app.input_cursor.min(chars.len());

    let width = area.width as usize;
    if width == 0 {
        return;
    }

    let reserved = 1;
    let window_width = width.saturating_sub(reserved);
    if window_width == 0 {
        return;
    }

    let (start, end) = visible_window(cursor, chars.len(), window_width);

    let mut spans: Vec<Span> = Vec::new();

    if start > 0 {
        spans.push(Span::styled("<", Style::default().fg(Color::DarkGray)));
    }

    let before: String = chars[start..cursor].iter().collect();
    if !before.is_empty() {
        spans.push(Span::raw(before));
    }

    spans.push(Span::styled(
        "|",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::SLOW_BLINK),
    ));

    let after: String = chars[cursor..end].iter().collect();
    if !after.is_empty() {
        spans.push(Span::raw(after));
    }

    if end < chars.len() {
        spans.push(Span::styled(">", Style::default().fg(Color::DarkGray)));
    }

    let p = Paragraph::new(Line::from(spans)).style(Style::default().fg(Color::White));
    f.render_widget(p, area);
}

fn visible_window(cursor: usize, total: usize, window_width: usize) -> (usize, usize) {
    if total <= window_width {
        return (0, total);
    }

    let half = window_width / 2;

    if cursor <= half {
        return (0, window_width);
    }

    let end = (cursor + half).min(total);
    let start = end.saturating_sub(window_width);

    (start, end)
}

fn render_about_modal(f: &mut Frame, _app: &App) {
    let area = about_rect(f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .title(" About ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let copyright = format!("Copyright (c) 2026 {APP_AUTHOR}");

    let lines = vec![
        Line::from(Span::styled(
            "Smart TODO TUI",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("binary: {APP_NAME}"),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("Version {APP_VERSION}"),
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(copyright, Style::default().fg(Color::Gray))),
        Line::from(Span::styled(
            format!("License: {APP_LICENSE}"),
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
        Line::from(Span::styled(APP_REPO_URL, Style::default().fg(Color::Blue))),
        Line::from(Span::styled(APP_SITE_URL, Style::default().fg(Color::Blue))),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc / q / i to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let p = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    f.render_widget(p, inner);
}

fn about_rect(area: Rect) -> Rect {
    let width = area.width.saturating_mul(80) / 100;
    let height = area.height.saturating_mul(80) / 100;

    let width = width.clamp(40, 72);
    let height = height.clamp(14, 20);

    let width = width.min(area.width);
    let height = height.min(area.height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;

    Rect {
        x,
        y,
        width,
        height,
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn truncate_with_ellipsis(s: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let count = s.chars().count();
    if count <= max_chars {
        return s.to_string();
    }

    if max_chars == 1 {
        return "…".to_string();
    }

    let take = max_chars - 1;
    let truncated: String = s.chars().take(take).collect();
    format!("{truncated}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate_with_ellipsis("hello", 10), "hello");
    }

    #[test]
    fn truncate_exact_length_unchanged() {
        assert_eq!(truncate_with_ellipsis("hello", 5), "hello");
    }

    #[test]
    fn truncate_long_string_adds_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello w…");
    }

    #[test]
    fn truncate_zero_width_returns_empty() {
        assert_eq!(truncate_with_ellipsis("hello", 0), "");
    }

    #[test]
    fn truncate_one_width_returns_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello", 1), "…");
    }

    #[test]
    fn truncate_handles_multibyte() {
        assert_eq!(truncate_with_ellipsis("привет мир", 7), "привет…");
    }

    #[test]
    fn visible_window_short_text_fits() {
        assert_eq!(visible_window(3, 5, 10), (0, 5));
    }

    #[test]
    fn visible_window_cursor_at_start() {
        assert_eq!(visible_window(0, 100, 20), (0, 20));
    }

    #[test]
    fn visible_window_cursor_in_middle() {
        let (start, end) = visible_window(50, 100, 20);
        assert_eq!(end - start, 20);
        assert!(start <= 50 && 50 <= end);
    }

    #[test]
    fn visible_window_cursor_at_end() {
        let (start, end) = visible_window(100, 100, 20);
        assert_eq!(end, 100);
        assert_eq!(end - start, 20);
    }
}