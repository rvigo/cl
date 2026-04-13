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
use crate::screen::key_mapping::KeyMapping;
use crate::screen::theme::Theme;
use crate::state::state_event::StateEvent;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::BTreeMap;
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
pub trait Layer: KeyMapping {
    fn render(&mut self, frame: &mut Frame, theme: &Theme);

    fn get_listeners(&self) -> &BTreeMap<TypeId, Vec<Rc<RefCell<dyn Observable>>>>;

    /// Called after this layer is pushed onto the screen and its listeners
    /// are registered.  The default implementation does nothing.
    fn on_attach(&mut self, _state_tx: &Sender<StateEvent>) {}

    /// Called before this layer is removed from the screen.
    fn on_detach(&mut self) {}
}
