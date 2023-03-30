use crate::gui::entities::events::app_events::{
    AppEvents, CommandEvents, MainScreenEvent, PopupCallbackAction, PopupEvent, PopupType,
    QueryboxEvent, RenderEvents, ScreenEvents,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key_event: KeyEvent) -> Result<Option<AppEvents>> {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::QueryBox(QueryboxEvent::Active))),
        KeyEvent {
            code: KeyCode::Char('q') | KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            ..
        }
        | KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => Ok(Some(AppEvents::Quit)),
        KeyEvent {
            code: KeyCode::Left | KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
            ..
        }
        | KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
            ..
        } => Ok(Some(AppEvents::Screen(ScreenEvents::Main(
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
        } => Ok(Some(AppEvents::Screen(ScreenEvents::Main(
            MainScreenEvent::NextNamespace,
        )))),
        KeyEvent {
            code: KeyCode::Down | KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Screen(ScreenEvents::Main(
            MainScreenEvent::NextCommand,
        )))),
        KeyEvent {
            code: KeyCode::Up | KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Screen(ScreenEvents::Main(
            MainScreenEvent::PreviousCommand,
        )))),
        KeyEvent {
            code: KeyCode::Insert | KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Render(RenderEvents::Insert))),
        KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Render(RenderEvents::Edit))),
        KeyEvent {
            code: KeyCode::Char('d') | KeyCode::Delete,
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Popup(PopupEvent::Enable(
            PopupType::Dialog {
                message: "Are you sure you want to delete the command?".to_owned(),
                callback_action: PopupCallbackAction::Delete,
            },
        )))),
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Run(CommandEvents::Execute))),
        KeyEvent {
            code: KeyCode::F(1) | KeyCode::Char('?'),
            modifiers: KeyModifiers::NONE,
            ..
        } => Ok(Some(AppEvents::Popup(PopupEvent::Enable(PopupType::Help)))),
        _ => Ok(None),
    }
}
