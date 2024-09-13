use std::io::{self};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

            let block1 = Block::default().title("Block 1").borders(Borders::ALL);
            f.render_widget(block1, chunks[0]);

            let block2 = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block2, chunks[1]);

            let items = vec![
                ListItem::new("Item 1").style(Style::default().fg(Color::Red)),
                ListItem::new("Item 2").style(Style::default().fg(Color::Green)),
                ListItem::new("Item 3").style(Style::default().fg(Color::Blue)),
            ];

            let list =
                List::new(items).block(Block::default().title("List Block").borders(Borders::ALL));

            f.render_widget(list, chunks[0]);
        })?;

        // handle events
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
