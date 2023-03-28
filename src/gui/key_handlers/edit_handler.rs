use crate::gui::entities::{
    application_context::ApplicationContext,
    events::app_events::{AppEvents, CommandEvents, RenderEvents},
};
use anyhow::{bail, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use parking_lot::Mutex;
use std::sync::Arc;

pub fn handle(
    key_event: KeyEvent,
    context: &mut Arc<Mutex<ApplicationContext>>,
) -> Result<Option<AppEvents>> {
    let mut c = context.lock();
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
            return Ok(Some(AppEvents::Render(RenderEvents::Main)));
        }
        KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            ..
        } => c.next_form_field(),
        KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
            ..
        } => c.previous_form_field(),

        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            let edited_command = c.edit_command();
            if let Some(old_command) = c.get_selected_command() {
                return Ok(Some(AppEvents::Run(CommandEvents::Edit {
                    old_command: old_command.to_owned(),
                    edited_command,
                })));
            } else {
                bail!("error: cannot edit command")
            }
        }
        KeyEvent {
            code: KeyCode::F(1),
            modifiers: KeyModifiers::NONE,
            ..
        } => c.set_show_help(true),
        input => c.handle_form_input(input),
    }

    Ok(None)
}
