use crate::gui::entities::events::app_events::{
    AppEvents, CommandEvents, FormScreenEvent, PopupEvent, PopupType, RenderEvents, ScreenEvents,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key_event: KeyEvent) -> Result<Option<AppEvents>> {
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
        } => Ok(Some(AppEvents::Render(RenderEvents::Main))),
        KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Screen(ScreenEvents::Form(
            FormScreenEvent::NextField,
        )))),

        KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
            ..
        } => Ok(Some(AppEvents::Screen(ScreenEvents::Form(
            FormScreenEvent::PreviousField,
        )))),
        KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Ok(Some(AppEvents::Run(CommandEvents::Insert))),
        KeyEvent {
            code: KeyCode::F(1),
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Popup(PopupEvent::Enable(PopupType::Help)))),
        input => Ok(Some(AppEvents::Screen(ScreenEvents::Form(
            FormScreenEvent::Input(input),
        )))),
    }
}
