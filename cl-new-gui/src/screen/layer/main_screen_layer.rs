use crate::component::{List, SharedComponent, Tabs, TextBox, TextBoxName};
use crate::render;
use crate::screen::layer::Layer;
use crate::screen::Listeners;
use std::any::TypeId;
use std::collections::BTreeMap;
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

pub struct MainScreenLayer {
    pub command: SharedComponent,
    pub description: SharedComponent,
    pub tags: SharedComponent,
    pub namespace: SharedComponent,
    pub list: SharedComponent,
    pub tabs: SharedComponent,
    pub listeners: Listeners,
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

        let command_shared = SharedComponent::new(command);
        let description_shared = SharedComponent::new(description);
        let tags_shared = SharedComponent::new(tags);
        let namespace_shared = SharedComponent::new(namespace);
        let tabs_shared = SharedComponent::new(tabs);
        let list_shared = SharedComponent::new(list);

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

        Self {
            command: command_shared.clone(),
            description: description_shared.clone(),
            tags: tags_shared.clone(),
            namespace: namespace_shared.clone(),
            list: list_shared.clone(),
            tabs: tabs_shared.clone(),
            listeners,
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

        self.list.borrow_mut().render(frame, left_side[1]);
        self.description.borrow_mut().render(frame, right_side[1]); // middle
        self.command.borrow_mut().render(frame, right_side[3]);
        self.namespace.borrow_mut().render(frame, namespace_area);
        self.tags.borrow_mut().render(frame, tags_area);
        self.tabs.borrow_mut().render(frame, right_side[0])
    }

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<SharedComponent>> {
        self.listeners.clone()
    }
}
