use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Text},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::app::{App};

pub fn ui(frame: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    //creates a block with a border and a default style
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    //creates a paragraph, sets its text, style and color. sets it to the title block.
    let title = Paragraph::new(Text::styled(
        "rock paper sissors",
        Style::default().fg(Color::Green),
    ))
    .block(title_block).centered();

    //renders it to the first chunk. 
    //frame.render_widget(title, chunks[0]);

    let score_text = Paragraph::new(Text::from(vec![format!("{}", app.game_score).into()])).block(Block::default().borders(Borders::ALL).style(Style::default())).centered();

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);
    frame.render_widget(title, header_chunks[0]);
    frame.render_widget(score_text, header_chunks[1]);

    let content_block = Block::default().borders(Borders::ALL).style(Style::default());
    let content = Tabs::new(vec!["rock", "paper", "scissors"])
        .block(content_block)
        .select(app.selected_tab);

    frame.render_widget(content, chunks[1]);

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let quit_footer = Paragraph::new(Text::styled(
        "q to quit",
        Style::default().fg(Color::Green),
    ))
    .block(footer_block).centered();

    frame.render_widget(quit_footer, chunks[2]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}