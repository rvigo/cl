use crate::context::{Application, UI};
use crate::screen::command_publisher::CommandPublisher;
use crate::screen::listener::{Event, Publisher};
use crate::screen::Screen;
use crate::state::{CommandListState, State};
use crate::terminal::TerminalSize;
use crate::theme::{DEFAULT_BACKGROUND_COLOR, DEFAULT_TEXT_COLOR, DEFAULT_WIDGET_NAME_COLOR};
use crate::widget::statusbar::Help;
use crate::widget::tabs::Tabs;
use crate::widget::text_field::FieldType;
use crate::widget::{ClibpoardWidget, Component, DisplayWidget, CommandList};
use crate::{default_commands, default_display_widget, default_tabs, maybe_render, render};
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
    command_publisher: CommandPublisher<'m>,
    command: Rc<RefCell<DisplayWidget<'m>>>,
    tags: Rc<RefCell<DisplayWidget<'m>>>,
    namespace: Rc<RefCell<DisplayWidget<'m>>>,
    description: Rc<RefCell<DisplayWidget<'m>>>,
    commands: CommandList<'m>,
    tabs: Tabs<'m>,
}

impl<'m> MainScreen<'m> {
    pub fn new() -> MainScreen<'m> {
        let command = default_display_widget!(FieldType::Command);
        let tags = default_display_widget!(FieldType::Tags);
        let namespace = default_display_widget!(FieldType::Namespace);
        let description = default_display_widget!(FieldType::Description);

        let commands = default_commands!();
        let tabs = default_tabs!();

        let command_refcell = Rc::new(RefCell::new(command));
        let tags_refcell = Rc::new(RefCell::new(tags));
        let namespace_refcell = Rc::new(RefCell::new(namespace));
        let description_refcell = Rc::new(RefCell::new(description));

        let mut command_publisher = CommandPublisher::new();

        command_publisher.register(Rc::clone(&command_refcell));
        command_publisher.register(Rc::clone(&tags_refcell));
        command_publisher.register(Rc::clone(&namespace_refcell));
        command_publisher.register(Rc::clone(&description_refcell));

        MainScreen {
            command_publisher,
            command: Rc::clone(&command_refcell),
            tags: Rc::clone(&tags_refcell),
            namespace: Rc::clone(&namespace_refcell),
            description: Rc::clone(&description_refcell),
            commands,
            tabs,
        }
    }
}

impl<'m> Screen<'m> for MainScreen<'m> {
    fn render(&mut self, frame: &mut Frame, application: &mut Application<'m>, ui: &mut UI<'m>) {
        let querybox = ui.querybox.to_owned();
        let query = querybox.input();

        // aliases
        let list_state = application.commands.state();
        let filtered_commands = application.filter(&query);
        self.commands.update(&filtered_commands, list_state);

        //
        let selected_command = application.get_current_command();
        ui.select_command(Some(&selected_command));

        // namespaces
        let namespace_context = &application.namespaces;
        self.tabs.update(
            namespace_context.items.to_owned(),
            namespace_context.selected(),
        );

        let should_highlight = application.should_highlight();
        let event = Event::new(selected_command, should_highlight, query);

        // widgets
        self.command_publisher.notify(event);

        match frame.size().into() {
            TerminalSize::Medium | TerminalSize::Large => render_medium_size(
                frame,
                self.tabs.to_owned(),
                self.command.borrow().to_owned(),
                self.commands.to_owned(),
                self.namespace.borrow().to_owned(),
                self.tags.borrow().to_owned(),
                self.description.borrow().to_owned(),
                querybox.to_owned(),
                ClibpoardWidget::new(&mut ui.clipboard_state),
                Help::new(),
            ),
            TerminalSize::Small => render_form_small(
                frame,
                self.tabs.to_owned(),
                self.commands.to_owned(),
                self.command.borrow().to_owned(),
                querybox,
            ),
        }

        maybe_render! { frame , {ui.popup.active_popup(), frame.size(), &mut ui.popup} };
    }
}

#[allow(clippy::too_many_arguments)]
fn render_medium_size(
    frame: &mut Frame,
    tabs: impl Component,
    command: impl Component,
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
        Constraint::Max(3),    // details
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

    render! { frame, {center, statusbar_layout[1]} };

    render! { frame, {footer, drawable_chunks[1]} };
    render! {
            frame,
            {left,  statusbar_layout[0]},
            {right, statusbar_layout[2]}
    };

    render! {
            frame,
            {app_name, left_side[0]},
            {aliases,  left_side[1]},
    }
    render! {
            frame,
            {tabs,        right_side[0]}, // top
            {description, right_side[1]}, // middle
            {command,     right_side[3]},
            {namespace,   details_chunks[0]},
            {tags,        details_chunks[1]},
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
            {querybox_block, left_side[0]},
            {querybox, left_side[0]},
            {commands, left_side[1]}
    }

    render! {
            frame,
            {tabs, right_side[0]},
            {command, right_side[1]},
    }
}
