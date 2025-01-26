use crate::component::{Component, List, Tabs, TextBox, TextBoxName};
use crate::render;
use crate::screen::layer::Layer;
use crate::screen::Listeners;
use std::any::TypeId;
use std::collections::BTreeMap;
use tui::layout::{Constraint, Direction, Layout};
use tui::prelude::Style;
use tui::style::Color as TuiColor;
use tui::widgets::{Block, Borders};
use tui::Frame;

pub const DEFAULT_TEXT_COLOR: TuiColor = TuiColor::Rgb(205, 214, 244);
pub const DEFAULT_WIDGET_NAME_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
pub const DEFAULT_SELECTED_COLOR: TuiColor = TuiColor::Rgb(203, 166, 247);
pub const DEFAULT_HIGHLIGHT_COLOR: TuiColor = TuiColor::Rgb(180, 190, 254);
pub const DEFAULT_BACKGROUND_COLOR: TuiColor = TuiColor::Rgb(30, 30, 46);
pub const DEFAULT_INFO_COLOR: TuiColor = TuiColor::Rgb(148, 226, 213);
pub const DEFAULT_CURSOR_COLOR: TuiColor = TuiColor::Rgb(245, 224, 220);
pub const DEFAULT_INACTIVE_TEXTBOX_COLOR: TuiColor = TuiColor::Rgb(108, 112, 134);

pub struct MainScreenLayer {
    pub command: Component,
    pub description: Component,
    pub tags: Component,
    pub namespace: Component,
    pub list: Component,
    pub tabs: Component,
    pub listeners: Listeners,
    pub app_name: Component,
}

impl Layer for MainScreenLayer {
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
        let tabs = Tabs::new();

        // components
        let mut listeners = Listeners::new();

        let command_shared = Component::new(command);
        let description_shared = Component::new(description);
        let tags_shared = Component::new(tags);
        let namespace_shared = Component::new(namespace);
        let tabs_shared = Component::new(tabs);
        let list_shared = Component::new(list);

        listeners.insert(
            TypeId::of::<TextBox>(),
            vec![
                command_shared.clone(),
                description_shared.clone(),
                tags_shared.clone(),
                namespace_shared.clone(),
            ],
        );
        listeners.insert(TypeId::of::<Tabs>(), vec![tabs_shared.clone()]);
        listeners.insert(TypeId::of::<List>(), vec![list_shared.clone()]);

        // static value
        let app_name = Component::new(TextBox {
            name: TextBoxName::Command,
            content: Some(format!("cl - {}", env!("CARGO_PKG_VERSION"))),
        });

        Self {
            command: command_shared.clone(),
            description: description_shared.clone(),
            tags: tags_shared.clone(),
            namespace: namespace_shared.clone(),
            list: list_shared.clone(),
            tabs: tabs_shared.clone(),
            listeners,
            app_name,
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

        let [app_name_rect, list_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Length(5)])
            .split(main_chunks[0])
        else {
            todo!()
        };

        let [tabs_rect, description_rect, details_rect, command_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(main_chunks[1])
        else {
            todo!()
        };

        let [namespace_area, tags_area] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(details_rect)
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
            { self.app_name, app_name_rect},
            { self.list, list_rect },
            { self.tabs, tabs_rect },
            { self.description, description_rect},
            { self.namespace, namespace_area },
            { self.tags, tags_area },
            { self.command, command_rect },
        }
    }

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Component>> {
        self.listeners.clone()
    }
}
