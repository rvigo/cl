mod clipboard_status;
mod editable_textbox_observable;
mod list_observable;
mod popup_observable;
mod screen_state_observable;
mod search_observable;
mod tabs_observable;
mod textbox_observable;

use crate::observer::event::Event;
use std::future::Future;
use std::pin::Pin;

pub type ObservableFuture = Pin<Box<dyn Future<Output = ()>>>;

/// Observable trait — implement this for components whose event handlers may be async.
/// Sync work (state mutations) happens in `on_listen` while the RefMut guard is held.
/// Any async work is returned as an owned future that the caller awaits after dropping the guard.
pub trait Observable {
    fn on_listen(&mut self, event: Event) -> Option<ObservableFuture>;
}

/// Sync observable trait — implement this for components whose event handlers
/// are pure state mutations with no async work. The blanket `Observable` impl
/// below delegates to this, removing unnecessary `async fn` boilerplate.
///
/// Implemented by: [`List`], [`Tabs`], [`TextBox`], [`ClipboardStatus`],
/// [`ScreenState`].
pub trait SyncObservable: std::fmt::Debug + std::any::Any {
    fn on_event(&mut self, event: Event);
}

/// Every `SyncObservable` automatically becomes an `Observable`.
impl<T> Observable for T
where
    T: SyncObservable,
{
    fn on_listen(&mut self, event: Event) -> Option<ObservableFuture> {
        self.on_event(event);
        None
    }
}
