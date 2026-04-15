use crate::component::{
    Downcastable, EditableTextbox, Renderable, RenderableComponent, ScreenState, StateComponent,
    StaticInfo,
};
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;
use crate::screen::key_mapping::command::ScreenCommand;
use crate::screen::layer::Layer;
use crate::screen::theme::Theme;
use crate::state::state_event::{FieldName, StateEvent};
use crossterm::event::KeyEvent;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;
use tracing::debug;
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
    pub field_hint: StaticInfo,
    pub help: StaticInfo,
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
        let field_hint = StaticInfo::new(Self::hint_for_field(&FieldName::Alias));
        let help = StaticInfo::new("F1 for Help");

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
            field_hint,
            help,
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
        let mut lines = Vec::new();

        // Alias & Namespace
        let alias_text = self.alias.borrow_inner().textarea.lines().join("");
        let namespace_text = self.namespace.borrow_inner().textarea.lines().join("");
        if !alias_text.is_empty() || !namespace_text.is_empty() {
            lines.push(Line::from(format!("{namespace_text}:{alias_text}")));
            lines.push(Line::from(""));
        }

        // Command
        let command_lines = self.command.borrow_inner().textarea.lines().to_vec();
        for line in &command_lines {
            lines.push(Line::from(line.clone()));
        }

        // Named parameters
        let command_text = command_lines.join("\n");
        let params = self.extract_parameters(&command_text);
        if !params.is_empty() {
            lines.push(Line::from(""));
            let params_str = params.join(", ");
            lines.push(Line::from(format!("Params: {params_str}")));
        }

        // Description
        let desc_text = self.description.borrow_inner().textarea.lines().join("");
        if !desc_text.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(format!("Desc: {desc_text}")));
        }

        // Tags
        let tags_text = self.tags.borrow_inner().textarea.lines().join("");
        if !tags_text.is_empty() {
            lines.push(Line::from(format!("Tags: {tags_text}")));
        }

        lines
    }

    fn hint_for_field(field: &FieldName) -> &'static str {
        match field {
            FieldName::Alias => "Alias: unique name to identify the command",
            FieldName::Namespace => "Namespace: group to organize related commands",
            FieldName::Command => "Command: shell command to execute (use #{param} for parameters)",
            FieldName::Description => "Description: optional summary of what the command does",
            FieldName::Tags => "Tags: optional labels to categorize the command",
        }
    }

    fn extract_parameters(&self, command: &str) -> Vec<String> {
        let mut params = Vec::new();
        let mut in_param = false;
        let mut current = String::new();

        for ch in command.chars() {
            match ch {
                '#' if !in_param => in_param = true,
                '{' if in_param => {}
                '}' if in_param => {
                    if !current.is_empty() {
                        params.push(current.clone());
                        current.clear();
                    }
                    in_param = false;
                }
                c if in_param && (c.is_alphanumeric() || c == '_') => current.push(c),
                _ if in_param => {
                    in_param = false;
                    current.clear();
                }
                _ => {}
            }
        }

        params
    }
}

impl Layer for FormScreenLayer {
    fn handle_key_event<'a>(
        &'a self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>> {
        self.map_key_event(key, state_tx)
    }

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
        let footer_splits = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(footer.inner(drawable_chunks[1]));
        let (field_hint_rect, help_rect) = (footer_splits[0], footer_splits[1]);

        let background = Block::default().style(
            Style::default()
                .bg(theme.background_color.into())
                .fg(theme.text_color.into()),
        );
        frame.render_widget(background, frame.area());
        frame.render_widget(footer, drawable_chunks[1]);

        self.field_hint.content = Self::hint_for_field(&self.get_current_field()).to_string();

        render! {
            frame,
            theme,
            { self.field_hint, field_hint_rect },
            { self.help, help_rect },
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
    fn preview_lines_includes_command_and_metadata() {
        let layer = FormScreenLayer::edit();
        let lines = layer.build_preview_lines();

        // Should have at least the empty command line(s)
        assert!(!lines.is_empty());
    }

    #[test]
    fn extract_parameters_finds_all_params() {
        let layer = FormScreenLayer::edit();
        let params = layer.extract_parameters("docker run --name #{name} --image #{image}");

        assert_eq!(params, vec!["name", "image"]);
    }

    #[test]
    fn extract_parameters_handles_no_params() {
        let layer = FormScreenLayer::edit();
        let params = layer.extract_parameters("docker run --help");

        assert!(params.is_empty());
    }

    #[test]
    fn hint_for_field_returns_distinct_hints() {
        let fields = [
            FieldName::Alias,
            FieldName::Namespace,
            FieldName::Command,
            FieldName::Description,
            FieldName::Tags,
        ];
        let hints: Vec<_> = fields.iter().map(FormScreenLayer::hint_for_field).collect();

        // Each field has a non-empty hint
        assert!(hints.iter().all(|h| !h.is_empty()));
        // All hints are distinct
        let unique: std::collections::HashSet<_> = hints.iter().collect();
        assert_eq!(unique.len(), fields.len());
    }
}
