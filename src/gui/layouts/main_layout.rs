use super::{
    centered_rect, get_default_block, TerminalSize, DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR,
};
use crate::{
    command::{Command, CommandBuilder},
    gui::{
        entities::{
            application_context::ApplicationContext, namespaces_context::NamespacesContext,
        },
        widgets::{
            base_widget::BaseWidget, display::DisplayWidget, help_footer::HelpFooter,
            help_popup::HelpPopup, query_box::QueryBox,
        },
    },
};
use parking_lot::Mutex;
use std::sync::Arc;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{List, ListItem, Tabs},
    Frame,
};

pub fn render<B: Backend>(
    frame: &mut Frame<B>,
    application_context: &mut Arc<Mutex<ApplicationContext>>,
) {
    let mut context = application_context.lock();
    let query_box = &mut context.query_box();

    render_base_widget(frame, query_box, &TerminalSize::Medium);

    let constraints = [
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Min(10),
        Constraint::Length(3),
    ];
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(frame.size());

    let filtered_commands = context.filter_commands();
    let selected_idx = context.get_selected_command_idx();
    let selected_command: Command = get_selected_command(selected_idx, &filtered_commands);
    let query = context.query_box().get_input();

    context.select_command(Some(selected_command.to_owned()));
    let should_highligh = context.should_highligh();
    let tags_str = &selected_command.tags_as_string();
    let command_str = &selected_command.command;
    let description_str = &selected_command.description();
    let command = create_command_details_widget(command_str, &query, should_highligh);
    let tabs = create_tab_menu_widget(context.namespaces_context());
    let tags = create_tags_menu_widget(tags_str, &query, should_highligh);
    let namespace = create_namespace_widget(&selected_command.namespace, &query, should_highligh);
    let description = create_command_description_widget(description_str, &query, should_highligh);
    let commands = create_command_items_widget(filtered_commands);

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

    frame.render_widget(tabs, chunks[0]);
    frame.render_stateful_widget(
        commands,
        central_chunk[0],
        &mut context.get_commands_state(),
    );
    frame.render_widget(command, command_detail_chunks[0]);
    frame.render_widget(namespace, namespace_and_tags_chunk[0]);
    frame.render_widget(tags, namespace_and_tags_chunk[1]);
    frame.render_widget(description, chunks[1]);

    if context.show_help() {
        frame.render_widget(
            HelpPopup::new(context.view_mode().to_owned(), TerminalSize::Medium),
            frame.size(),
        );
    }

    if context.popup().is_some() && context.get_popup_answer().is_none() {
        let popup = &context.popup().as_ref().unwrap().to_owned();

        //TODO move this to `UiContext`
        // let area = if terminal_size != TerminalSize::Small {
        //     centered_rect(45, 40, frame.size())
        // } else {
        //     frame.size()
        // };

        frame.render_stateful_widget(
            popup.to_owned(),
            centered_rect(45, 40, frame.size()),
            context.get_choices_state_mut(),
        );
    }
}

fn render_base_widget<B: Backend>(
    frame: &mut Frame<B>,
    query_box: &QueryBox,
    terminal_size: &TerminalSize,
) {
    frame.render_widget(
        BaseWidget::new(terminal_size, Some(query_box), HelpFooter::new()),
        frame.size(),
    );
}

fn render_form_medium<B: Backend>(frame: &mut Frame<B>, context: &mut ApplicationContext<'static>) {
    let constraints = [
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Min(10),
        Constraint::Length(3),
    ];
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(frame.size());

    let filtered_commands = context.filter_commands();
    let selected_idx = context.get_selected_command_idx();
    let selected_command: Command = get_selected_command(selected_idx, &filtered_commands);
    let query = context.query_box().get_input();

    context.select_command(Some(selected_command.to_owned()));
    let should_highligh = context.should_highligh();
    let tags_str = &selected_command.tags_as_string();
    let command_str = &selected_command.command;
    let description_str = &selected_command.description();
    let command = create_command_details_widget(command_str, &query, should_highligh);
    let tabs = create_tab_menu_widget(context.namespaces_context());
    let tags = create_tags_menu_widget(tags_str, &query, should_highligh);
    let namespace = create_namespace_widget(&selected_command.namespace, &query, should_highligh);
    let description = create_command_description_widget(description_str, &query, should_highligh);
    let commands = create_command_items_widget(filtered_commands);

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

    frame.render_widget(tabs, chunks[0]);
    frame.render_stateful_widget(
        commands,
        central_chunk[0],
        &mut context.get_commands_state(),
    );
    frame.render_widget(command, command_detail_chunks[0]);
    frame.render_widget(namespace, namespace_and_tags_chunk[0]);
    frame.render_widget(tags, namespace_and_tags_chunk[1]);
    frame.render_widget(description, chunks[1]);
}

