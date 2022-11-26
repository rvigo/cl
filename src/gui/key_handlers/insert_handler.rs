use super::Handler;
use crate::{
    gui::{
        entities::state::State,
        layouts::ViewMode,
        widgets::popup::{MessageType, Popup},
    },
    resources::file_service,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct InsertHandler;

impl Handler for InsertHandler {
    fn handle<'a>(&self, key_event: KeyEvent, state: &mut State<'a>) {
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
                state.form_fields_context.clear_fields_input();
                state.view_mode = ViewMode::Main;
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => state.form_fields_context.next_field(),

            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => state.form_fields_context.previous_field(),

            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => match state.form_fields_context.build_new_command() {
                Ok(command) => match state.commands.add_command(&command) {
                    Ok(commands) => {
                        if let Ok(()) = file_service::write_to_command_file(commands) {
                            state.form_fields_context.clear_fields_input();
                            state.reload_state();
                            state.view_mode = ViewMode::Main
                        }
                    }
                    Err(error) => {
                        let popup =
                            Popup::new(error.to_string(), "Error", Some(MessageType::Error));
                        state.popup_context.popup = Some(popup);
                    }
                },
                Err(error) => {
                    let popup = Popup::new(error.to_string(), "Error", Some(MessageType::Error));
                    state.popup_context.popup = Some(popup);
                }
            },
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
                ..
            } => state.show_help = true,
            input => state
                .form_fields_context
                .selected_mut_field()
                .unwrap()
                .on_input(input),
        }
    }
}
