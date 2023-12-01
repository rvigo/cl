use super::{field_context::FieldContext, popup_context::PopupContext, Selectable};
use crate::{
    entities::{
        events::app_events::PopupCallbackAction,
        states::{
            popup_state::PopupState,
            ui_state::{UiState, ViewMode},
            State,
        },
    },
    screens::ScreenSize,
    widgets::{
        popup::{option::Choice, popup_type::PopupType},
        statusbar::querybox::QueryBox,
        text_field::{FieldType, TextField},
        WidgetKeyHandler,
    },
};
use cl_core::command::Command;
use crossterm::event::KeyEvent;

pub struct PopupInfoContainer {
    pub title: String,
    pub message: String,
    pub popup_type: PopupType,
    pub callback: PopupCallbackAction,
}

impl PopupInfoContainer {
    pub fn new() -> PopupInfoContainer {
        Self {
            title: String::default(),
            message: String::default(),
            popup_type: PopupType::Error,
            callback: PopupCallbackAction::None,
        }
    }

    pub fn set<T: Into<String>>(
        &mut self,
        title: T,
        popup_type: PopupType,
        message: String,
        callback: PopupCallbackAction,
    ) {
        self.title = title.into();
        self.popup_type = popup_type;
        self.callback = callback;
        self.message = message
    }
}

pub struct UIContext<'a> {
    form_fields_context: FieldContext<'a>,
    popup_context: PopupContext,
    ui_state: UiState,
    query_box: QueryBox<'a>,
    pub popup_container: PopupInfoContainer,
}

impl<'a> UIContext<'a> {
    pub fn new(size: ScreenSize) -> UIContext<'a> {
        let mut context = UIContext {
            form_fields_context: FieldContext::new(&size),
            popup_context: PopupContext::new(),
            ui_state: UiState::new(&size),
            query_box: QueryBox::default(),
            popup_container: PopupInfoContainer::new(),
        };

        context.sort_fields(size);
        context.select_form_field_type(Some(FieldType::default()));
        context.select_command(None);
        context
    }

    //// popup
    pub fn set_popup(
        &mut self,
        popup_type: PopupType,
        message: String,
        callback_action: PopupCallbackAction,
    ) {
        let answers = match popup_type {
            PopupType::Error => Choice::confirm(),
            PopupType::Warning => Choice::dialog(),
            PopupType::Help => Choice::empty(),
        };
        self.popup_context.set_available_choices(answers);
        self.popup_container
            .set(popup_type.to_string(), popup_type, message, callback_action);
    }

    pub fn get_available_choices(&self) -> Vec<Choice> {
        self.popup_context.get_available_choices()
    }

    pub fn clear_popup_context(&mut self) {
        self.popup_context.clear()
    }

    pub fn next_choice(&mut self) {
        self.popup_context.next()
    }

    pub fn previous_choice(&mut self) {
        self.popup_context.previous()
    }

    pub fn get_selected_choice(&self) -> Option<Choice> {
        self.popup_context
            .state()
            .selected()
            .map(|choice| self.popup_context.get_available_choices()[choice].to_owned())
    }

    pub fn get_choices_state_mut(&mut self) -> &mut PopupState {
        self.popup_context.state_mut()
    }

    pub fn show_popup(&self) -> bool {
        self.ui_state.show_popup()
    }

    pub fn set_show_popup(&mut self, should_show: bool) {
        self.ui_state.set_show_popup(should_show)
    }

    pub fn show_help(&self) -> bool {
        self.ui_state.show_help()
    }

    pub fn set_show_help(&mut self, should_show: bool) {
        self.ui_state.set_show_help(should_show)
    }

    pub fn get_selected_command(&self) -> Option<&Command> {
        self.form_fields_context.selected_command()
    }

    pub fn set_selected_command_input(&mut self) {
        self.form_fields_context.popuplate_form();
    }

    pub fn select_command(&mut self, selected_command: Option<Command>) {
        self.form_fields_context.select_command(selected_command)
    }

    // form
    pub fn select_form_field_type(&mut self, field_type: Option<FieldType>) {
        self.form_fields_context
            .get_field_state_mut()
            .select(field_type);
    }

    pub fn clear_form_fields(&mut self) {
        self.form_fields_context.clear_inputs()
    }

    pub fn get_form_fields_iter(&self) -> impl Iterator<Item = TextField> {
        self.form_fields_context.get_fields_iter()
    }

    pub fn edit_command(&mut self) -> Command {
        self.form_fields_context.build_edited_command()
    }

    pub fn build_new_command(&mut self) -> Command {
        self.form_fields_context.build_new_command()
    }

    pub fn next_form_field(&mut self) {
        self.form_fields_context.next()
    }

    pub fn previous_form_field(&mut self) {
        self.form_fields_context.previous()
    }

    /// Resets forms selection
    pub fn reset_form_field_selected_field(&mut self) {
        let default_field = FieldType::default();
        self.form_fields_context.clear_selection();
        if let Some(selected) = self.form_fields_context.selected_field_mut() {
            selected.deactivate_focus()
        }
        self.select_form_field_type(Some(default_field));
        if let Some(selected) = self.form_fields_context.selected_field_mut() {
            selected.activate_focus()
        }
    }

    pub fn handle_form_input(&mut self, input: KeyEvent) {
        self.form_fields_context.handle_input(input)
    }

    pub fn is_form_modified(&self) -> bool {
        self.form_fields_context.is_modified()
    }

    pub fn sort_fields<I>(&mut self, screen_size: I)
    where
        I: Into<ScreenSize>,
    {
        let s = screen_size.into();
        if self.screen_size() != s {
            self.form_fields_context.sort_field_by_size(&s)
        }
    }

    pub fn screen_size(&self) -> ScreenSize {
        self.ui_state.screen_size()
    }

    pub fn set_screen_size<I>(&mut self, screen_size: I)
    where
        I: Into<ScreenSize>,
    {
        self.ui_state.set_screen_size(screen_size.into())
    }

    pub fn update_screen_size<I>(&mut self, screen_size: I)
    where
        I: Into<ScreenSize>,
    {
        let s: ScreenSize = screen_size.into();
        if s != self.screen_size() {
            self.sort_fields(s.clone());
            self.set_screen_size(s)
        }
    }

    // querybox
    pub fn get_querybox_input(&self) -> String {
        self.query_box.get_input()
    }

    pub fn activate_querybox_focus(&mut self) {
        self.query_box.activate_focus()
    }

    pub fn deactivate_querybox_focus(&mut self) {
        self.query_box.deactivate_focus()
    }

    pub fn querybox_ref(&self) -> &QueryBox {
        &self.query_box
    }

    pub fn handle_querybox_input(&mut self, key_event: KeyEvent) {
        self.query_box.handle_input(key_event)
    }

    ///
    pub fn view_mode(&self) -> ViewMode {
        self.ui_state.view_mode()
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.ui_state.set_view_mode(view_mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_clear_input_when_enter_insert_screen() {
        let mut ui = UIContext::new(ScreenSize::Medium);
        let command = Command::default();

        // enters edit mode
        ui.select_command(Some(command));
        ui.reset_form_field_selected_field();
        ui.clear_form_fields();
        ui.set_selected_command_input();

        // enters insert mode
        ui.reset_form_field_selected_field();
        ui.clear_form_fields();

        let mut fields = ui.get_form_fields_iter();
        assert!(fields.all(|c| c.text().is_empty()));
    }
}
