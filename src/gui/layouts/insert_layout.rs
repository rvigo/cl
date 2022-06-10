use crate::gui::structs::state::State;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use super::cursor::set_cursor_positition;

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);
    let thidr_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    render_tags_input_widget(frame, state, thidr_row[1]);
    render_namespace_input_widget(frame, state, first_row[1]);
    render_alias_input_widget(frame, state, first_row[0]);
    render_commannd_input_widget(frame, state, second_row[0]);
    render_description_input_widget(frame, state, thidr_row[0]);
}

//TODO factory????
fn render_tags_input_widget<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, area: Rect) {
    let component_name = "tags";
    let widget = Paragraph::new(
        state
            .get_mut_ref()
            .insert_context
            .get_component_input(component_name),
    )
    .style(get_style(state, component_name))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Tags ")
            .border_type(BorderType::Plain),
    );

    frame.render_widget(widget, area);
    if state.insert_context.is_in_focus(component_name) {
        set_cursor_positition(frame, state, area)
    }
}

fn render_namespace_input_widget<'a, B: Backend>(
    frame: &mut Frame<B>,
    state: &mut State,
    area: Rect,
) {
    let component_name = "namespace";

    let widget = Paragraph::new(
        state
            .get_mut_ref()
            .insert_context
            .get_component_input(component_name),
    )
    .style(get_style(state.get_mut_ref(), component_name))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Namespace ")
            .border_type(BorderType::Plain),
    );
    frame.render_widget(widget, area);
    if state.insert_context.is_in_focus(component_name) {
        set_cursor_positition(frame, state, area)
    }
}
fn render_commannd_input_widget<'a, B: Backend>(
    frame: &mut Frame<B>,
    state: &mut State,
    area: Rect,
) {
    let component_name = "command";

    let widget = Paragraph::new(
        state
            .get_mut_ref()
            .insert_context
            .get_component_input(component_name),
    )
    .style(get_style(state.get_mut_ref(), component_name))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Command ")
            .border_type(BorderType::Plain),
    );
    frame.render_widget(widget, area);
    if state.insert_context.is_in_focus(component_name) {
        set_cursor_positition(frame, state, area)
    }
}
fn render_alias_input_widget<'a, B: Backend>(frame: &mut Frame<B>, state: &mut State, area: Rect) {
    let component_name = "alias";

    let widget = Paragraph::new(
        state
            .get_mut_ref()
            .insert_context
            .get_component_input(component_name),
    )
    .style(get_style(state.get_mut_ref(), component_name))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Alias ")
            .border_type(BorderType::Plain),
    );
    frame.render_widget(widget, area);
    if state.insert_context.is_in_focus(component_name) {
        set_cursor_positition(frame, state, area)
    }
}
fn render_description_input_widget<'a, B: Backend>(
    frame: &mut Frame<B>,
    state: &mut State,
    area: Rect,
) {
    let component_name = "description";

    let widget = Paragraph::new(
        state
            .get_mut_ref()
            .insert_context
            .get_component_input(component_name),
    )
    .style(get_style(state.get_mut_ref(), component_name))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Descritpion ")
            .border_type(BorderType::Plain),
    );
    frame.render_widget(widget, area);
    if state.insert_context.is_in_focus(component_name) {
        set_cursor_positition(frame, state, area)
    }
}

fn get_style(state: &mut State, component_name: &str) -> Style {
    if state.insert_context.is_in_focus(component_name) {
        Style::default()
            .fg(Color::Rgb(229, 229, 229))
            .bg(Color::Rgb(201, 165, 249))
    } else {
        Style::default().fg(Color::Rgb(229, 229, 229))
    }
}
