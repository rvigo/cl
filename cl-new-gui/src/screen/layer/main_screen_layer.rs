use crate::clipboard::Clipboard;
use crate::component::{
    ClipboardStatus, Component, List, Renderable, StaticInfo, Tabs, TextBox, TextBoxName,
};
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

        let quick_search = TextBox {
            placeholder: Some("Press '/' to search".to_owned()),
            ..Default::default()
        };

        let list = List::new();
        let tabs = Tabs::new();

        // components
        let mut listeners = Listeners::new();

        let command_shared = RenderableComponent(Component::new(command));
        let description_shared = RenderableComponent(Component::new(description));
        let tags_shared = RenderableComponent(Component::new(tags));
        let namespace_shared = RenderableComponent(Component::new(namespace));
        let tabs_shared = RenderableComponent(Component::new(tabs));
        let list_shared = RenderableComponent(Component::new(list));

        let quick_search_share = RenderableComponent(Component::new(quick_search));
        listeners.insert(
            TypeId::of::<Search>(),
            vec![quick_search_share.get_observable()],
        );

        listeners.insert(
            TypeId::of::<TextBox>(),
            vec![
                command_shared.get_observable(),
                description_shared.get_observable(),
                tags_shared.get_observable(),
                namespace_shared.get_observable(),
            ],
        );
        listeners.insert(TypeId::of::<Tabs>(), vec![tabs_shared.get_observable()]);
        listeners.insert(TypeId::of::<List>(), vec![list_shared.get_observable()]);

        let clipboard = RenderableComponent(Component::new(ClipboardStatus::new()));

        listeners.insert(TypeId::of::<Clipboard>(), vec![clipboard.clone()]);

        // statics
        let app_name = StaticInfo::new(format!("cl - {}", env!("CARGO_PKG_VERSION")));
        let help = StaticInfo::new("F1/? for Help");

        Self {
            command: command_shared,
            description: description_shared,
            tags: tags_shared,
            namespace: namespace_shared,
            list: list_shared,
            tabs: tabs_shared,
            quick_search: quick_search_share,
            listeners,
            app_name,
            clipboard,
            help,
        }
    }

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

        let [app_name_rect, list_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Fill(1)])
            .split(main_chunks[0])
        else {
            todo!()
        };

        let [tabs_rect, description_rect, details_rect, command_rect] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints(details)
            .split(main_chunks[1])
        else {
            todo!()
        };

        let [namespace_area, tags_area] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints(Constraint::from_percentages([40, 60]))
            .split(details_rect)
        else {
            panic!() // TODO improve this
        };

        //

        let footer = Block::default().style(
            Style::default()
                .bg(theme.to_owned().background_color.into())
                .fg(theme.to_owned().text_color.into()),
        );
        let [quick_search_rect, clipboard_rect, help_rect] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(footer.inner(drawable_chunks[1]))
        else {
            todo!()
        };

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

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Rc<RefCell<(dyn Observable + 'static)>>>> {
        self.listeners.clone()
    }
}
