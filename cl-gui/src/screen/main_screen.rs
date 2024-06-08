use super::{Screen, ScreenExt};
use crate::{
    context::{Application, UI},
    default_block, display_widget, popup, render,
    terminal::{TerminalSize, TerminalSizeExt},
    widget::{
        popup::{HelpPopup, RenderPopup},
        statusbar::{Help, Info},
        AliasListWidget, DisplayWidget,
    },
    State, DEFAULT_SELECTED_COLOR,
};
use cl_core::Namespace;
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Tabs,
    Frame,
};

pub struct MainScreen;

impl Screen for MainScreen {
    fn render(&self, frame: &mut Frame, context: &mut Application, ui_context: &mut UI) {
        let query = ui_context.querybox.input();
        let filtered_commands = context.filter_commands(&query);

        let selected_idx = context.commands.selected_command_idx();
        let selected_command = filtered_commands
            .get(selected_idx)
            .expect("No command found");

        //
        ui_context.select_command(Some(selected_command));

        let should_highlight = context.should_highlight();

        let namespaces = context.namespaces.items.to_owned();
        let selected_namespace = context.namespaces.state.selected();

        let command_state = context.commands.state();
        let command_str = &selected_command.command;
        let tags_str = &selected_command.tags_as_string();
        let description_str = &selected_command.description();

        let commands = AliasListWidget::new(&filtered_commands, command_state);
        let tabs = create_namespaces_menu_widget(namespaces, selected_namespace);

        let command = display_widget!("Command", command_str, true, should_highlight, &query);
        let tags = display_widget!("Tags", tags_str, true, should_highlight, &query);
        let namespace = display_widget!(
            "Namespace",
            &selected_command.namespace,
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
            let help = popup!(&ui_context.view_mode());
            frame.render_popup(help, frame.size());
        }

        //
        if ui_context.popup.show_popup() {
            let popup_ctx = &mut ui_context.popup;
            let content = &popup_ctx.content;
            let choices = popup_ctx.choices();
            let popup = popup!(content, choices);
            frame.render_stateful_popup(popup, frame.size(), popup_ctx);
        }

        //
        let center = if ui_context.clipboard_state.yanked() {
            let info = Info::new("Command copied to clipboard!");
            ui_context.clipboard_state.check();

            Some(info)
        } else {
            None
        };
        let left = ui_context.querybox.to_owned();
        self.render_base(frame, Some(left), center, Some(Help::default()));
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
