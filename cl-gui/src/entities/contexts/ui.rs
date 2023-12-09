use super::{fields::Fields, popup_context::PopupContext, Selectable};
use crate::{
    entities::{
        events::app_events::PopupCallbackAction,
        popup_info::PopupInfo,
        states::{clipboard_state::ClipboardState, State},
        terminal::TerminalSize,
        view_mode::ViewMode,
    },
    widgets::{
        popup::{choice::Choice, popup_type::PopupType},
        statusbar::querybox::QueryBox,
        text_field::{FieldType, TextField},
        WidgetKeyHandler,
    },
};
use cl_core::command::Command;
use crossterm::event::KeyEvent;

pub struct UI<'ui> {
    fields_context: Fields<'ui>,
    popup_context: PopupContext,
    query_box: QueryBox<'ui>,
    pub clipboard_state: ClipboardState,
    view_mode: ViewMode,
    show_popup: bool,
    show_help: bool,
}

impl<'ui> UI<'ui> {
    pub fn new(size: TerminalSize) -> UI<'ui> {
        UI {
            fields_context: Fields::new(&size),
            popup_context: PopupContext::new(),
            query_box: QueryBox::default(),
            clipboard_state: ClipboardState::default(),
            view_mode: ViewMode::Main,
            show_popup: false,
            show_help: false,
        }
    }

    pub fn popup_info_mut(&mut self) -> &mut PopupInfo {
        &mut self.popup_context.info
    }

    pub fn set_popup_info(
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
        self.popup_context
            .info
            .set(popup_type.to_string(), popup_type, message, callback_action);
    }

    pub fn popup_context_mut(&mut self) -> &mut PopupContext {
        &mut self.popup_context
    }

    pub fn clear_popup_context(&mut self) {
        self.popup_context.clear()
    }

    pub fn get_selected_choice(&self) -> Choice {
        self.popup_context.get_available_choices()[self.popup_context.selected()].to_owned()
    }

    pub fn show_popup(&self) -> bool {
        self.show_popup
    }

    pub fn set_show_popup(&mut self, should_show: bool) {
        self.show_popup = should_show
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn set_show_help(&mut self, should_show: bool) {
        self.show_help = should_show
    }

    pub fn get_selected_command(&self) -> Option<&Command> {
        self.fields_context.selected_command()
    }

    pub fn set_selected_command_input(&mut self) {
        self.fields_context.popuplate_form();
    }

    pub fn select_command(&mut self, selected_command: Option<Command>) {
        self.fields_context.select_command(selected_command)
    }

    // form
    pub fn select_form_field_type(&mut self, field_type: Option<FieldType>) {
        self.fields_context.select(field_type);
    }

    pub fn clear_form_fields(&mut self) {
        self.fields_context.clear_inputs()
    }

    pub fn get_form_fields_iter(&self) -> impl Iterator<Item = TextField> {
        self.fields_context.get_fields_iter()
    }

    pub fn edit_command(&mut self) -> Command {
        self.fields_context.build_edited_command()
    }

    pub fn build_new_command(&mut self) -> Command {
        self.fields_context.build_new_command()
    }

    pub fn next_field(&mut self) {
        self.fields_context.next()
    }

    pub fn previous_field(&mut self) {
        self.fields_context.previous()
    }

    /// Resets forms selection
    pub fn reset_form_field_selected_field(&mut self) {
        self.fields_context.reset();
    }

    pub fn handle_input(&mut self, input: KeyEvent) {
        self.fields_context.handle_input(input)
    }

    pub fn is_form_modified(&self) -> bool {
        self.fields_context.is_modified()
    }

    pub fn sort_fields<I>(&mut self, terminal_size: I)
    where
        I: Into<TerminalSize>,
    {
        self.fields_context
            .sort_field_by_size(&terminal_size.into())
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
        self.view_mode.clone()
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::terminal::TerminalSize;

    #[test]
    fn should_clear_input_when_enter_insert_screen() {
        let mut ui = UI::new(TerminalSize::Medium);
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
