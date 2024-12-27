use super::{AppEventResult, KeyEventHandler};
use crate::event::{
    AppEvent, CommandEvent, DialogType, MainScreenEvent, PopupEvent, PopupType, QueryboxEvent,
    RenderEvent, ScreenEvent,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct MainScreenHandler;

impl KeyEventHandler for MainScreenHandler {
    fn handle(&self, key_event: KeyEvent) -> AppEventResult {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('/'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::QueryBox(QueryboxEvent::Active))),
            KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Ok(Some(AppEvent::Quit)),
            KeyEvent {
                code: KeyCode::Left | KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => Ok(Some(AppEvent::Screen(ScreenEvent::Main(
                MainScreenEvent::PreviousNamespace,
            )))),
            KeyEvent {
                code: KeyCode::Right | KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Screen(ScreenEvent::Main(
                MainScreenEvent::NextNamespace,
            )))),
            KeyEvent {
                code: KeyCode::Down | KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Screen(ScreenEvent::Main(
                MainScreenEvent::NextCommand,
            )))),
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Screen(ScreenEvent::Main(
                MainScreenEvent::PreviousCommand,
            )))),
            KeyEvent {
                code: KeyCode::Insert | KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Render(RenderEvent::Insert))),
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Render(RenderEvent::Edit))),
            KeyEvent {
                code: KeyCode::Char('d') | KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Popup(PopupEvent::Enable(
                PopupType::Dialog(DialogType::CommandDeletionConfimation),
            )))),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Run(CommandEvent::Execute))),
            KeyEvent {
                code: KeyCode::F(1) | KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Popup(PopupEvent::Enable(PopupType::Help)))),
            KeyEvent {
                code: KeyCode::Char('y'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Ok(Some(AppEvent::Run(CommandEvent::Copy))),
            _ => Ok(None),
        }
    }
}
