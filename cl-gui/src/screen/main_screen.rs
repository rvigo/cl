use super::{Screen, ScreenExt};
use crate::{
    default_block, display_widget,
    entity::{
        context::{application_context::ApplicationContext, ui::UI},
        state::State,
        terminal::{TerminalSize, TerminalSizeExt},
    },
    popup, render,
    widget::{
        alias_list::AliasListWidget,
        display::DisplayWidget,
        highlight::Highlight,
        popup::{help_popup::HelpPopup, RenderPopup},
        statusbar::{help::Help, info::Info},
    },
    DEFAULT_SELECTED_COLOR,
};
use cl_core::{
    command::{Command, CommandBuilder},
    Namespace,
};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Tabs,
    Frame,
};

pub struct MainScreen;

impl Screen for MainScreen {
    fn render(&self, frame: &mut Frame, context: &mut ApplicationContext, ui_context: &mut UI) {
        let query = ui_context.query_box.get_input();
        let filtered_commands = context.filter_commands(&query);

        let selected_idx = context.commands_context.selected_command_idx();

        let selected_command = get_selected_command(selected_idx, &filtered_commands);

        //
        ui_context.select_command(Some(selected_command.to_owned()));

        let should_highlight = context.should_highlight();

        let namespaces = context.commands_context.namespaces.items.to_owned();
        let selected_namespace = context.commands_context.namespaces.state.selected();

        let command_state = context.get_commands_state();
        let command_str = &selected_command.command;
        let tags_str = &selected_command.tags_as_string();
        let description_str = &selected_command.description();

        let commands = AliasListWidget::new(filtered_commands, command_state);
        let tabs = create_namespaces_menu_widget(namespaces, selected_namespace);

        let command = display_widget!("Command", command_str, true, should_highlight, &query);
        let tags = display_widget!("Tags", tags_str, true, should_highlight, &query);
        let namespace = display_widget!(
            "Namespace",
            selected_command.namespace,
            true,
            should_highlight,
            &query
        );
        let description = display_widget!(
            "Description",
            description_str,
            true,
            should_highlight,
            &query
        );

        //
        match frame.size().as_terminal_size() {
            TerminalSize::Medium => {
                render_form_medium(frame, tabs, command, commands, namespace, tags, description)
            }
            TerminalSize::Large => {
                render_form_medium(frame, tabs, command, commands, namespace, tags, description)
            }
            TerminalSize::Small => render_form_small(frame, tabs, commands, command),
        }

        //
        if ui_context.show_help() {
            let help_popup = popup!(&ui_context.view_mode());
            frame.render_popup(help_popup, frame.size());
        }

        //
        let info = ui_context.popup_info_mut().to_owned();
        if ui_context.show_popup() {
            let popup = popup!(info, ui_context.popup_context_mut().get_available_choices());
            frame.render_stateful_popup(popup, frame.size(), ui_context.popup_context_mut());
        }

        let center = if ui_context.clipboard_state.yanked() {
            let info = Info::new("Command copied to clipboard!");
            ui_context.clipboard_state.check();

            Some(info)
        } else {
            None
        };

        let querybox = ui_context.querybox_ref();
        let help = Help::default();
        //
        self.render_base(frame, Some(querybox), center, Some(help));
    }
}

fn get_selected_command(selected_idx: usize, filtered_commands: &[Command]) -> Command {
    if let Some(command) = filtered_commands.get(selected_idx) {
        command.to_owned()
    } else {
        CommandBuilder::default().build()
    }
}

fn create_namespaces_menu_widget<'a>(namespaces: Vec<Namespace>, selected: usize) -> Tabs<'a> {
    let namespaces = namespaces.into_iter().map(Line::from).collect();

    Tabs::new(namespaces)
        .select(selected)
        .block(default_block!("Namespaces"))
        .highlight_style(
            Style::default()
                .fg(DEFAULT_SELECTED_COLOR)
                .add_modifier(Modifier::UNDERLINED),
        )
        .divider(Span::raw("|"))
}

fn render_form_medium(
    frame: &mut Frame,
    tabs: Tabs,
    command: DisplayWidget,
    commands: AliasListWidget,
    namespace: DisplayWidget,
    tags: DisplayWidget,
    description: DisplayWidget,
) {
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

    render!(
        frame,
        { tabs, chunks[0] },
        { commands, central_chunk[0] },
        { command, command_detail_chunks[0] },
        { namespace, namespace_and_tags_chunk[0] },
        { tags, namespace_and_tags_chunk[1] },
        { description, chunks[1] },
    );
}

fn render_form_small(
    frame: &mut Frame,
    tabs: Tabs,
    commands: AliasListWidget,
    command: DisplayWidget,
) {
    let constraints = [Constraint::Length(3), Constraint::Min(5)];
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(frame.size());

    let central_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);

    let command_detail_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(central_chunk[1]);
    render!(
        frame,
        { tabs, chunks[0] },
        { commands, central_chunk[0] },
        { command, command_detail_chunks[0] },
    );
}
