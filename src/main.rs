mod config;
mod draw;
mod input;

//use color_eyre::Result;

fn main() {
    //color_eyre::install()?;
    let mut terminal = ratatui::init();

    let config_path = config::check_config_path::get_dotconfig_path("wonderful", "wonderful.toml");
    config::check_config_path::ensure_config_exists(&config_path);

    let mut selected = 0;

    let items = config::scan_toml::get_items_name(&config_path);

    loop {
        terminal.draw(|f| draw::draw::draw(f, selected, &items));
        
        if input::keyboard_push::keyboard_push(&mut selected, &items, &config_path).expect("REASON") {
            break;
        }

    }
    ratatui::restore();
}
