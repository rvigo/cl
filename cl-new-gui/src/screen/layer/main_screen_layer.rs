use crate::clipboard::Clipboard;
use crate::component::{
    ClipboardStatus, Component, List, Renderable, StaticInfo, Tabs, TextBox, TextBoxName,
};

use crate::component::Search;
use crate::render;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::screen::Listeners;
use std::any::TypeId;
use std::collections::BTreeMap;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

pub struct MainScreenLayer {
    pub command: Component,
    pub description: Component,
    pub tags: Component,
    pub namespace: Component,
    pub list: Component,
    pub tabs: Component,
    pub clipboard: Component,
    pub quick_search: Component,
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

        let quick_search_share = Component::new(quick_search);
        listeners.insert(TypeId::of::<Search>(), vec![quick_search_share.clone()]);

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

        let clipboard = Component::new(ClipboardStatus::new());

        listeners.insert(TypeId::of::<Clipboard>(), vec![clipboard.clone()]);

        // statics
        let app_name = StaticInfo::new(format!("cl - {}", env!("CARGO_PKG_VERSION")));
        let help = StaticInfo::new("F1/? for Help");

        Self {
            command: command_shared.clone(),
            description: description_shared.clone(),
            tags: tags_shared.clone(),
            namespace: namespace_shared.clone(),
            list: list_shared.clone(),
            tabs: tabs_shared.clone(),
            quick_search: quick_search_share.clone(),
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
            .constraints(Constraint::from_percentages([30, 70]))
            .split(details_rect)
        else {
            panic!() // TODO improve this
        };

        //
        let [quick_search_rect, clipboard_rect, help_rect] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(drawable_chunks[1])
        else {
            todo!()
        };

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

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Component>> {
        self.listeners.clone()
    }
}
