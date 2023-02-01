use super::Handler;
use crate::gui::{
    entities::state::State,
    layouts::ViewMode,
    widgets::popup::{MessageType, Popup},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct MainHandler;

impl Handler for MainHandler {
    fn handle(&self, key_event: KeyEvent, state: &mut State) {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                ..
            } => state.query_box.toggle_focus(),
            KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                state.should_quit = true;
            }
            KeyEvent {
                code: KeyCode::Left | KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                state.previous_namespace();
            }
            KeyEvent {
                code: KeyCode::Right | KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.next_namespace();
            }
            KeyEvent {
                code: KeyCode::Down | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.next_command();
            }
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.previous_command();
            }
            KeyEvent {
                code: KeyCode::Insert | KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.form_fields_context.reset_fields();
                state.view_mode = ViewMode::Insert;
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if state.form_fields_context.selected_command().is_some() {
                    state.form_fields_context.reset_fields();
                    state.form_fields_context.set_selected_command_input();
                    state.view_mode = ViewMode::Edit;
                }
            }

            KeyEvent {
                code: KeyCode::Char('d') | KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let Some(selected_command) = state.form_fields_context.selected_command() {
                    if !selected_command.is_empty() {
                        let popup = Popup::new(
                            "Are you sure you want to delete the command?",
                            "Delete",
                            Some(MessageType::Delete),
                        );
                        state.popup_context.popup = Some(popup);
                    }
                }
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let Some(selected_command) = state.form_fields_context.selected_command() {
                    if !selected_command.is_empty() {
                        let filtered_commands = state.filter_commands();
                        let selected_index = state.commands_state.selected();
                        if let Some(index) = selected_index {
                            state.to_be_executed = filtered_commands.get(index).cloned();
                            state.should_quit = true
                        }
                    }
                }
            }
            KeyEvent {
                code: KeyCode::F(1) | KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            } => state.show_help = true,
            _ => {}
        }
    }
}
