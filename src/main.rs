mod ui;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use std::io::stdout;
use anyhow::Result;

use std::path::Path;

mod io;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: tea <file>");
        std::process::exit(1);
    });

    eprintln!("Trying to open: {}", path);
    let (rope, _line_ending, _trailing) = io::load_file(Path::new(&path))?;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    use crossterm::event::{self, Event, KeyCode};

    loop {
        ui::draw(&mut terminal, &rope)?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
