use super::{AppEventResult, KeyEventHandler};
use crate::event::{
    AppEvent, CommandEvent, FormScreenEvent, PopupEvent, PopupType, RenderEvent, ScreenEvent,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct EditScreenHandler;

impl KeyEventHandler for EditScreenHandler {
    fn handle(&self, key_event: KeyEvent) -> AppEventResult {
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
            } => Ok(Some(AppEvent::Render(RenderEvent::Main))),
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Screen(ScreenEvent::Form(
                FormScreenEvent::NextField,
            )))),
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => Ok(Some(AppEvent::Screen(ScreenEvent::Form(
                FormScreenEvent::PreviousField,
            )))),
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Ok(Some(AppEvent::Run(CommandEvent::Edit))),
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Popup(PopupEvent::Enable(PopupType::Help)))),
            input => Ok(Some(AppEvent::Screen(ScreenEvent::Form(
                FormScreenEvent::Input(input),
            )))),
        }
    }
}
