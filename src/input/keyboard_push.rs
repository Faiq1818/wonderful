use crate::config;
use crossterm::event::{self, Event, KeyCode};

pub fn keyboard_push(selected: &mut usize, items: &[String], config_path: &std::path::PathBuf)-> Result<bool, Box<dyn std::error::Error>>  {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Up => {
                //if selected > 0 {
                //    selected -= 1;
                //}
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

            KeyCode::Char('t') => {
                config::scan_toml::get_items_name(config_path);
            }

            KeyCode::Char('q') => {
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}
