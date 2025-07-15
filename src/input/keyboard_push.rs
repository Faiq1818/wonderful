use crate::config;
use crossterm::event::{self, Event, KeyCode};

pub fn keyboard_push(
    selected: &mut usize,
    items: &[String],
    config_path: &std::path::PathBuf,
    find_string: &mut String,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char(c) => {
                find_string.push(c);
            }
            KeyCode::Backspace => {
                find_string.pop();
            }
            KeyCode::Up => {
                *selected = selected.saturating_sub(1);
            }

            KeyCode::Down => {
                if *selected < items.len() - 1 {
                    *selected += 1;
                }
            }

            KeyCode::Enter => {
                config::scan_toml::scan_toml(config_path, &items[*selected]);
                println!("{}", items[*selected])
            }

            KeyCode::Esc => {
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}
