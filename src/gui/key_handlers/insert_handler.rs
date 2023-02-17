use super::KeyHandler;
use crate::gui::{
    entities::application_context::ApplicationContext, layouts::ViewMode, widgets::popup::Popup,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct InsertHandler;

impl KeyHandler for InsertHandler {
    fn handle(&self, key_event: KeyEvent, application_context: &mut ApplicationContext) {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                application_context.ui_context.clear_form_fields_inputs();
                application_context.ui_context.select_form(Some(0));
                application_context.ui_context.set_view_mode(ViewMode::Main);
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => application_context.ui_context.next_form_field(),

            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => application_context.ui_context.previous_form_field(),

            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let command = application_context.ui_context.build_new_command();

                match application_context.commands_context.add_command(&command) {
                    Ok(()) => {
                        application_context.ui_context.clear_form_fields_inputs();
                        application_context.reload_state();
                        application_context.ui_context.set_view_mode(ViewMode::Main)
                    }
                    Err(error) => {
                        let popup = Popup::from_error(error.to_string());
                        application_context.ui_context.set_popup(Some(popup));
                    }
                }
            }
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
                ..
            } => application_context.set_show_help(true),
            input => {
                if let Some(field) = application_context.ui_context.get_selected_form_field_mut() {
                    field.on_input(input)
                }
            }
        }
    }
}
