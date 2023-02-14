use super::KeyHandler;
use crate::{
    gui::{
        entities::application_context::ApplicationContext, layouts::ViewMode, widgets::popup::Popup,
    },
    resources::file_service,
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
                application_context
                    .ui_context
                    .form_fields_context
                    .fields
                    .clear_fields_input();
                application_context
                    .ui_context
                    .form_fields_context
                    .focus_state
                    .select(Some(0));
                application_context.ui_context.set_view_mode(ViewMode::Main);
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => application_context
                .ui_context
                .form_fields_context
                .next_field(),

            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => application_context
                .ui_context
                .form_fields_context
                .previous_field(),

            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                if let Ok(command) = application_context
                    .ui_context
                    .form_fields_context
                    .build_new_command()
                {
                    match application_context.commands.add_command(&command) {
                        Ok(commands) => {
                            if let Ok(()) = file_service::write_to_command_file(commands) {
                                application_context
                                    .ui_context
                                    .form_fields_context
                                    .fields
                                    .clear_fields_input();
                                application_context.reload_state();
                                application_context.ui_context.set_view_mode(ViewMode::Main)
                            }
                        }
                        Err(error) => {
                            let popup = Popup::from_error(error.to_string());
                            application_context.ui_context.popup_context.popup = Some(popup);
                        }
                    }
                }
            }
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
                ..
            } => application_context.set_show_help(true),
            input => {
                if let Some(field) = application_context
                    .ui_context
                    .form_fields_context
                    .selected_mut_field()
                {
                    field.on_input(input)
                }
            }
        }
    }
}
