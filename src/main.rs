mod config;
mod draw;
mod input;

fn main() {
    let mut terminal = ratatui::init();
    let config_path = config::check_config_path::get_dotconfig_path("wonderful", "wonderful.toml");
    config::check_config_path::ensure_config_exists(&config_path);
    let mut selected = 0;
    let mut find_string = String::from("");

    let items = config::scan_toml::get_items_name(&config_path);
    let mut focus = String::from("find_section");

    loop {
        //draw the ratatui
        let _ =
            terminal.draw(|f| draw::draw::draw(f, selected, &items, &mut find_string, &mut focus));

        //cheking the keyboard input
        if input::keyboard_push::keyboard_push(
            &mut selected,
            &items,
            &config_path,
            &mut find_string,
            &mut focus,
        )
        .expect("REASON")
        {
            break;
        }
    }
    ratatui::restore();
}
