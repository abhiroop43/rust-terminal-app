use std::io::{self};

mod app_state;
use app_state::AppState;

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app_state = AppState::new();

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

            let items: Vec<ListItem> = app_state
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == app_state.selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Reset)
                    };

                    ListItem::new(item.clone()).style(style)
                })
                .collect();

            let list =
                List::new(items).block(Block::default().title("List Block").borders(Borders::ALL));

            f.render_widget(list, chunks[0]);
        })?;

        // handle events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => app_state.previous(),
                KeyCode::Down => app_state.next(),
                _ => {}
            }
        }

        if let Event::Mouse(_mouse_event) = event::read()? {
            //TODO: Handle mouse events
        }
    }

    drop(terminal);

    disable_raw_mode()?;
    execute!(stdout, DisableMouseCapture)?;
    Ok(())
}
