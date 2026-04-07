use crate::clipboard::Clipboard;
use crate::component::{ClipboardStatus, List, Renderable, StaticInfo, Tabs, TextBox};
use crate::state::state_event::FieldName;
use crate::component::{RenderableComponent, Search};
use crate::observer::observable::Observable;
use crate::render;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::screen::Listeners;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::layout::{Constraint, Direction, Layout};
use tui::prelude::Style;
use tui::widgets::Block;
use tui::Frame;

pub struct MainScreenLayer {
    pub command: RenderableComponent<TextBox>,
    pub description: RenderableComponent<TextBox>,
    pub tags: RenderableComponent<TextBox>,
    pub namespace: RenderableComponent<TextBox>,
    pub list: RenderableComponent<List>,
    pub tabs: RenderableComponent<Tabs>,
    pub clipboard: RenderableComponent<ClipboardStatus>,
    pub quick_search: RenderableComponent<TextBox>,
    pub listeners: Listeners,
    pub app_name: StaticInfo,
    pub help: StaticInfo,
}

impl Default for MainScreenLayer {
    fn default() -> Self {
        let command = TextBox {
            name: FieldName::Command,
            ..Default::default()
        };
        let description = TextBox {
            name: FieldName::Description,
            ..Default::default()
        };
        let tags = TextBox {
            name: FieldName::Tags,
            ..Default::default()
        };
        let namespace = TextBox {
            name: FieldName::Namespace,
            ..Default::default()
        };

        let quick_search = TextBox {
            placeholder: Some("Press '/' to search".to_owned()),
            ..Default::default()
        };

        let list = List::new();
        let tabs = Tabs::default();

        // components
        let mut listeners = Listeners::new();

        let command_component = RenderableComponent::new(command);
        let description_component = RenderableComponent::new(description);
        let tags_component = RenderableComponent::new(tags);
        let namespace_component = RenderableComponent::new(namespace);
        let tabs_component = RenderableComponent::new(tabs);
        let list_component = RenderableComponent::new(list);
        let quick_search_component = RenderableComponent::new(quick_search);

        listeners.insert(
            TypeId::of::<Search>(),
            vec![quick_search_component.get_observable()],
        );

        listeners.insert(
            TypeId::of::<TextBox>(),
            vec![
                command_component.get_observable(),
                description_component.get_observable(),
                tags_component.get_observable(),
                namespace_component.get_observable(),
            ],
        );
        listeners.insert(TypeId::of::<Tabs>(), vec![tabs_component.get_observable()]);
        listeners.insert(TypeId::of::<List>(), vec![list_component.get_observable()]);

        let clipboard = RenderableComponent::new(ClipboardStatus::default());

        listeners.insert(TypeId::of::<Clipboard>(), vec![clipboard.get_observable()]);

        // statics
        let app_name = StaticInfo::new(format!("cl - {}", env!("CARGO_PKG_VERSION")));
        let help = StaticInfo::new("F1/? for Help");

        Self {
            command: command_component,
            description: description_component,
            tags: tags_component,
            namespace: namespace_component,
            list: list_component,
            tabs: tabs_component,
            quick_search: quick_search_component,
            listeners,
            app_name,
            clipboard,
            help,
        }
    }
}

impl Layer for MainScreenLayer {
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        let drawable_area = [Constraint::Fill(2), Constraint::Max(3)];
        let areas = [
            Constraint::Length(30), // name & aliases
            Constraint::Fill(1),    // right side
        ];

        let details = [
            Constraint::Length(3),   // tabs
            Constraint::Ratio(1, 2), // description
            Constraint::Length(3),   // details
            Constraint::Ratio(1, 2), // command
        ];

        let drawable_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(drawable_area)
            .split(frame.area());

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(areas)
            .split(drawable_chunks[0]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Fill(1)])
            .split(main_chunks[0]);
        let (app_name_rect, list_rect) = (left_chunks[0], left_chunks[1]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(details)
            .split(main_chunks[1]);
        let (tabs_rect, description_rect, details_rect, command_rect) =
            (right_chunks[0], right_chunks[1], right_chunks[2], right_chunks[3]);

        let detail_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_percentages([40, 60]))
            .split(details_rect);
        let (namespace_area, tags_area) = (detail_chunks[0], detail_chunks[1]);

        let footer = Block::default().style(
            Style::default()
                .bg(theme.background_color.clone().into())
                .fg(theme.text_color.clone().into()),
        );
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(34),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(footer.inner(drawable_chunks[1]));
        let (quick_search_rect, clipboard_rect, help_rect) =
            (footer_chunks[0], footer_chunks[1], footer_chunks[2]);

        frame.render_widget(footer, drawable_chunks[1]);

        render! {
            frame,
            theme,
            { self.app_name, app_name_rect },
            { self.list, list_rect },
            { self.tabs, tabs_rect },
            { self.description, description_rect},
            { self.namespace, namespace_area },
            { self.tags, tags_area },
            { self.command, command_rect },
            { self.clipboard, clipboard_rect },
            { self.help, help_rect },
            { self.quick_search, quick_search_rect }
        }
    }

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
        &self.listeners
    }
}
