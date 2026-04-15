mod form_screen_layer;
mod main_screen_layer;
mod popup_layer;
mod quick_search_layer;

pub use form_screen_layer::FormMode;
pub use form_screen_layer::FormScreenLayer;
pub use main_screen_layer::MainScreenLayer;
pub use popup_layer::PopupLayer;
pub use quick_search_layer::QuickSearchLayer;

use crate::observer::observable::Observable;
use crate::screen::key_mapping::command::ScreenCommand;
use crate::screen::theme::Theme;
use crate::state::state_event::StateEvent;
use crossterm::event::KeyEvent;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use tokio::sync::mpsc::Sender;
use tui::Frame;

/// A layer in the UI layer stack.
///
/// Layers are pushed/popped on the [`Screen`](crate::screen::Screen).  Each
/// layer renders itself and provides a set of observable listeners that
/// receive events dispatched by the screen.
///
/// # Lifecycle
///
/// * [`on_attach`](Layer::on_attach) — called by the screen *after* the layer
///   is pushed and its listeners are registered.  Use this to pre-populate
///   component state (e.g. loading the command being edited).
/// * [`on_detach`](Layer::on_detach) — called by the screen *before* the
///   layer is popped and its listeners are removed.  Use this to clean up
///   any state the layer holds.
pub trait Layer {
    fn render(&mut self, frame: &mut Frame, theme: &Theme);

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>;

    /// Handle a key event and return a list of screen commands to execute.
    ///
    /// Returns `Pin<Box<dyn Future>>` because `Layer` is used as a trait
    /// object (`dyn Layer`), which requires object-safe method signatures.
    fn handle_key_event<'a>(
        &'a self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>>;

    /// Called after this layer is pushed onto the screen and its listeners
    /// are registered.  The default implementation does nothing.
    fn on_attach(&mut self, _state_tx: &Sender<StateEvent>) {}

    /// Called before this layer is removed from the screen.
    fn on_detach(&mut self) {}
}
