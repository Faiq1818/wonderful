mod config;
mod draw;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};

enum AppState {
    MainMenu,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let config_path = config::check_config_path::get_dotconfig_path("wonderful", "wonderful.toml");
    config::check_config_path::ensure_config_exists(&config_path);

    let mut selected = 0;
    let state = AppState::MainMenu;

    let items = match state {
        AppState::MainMenu => vec!["TomlFolder", "OpenRustProject", "HyprlandConfig", "Dolphin"],
    };

    loop {
        terminal.draw(|f| draw::draw::draw(f, selected, &items))?;

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

                KeyCode::Enter => {
                    config::scan_toml::scan_toml(&config_path, &items[selected]);
                    println!("{}", items[selected])
                }

                KeyCode::Char('q') => break,
                _ => {}
            },
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}
