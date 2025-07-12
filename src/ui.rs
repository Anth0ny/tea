use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use ropey::Rope;

pub fn draw<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    rope: &Rope,
) -> std::io::Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1)].as_ref())
            .split(f.size());

        let lines: Vec<Line> = rope
            .lines()
            .take(chunks[0].height as usize)
            .map(|line| {
                let raw = line.as_str().unwrap_or("<invalid utf8>");
                Line::from(Span::raw(raw.trim_end_matches('\n')))
            })
            .collect();

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Text View"))
            .style(Style::default());

        f.render_widget(paragraph, chunks[0]);
    })?;
    Ok(())
}