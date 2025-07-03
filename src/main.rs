use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::process::Command;

enum AppState {
    MainMenu,
    ScreenshotMenu,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let mut selected = 0;
    let mut state = AppState::MainMenu;

    loop {
        let items = match state {
            AppState::MainMenu => vec!["Screenshot", "Item 2", "Item 3", "Item 4"],
            AppState::ScreenshotMenu => vec!["Fullscreen", "Snippet"],
        };

        terminal.draw(|f| draw(f, selected, &items))?;

        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < items.len() - 1 {
                        selected += 1;
                    }
                }

                KeyCode::Enter => match state {
                    AppState::MainMenu => match selected {
                        0 => {
                            state = AppState::ScreenshotMenu;
                            selected = 0
                        }
                        _ => {}
                    },
                    AppState::ScreenshotMenu => {
                        match selected {
                            0 => {
                                // Aksi untuk "Fullscreen"
                                let _ = Command::new("grim").arg("-l").arg("0").spawn();
                            }
                            1 => {
                                // Aksi untuk "Snippet"
                                println!("Snippet dipilih");
                            }
                            _ => {}
                        }
                    }
                },

                KeyCode::Char('q') => break,
                _ => {}
            },
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame, selected: usize, items: &[&str]) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(frame.area());

    let mut text = Text::default();
    for (i, item) in items.iter().enumerate() {
        if i == selected {
            text.extend(Text::styled(
                format!("> {}\n", item),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            text.extend(Text::raw(format!("  {}\n", item)));
        }
    }

    frame.render_widget(
        Paragraph::new("").block(Block::default().title("Find").borders(Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new(text).block(Block::default().title("Apps").borders(Borders::ALL)),
        layout[1],
    );
}
