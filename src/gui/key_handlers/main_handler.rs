use super::KeyHandler;
use crate::gui::{
    entities::application_context::ApplicationContext, layouts::ViewMode, widgets::popup::Popup,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct MainHandler;

impl KeyHandler for MainHandler {
    fn handle(&self, key_event: KeyEvent, application_context: &mut ApplicationContext) {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                ..
            } => application_context.ui_context.query_box.toggle_focus(),
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
                application_context.quit();
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
                application_context.previous_namespace();
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
                application_context.next_namespace();
            }
            KeyEvent {
                code: KeyCode::Down | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                application_context.next_command();
            }
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                application_context.previous_command();
            }
            KeyEvent {
                code: KeyCode::Insert | KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                application_context
                    .ui_context
                    .form_fields_context
                    .reset_fields();
                application_context
                    .ui_context
                    .set_view_mode(ViewMode::Insert)
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if application_context
                    .ui_context
                    .form_fields_context
                    .selected_command()
                    .is_some()
                {
                    application_context
                        .ui_context
                        .form_fields_context
                        .reset_fields();
                    application_context
                        .ui_context
                        .form_fields_context
                        .set_selected_command_input();
                    application_context.ui_context.set_view_mode(ViewMode::Edit);
                }
            }

            KeyEvent {
                code: KeyCode::Char('d') | KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let Some(selected_command) = application_context
                    .ui_context
                    .form_fields_context
                    .selected_command()
                {
                    if !selected_command.is_empty() {
                        let popup =
                            Popup::from_warning("Are you sure you want to delete the command?");
                        application_context.ui_context.popup_context.popup = Some(popup);
                    }
                }
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let Some(selected_command) = application_context
                    .ui_context
                    .form_fields_context
                    .selected_command()
                {
                    if !selected_command.is_empty() {
                        let filtered_commands = application_context.filter_commands();
                        let selected_index = application_context.commands_state.selected();
                        if let Some(index) = selected_index {
                            application_context.to_be_executed =
                                filtered_commands.get(index).cloned();
                            application_context.quit()
                        }
                    }
                }
            }
            KeyEvent {
                code: KeyCode::F(1) | KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            } => application_context.set_show_help(true),
            _ => {}
        }
    }
}
