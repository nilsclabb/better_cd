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
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

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

    f.render_stateful_widget(list, chunks[1], &mut app.state);
}
