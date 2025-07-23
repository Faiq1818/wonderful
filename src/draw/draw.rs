use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(
    frame: &mut Frame,
    selected: usize,
    items: &[String],
    find_string: &mut String,
    focus: &mut String,
) {
    let find_section_border = if *focus == "find_section" {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::White)
    };

    let apps_section_border = if *focus == "apps_section" {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::White)
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(frame.area());

    let mut text = Text::default();
    for (i, item) in items.iter().enumerate() {
        if i == selected {
            text.extend(Text::styled(
                format!("--> {}\n", item),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            text.extend(Text::raw(format!("  {}\n", item)));
        }
    }

    frame.render_widget(
        Paragraph::new(find_string.clone())
            .block(
                Block::default()
                    .title("Find")
                    .borders(Borders::ALL)
                    .border_style(find_section_border),
            )
            .alignment(Alignment::Center),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new(text).block(
            Block::default()
                .title("Apps")
                .borders(Borders::ALL)
                .border_style(apps_section_border),
        ),
        layout[1],
    );
}
