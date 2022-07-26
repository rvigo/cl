use super::{
    cursor::set_cursor_positition,
    help_layout::{render_help, render_helper_footer},
    popup_layout::render_popup,
};
use crate::gui::contexts::{context::Item, state::State};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Max(3),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let insert_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(" Insert ")
        .border_type(BorderType::Plain);

    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);
    let third_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);
    let fourth_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[3]);

    frame.render_widget(insert_block, frame.size());

    for item in state.context.items().iter() {
        match item.name() {
            "tags" => render_widget(frame, state, third_row[1], item),
            "namespace" => render_widget(frame, state, first_row[1], item),
            "alias" => render_widget(frame, state, first_row[0], item),
            "command" => render_widget(frame, state, second_row[0], item),
            "description" => render_widget(frame, state, third_row[0], item),
            _ => {}
        }
    }

    frame.render_widget(render_helper_footer(), fourth_row[0]);

    if state.show_help {
        render_help(frame, state)
    }
    if state.popup.show_popup {
        render_popup(frame, state);
    }
}

fn render_widget<B: Backend>(frame: &mut Frame<B>, state: &State, area: Rect, item: &Item) {
    let widget = Paragraph::new(state.context.get_component_input(item.name()))
        .style(get_style(state, item.name()))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(item.title())
                .border_type(BorderType::Plain),
        );

    frame.render_widget(widget, area);
    if state.context.is_in_focus(item.name()) {
        set_cursor_positition(frame, state.context.get_current_in_focus().unwrap(), area)
    }
}

fn get_style(state: &State, component_name: &str) -> Style {
    if state.context.is_in_focus(component_name) {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Rgb(201, 165, 249))
    } else {
        Style::default().fg(Color::Rgb(229, 229, 229))
    }
}
