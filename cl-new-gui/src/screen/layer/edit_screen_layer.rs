use crate::component::{
    Component, EditableTextbox, EditableTextboxName, RenderableComponent,
    StateComponent,
};
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use crate::render;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::state::state_event::FieldType;
use log::debug;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::Clear;
use tui::Frame;

pub struct EditScreenLayer {
    pub alias: RenderableComponent<EditableTextbox>,
    pub namespace: RenderableComponent<EditableTextbox>,
    pub command: RenderableComponent<EditableTextbox>,
    pub tags: RenderableComponent<EditableTextbox>,
    pub description: RenderableComponent<EditableTextbox>,
    pub current_field: StateComponent<FieldType>,
    pub listeners: BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>,
}

impl Layer for EditScreenLayer {
    fn new() -> Self
    where
        Self: Sized,
    {
        let current_field = FieldType::Alias;

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

        let alias_component =RenderableComponent( Component::new(alias));
        let namespace_component =RenderableComponent(Component::new(namespace));
        let command_component = RenderableComponent(Component::new(command));
        let tags_component = RenderableComponent(Component::new(tags));
        let description_component = RenderableComponent(Component::new(description));

        let current_field_component = StateComponent(Component::new(current_field));

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
            TypeId::of::<FieldType>(),
            vec![current_field_component.get_observable()],
        );

        Self {
            alias: alias_component,
            namespace: namespace_component,
            command: command_component,
            tags: tags_component,
            description: description_component,
            listeners,
            current_field: current_field_component,
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

        frame.render_widget(Clear, frame.area());

        render! {
            frame,
            theme,
            { self.alias, first_row1 },
            { self.namespace, first_row2 },
            { self.command, second_row1 },
            { self.description, third_row1},
            { self.tags, third_row2 }
        }
    }

    fn get_listeners(&self) -> BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>> {
        debug!(target: "EditScreenLayer", "Getting listeners for EditScreenLayer");

        self.listeners.clone()
    }
}

const FIELD_ORDER: &[FieldType] = &[
    FieldType::Alias,
    FieldType::Namespace,
    FieldType::Command,
    FieldType::Description,
    FieldType::Tags,
];

impl EditScreenLayer {
    pub fn change_current_field(&mut self, field: FieldType) {
        if FIELD_ORDER.contains(&field) {
            debug!(target: "EditScreenLayer", "Changed current field to {:?}", field);
            self.current_field = StateComponent(Component::new(field));
        } else {
            debug!(target: "EditScreenLayer", "Invalid field type: {:?}", field);
        }
    }

    pub fn get_next_field(&self) -> FieldType {
        let current_field = self.get_current_field();

        let pos = FIELD_ORDER
            .iter()
            .position(|f| f == &current_field)
            .expect("Current field not found in FIELD_ORDER");
        let next_idx = (pos + 1) % FIELD_ORDER.len();

        FIELD_ORDER[next_idx].clone()
    }

    pub fn get_previous_field(&self) -> FieldType {
        let current_field = self.get_current_field();

        let pos = FIELD_ORDER
            .iter()
            .position(|f| f == &current_field)
            .expect("Current field not found in FIELD_ORDER");
        let previous_idx = (pos + FIELD_ORDER.len() - 1) % FIELD_ORDER.len();

        debug!("current: {} - previous: {}", pos, previous_idx);
        FIELD_ORDER[previous_idx].clone()
    }

    fn get_current_field(&self) -> FieldType {
        let inner_ref: &dyn ObservableComponent = &*self.current_field.borrow();
        let current_field = if let Some(field) = inner_ref.as_any().downcast_ref::<FieldType>() {
            field.clone()
        } else {
            panic!("Current field is not of type FieldType");
        };

        current_field.clone()
    }
}
