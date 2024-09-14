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
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app_state = AppState::new();

    let mut task_title = String::new();
    let mut task_description = String::new();

    let mut focus_index = 0;

    terminal.clear()?;

    loop {
        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(size);

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

                    ListItem::new(format!("{}. {}", (i + 1), item)).style(style)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Things to do").borders(Borders::ALL));

            f.render_widget(list, chunks[0]);

            let input_style = if focus_index == 0 {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let input_block = Paragraph::new::<&str>(task_title.as_ref())
                .block(Block::default().title("Title").borders(Borders::ALL))
                .style(input_style);

            f.render_widget(input_block, chunks[1]);
        })?;

        // handle events
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    if focus_index == 0 {
                        task_title.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if focus_index == 0 {
                        task_title.pop();
                    }
                }
                KeyCode::Tab => {
                    focus_index = (focus_index + 1) % 1;
                }
                KeyCode::Enter => {
                    if !task_title.is_empty() {
                        app_state.items.push(task_title.clone());
                        task_title.clear();
                    }
                }
                KeyCode::Esc => {
                    terminal.clear()?;
                    break;
                }
                KeyCode::Up => {
                    app_state.previous();
                }
                KeyCode::Down => {
                    app_state.next();
                }
                _ => {}
            }
        }
    }

    drop(terminal);

    disable_raw_mode()?;
    execute!(stdout, DisableMouseCapture)?;
    Ok(())
}
