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
        if let Err(e) = $state_tx.send(_event).await {
            tracing::error!("oneshot send failed for {}: {}", stringify!($event), e);
        }
        rx.await.map_err(|e| {
            tracing::error!("oneshot receive failed for {}: {}", stringify!($event), e);
            e
        }).ok()
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

