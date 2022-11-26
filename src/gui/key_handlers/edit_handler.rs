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

pub struct EditHandler;

impl Handler for EditHandler {
    fn handle(&self, key_event: KeyEvent, state: &mut State) {
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
            } => {
                let field_context = &mut state.form_fields_context;
                let edited_command = field_context.edit_command();

                match edited_command {
                    Ok(command) => match state.commands.add_edited_command(
                        &command,
                        field_context
                            .selected_command()
                            .expect("A command should always be selected"),
                    ) {
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
                        let popup =
                            Popup::new(error.to_string(), "Error", Some(MessageType::Error));
                        state.popup_context.popup = Some(popup);
                    }
                }
            }
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
