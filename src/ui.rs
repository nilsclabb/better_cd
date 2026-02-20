use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header (Path)
            Constraint::Length(3), // Search Query
            Constraint::Min(0),    // List
        ].as_ref())
        .split(f.area());

    // 1. Header Line (Path)
    let header_line = Line::from(vec![
        Span::styled(
            " Better CD (bcd) ",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | Path: "),
        Span::styled(
            app.current_dir.to_string_lossy(),
            Style::default().fg(Color::Cyan),
        ),
    ]);
    let header_block = Block::default()
        .borders(Borders::ALL)
        .title(" Directory Navigator ");
    f.render_widget(header_block.clone(), chunks[0]);
    let inner_header_area = header_block.inner(chunks[0]);
    let header_paragraph = ratatui::widgets::Paragraph::new(header_line);
    f.render_widget(header_paragraph, inner_header_area);

    // 2. Search Box
    let search_line = if app.search_query.is_empty() {
        Line::from(vec![Span::styled(
            " Type to filter...",
            Style::default().fg(Color::DarkGray),
        )])
    } else {
        Line::from(vec![
            Span::raw(" Filter: "),
            Span::styled(
                format!("{}_", app.search_query),
                Style::default().fg(Color::Yellow),
            ),
        ])
    };
    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(" Search ");
    f.render_widget(search_block.clone(), chunks[1]);
    let inner_search_area = search_block.inner(chunks[1]);
    let search_paragraph = ratatui::widgets::Paragraph::new(search_line);
    f.render_widget(search_paragraph, inner_search_area);

    // 3. List
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|p| {
            let is_dir = p.is_dir();
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            let mut style = Style::default();
            let icon = if is_dir {
                style = style.fg(Color::Blue);
                "📁 "
            } else {
                style = style.fg(Color::White);
                "📄 "
            };
            
            ListItem::new(Line::from(vec![
                Span::styled(icon, style.clone()),
                Span::styled(name, style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Contents "))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[2], &mut app.state);
}
