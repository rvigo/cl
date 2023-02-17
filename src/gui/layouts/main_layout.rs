use super::{
    centered_rect, get_default_block, TerminalSize, DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR,
};
use crate::{
    command::{Command, CommandBuilder},
    gui::{
        entities::application_context::ApplicationContext,
        widgets::{
            base_widget::BaseWidget, display::DisplayWidget, help_popup::HelpPopup,
            query_box::QueryBox,
        },
    },
};
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
    application_context: &mut ApplicationContext,
    terminal_size: TerminalSize,
) {
    let query_box = &mut application_context.ui_context.query_box;
    render_base_widget(frame, query_box, &terminal_size);

    match terminal_size {
        TerminalSize::Medium => render_form_medium(frame, application_context),
        TerminalSize::Large => render_form_medium(frame, application_context),
        TerminalSize::Small => render_form_small(frame, application_context),
    }

    if application_context.show_help() {
        frame.render_widget(
            HelpPopup::new(
                application_context.ui_context.view_mode().clone(),
                terminal_size.clone(),
            ),
            frame.size(),
        );
    }

    if application_context.ui_context.popup_context.popup.is_some()
        && application_context
            .ui_context
            .popup_context
            .answer
            .is_none()
    {
        let popup = &application_context
            .ui_context
            .popup_context
            .popup
            .as_ref()
            .unwrap()
            .clone();

        //TODO move this to another place
        let area = if terminal_size != TerminalSize::Small {
            centered_rect(45, 40, frame.size())
        } else {
            frame.size()
        };

        frame.render_stateful_widget(
            popup.to_owned(),
            area,
            &mut application_context.ui_context.popup_context.choices_state,
        );
    }
}

fn render_base_widget<B: Backend>(
    frame: &mut Frame<B>,
    query_box: &QueryBox,
    terminal_size: &TerminalSize,
) {
    frame.render_widget(
        BaseWidget::new(Some(query_box), terminal_size),
        frame.size(),
    );
}

fn render_form_medium<B: Backend>(
    frame: &mut Frame<B>,
    application_context: &mut ApplicationContext,
) {
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

    let selected_command: Command = get_selected_command(application_context);

    application_context
        .ui_context
        .form_fields_context
        .select_command(Some(selected_command.clone()));

    let tags_str = selected_command.tags_as_string();

    let command_str: String = selected_command.command.clone();
    let description_str: String = selected_command
        .description
        .unwrap_or_else(|| String::from(""));
    let command = create_command_details_widget(command_str);
    let tabs = create_tab_menu_widget(application_context);
    let tags = create_tags_menu_widget(tags_str);
    let namespace = create_namespace_widget(selected_command.namespace);
    let description = create_command_description_widget(description_str);
    let commands = create_command_items_widget(application_context);

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
        &mut application_context.commands_context.commands_list_state(),
    );
    frame.render_widget(command, command_detail_chunks[0]);
    frame.render_widget(namespace, namespace_and_tags_chunk[0]);
    frame.render_widget(tags, namespace_and_tags_chunk[1]);
    frame.render_widget(description, chunks[1]);
}

fn render_form_small<B: Backend>(
    frame: &mut Frame<B>,
    application_context: &mut ApplicationContext,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(5)].as_ref())
        .split(frame.size());

    let selected_command: Command = get_selected_command(application_context);

    application_context
        .ui_context
        .form_fields_context
        .select_command(Some(selected_command.clone()));

    let command_str: String = selected_command.command;
    let command = create_command_details_widget(command_str);
    let tabs = create_tab_menu_widget(application_context);
    let commands = create_command_items_widget(application_context);

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
        &mut application_context.commands_context.commands_list_state(),
    );
    frame.render_widget(command, command_detail_chunks[0]);
}

fn get_selected_command(application_context: &mut ApplicationContext) -> Command {
    let idx = application_context
        .commands_context
        .get_selected_command_idx();

    if application_context.filter_commands().is_empty()
        || application_context.filter_commands().get(idx).is_none()
    {
        //creates an empty command
        CommandBuilder::default().build()
    } else {
        application_context
            .filter_commands()
            .get(idx)
            .unwrap()
            .to_owned()
    }
}

fn create_display_widget<'a>(title: String, content: String) -> DisplayWidget<'a> {
    DisplayWidget::new(content, true)
        .title(title.clone())
        .block(get_default_block(title))
}

fn create_tab_menu_widget<'a>(application_context: &ApplicationContext) -> Tabs<'a> {
    let tab_menu: Vec<Spans> = application_context
        .commands_context
        .commands
        .namespaces()
        .iter()
        .cloned()
        .map(|tab| Spans::from(vec![Span::styled(tab, Style::default())]))
        .collect();
    Tabs::new(tab_menu)
        .select(application_context.namespace_state.selected().unwrap())
        .block(get_default_block("Namespaces".to_string()))
        .style(Style::default())
        .highlight_style(
            Style::default()
                .fg(DEFAULT_SELECTED_COLOR)
                .add_modifier(Modifier::UNDERLINED),
        )
        .divider(Span::raw("|"))
}

fn create_command_items_widget<'a>(application_context: &mut ApplicationContext) -> List<'a> {
    let list_items: Vec<ListItem> = application_context
        .filter_commands()
        .into_iter()
        .map(|c| {
            let lines = vec![Spans::from(c.alias)];

            ListItem::new(lines.clone().to_owned()).style(Style::default().fg(DEFAULT_TEXT_COLOR))
        })
        .collect();

    List::new(list_items)
        .block(get_default_block("Aliases".to_string()))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(DEFAULT_SELECTED_COLOR)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}

fn create_command_details_widget<'a>(command: String) -> DisplayWidget<'a> {
    create_display_widget("Command".to_string(), command)
}

fn create_command_description_widget<'a>(description: String) -> DisplayWidget<'a> {
    create_display_widget("Description".to_string(), description)
}

fn create_tags_menu_widget<'a>(tags: String) -> DisplayWidget<'a> {
    create_display_widget("Tags".to_string(), tags)
}

fn create_namespace_widget<'a>(namespace: String) -> DisplayWidget<'a> {
    create_display_widget("Namespace".to_string(), namespace)
}
