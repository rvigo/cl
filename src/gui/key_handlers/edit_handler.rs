use super::KeyHandler;
use crate::gui::entities::application_context::ApplicationContext;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct EditHandler;

impl KeyHandler for EditHandler {
    fn handle(&self, key_event: KeyEvent, context: &mut ApplicationContext) {
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
                context.enter_main_mode();
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => context.next_form_field(),
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => context.previous_form_field(),

            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => context.add_edited_command(),
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
                ..
            } => context.set_show_help(true),
            input => context.handle_form_input(input),
        }
    }
}
