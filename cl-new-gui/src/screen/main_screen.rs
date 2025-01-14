use crate::component::{Component, List, StatefulComponent, Tabs, TextBox, TextBoxName};
use crate::observer::listener::{Listener, ListenerId};
use crate::observer::publisher::{
    ListPublisher, Publisher, PublisherContainer, TabsPublisher, TextBoxPublisher,
};
use crate::screen::Screen;
use crate::{render, SharedCell};
use std::collections::HashMap;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::prelude::{Modifier, Style, Text};
use tui::style::Color as TuiColor;
use tui::widgets::{Block, BorderType, Borders, Padding, Paragraph};
use tui::Frame;

pub const DEFAULT_TEXT_COLOR: TuiColor = TuiColor::Rgb(205, 214, 244);
pub const DEFAULT_WIDGET_NAME_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
pub const DEFAULT_SELECTED_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
pub const DEFAULT_HIGHLIGHT_COLOR: TuiColor = TuiColor::Rgb(180, 190, 254);
pub const DEFAULT_BACKGROUND_COLOR: TuiColor = TuiColor::Rgb(30, 30, 46);
pub const DEFAULT_INFO_COLOR: TuiColor = TuiColor::Rgb(148, 226, 213);
pub const DEFAULT_CURSOR_COLOR: TuiColor = TuiColor::Rgb(245, 224, 220);
pub const DEFAULT_INACTIVE_TEXTBOX_COLOR: TuiColor = TuiColor::Rgb(108, 112, 134);

pub struct MainScreen {
    pub command: SharedCell<TextBox>,
    pub description: SharedCell<TextBox>,
    pub tags: SharedCell<TextBox>,
    pub namespace: SharedCell<TextBox>,
    pub list: SharedCell<List>,
    pub tabs: SharedCell<Tabs>,
    pub publishers: HashMap<ListenerId, PublisherContainer>,
}

impl Screen for MainScreen {
    fn new() -> Self {
        let command = TextBox {
            name: TextBoxName::Command,
            ..Default::default()
        };
        let description = TextBox {
            name: TextBoxName::Description,
            ..Default::default()
        };
        let tags = TextBox {
            name: TextBoxName::Tags,
            ..Default::default()
        };
        let namespace = TextBox {
            name: TextBoxName::Namespace,
            ..Default::default()
        };

        let list = List::new();

        let command_refcell = SharedCell::new(command);
        let description_refcell = SharedCell::new(description);
        let tags_refcell = SharedCell::new(tags);
        let namespace_refcell = SharedCell::new(namespace);
        let list_refcell = SharedCell::new(list);

        // texbox
        let mut textbox_publisher = TextBoxPublisher::default();
        textbox_publisher.register(SharedCell::clone(&command_refcell));
        textbox_publisher.register(SharedCell::clone(&description_refcell));
        textbox_publisher.register(SharedCell::clone(&tags_refcell));
        textbox_publisher.register(SharedCell::clone(&namespace_refcell));

        // list
        let mut list_publisher = ListPublisher::new();
        list_publisher.register(SharedCell::clone(&list_refcell));

        // tabs
        let tabs = Tabs::new();
        let tabs_refcell = SharedCell::new(tabs);

        let mut tabs_publisher = TabsPublisher::new();
        tabs_publisher.register(SharedCell::clone(&tabs_refcell));
        
        // publisher container
        let mut map: HashMap<ListenerId, PublisherContainer> = HashMap::new();
        map.insert(List::get_id(), PublisherContainer::List(list_publisher));
        map.insert(
            TextBox::get_id(),
            PublisherContainer::TextBox(textbox_publisher),
        );
        map.insert(Tabs::get_id(), PublisherContainer::Tabs(tabs_publisher));

        Self {
            command: SharedCell::clone(&command_refcell),
            description: SharedCell::clone(&description_refcell),
            tags: SharedCell::clone(&tags_refcell),
            namespace: SharedCell::clone(&namespace_refcell),
            list: SharedCell::clone(&list_refcell),
            tabs: SharedCell::clone(&tabs_refcell),
            publishers: map,
        }
    }

    fn render(&mut self, frame: &mut Frame) {
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

        let [namespace_area, tags_area] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(right_side[2])
        else {
            panic!() // TODO improve this
        };

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

        render! {
                frame,
                {app_name, left_side[0]},
                // {aliases,  left_side[1]},
        }

        self.list.borrow_mut().render_stateful(frame, left_side[1]);
        self.description.borrow().render(frame, right_side[1]); // middle
        self.command.borrow().render(frame, right_side[3]);
        self.namespace.borrow().render(frame, namespace_area);
        self.tags.borrow().render(frame, tags_area);
        self.tabs.borrow().render(frame, right_side[0])
    }

    fn get_publisher(&mut self, id: ListenerId) -> &mut PublisherContainer {
        if let Some(p) = self.publishers.get_mut(&id) {
            p
        } else {
            panic!("Publisher not found for {}", id.0)
        }
    }
}
