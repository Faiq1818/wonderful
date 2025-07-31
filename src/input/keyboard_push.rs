use crate::config;
use crossterm::event::{self, Event, KeyCode};

pub fn keyboard_push(
    selected: &mut usize,
    items: &[&String],
    config_path: &std::path::PathBuf,
    find_string: &mut String,
    focus: &mut String,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char(c) => {
                find_string.push(c);
                *focus = String::from("find_section");
                *selected = 0;
            }
            KeyCode::Backspace => {
                find_string.pop();
                *selected = 0;
            }
            KeyCode::Up => {
                *selected = selected.saturating_sub(1);
                *focus = String::from("apps_section");
            }
            KeyCode::BackTab => {
                *selected = selected.saturating_sub(1);
                *focus = String::from("apps_section");
            }
            KeyCode::Down => {
                if *selected < items.len() - 1 {
                    *selected += 1;
                }
                *focus = String::from("apps_section");
            }
            KeyCode::Tab => {
                if *selected < items.len() - 1 {
                    *selected += 1;
                }
                *focus = String::from("apps_section");
            }

            KeyCode::Enter => {
                config::scan_toml::scan_toml(config_path, &items[*selected]);
                //println!("{}", items[*selected])
                return Ok(true);
            }

            KeyCode::Esc => {
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}
