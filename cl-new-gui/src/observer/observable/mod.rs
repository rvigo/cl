mod clipboard_status;
mod editable_textbox_observable;
mod list_observable;
mod popup_observable;
mod screen_state_observable;
mod search_observable;
mod tabs_observable;
mod textbox_observable;

use crate::observer::event::Event;
use async_trait::async_trait;

/// Async observable trait — implement this for components whose event handlers
/// need to `.await` (e.g. sending on a channel). Popup, Search, and
/// EditableTextbox use this directly.
#[async_trait(?Send)]
pub trait Observable {
    async fn on_listen(&mut self, event: Event);
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
#[async_trait(?Send)]
impl<T> Observable for T
where
    T: SyncObservable,
{
    async fn on_listen(&mut self, event: Event) {
        self.on_event(event);
    }
}
