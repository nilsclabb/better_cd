use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

const ACCENT: Color = Color::Rgb(110, 200, 220);
const DIM: Color = Color::Rgb(90, 90, 110);
const SUBTLE: Color = Color::Rgb(60, 60, 75);
const DIR_COLOR: Color = Color::Rgb(130, 180, 255);
const FILE_COLOR: Color = Color::Rgb(160, 160, 175);
const HIGHLIGHT_BG: Color = Color::Rgb(45, 50, 70);

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // breadcrumb path
            Constraint::Min(0),   // file list
            Constraint::Length(1), // bottom bar (search / hints)
        ])
        .split(area);

    draw_breadcrumb(f, app, chunks[0]);
    draw_file_list(f, app, chunks[1]);
    draw_bottom_bar(f, app, chunks[2]);
}

fn draw_breadcrumb(f: &mut Frame, app: &App, area: Rect) {
    let path_str = app.current_dir.to_string_lossy();

    let mut spans = vec![
        Span::styled(" cd+ ", Style::default().fg(Color::Black).bg(ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
    ];

    // Show abbreviated path: ~/... for home dir
    let home = std::env::var("HOME").unwrap_or_default();
    let display_path = if path_str.starts_with(&home) {
        format!("~{}", &path_str[home.len()..])
    } else {
        path_str.to_string()
    };

    let display_parts: Vec<&str> = display_path.split('/').filter(|s| !s.is_empty()).collect();

    for (i, part) in display_parts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" › ", Style::default().fg(DIM)));
        }
        let style = if i == display_parts.len() - 1 {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(DIM)
        };
        spans.push(Span::styled(part.to_string(), style));
    }

    // Add item count
    let count_text = format!("  ({} items)", app.items.len());
    spans.push(Span::styled(count_text, Style::default().fg(SUBTLE)));

    let breadcrumb = Paragraph::new(Line::from(spans));
    // Render on second line of the 2-line area for visual breathing room
    let inner = Rect::new(area.x, area.y + 1, area.width, 1);
    f.render_widget(breadcrumb, inner);
}

fn draw_file_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            let is_dir = p.is_dir();
            let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
            let is_selected = app.state.selected() == Some(idx);

            let (icon, name_color) = if is_dir {
                ("  ", DIR_COLOR)
            } else {
                ("  ", FILE_COLOR)
            };

            let bg = if is_selected { HIGHLIGHT_BG } else { Color::Reset };

            // Build the line with a subtle left-edge accent on selected
            let mut spans = vec![];

            if is_selected {
                spans.push(Span::styled("▌", Style::default().fg(ACCENT).bg(bg)));
            } else {
                spans.push(Span::styled(" ", Style::default().bg(bg)));
            }

            spans.push(Span::styled(icon, Style::default().fg(name_color).bg(bg)));

            let name_len = name.len();
            spans.push(Span::styled(
                name,
                Style::default()
                    .fg(if is_selected { Color::White } else { name_color })
                    .bg(bg)
                    .add_modifier(if is_dir { Modifier::BOLD } else { Modifier::empty() }),
            ));

            // Pad the rest of the line with bg color
            let used = 1 + icon.len() + name_len;
            let remaining = (area.width as usize).saturating_sub(used);
            spans.push(Span::styled(
                " ".repeat(remaining),
                Style::default().bg(bg),
            ));

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items);
    f.render_stateful_widget(list, area, &mut app.state);
}

fn draw_bottom_bar(f: &mut Frame, app: &App, area: Rect) {
    let line = if !app.search_query.is_empty() {
        // Search mode: show the query
        Line::from(vec![
            Span::styled(" / ", Style::default().fg(Color::Black).bg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!(" {}", app.search_query),
                Style::default().fg(ACCENT),
            ),
            Span::styled("▎", Style::default().fg(ACCENT)),
        ])
    } else {
        // Hint mode: show keybindings
        Line::from(vec![
            Span::styled(" ↑↓", Style::default().fg(ACCENT)),
            Span::styled(" navigate ", Style::default().fg(DIM)),
            Span::styled(" →", Style::default().fg(ACCENT)),
            Span::styled(" enter dir ", Style::default().fg(DIM)),
            Span::styled(" ←", Style::default().fg(ACCENT)),
            Span::styled(" back ", Style::default().fg(DIM)),
            Span::styled(" ⏎", Style::default().fg(ACCENT)),
            Span::styled(" select ", Style::default().fg(DIM)),
            Span::styled(" esc", Style::default().fg(ACCENT)),
            Span::styled(" quit ", Style::default().fg(DIM)),
            Span::styled(" type", Style::default().fg(ACCENT)),
            Span::styled(" to filter", Style::default().fg(DIM)),
        ])
    };

    let bar = Paragraph::new(line);
    f.render_widget(bar, area);
}
