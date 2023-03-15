use super::{field_context::FieldContext, popup_context::PopupContext};
use crate::{
    command::Command,
    gui::{
        layouts::{TerminalSize, ViewMode},
        widgets::{
            field::{Field, FieldType},
            fields::Fields,
            popup::{Answer, ChoicesState, Popup},
            query_box::QueryBox,
        },
    },
};
use crossterm::event::KeyEvent;

pub struct UIContext<'a> {
    form_fields_context: FieldContext<'a>,
    popup_context: PopupContext<'a>,
    query_box: QueryBox<'a>,
    terminal_size: TerminalSize,
    view_mode: ViewMode,
}

impl<'a> UIContext<'a> {
    pub fn new(terminal_size: TerminalSize) -> UIContext<'a> {
        let mut context = UIContext {
            form_fields_context: FieldContext::default(),
            popup_context: PopupContext::new(),
            query_box: QueryBox::default(),
            terminal_size,
            view_mode: ViewMode::Main,
        };
        context.select_form_idx(Some(0));
        context.select_command(None);
        context
    }

    pub fn view_mode(&self) -> &ViewMode {
        &self.view_mode
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode
    }

    pub fn terminal_size(&self) -> TerminalSize {
        self.terminal_size.to_owned()
    }

    pub fn update_terminal_size(&mut self, new_size: TerminalSize) {
        self.terminal_size = new_size;
        self.reorder_fields()
    }

    pub fn build_form_fields(&mut self) {
        self.form_fields_context.build_form_fields()
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

    pub fn select_form_idx(&mut self, idx: Option<usize>) {
        self.form_fields_context.get_focus_state_mut().select(idx);
    }

    pub fn get_form_fields(&self) -> &Fields {
        self.form_fields_context.get_fields()
    }

    pub fn edit_command(&mut self) -> Command {
        self.form_fields_context.edit_command()
    }

    pub fn build_new_command(&mut self) -> Command {
        self.form_fields_context.build_new_command()
    }

    pub fn get_selected_form_field_mut(&mut self) -> Option<&mut Field<'a>> {
        self.form_fields_context.selected_field_mut()
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

    pub fn toogle_querybox_focus(&mut self) {
        self.query_box.toggle_focus()
    }

    pub fn querybox(&self) -> QueryBox {
        self.query_box.to_owned()
    }

    pub fn handle_querybox_input(&mut self, key_event: KeyEvent) {
        self.query_box.handle(key_event)
    }

    pub fn querybox_focus(&self) -> bool {
        self.query_box.is_on_focus()
    }

    pub fn popup(&self) -> Option<Popup<'a>> {
        self.popup_context.get_popup()
    }

    pub fn set_popup(&mut self, popup: Option<Popup<'a>>) {
        self.popup_context.set_popup(popup);
    }

    pub fn get_popup_answer(&self) -> Option<Answer> {
        self.popup_context.answer()
    }

    pub fn clear_popup_context(&mut self) {
        self.popup_context.clear()
    }

    pub fn next_choice(&mut self, choices: Vec<Answer>) {
        self.popup_context.state_mut().next(choices)
    }

    pub fn previous_choice(&mut self, choices: Vec<Answer>) {
        self.popup_context.state_mut().previous(choices)
    }

    pub fn get_selected_choice(&self) -> Option<usize> {
        self.popup_context.state().selected()
    }

    pub fn get_choices_state_mut(&mut self) -> &mut ChoicesState {
        self.popup_context.state_mut()
    }

    pub fn enter_main_mode(&mut self) {
        self.select_form_idx(Some(0));
        self.set_view_mode(ViewMode::Main);
    }

    fn reorder_fields(&mut self) {
        match &self.terminal_size {
            TerminalSize::Small => {
                let order = vec![
                    FieldType::Alias,
                    FieldType::Namespace,
                    FieldType::Description,
                    FieldType::Tags,
                    FieldType::Command,
                ];
                let fields = &mut self.form_fields_context.get_fields_mut();

                fields.sort_by(|a, b| {
                    order
                        .iter()
                        .position(|x| x.eq(&a.field_type))
                        .cmp(&order.iter().position(|x| x.eq(&b.field_type)))
                });
            }

            TerminalSize::Medium | TerminalSize::Large => {
                let order = vec![
                    FieldType::Alias,
                    FieldType::Namespace,
                    FieldType::Command,
                    FieldType::Description,
                    FieldType::Tags,
                ];
                let fields = &mut self.form_fields_context.get_fields_mut();

                fields.sort_by(|a, b| {
                    order
                        .iter()
                        .position(|x| x.eq(&a.field_type))
                        .cmp(&order.iter().position(|x| x.eq(&b.field_type)))
                });
            }
        }
    }
}
