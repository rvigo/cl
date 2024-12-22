use super::{observer::Subject, Screen};
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
        statusbar::Help,
        tabs::Tabs,
        text_field::FieldType::{self},
        Component, DisplayWidget,
    },
    State,
};
use cl_core::{CommandBuilder, Namespace};
use std::{cell::RefCell, rc::Rc};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

#[derive(Clone)]
pub struct MainScreen<'m> {
    pub list_subject: Subject<DisplayWidget<'m>>,
    command: Rc<RefCell<DisplayWidget<'m>>>,
    tags: Rc<RefCell<DisplayWidget<'m>>>,
    namespace: Rc<RefCell<DisplayWidget<'m>>>,
    description: Rc<RefCell<DisplayWidget<'m>>>,
}

impl<'m> MainScreen<'m> {
    pub fn new() -> Self {
        let mut list_subject = Subject::default();

        let command = display_widget!("Command", "", true, true, "");
        let tags = display_widget!("Tags", "", true, true, "");
        let namespace = display_widget!("Namespace", "", true, true, "");
        let description = display_widget!("Description", "", true, true, "");

        let command_refcell = Rc::new(RefCell::new(command));
        let tags_refcell = Rc::new(RefCell::new(tags));
        let namespace_refcell = Rc::new(RefCell::new(namespace));
        let description_refcell = Rc::new(RefCell::new(description));

        list_subject.register(FieldType::Command, Rc::clone(&command_refcell));
        list_subject.register(FieldType::Tags, tags_refcell.to_owned());
        list_subject.register(FieldType::Namespace, namespace_refcell.to_owned());
        list_subject.register(FieldType::Description, description_refcell.to_owned());

        let screen = MainScreen {
            list_subject,
            command: Rc::clone(&command_refcell),
            tags: tags_refcell,
            namespace: namespace_refcell,
            description: description_refcell,
        };

        screen
    }
}

impl<'m> Screen for MainScreen<'m> {
    fn render(&mut self, frame: &mut Frame, application: &mut Application, ui: &mut UI) {
        let query = ui.querybox.input();
        let filtered_commands = application.filter_commands(&query);
        let selected_idx = application.commands.selected_command_idx();
        let selected_command = filtered_commands
            .get(selected_idx)
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| CommandBuilder::default().build())
            .to_owned();

        //
        ui.select_command(Some(&selected_command));

        let should_highlight = application.should_highlight();

        let namespaces = application.namespaces.items.to_owned();
        let selected_namespace = application.namespaces.state.selected();

        let command_state = application.commands.state();

        let aliases = List::new(&filtered_commands, command_state);
        let tabs = create_tabs(namespaces, selected_namespace);

        self.list_subject.notify(&selected_command);

        let querybox = ui.querybox.to_owned();

        match frame.size().as_terminal_size() {
            TerminalSize::Medium | TerminalSize::Large | TerminalSize::Small => render_medium_size(
                frame,
                tabs,
                self.command.borrow().to_owned(),
                aliases,
                self.namespace.borrow().to_owned(),
                self.tags.borrow().to_owned(),
                self.description.borrow().to_owned(),
                querybox,
                ClibpoardWidget::new(&mut ui.clipboard_state),
                Help::new(),
            ),
            // TerminalSize::Small => render_form_small(frame, tabs, aliases, command, querybox),
        }

        //
        if ui.show_help() {
            let help = popup!(&ui.view_mode());
            frame.render_popup(help, frame.size());
        }

        //
        if ui.popup.show_popup() {
            let popup_ctx = &mut ui.popup;
            let content = &popup_ctx.content;
            let choices = &popup_ctx.choices().to_owned();
            let popup = popup!(content, choices);
            frame.render_stateful_popup(popup, frame.size(), popup_ctx);
        }
    }
}

fn create_tabs<'a>(namespaces: Vec<Namespace>, selected: usize) -> Tabs<'a> {
    Tabs::new(namespaces)
        .select(selected)
        .block(default_widget_block!("Namespaces"))
        .highlight_style(
            Style::default()
                .fg(DEFAULT_HIGHLIGHT_COLOR)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        )
        .divider('|')
}

#[allow(clippy::too_many_arguments)]
fn render_medium_size(
    frame: &mut Frame,
    tabs: impl Component,
    command: DisplayWidget,
    aliases: impl Component,
    namespace: impl Component,
    tags: impl Component,
    description: impl Component,
    left: impl Component,
    center: impl Component,
    right: impl Component,
) {
    let drawable_area = [
        Constraint::Length(5), // drawable area
        Constraint::Max(3),    // footer
    ];
    let areas = [
        Constraint::Percentage(20), // name & aliases
        Constraint::Percentage(80), // right side
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
    tabs: impl Component,
    commands: impl Component,
    command: impl Component,
    querybox: impl Component,
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
