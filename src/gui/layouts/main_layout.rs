use crate::{
    command::{Command, CommandBuilder},
    gui::{
        entities::state::State,
        widgets::{base_widget::BaseWidget, display::DisplayWidget, help_popup::HelpPopup},
    },
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Tabs},
    Frame,
};

use super::{get_default_block, DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR};

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let idx = state
        .commands_state
        .selected()
        .expect("Error retrieving the selected command");

    log::debug!("idx: {idx}");
    let selected_command: Command =
        if state.filter_commands().is_empty() || state.filter_commands().get(idx).is_none() {
            //creates an empty command
            CommandBuilder::default().build()
        } else {
            state.filter_commands().get(idx).unwrap().to_owned()
        };

    state
        .form_fields_context
        .select_command(Some(selected_command.clone()));

    let tags_str = selected_command.tags_as_string();

    let command_str: String = selected_command.command.clone();
    let description_str: String = selected_command
        .description
        .unwrap_or_else(|| String::from(""));
    let command = create_command_details_widget(command_str);
    let tabs = create_tab_menu_widget(state);
    let tags = create_tags_menu_widget(tags_str);
    let namespace = create_namespace_widget(selected_command.namespace);
    let description = create_command_description_widget(description_str);
    let commands = create_command_items_widget(state);

    let central_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[2]);

    let command_detail_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
        .split(central_chunk[1]);

    let namespace_and_tags_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(command_detail_chunks[1]);

    let query_box = &mut state.query_box;
    frame.render_widget(BaseWidget::new(Some(query_box)), frame.size());
    frame.render_widget(tabs, chunks[0]);
    frame.render_stateful_widget(commands, central_chunk[0], &mut state.commands_state);
    frame.render_widget(command, command_detail_chunks[0]);
    frame.render_widget(namespace, namespace_and_tags_chunk[0]);
    frame.render_widget(tags, namespace_and_tags_chunk[1]);
    frame.render_widget(description, chunks[1]);

    if state.show_help {
        frame.render_widget(HelpPopup::new(state.view_mode.clone()), frame.size());
    }

    if state.popup_context.popup.is_some() && state.popup_context.answer.is_none() {
        if let Some(popup) = &state.popup_context.popup {
            let popup = popup.clone();
            frame.render_stateful_widget(
                popup,
                frame.size(),
                &mut state.popup_context.choices_state,
            );
        }
    }
}

fn create_tab_menu_widget<'a>(state: &State) -> Tabs<'a> {
    let tab_menu: Vec<Spans> = state
        .namespaces
        .clone()
        .into_iter()
        .map(|tab| Spans::from(vec![Span::styled(tab, Style::default())]))
        .collect();
    Tabs::new(tab_menu)
        .select(state.namespace_state.selected().unwrap())
        .block(Block::default().title(" Namespaces ").borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(
            Style::default()
                .fg(DEFAULT_SELECTED_COLOR)
                .add_modifier(Modifier::UNDERLINED),
        )
        .divider(Span::raw("|"))
}

fn create_command_items_widget<'a>(state: &mut State) -> List<'a> {
    let list_items: Vec<ListItem> = state
        .filter_commands()
        .into_iter()
        .map(|c| {
            let lines = vec![Spans::from(c.alias)];

            ListItem::new(lines.clone().to_owned()).style(Style::default().fg(DEFAULT_TEXT_COLOR))
        })
        .collect();

    List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(" Aliases "))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(DEFAULT_SELECTED_COLOR)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}

//TODO Create a factory
fn create_command_details_widget<'a>(command: String) -> DisplayWidget<'a> {
    let title = "Command".to_string();
    DisplayWidget::new(command, true)
        .title(title.clone())
        .block(get_default_block(title))
}

fn create_command_description_widget<'a>(description: String) -> DisplayWidget<'a> {
    let title = "Description".to_string();
    DisplayWidget::new(description, true)
        .title(title.clone())
        .block(get_default_block(title))
}

fn create_tags_menu_widget<'a>(tags: String) -> DisplayWidget<'a> {
    let title = "Tags".to_string();
    DisplayWidget::new(tags, true)
        .title(title.clone())
        .block(get_default_block(title))
}

fn create_namespace_widget<'a>(namespace: String) -> DisplayWidget<'a> {
    let title = "Namespace".to_string();
    DisplayWidget::new(namespace, true)
        .title(title.clone())
        .block(get_default_block(title))
}
