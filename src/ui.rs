use ratatui::{prelude::*, style::Style, widgets::*, Frame};

use crate::app::App;

/// Renders the user interface.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let text: Vec<Line> = vec![Line::from(vec![
        "Press ".into(),
        "Esc".bold(),
        " or ".into(),
        "Ctrl+C".bold(),
        " to quit, and ".into(),
        "Enter".bold(),
        " to submit your query.".into(),
    ])];
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, chunks[0]);

    let width = chunks[0].width.max(3) - 3;

    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .style(Style::default().white().not_bold())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search (Anime)")
                .style(match app.editing {
                    false => Style::default(),
                    true => Style::default().yellow().bold(),
                }),
        );
    frame.render_widget(input, chunks[1]);

    let raw_results = &*app.list.items;
    let mut results: Vec<String> = Vec::new();
    for result in raw_results {
        results.push(format!("{}\n{}", result.title, result.releaseDate));
    }

    // Render the cursor in the input widget when writing the search query
    match app.editing {
        false => {}
        true => {
            frame.set_cursor(
                chunks[1].x
                    + ((app.input.visual_cursor()).max(scroll) - scroll) as u16
                    + 1,
                chunks[1].y + 1,
            )
        }
    }

    let list = List::new(results)
        .block(
            Block::bordered()
                .title("Search Results")
                .style(match app.editing {
                    true => Style::default(),
                    false => Style::default().yellow().bold(),
                }),
        )
        .style(Style::default().white().not_bold())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("┃ ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, chunks[2], &mut app.list.state);
}
