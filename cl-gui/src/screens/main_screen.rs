use super::{Screen, ScreenExt};
use crate::{
    default_block,
    entities::{
        contexts::{
            application_context::ApplicationContext, namespaces_context::NamespacesContext,
            ui_context::UIContext,
        },
        terminal::{TerminalSize, TerminalSizeExt},
    },
    popup,
    widgets::{
        alias_list::AliasListWidget,
        display::DisplayWidget,
        highlight::Highlight,
        popup::{help_popup::HelpPopup, question_popup::QuestionPopup, RenderPopup},
        statusbar::{help::Help, info::Info},
    },
    DEFAULT_SELECTED_COLOR,
};
use cl_core::command::{Command, CommandBuilder};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{ListState, Tabs},
    Frame,
};

pub struct MainScreen;

impl MainScreen {
    fn get_selected_command(
        &self,
        selected_command_index: usize,
        filtered_commands: &[Command],
    ) -> Command {
        if let Some(command) = filtered_commands.get(selected_command_index) {
            command.to_owned()
        } else {
            CommandBuilder::default().build()
        }
    }

    fn create_namespace_widget<'a>(
        &self,
        namespace: &'a str,
        query: &'a str,
        should_highligh: bool,
    ) -> DisplayWidget<'a> {
        self.create_display_widget("Namespace", namespace, should_highligh)
            .highlight(query)
    }

    fn create_tab_menu_widget<'a>(&self, namespaces_context: &NamespacesContext) -> Tabs<'a> {
        let namespaces = namespaces_context.namespaces();
        let tab_menu = namespaces.iter().cloned().map(Line::from).collect();

        Tabs::new(tab_menu)
            .select(namespaces_context.get_selected_namespace_idx())
            .block(default_block!("Namespaces"))
            .highlight_style(
                Style::default()
                    .fg(DEFAULT_SELECTED_COLOR)
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider(Span::raw("|"))
    }

    fn create_command_items_widget<'a>(
        &self,
        commands: Vec<Command>,
        state: ListState,
    ) -> AliasListWidget<'a> {
        AliasListWidget::new(commands, state)
    }

    fn create_command_details_widget<'a>(
        &self,
        command: &'a str,
        query: &'a str,
        should_highligh: bool,
    ) -> DisplayWidget<'a> {
        self.create_display_widget("Command", command, should_highligh)
            .highlight(query)
    }

    fn create_command_description_widget<'a>(
        &self,
        description: &'a str,
        query: &'a str,
        should_highligh: bool,
    ) -> DisplayWidget<'a> {
        self.create_display_widget("Description", description, should_highligh)
            .highlight(query)
    }

    fn create_tags_menu_widget<'a>(
        &self,
        tags: &'a str,
        query: &'a str,
        should_highligh: bool,
    ) -> DisplayWidget<'a> {
        self.create_display_widget("Tags", tags, should_highligh)
            .highlight(query)
    }

    fn create_display_widget<'a>(
        &self,
        title: &str,
        content: &str,
        should_highligh: bool,
    ) -> DisplayWidget<'a> {
        DisplayWidget::new(content, true, should_highligh).block(default_block!(title))
    }
}

impl Screen for MainScreen {
    fn render(
        &mut self,
        frame: &mut Frame,
        context: &mut ApplicationContext,
        ui_context: &mut UIContext,
    ) {
        let filtered_commands = context.filter_commands(ui_context.get_querybox_input());
        let selected_idx = context.get_selected_command_idx();
        let selected_command = self.get_selected_command(selected_idx, &filtered_commands);

        //
        ui_context.select_command(Some(selected_command.to_owned()));

        let should_highlight = context.should_highlight();
        let query = ui_context.querybox_ref().get_input();
        let namespaces_context = context.namespaces_context();
        let command_state = context.get_commands_state();
        let tags_str = &selected_command.tags_as_string();
        let command_str = &selected_command.command;
        let description_str = &selected_command.description();
        let command = self.create_command_details_widget(command_str, &query, should_highlight);
        let tabs = self.create_tab_menu_widget(namespaces_context);
        let tags = self.create_tags_menu_widget(tags_str, &query, should_highlight);
        let namespace =
            self.create_namespace_widget(&selected_command.namespace, &query, should_highlight);
        let description =
            self.create_command_description_widget(description_str, &query, should_highlight);
        let commands = self.create_command_items_widget(filtered_commands, command_state);

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
            let help_popup = popup!(help => &ui_context.view_mode());
            frame.render_popup(help_popup, frame.size());
        }

        //
        if ui_context.show_popup() {
            let popup = popup!(question => ui_context);
            frame.render_stateful_popup(popup, frame.size(), ui_context.get_choices_state_mut());
        }

        let center = if ui_context.clipboard_state.yanked() {
            let info = Info::new("Command copied to clipboard!");
            ui_context.clipboard_state.check();

            Some(info)
        } else {
            None
        };

        let querybox = ui_context.querybox_ref();
        let help = Help::new();
        //
        self.render_base(frame, Some(querybox), center, Some(help));
    }
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

    frame.render_widget(tabs, chunks[0]);
    frame.render_widget(commands, central_chunk[0]);
    frame.render_widget(command, command_detail_chunks[0]);
    frame.render_widget(namespace, namespace_and_tags_chunk[0]);
    frame.render_widget(tags, namespace_and_tags_chunk[1]);
    frame.render_widget(description, chunks[1]);
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

    frame.render_widget(tabs, chunks[0]);
    frame.render_widget(commands, central_chunk[0]);
    frame.render_widget(command, command_detail_chunks[0]);
}
