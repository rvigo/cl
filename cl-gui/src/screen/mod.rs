mod command_dispatcher;
mod key_mapping;
pub mod layer;
pub mod theme;

use crate::clipboard::Clipboard;
use crate::observer::event::Event;
use crate::observer::observable::Observable;
use crate::screen::command_dispatcher::{CommandDispatcher, LayerStack, NavigationSnapshot};
use crate::screen::theme::Theme;
use crate::signal_handler::SignalHandler;
use crate::state::state_event::StateEvent;
use cl_core::Command;
use crossterm::event::Event as CrosstermEvent;
use layer::MainScreenLayer;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;
use tracing::debug;
use tui::Frame;

pub type Listeners = BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>;

pub use key_mapping::command;

#[derive(Default, Clone, Debug)]
pub enum ActiveScreen {
    #[default]
    Main,
    Form,
}

pub struct Screen {
    layer_stack: LayerStack,
    pub clipboard: Option<Clipboard>,
    pub theme: Theme,
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen {
    pub fn new() -> Screen {
        let initial = MainScreenLayer::default();
        Self {
            layer_stack: LayerStack::new(initial),
            clipboard: Clipboard::new()
                .map_err(|e| {
                    tracing::warn!("clipboard unavailable: {e}");
                    e
                })
                .ok(),
            theme: Theme::default(),
        }
    }

    pub async fn handle_key_event(
        &mut self,
        event: Option<std::io::Result<CrosstermEvent>>,
        state_tx: &Sender<StateEvent>,
        sig_handler: &mut SignalHandler,
    ) {
        if let Some(Ok(CrosstermEvent::Resize(_, _))) = &event {
            debug!("terminal resized — redraw needed");
        }

        if let Some(Ok(CrosstermEvent::Key(event))) = event {
            if let Some(layer) = self.layer_stack.layers.last_mut() {
                if let Some(commands) = layer.handle_key_event(event, state_tx.clone()).await {
                    CommandDispatcher::dispatch(
                        commands,
                        &mut self.layer_stack,
                        &mut self.clipboard,
                        state_tx,
                        sig_handler,
                    )
                    .await;
                }
            }
        }
    }

    pub async fn notify(&mut self, id: TypeId, event: Event) {
        self.layer_stack.notify(id, event).await;
    }

    /// Set the navigation snapshot used by j/k for UI-local navigation.
    pub fn set_snapshot(&mut self, items: Vec<Command<'static>>, selected_idx: usize) {
        self.layer_stack.snapshot = NavigationSnapshot {
            items,
            selected_idx,
        };
    }

    pub fn render_layers(&mut self, frame: &mut Frame) {
        self.layer_stack.render_layers(frame, &self.theme);
    }
}
