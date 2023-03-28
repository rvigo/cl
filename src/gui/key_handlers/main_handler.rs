use crate::gui::entities::{
    application_context::ApplicationContext,
    events::app_events::{AppEvents, RenderEvents},
};
use anyhow::Result;
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
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::NONE,
            ..
        } => c.toogle_querybox_focus(),
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
            return Ok(Some(AppEvents::Quit));
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
            c.previous_namespace();
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
            c.next_namespace();
        }
        KeyEvent {
            code: KeyCode::Down | KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            c.next_command();
        }
        KeyEvent {
            code: KeyCode::Up | KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            c.previous_command();
        }
        KeyEvent {
            code: KeyCode::Insert | KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            return Ok(Some(AppEvents::Render(RenderEvents::Insert)));
        }
        KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
            ..
        } => return Ok(Some(AppEvents::Render(RenderEvents::Edit))),
        KeyEvent {
            code: KeyCode::Char('d') | KeyCode::Delete,
            modifiers: KeyModifiers::NONE,
            ..
        } => c.show_delete_popup(),
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            if let Some(command) = c.get_selected_command() {
                return Ok(Some(AppEvents::Run(
                    crate::gui::entities::events::app_events::CommandEvents::Execute(
                        command.to_owned(),
                    ),
                )));
            }
        }
        KeyEvent {
            code: KeyCode::F(1) | KeyCode::Char('?'),
            modifiers: KeyModifiers::NONE,
            ..
        } => c.set_show_help(true),
        _ => {}
    }

    Ok(None)
}
