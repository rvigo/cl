use super::{
    answer_state::AnswerState,
    events::app_events::PopupCallbackAction,
    field_context::FieldContext,
    popup_context::PopupContext,
    ui_state::{UiState, ViewMode},
};
use crate::{
    command::Command,
    gui::{
        layouts::TerminalSize,
        widgets::{
            field::{Field, FieldType},
            popup::{Answer, Popup},
            query_box::QueryBox,
        },
    },
};
use crossterm::event::KeyEvent;

#[derive(Clone)]
pub struct UIContext<'a> {
    form_fields_context: FieldContext<'a>,
    popup_context: PopupContext,
    ui_state: UiState,
    query_box: QueryBox<'a>,
}

impl<'a> UIContext<'a> {
    pub fn new() -> UIContext<'a> {
        let mut context = UIContext {
            form_fields_context: FieldContext::default(),
            popup_context: PopupContext::new(),
            ui_state: UiState::new(TerminalSize::default()),
            query_box: QueryBox::default(),
        };
        context.select_form_field_type(Some(FieldType::default()));
        context.select_command(None);
        context
    }

    //// popup
    pub fn set_dialog_popup(&mut self, message: String, callback_action: PopupCallbackAction) {
        self.set_popup(Some(Popup::from_warning(message, callback_action)))
    }

    pub fn set_error_popup(&mut self, message: String) {
        self.set_popup(Some(Popup::from_error(message, None)))
    }

    pub fn get_selected_command(&self) -> Option<&Command> {
        self.form_fields_context.selected_command()
    }

    pub fn set_selected_command_input(&mut self) {
        self.form_fields_context.set_selected_command_input();
    }

    pub fn select_command(&mut self, selected_command: Option<Command>) {
        self.form_fields_context.select_command(selected_command)
    }

    pub fn select_form_field_type(&mut self, field_type: Option<FieldType>) {
        self.form_fields_context
            .get_focus_state_mut()
            .select(field_type);
    }

    pub fn clear_form_fields(&mut self) {
        self.form_fields_context.clear_inputs()
    }

    pub fn get_form_fields(&self) -> Vec<Field> {
        self.form_fields_context.get_fields()
    }

    pub fn edit_command(&mut self) -> Command {
        self.form_fields_context.edit_command()
    }

    pub fn build_new_command(&mut self) -> Command {
        self.form_fields_context.build_new_command()
    }

    pub fn get_selected_form_field_mut(&mut self) -> Option<&mut Field<'a>> {
        self.form_fields_context.selected_field()
    }

    pub fn next_form_field(&mut self) {
        self.form_fields_context.next_field()
    }

    pub fn previous_form_field(&mut self) {
        self.form_fields_context.previous_field()
    }

    pub fn get_querybox_input(&self) -> String {
        self.query_box.get_input()
    }

    pub fn activate_focus(&mut self) {
        self.query_box.activate_focus()
    }

    pub fn deactivate_focus(&mut self) {
        self.query_box.deactivate_focus()
    }

    pub fn querybox(&self) -> QueryBox {
        self.query_box.to_owned()
    }

    pub fn handle_querybox_input(&mut self, key_event: KeyEvent) {
        self.query_box.handle(key_event)
    }

    pub fn popup(&self) -> Option<Popup> {
        self.popup_context.get_popup()
    }

    pub fn set_popup(&mut self, popup: Option<Popup>) {
        self.popup_context.set_popup(popup);
    }

    pub fn get_popup_answer(&self) -> Option<Answer> {
        self.popup_context.answer()
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

    pub fn get_selected_choice(&self) -> Option<Answer> {
        if let Some(choice) = self.popup_context.state().selected() {
            self.popup().map(|popup| popup.choices()[choice].clone())
        } else {
            None
        }
    }

    pub fn get_choices_state_mut(&mut self) -> &mut AnswerState {
        self.popup_context.state_mut()
    }

    /// Resets forms selection
    pub fn reset_form_field_selected_field(&mut self) {
        let default_field = FieldType::default();
        self.form_fields_context.clear_selection();
        if let Some(selected) = self.form_fields_context.selected_field() {
            selected.deactivate_focus()
        }
        self.select_form_field_type(Some(default_field));
        if let Some(selected) = self.form_fields_context.selected_field() {
            selected.activate_focus()
        }
    }

    pub fn handle_form_input(&mut self, input: KeyEvent) {
        if let Some(selected_field) = self.get_selected_form_field_mut() {
            selected_field.on_input(input)
        }
    }

    pub fn resize_to(&mut self, size: TerminalSize) {
        self.ui_state.set_terminal_size(size);
        self.order_fields();
    }

    pub fn order_fields(&mut self) {
        let size = &self.ui_state.terminal_size();
        self.form_fields_context.order_field_by_size(size)
    }

    pub fn querybox_focus(&self) -> bool {
        self.ui_state.querybox_focus()
    }

    pub fn set_querybox_focus(&mut self, focus: bool) {
        self.ui_state.set_querybox_focus(focus)
    }

    pub fn view_mode(&self) -> ViewMode {
        self.ui_state.view_mode()
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.ui_state.set_view_mode(view_mode)
    }

    pub fn terminal_size(&self) -> &TerminalSize {
        self.ui_state.terminal_size()
    }

    pub fn set_terminal_size(&mut self, terminal_size: TerminalSize) {
        self.ui_state.set_terminal_size(terminal_size)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_clear_input_when_enter_insert_screen() {
        let mut ui = UIContext::new();
        let command = Command::default();

        // enters edit mode
        ui.select_command(Some(command));
        ui.reset_form_field_selected_field();
        ui.order_fields();
        ui.clear_form_fields();
        ui.set_selected_command_input();

        let alias_form = ui.get_selected_form_field_mut();

        assert!(alias_form.is_some());
        assert!(!alias_form.unwrap().input_as_string().is_empty());

        // enters insert mode
        ui.reset_form_field_selected_field();
        ui.order_fields();
        ui.clear_form_fields();

        let alias_form = ui.get_selected_form_field_mut();

        assert!(alias_form.is_some());
        assert!(alias_form.unwrap().input_as_string().is_empty());
    }
}
