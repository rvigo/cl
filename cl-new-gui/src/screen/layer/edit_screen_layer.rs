use crate::component::{Downcastable, EditableTextbox, EditableTextboxName, Renderable, RenderableComponent, ScreenState, StateComponent, StaticInfo};
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use crate::render;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::state::state_event::FieldName;
use log::debug;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::layout::{Constraint, Direction, Layout};
use tui::prelude::Style;
use tui::widgets::{Block, Clear, Paragraph};
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
}

impl Layer for EditScreenLayer {
    fn new() -> Self
    where
        Self: Sized,
    {
        let current_field = FieldName::Alias;
        let screen_state = ScreenState::new(current_field);

        let alias = EditableTextbox {
            name: EditableTextboxName::Alias,
            active: true,
            ..Default::default()
        };
        let namespace = EditableTextbox {
            name: EditableTextboxName::Namespace,
            ..Default::default()
        };
        let command = EditableTextbox {
            name: EditableTextboxName::Command,
            ..Default::default()
        };
        let tags = EditableTextbox {
            name: EditableTextboxName::Tags,
            ..Default::default()
        };
        let description = EditableTextbox {
            name: EditableTextboxName::Description,
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

        Self {
            alias: alias_component,
            namespace: namespace_component,
            command: command_component,
            tags: tags_component,
            description: description_component,
            screen_state: screen_state_component,
            listeners,
            app_name,
        }
    }

    // TODO split the fields into the screen
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
            Constraint::Length(5),  //Alias & Namespace
            Constraint::Length(10), //Command
            Constraint::Length(5),  //Desc & Tags
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

        let [first_row1, first_row2] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(right_side[0])
        else {
            todo!()
        };

        let [second_row1] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(right_side[1])
        else {
            todo!()
        };

        let [third_row1, third_row2] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(right_side[2])
        else {
            todo!()
        };

        let [app_name_rect, _] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Max(3)])
            .split(form_chunks[0])
        else {
            todo!()
        };

        let footer = Block::default().style(
            Style::default()
                .bg(theme.to_owned().background_color.into())
                .fg(theme.to_owned().text_color.into()),
        );
        let [_, modified_status_rect, _] = *Layout::default()
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

        frame.render_widget(Clear, frame.area()); // TODO make sure the clear event occurs before the screen
        frame.render_widget(footer, drawable_chunks[1]);

        if self.get_modified_status() {
            let mut modified_status = StaticInfo::new("MODIFIED");
            render! {
                frame,
                theme,
                { modified_status, modified_status_rect },
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

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
        debug!(target: "EditScreenLayer", "Getting listeners for EditScreenLayer");

        self.listeners.clone()
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
        let inner_ref: &dyn ObservableComponent = &*self.screen_state.borrow();
        let screen_state = if let Some(field) = inner_ref.downcast_to::<ScreenState>() {
            field
        } else {
            panic!("Cannot get the current screen state");
        };

        let current = screen_state.current_field.clone();
        debug!("getting current field: {:?}", current);
        current
    }

    fn get_modified_status(&self) -> bool {
        let inner_ref: &dyn ObservableComponent = &*self.screen_state.borrow();
        let screen_state = if let Some(field) = inner_ref.downcast_to::<ScreenState>() {
            field
        } else {
            panic!("Cannot get the current screen state");
        };

        screen_state.has_changes
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_the_current_field() {
        let layer = EditScreenLayer::new();

        let current = layer.get_current_field();

        assert_eq!(current, FieldName::Alias)
    }

    #[test]
    fn should_get_the_next_field() {
        let layer = EditScreenLayer::new();

        // assuming the default field is ALIAS
        let next_field = layer.get_next_field();

        assert_eq!(next_field, FieldName::Namespace);
    }

    #[test]
    fn should_get_the_previous_field() {
        let layer = EditScreenLayer::new();

        // assuming the default field is ALIAS
        let previous_field = layer.get_previous_field();

        assert_eq!(previous_field, FieldName::Tags);
    }
}
