use super::Screen;
use crate::{
    context::{Application, UI},
    default_widget_block, display_widget, popup, render,
    terminal::{TerminalSize, TerminalSizeExt},
    theme::{
        DEFAULT_BACKGROUND_COLOR, DEFAULT_HIGHLIGHT_COLOR, DEFAULT_TEXT_COLOR,
        DEFAULT_WIDGET_NAME_COLOR,
    },
    widget::{
        clipboard::ClibpoardWidget,
        list::List,
        popup::{HelpPopup, RenderPopup},
        statusbar::{Help, QueryBox},
        DisplayWidget,
    },
    State,
};
use cl_core::{CommandBuilder, Namespace};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Tabs},
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
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| CommandBuilder::default().build())
            .to_owned();

        //
        ui_context.select_command(Some(&selected_command));

        let should_highlight = context.should_highlight();

        let namespaces = context.namespaces.items.to_owned();
        let selected_namespace = context.namespaces.state.selected();

        let command_state = context.commands.state();
        let command_str = &selected_command.command;
        let tags_str = &selected_command.tags_as_string();
        let description_str = &selected_command.description();

        let aliases = List::new(&filtered_commands, command_state);
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

        let left = ui_context.querybox.to_owned();

        let clipboard = ClibpoardWidget::new(&mut ui_context.clipboard_state);

        let right = Help::new();
        //
        match frame.size().as_terminal_size() {
            TerminalSize::Medium => render_medium_size(
                frame,
                tabs,
                command,
                aliases,
                namespace,
                tags,
                description,
                left,
                clipboard,
                right,
            ),
            TerminalSize::Large => render_medium_size(
                frame,
                tabs,
                command,
                aliases,
                namespace,
                tags,
                description,
                left,
                clipboard,
                right,
            ),
            TerminalSize::Small => render_form_small(frame, tabs, aliases, command, left),
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
            let choices = &popup_ctx.choices().to_owned();
            let popup = popup!(content, choices);
            frame.render_stateful_popup(popup, frame.size(), popup_ctx);
        }
    }
}

fn create_namespaces_menu_widget<'a>(namespaces: Vec<Namespace>, selected: usize) -> Tabs<'a> {
    let namespaces = namespaces.into_iter().map(Line::from).collect();

    Tabs::new(namespaces)
        .select(selected)
        .block(default_widget_block!("Namespaces"))
        .highlight_style(
            Style::default()
                .fg(DEFAULT_HIGHLIGHT_COLOR)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        )
        .divider(Span::raw("|"))
}

#[allow(clippy::too_many_arguments)]
fn render_medium_size(
    frame: &mut Frame,
    tabs: Tabs,
    command: DisplayWidget,
    aliases: List,
    namespace: DisplayWidget,
    tags: DisplayWidget,
    description: DisplayWidget,
    left: QueryBox,
    center: ClibpoardWidget,
    right: Help,
) {
    let drawable_area = [
        Constraint::Length(5), // drawable area
        Constraint::Max(3),    // footer
    ];
    let areas = [
        Constraint::Percentage(25), // name & aliases
        Constraint::Percentage(75), // right side
    ];

    let constraints = [
        Constraint::Max(3),    // tabs
        Constraint::Max(5),    // description
        Constraint::Max(3),    // details // TODO adjust how the details are displayed
        Constraint::Length(3), // command
    ];

    let drawable_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(drawable_area)
        .split(frame.size());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(areas)
        .split(drawable_chunks[0]);

    let left_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Length(5)])
        .split(main_chunks[0]);

    let app_name = Paragraph::new(Text::styled(
        format!("cl - {}", env!("CARGO_PKG_VERSION")),
        Style::default()
            .fg(DEFAULT_WIDGET_NAME_COLOR)
            .add_modifier(Modifier::BOLD | Modifier::ITALIC),
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .style(
                Style::default()
                    .bg(DEFAULT_BACKGROUND_COLOR)
                    .fg(DEFAULT_TEXT_COLOR),
            )
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2)),
    );

    let right_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(main_chunks[1]);

    let details_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(right_side[2]);

    //
    let footer = Block::default()
        .borders(Borders::BOTTOM | Borders::RIGHT)
        .style(
            Style::default()
                .bg(DEFAULT_BACKGROUND_COLOR)
                .fg(DEFAULT_TEXT_COLOR),
        );

    let statusbar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(footer.inner(drawable_chunks[1]));

    render! {frame, {center, statusbar_layout[1]}};

    render! { frame, { footer, drawable_chunks[1] }};
    render! {
        frame,
        { left,  statusbar_layout[0] },
        { right, statusbar_layout[2] }
    };

    render! {
        frame,
        { app_name, left_side[0]},
        { aliases,  left_side[1]},
    }

    render! {
        frame,
        { tabs,        right_side[0] }, // top
        { description, right_side[1] }, // middle
        { command,     right_side[3] },
        { namespace,   details_chunks[0] },
        { tags,        details_chunks[1] },
    }
}

fn render_form_small(
    frame: &mut Frame,
    tabs: Tabs,
    commands: List,
    command: DisplayWidget,
    querybox: QueryBox,
) {
    let areas = [
        Constraint::Percentage(25), // name & aliases
        Constraint::Percentage(75), // right side
    ];
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(areas)
        .split(frame.size());

    let left_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Length(5)])
        .split(chunks[0]);

    let right_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    let querybox_block = Block::default()
        .borders(Borders::TOP | Borders::RIGHT)
        .style(
            Style::default()
                .bg(DEFAULT_BACKGROUND_COLOR)
                .fg(DEFAULT_TEXT_COLOR),
        )
        .border_type(BorderType::Rounded);

    render! {
        frame,
        { querybox_block, left_side[0] },
        { querybox, left_side[0] },
        { commands, left_side[1] }
    }

    render! {
        frame,
        { tabs, right_side[0] },
        { command, right_side[1] },
    }
}
