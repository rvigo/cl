/// Render one or more components into their respective areas.
///
/// ```ignore
/// render! { frame, theme, { self.list, list_rect }, { self.tabs, tabs_rect } }
/// ```
#[macro_export]
macro_rules! render {
    ($frame:ident, $theme:expr, $({ $what:expr , $_where:expr}),* $(,)?) => {
        $(
            $what.render($frame, $_where, $theme);
        )+
    };
}

/// Send a one-shot request to the `StateActor` and await the reply.
///
/// ```ignore
/// let cmd = oneshot!(state_tx, CurrentCommand);
/// ```
#[macro_export]
macro_rules! oneshot {
    ($state_tx:expr, $event:ident) => {{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _event = $event { respond_to: tx };
        $state_tx.send(_event).await.ok();
        rx.await.ok()
    }};
}

/// Box a set of statements into a pinned async future — used for
/// `FutureEventType::State` callbacks.
#[macro_export]
macro_rules! async_fn_body {
    ($($body:stmt);*) => {
        Box::pin(async move {
            $($body)*
        })
    };
}

/// Build a `ScreenCommand::Notify` for a specific component type.
///
/// The component name (first argument) must match both:
/// - a type imported in the calling module (for `TypeId::of::<$component>()`), and
/// - a variant of `Event` with the same name (e.g. `Event::List`, `Event::Tabs`).
///
/// ```ignore
/// event!(List, ListEvent::Next(idx))
/// event!(Popup, PopupEvent::Create(Dialog(...)))
/// event!(ClipboardStatus, ClipboardAction::Copied)
/// ```
#[macro_export]
macro_rules! event {
    ($component:ident, $e:expr) => {
        ScreenCommand::Notify((
            std::any::TypeId::of::<$component>(),
            Event::$component($e),
        ))
    };
}

/// Build the `Listeners` map used by a `Layer`'s `get_listeners()` method.
///
/// Each `Type => component` pair registers `component.get_observable()` under
/// `TypeId::of::<Type>()`.  Multiple components of the same type are all
/// registered under the same key.
///
/// ```ignore
/// let listeners = listeners! {
///     List  => list_component,
///     Tabs  => tabs_component,
///     TextBox => cmd_component,
///     TextBox => desc_component,
/// };
/// ```
#[macro_export]
macro_rules! listeners {
    ($($type_:ty => $component:expr),* $(,)?) => {{
        let mut _listeners: $crate::screen::Listeners = std::collections::BTreeMap::new();
        $(
            _listeners
                .entry(std::any::TypeId::of::<$type_>())
                .or_default()
                .push($component.get_observable());
        )*
        _listeners
    }};
}

#[macro_export]
macro_rules! run_if_some {
    ($option:expr, $callback:expr) => {
        if let Some(value) = $option {
            $callback(value)
        } else {
            None
        }
    };
}

#[macro_export]
macro_rules! try_get_renderable {
    ($component:expr, $type_:ty) => {
        $component.borrow_mut().as_any().downcast_mut::<$type_>()
    };
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::observer::event::{Event, ListEvent, TabsEvent};
    use crate::screen::Listeners;
    use crate::component::{List, Tabs};
    use std::any::TypeId;

    #[test]
    fn listeners_macro_registers_single_component() {
        let list = crate::component::RenderableComponent::new(List::new());
        let listeners: Listeners = listeners! {
            List => list,
        };
        assert!(listeners.contains_key(&TypeId::of::<List>()));
        assert_eq!(listeners[&TypeId::of::<List>()].len(), 1);
    }

    #[test]
    fn listeners_macro_registers_multiple_components_under_same_type() {
        use crate::component::{TextBox, TextBoxName};
        let cmd = crate::component::RenderableComponent::new(TextBox {
            name: TextBoxName::Command,
            ..Default::default()
        });
        let desc = crate::component::RenderableComponent::new(TextBox {
            name: TextBoxName::Description,
            ..Default::default()
        });
        let listeners: Listeners = listeners! {
            TextBox => cmd,
            TextBox => desc,
        };
        assert_eq!(listeners[&TypeId::of::<TextBox>()].len(), 2);
    }

    #[test]
    fn listeners_macro_handles_multiple_different_types() {
        let list = crate::component::RenderableComponent::new(List::new());
        let tabs = crate::component::RenderableComponent::new(Tabs::default());
        let listeners: Listeners = listeners! {
            List => list,
            Tabs => tabs,
        };
        assert!(listeners.contains_key(&TypeId::of::<List>()));
        assert!(listeners.contains_key(&TypeId::of::<Tabs>()));
    }
}
