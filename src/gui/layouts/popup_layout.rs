use crate::gui::entities::{
    popup::{Answer, MessageType},
    state::State,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render_popup<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let block = Paragraph::new(state.popup.message.clone())
        .style(Style::default().fg(Color::Rgb(229, 229, 229)))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(state.popup.message_type.to_string())
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Plain),
        );

    let area = centered_rect(45, 45, frame.size());

    frame.render_widget(Clear, area[1]);
    frame.render_widget(block, area[1]);

    match state.popup.message_type {
        MessageType::Error => {
            state.popup.options = vec![Answer::Ok];
            draw_option_buttons(frame, area[1], state)
        }

        MessageType::Delete => {
            state.popup.options = vec![Answer::Ok, Answer::Cancel];
            draw_option_buttons(frame, area[1], state)
        }
        MessageType::None => {}
    }
}

fn draw_option_buttons<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &mut State) {
    let layout = create_buttom_area(area, state);

    let tab_menu: Vec<Spans> = state
        .popup
        .options
        .iter()
        .map(|tab| Spans::from(vec![Span::styled(tab.to_string(), Style::default())]))
        .collect();

    let tab = Tabs::new(tab_menu)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default())
        .select(state.popup.options_state.selected().unwrap())
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(201, 165, 249))
                .add_modifier(Modifier::UNDERLINED),
        )
        .divider(Span::raw(""));

    frame.render_widget(tab, layout[layout.len() - 1]);
}

fn create_buttom_area(area: Rect, state: &mut State) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(create_buttom_layout(area)[4]);

    let constraints = if state.popup.options.len() == 2 {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]
    };
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(layout[0])
}

// uses the lower right space to render buttons
fn create_buttom_layout(area: Rect) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(area);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(layout[3])
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Vec<Rect> {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])
}
