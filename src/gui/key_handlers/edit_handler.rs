use super::{KeyEventHandler, ViKeyEventHandler};
use crate::gui::entities::{
    events::app_events::{
        AppEvent, CommandEvent, FormScreenEvent, PopupEvent, PopupType, RenderEvent, ScreenEvent,
    },
    states::vi_state::ViMode,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct EditScreenHandler;

impl KeyEventHandler for EditScreenHandler {
    fn handle(&self, key_event: KeyEvent) -> Result<Option<AppEvent>> {
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

impl ViKeyEventHandler for EditScreenHandler {
    fn handle(&self, key_event: KeyEvent, mode: &ViMode) -> Result<Option<AppEvent>> {
        match mode {
            ViMode::Normal => match key_event {
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
            },
            ViMode::Insert => Ok(Some(AppEvent::Screen(ScreenEvent::Form(
                FormScreenEvent::Input(key_event),
            )))),
        }
    }
}
