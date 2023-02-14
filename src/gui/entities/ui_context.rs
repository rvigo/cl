use super::{field_context::FieldContext, popup_context::PopupContext};
use crate::gui::{
    layouts::{TerminalSize, ViewMode},
    widgets::{field::FieldType, query_box::QueryBox},
};

pub struct UIContext<'a> {
    pub form_fields_context: FieldContext<'a>,
    pub popup_context: PopupContext<'a>,
    pub query_box: QueryBox<'a>,
    terminal_size: TerminalSize,
    view_mode: ViewMode,
}

impl<'a> UIContext<'a> {
    pub fn new(terminal_size: TerminalSize) -> UIContext<'a> {
        UIContext {
            form_fields_context: FieldContext::default(),
            popup_context: PopupContext::default(),
            query_box: QueryBox::default(),
            terminal_size,
            view_mode: ViewMode::Main,
        }
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
                let fields = &mut self.form_fields_context.fields;

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
                let fields = &mut self.form_fields_context.fields;

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
