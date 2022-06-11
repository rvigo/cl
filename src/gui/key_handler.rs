use crate::gui::structs::{state::State, view_mode::ViewMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::{error, info};

pub fn handle(key_event: KeyEvent, state: &mut State) {
    match state.view_mode {
        ViewMode::List => handle_list(key_event, state),
        ViewMode::New => handle_insert(key_event, state),
    }
}

pub fn handle_insert(key_event: KeyEvent, state: &mut State) {
    match key_event {
        KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
        } => {
            info!("changing ViewMode to LIST");
            state.view_mode = ViewMode::List;
        }
        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        }
        | KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
        } => {
            state.insert_context.next();
        }
        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        }
        | KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
        } => {
            state.insert_context.previous();
        }
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
        }
        | KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::SHIFT,
        } => state.insert_context.get_current_in_focus().input.push(c),
        KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
        } => {
            state.insert_context.get_current_in_focus().input.pop();
        }
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        } => {
            //TODO implementar popup com msg de erro
            match state.insert_context.build_command() {
                Ok(command) => match state.commands.add_command(command) {
                    Ok(_) => {
                        state.reload_namespaces();
                        state.view_mode = ViewMode::List
                    }
                    Err(error) => error!("{error}"),
                },
                Err(error) => error!("{error}"),
            }
        }
        _ => {}
    }
}

pub fn handle_list(key_event: KeyEvent, state: &mut State) {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
        } => {
            info!("shoul quit = true");
            state.should_quit = true;
        }
        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        }
        | KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
        } => {
            state.previous_namespace();
        }
        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        }
        | KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
        } => {
            state.next_namespace();
        }
        KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        } => {
            state.next_command_item();
        }
        KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        } => {
            state.previous_command_item();
        }
        KeyEvent {
            code: KeyCode::Insert,
            modifiers: KeyModifiers::NONE,
        } => {
            info!("changing ViewMode to NEW");
            state.view_mode = ViewMode::New;
        }
        _ => {}
    }
}