fn render_form_small<B: Backend>(
    frame: &mut Frame<B>,
    application_context: &mut Arc<Mutex<ApplicationContext>>,
) {
    let mut context = application_context.lock();

    let constraints = [Constraint::Length(3), Constraint::Min(5)];
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(frame.size());

    let filtered_commands = context.filter_commands();
    let selected_idx = context.get_selected_command_idx();
    let selected_command: Command = get_selected_command(selected_idx, &filtered_commands);
    let query = context.query_box().get_input();

    context.select_command(Some(selected_command.clone()));
    let should_highligh = context.should_highligh();
    let command_str = &selected_command.command;
    let command = create_command_details_widget(command_str, &query, should_highligh);
    let tabs = create_tab_menu_widget(context.namespaces_context());
    let commands = create_command_items_widget(filtered_commands);

    let central_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);

    let command_detail_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(central_chunk[1]);

    frame.render_widget(tabs, chunks[0]);
    frame.render_stateful_widget(
        commands,
        central_chunk[0],
        &mut context.get_commands_state(),
    );
    frame.render_widget(command, command_detail_chunks[0]);
}

fn get_selected_command(
    selected_command_index: usize,
    filtered_commands: &Vec<Command>,
) -> Command {
    if filtered_commands.is_empty() || filtered_commands.get(selected_command_index).is_none() {
        //creates an empty command
        CommandBuilder::default().build()
    } else {
        filtered_commands
            .get(selected_command_index)
            .unwrap()
            .to_owned()
    }
}

fn create_display_widget<'a>(
    title: &str,
    content: &'a str,
    should_highligh: bool,
) -> DisplayWidget<'a> {
    DisplayWidget::new(content, true, should_highligh)
        .title(title)
        .block(get_default_block(title))
}

fn create_tab_menu_widget<'a>(namespaces_context: &NamespacesContext) -> Tabs<'a> {
    let namespaces = namespaces_context.namespaces();
    let tab_menu: Vec<Spans> = namespaces
        .iter()
        .cloned()
        .map(|tab| Spans::from(vec![Span::styled(tab, Style::default())]))
        .collect();
    Tabs::new(tab_menu)
        .select(namespaces_context.get_selected_namespace_idx())
        .block(get_default_block("Namespaces"))
        .style(Style::default())
        .highlight_style(
            Style::default()
                .fg(DEFAULT_SELECTED_COLOR)
                .add_modifier(Modifier::UNDERLINED),
        )
        .divider(Span::raw("|"))
}

fn create_command_items_widget<'a>(commands: Vec<Command>) -> List<'a> {
    let list_items: Vec<ListItem> = commands
        .into_iter()
        .map(|c| {
            let lines = vec![Spans::from(c.alias)];
            ListItem::new(lines.clone().to_owned()).style(Style::default().fg(DEFAULT_TEXT_COLOR))
        })
        .collect();

    List::new(list_items)
        .block(get_default_block("Aliases"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(DEFAULT_SELECTED_COLOR)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}

fn create_command_details_widget<'a>(
    command: &'a str,
    query: &'a str,
    should_highligh: bool,
) -> DisplayWidget<'a> {
    create_display_widget("Command", command, should_highligh).highlight(query)
}

fn create_command_description_widget<'a>(
    description: &'a str,
    query: &'a str,
    should_highligh: bool,
) -> DisplayWidget<'a> {
    create_display_widget("Description", description, should_highligh).highlight(query)
}

fn create_tags_menu_widget<'a>(
    tags: &'a str,
    query: &'a str,
    should_highligh: bool,
) -> DisplayWidget<'a> {
    create_display_widget("Tags", tags, should_highligh).highlight(query)
}

fn create_namespace_widget<'a>(
    namespace: &'a str,
    query: &'a str,
    should_highligh: bool,
) -> DisplayWidget<'a> {
    create_display_widget("Namespace", namespace, should_highligh).highlight(query)
}
