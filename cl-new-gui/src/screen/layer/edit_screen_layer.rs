use crate::component::{Downcastable, EditableTextbox, Renderable, RenderableComponent, ScreenState, StateComponent, StaticInfo};
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use crate::render;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::state::state_event::FieldName;
use log::debug;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::layout::{Constraint, Direction, Layout};
use tui::prelude::Style;
use tui::widgets::Block;
use tui::Frame;

pub struct EditScreenLayer {
    pub alias: RenderableComponent<EditableTextbox>,
    pub namespace: RenderableComponent<EditableTextbox>,
    pub command: RenderableComponent<EditableTextbox>,
    pub tags: RenderableComponent<EditableTextbox>,
    pub description: RenderableComponent<EditableTextbox>,
    pub screen_state: StateComponent<ScreenState>,
    pub listeners: BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>,
    pub app_name: StaticInfo,
    pub modified_status: StaticInfo,
}

impl Default for EditScreenLayer {
    fn default() -> Self {
        let current_field = FieldName::Alias;
        let screen_state = ScreenState::new(current_field);

        let alias = EditableTextbox {
            name: FieldName::Alias,
            active: true,
            ..Default::default()
        };
        let namespace = EditableTextbox {
            name: FieldName::Namespace,
            ..Default::default()
        };
        let command = EditableTextbox {
            name: FieldName::Command,
            ..Default::default()
        };
        let tags = EditableTextbox {
            name: FieldName::Tags,
            ..Default::default()
        };
        let description = EditableTextbox {
            name: FieldName::Description,
            ..Default::default()
        };

        let alias_component = RenderableComponent::new(alias);
        let namespace_component = RenderableComponent::new(namespace);
        let command_component = RenderableComponent::new(command);
        let tags_component = RenderableComponent::new(tags);
        let description_component = RenderableComponent::new(description);

        let screen_state_component = StateComponent::new(screen_state);

        let mut listeners = BTreeMap::new();
        listeners.insert(
            TypeId::of::<EditableTextbox>(),
            vec![
                alias_component.get_observable(),
                namespace_component.get_observable(),
                command_component.get_observable(),
                tags_component.get_observable(),
                description_component.get_observable(),
            ],
        );

        listeners.insert(
            TypeId::of::<ScreenState>(),
            vec![screen_state_component.get_observable()],
        );

        let app_name = StaticInfo::new("cl - edit");
        let modified_status = StaticInfo::new("MODIFIED");

        Self {
            alias: alias_component,
            namespace: namespace_component,
            command: command_component,
            tags: tags_component,
            description: description_component,
            screen_state: screen_state_component,
            listeners,
            app_name,
            modified_status,
        }
    }
}

impl Layer for EditScreenLayer {
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        let drawable_area = [
            Constraint::Fill(5), // drawable area
            Constraint::Max(3),  // footer
        ];

        let areas = [
            Constraint::Percentage(20), // name & preview
            Constraint::Percentage(80), // right side
        ];

        let constraints = [
            Constraint::Length(5), //Alias & Namespace
            Constraint::Fill(1),   //Command
            Constraint::Length(5), //Desc & Tags
        ];

        let drawable_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(drawable_area)
            .split(frame.area());

        let form_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(areas)
            .split(drawable_chunks[0]);

        let right_side = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(form_chunks[1]);

        let first_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(right_side[0]);
        let (first_row1, first_row2) = (first_row[0], first_row[1]);

        let second_row1 = right_side[1];

        let third_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(right_side[2]);
        let (third_row1, third_row2) = (third_row[0], third_row[1]);

        let app_name_rect = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Fill(1)])
            .split(form_chunks[0])[0];

        let footer = Block::default().style(
            Style::default()
                .bg(theme.background_color.clone().into())
                .fg(theme.text_color.clone().into()),
        );
        let modified_status_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(footer.inner(drawable_chunks[1]))[1];

        let background = Block::default().style(
            Style::default()
                .bg(theme.background_color.into())
                .fg(theme.text_color.into()),
        );
        frame.render_widget(background, frame.area());
        frame.render_widget(footer, drawable_chunks[1]);

        if self.get_modified_status() {
            render! {
                frame,
                theme,
                { self.modified_status, modified_status_rect },
            }
        }

        render! {
            frame,
            theme,
            { self.app_name, app_name_rect },
            { self.alias, first_row1 },
            { self.namespace, first_row2 },
            { self.command, second_row1 },
            { self.description, third_row1},
            { self.tags, third_row2 },
        }
    }

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
        &self.listeners
    }
}

const FIELD_ORDER: &[FieldName] = &[
    FieldName::Alias,
    FieldName::Namespace,
    FieldName::Command,
    FieldName::Description,
    FieldName::Tags,
];

impl EditScreenLayer {
    pub fn get_next_field(&self) -> FieldName {
        let current_field = self.get_current_field();

        let pos = FIELD_ORDER
            .iter()
            .position(|f| f == &current_field)
            .expect("Current field not found in FIELD_ORDER");
        let next_idx = (pos + 1) % FIELD_ORDER.len();

        debug!("current: {} - next: {}", pos, next_idx);
        FIELD_ORDER[next_idx].clone()
    }

    pub fn get_previous_field(&self) -> FieldName {
        let current_field = self.get_current_field();

        let pos = FIELD_ORDER
            .iter()
            .position(|f| f == &current_field)
            .expect("Current field not found in FIELD_ORDER");
        let previous_idx = (pos + FIELD_ORDER.len() - 1) % FIELD_ORDER.len();

        debug!("current: {} - previous: {}", pos, previous_idx);
        FIELD_ORDER[previous_idx].clone()
    }

    fn get_current_field(&self) -> FieldName {
        self.with_screen_state(|s| {
            debug!("getting current field: {:?}", s.current_field);
            s.current_field.clone()
        })
    }

    fn get_modified_status(&self) -> bool {
        self.with_screen_state(|s| s.has_changes)
    }

    fn with_screen_state<T>(&self, f: impl FnOnce(&ScreenState) -> T) -> T {
        let inner_ref: &dyn ObservableComponent = &*self.screen_state.as_observable();
        let screen_state = inner_ref
            .downcast_to::<ScreenState>()
            .expect("Cannot get the current screen state");
        f(screen_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_the_current_field() {
        let layer = EditScreenLayer::default();

        let current = layer.get_current_field();

        assert_eq!(current, FieldName::Alias)
    }

    #[test]
    fn should_get_the_next_field() {
        let layer = EditScreenLayer::default();

        // assuming the default field is ALIAS
        let next_field = layer.get_next_field();

        assert_eq!(next_field, FieldName::Namespace);
    }

    #[test]
    fn should_get_the_previous_field() {
        let layer = EditScreenLayer::default();

        // assuming the default field is ALIAS
        let previous_field = layer.get_previous_field();

        assert_eq!(previous_field, FieldName::Tags);
    }
}
