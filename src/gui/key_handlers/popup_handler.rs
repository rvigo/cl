use super::WidgetKeyEventHandler;
use crate::gui::{
    entities::{
        events::app_events::{AppEvent, PopupEvent},
        ui_context::UIContext,
    },
    widgets::popup::MessageType,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub struct PopupHandler;

impl WidgetKeyEventHandler for PopupHandler {
    fn handle(&self, key_event: KeyEvent, ui_context: &mut UIContext) -> Result<Option<AppEvent>> {
        if let Some(popup) = ui_context.popup().as_mut() {
            if let Some(message_type) = popup.message_type().as_ref() {
                match message_type {
                    MessageType::Error => return Ok(Some(AppEvent::Popup(PopupEvent::Disable))),
                    MessageType::Warning => match key_event {
                        KeyEvent {
                            code: KeyCode::Right,
                            ..
                        } => ui_context.next_choice(),
                        KeyEvent {
                            code: KeyCode::Left,
                            ..
                        } => ui_context.previous_choice(),
                        KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        } => {
                            let answer = ui_context.get_selected_choice();
                            return Ok(Some(AppEvent::Popup(PopupEvent::Answer(answer))));
                        }
                        KeyEvent {
                            code: KeyCode::Esc | KeyCode::Char('q'),
                            ..
                        } => {
                            return Ok(Some(AppEvent::Popup(PopupEvent::Disable)));
                        }
                        _ => {}
                    },
                }
            }
        }
        Ok(None)
    }
}
