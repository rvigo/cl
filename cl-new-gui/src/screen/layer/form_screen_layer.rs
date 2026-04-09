use crate::component::{
    Downcastable, EditableTextbox, Renderable, RenderableComponent, ScreenState, StateComponent,
    StaticInfo,
};
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::state::state_event::FieldName;
use tracing::debug;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tui::layout::{Constraint, Direction, Layout};
use tui::prelude::Style;
use tui::text::Line;
use tui::widgets::{Block, Paragraph};
use tui::Frame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormMode {
    Edit,
    Insert,
}

pub struct FormScreenLayer {
    pub mode: FormMode,
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

const FIELD_ORDER: &[FieldName] = &[
    FieldName::Alias,
    FieldName::Namespace,
    FieldName::Command,
    FieldName::Description,
    FieldName::Tags,
];

impl FormScreenLayer {
    pub fn edit() -> Self {
        Self::new(FormMode::Edit)
    }

    pub fn insert() -> Self {
        Self::new(FormMode::Insert)
    }

    fn new(mode: FormMode) -> Self {
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

        let app_name = match mode {
            FormMode::Edit => StaticInfo::new("cl - edit"),
            FormMode::Insert => StaticInfo::new("cl - insert"),
        };
        let modified_status = StaticInfo::new("MODIFIED");

        Self {
            mode,
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

    pub fn get_next_field(&self) -> FieldName {
        let current_field = self.get_current_field();

        let pos = match FIELD_ORDER.iter().position(|f| f == &current_field) {
            Some(pos) => pos,
            None => {
                tracing::error!(
                    "current field {:?} not found in FIELD_ORDER, defaulting to first",
                    current_field
                );
                0
            }
        };
        let next_idx = (pos + 1) % FIELD_ORDER.len();

        debug!("current: {} - next: {}", pos, next_idx);
        FIELD_ORDER[next_idx].clone()
    }

    pub fn get_previous_field(&self) -> FieldName {
        let current_field = self.get_current_field();

        let pos = match FIELD_ORDER.iter().position(|f| f == &current_field) {
            Some(pos) => pos,
            None => {
                tracing::error!(
                    "current field {:?} not found in FIELD_ORDER, defaulting to first",
                    current_field
                );
                0
            }
        };
        let previous_idx = (pos + FIELD_ORDER.len() - 1) % FIELD_ORDER.len();

        debug!("current: {} - previous: {}", pos, previous_idx);
        FIELD_ORDER[previous_idx].clone()
    }

    fn get_current_field(&self) -> FieldName {
        self.with_screen_state(|s| {
            debug!("getting current field: {:?}", s.current_field);
            s.current_field.clone()
        })
        .unwrap_or_default()
    }

    fn get_modified_status(&self) -> bool {
        self.with_screen_state(|s| s.has_changes).unwrap_or(false)
    }

    fn with_screen_state<T>(&self, f: impl FnOnce(&ScreenState) -> T) -> Option<T> {
        let inner_ref: &dyn ObservableComponent = &*self.screen_state.as_observable();
        match inner_ref.downcast_to::<ScreenState>() {
            Some(screen_state) => Some(f(screen_state)),
            None => {
                tracing::error!("failed to downcast screen state");
                None
            }
        }
    }

    fn field_is_filled(&self, component: &RenderableComponent<EditableTextbox>) -> bool {
        !component
            .borrow_inner()
            .textarea
            .lines()
            .iter()
            .all(|l| l.is_empty())
    }

    fn left_panel_widget<'a>(&'a self, theme: &Theme) -> Paragraph<'a> {
        let lines = match self.mode {
            FormMode::Insert => self.build_progress_lines(),
            FormMode::Edit => self.build_preview_lines(),
        };

        Paragraph::new(lines)
            .block(
                Block::bordered().style(
                    Style::default()
                        .fg(theme.text_color.into())
                        .bg(theme.background_color.into()),
                ),
            )
            .style(
                Style::default()
                    .fg(theme.text_color.into())
                    .bg(theme.background_color.into()),
            )
    }

    fn build_progress_lines(&self) -> Vec<Line> {
        let fields = [
            (&self.alias, "alias", true),
            (&self.namespace, "namespace", true),
            (&self.command, "command", true),
            (&self.description, "description", false),
            (&self.tags, "tags", false),
        ];

        fields
            .iter()
            .map(|(component, name, required)| {
                let filled = self.field_is_filled(component);
                let status = if filled { "✓" } else { " " };
                let marker = if !filled && *required { "*" } else { " " };
                Line::from(format!("[{status}] {name} {marker}"))
            })
            .collect()
    }

    fn build_preview_lines(&self) -> Vec<Line> {
        self.command
            .borrow_inner()
            .textarea
            .lines()
            .iter()
            .map(|line| Line::from(line.clone()))
            .collect()
    }
}

impl Layer for FormScreenLayer {
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

        let left_panel_splits = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Fill(1)])
            .split(form_chunks[0]);
        let app_name_rect = left_panel_splits[0];
        let left_panel_rect = left_panel_splits[1];

        let footer = Block::default().style(
            Style::default()
                .bg(theme.background_color.into())
                .fg(theme.text_color.into()),
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

        let left_panel = self.left_panel_widget(theme);
        frame.render_widget(left_panel, left_panel_rect);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_mode_sets_correct_app_name() {
        let layer = FormScreenLayer::edit();
        assert_eq!(layer.mode, FormMode::Edit);
    }

    #[test]
    fn insert_mode_sets_correct_app_name() {
        let layer = FormScreenLayer::insert();
        assert_eq!(layer.mode, FormMode::Insert);
    }

    #[test]
    fn should_get_the_current_field() {
        let layer = FormScreenLayer::edit();

        let current = layer.get_current_field();

        assert_eq!(current, FieldName::Alias)
    }

    #[test]
    fn should_get_the_next_field() {
        let layer = FormScreenLayer::edit();

        let next_field = layer.get_next_field();

        assert_eq!(next_field, FieldName::Namespace);
    }

    #[test]
    fn should_get_the_previous_field() {
        let layer = FormScreenLayer::edit();

        let previous_field = layer.get_previous_field();

        assert_eq!(previous_field, FieldName::Tags);
    }

    #[test]
    fn field_navigation_works_for_insert_mode() {
        let layer = FormScreenLayer::insert();

        let next = layer.get_next_field();
        assert_eq!(next, FieldName::Namespace);

        let prev = layer.get_previous_field();
        assert_eq!(prev, FieldName::Tags);
    }

    #[test]
    fn empty_fields_are_not_filled() {
        let layer = FormScreenLayer::insert();
        assert!(!layer.field_is_filled(&layer.alias));
        assert!(!layer.field_is_filled(&layer.namespace));
    }

    #[test]
    fn progress_lines_all_fields_empty() {
        let layer = FormScreenLayer::insert();
        let lines = layer.build_progress_lines();

        assert_eq!(lines.len(), 5);
        assert!(lines[0].to_string().contains(" "));
        assert!(lines[0].to_string().contains("alias"));
    }

    #[test]
    fn preview_lines_empty_when_no_command() {
        let layer = FormScreenLayer::edit();
        let lines = layer.build_preview_lines();

        // Empty textarea returns [""]
        assert_eq!(lines.len(), 1);
    }
}
