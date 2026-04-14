/// Render one or more components into their respective areas.
///
/// ```ignore
/// render! { frame, theme, { self.list, list_rect }, { self.tabs, tabs_rect } }
/// ```
macro_rules! render {
    ($frame:ident, $theme:expr, $({ $what:expr , $_where:expr}),* $(,)?) => {
        $(
            $what.render($frame, $_where, $theme);
        )+
    };
}

/// Send a one-shot request to the `StateActor` and await the reply.
///
/// Returns `Result<T, anyhow::Error>` so callers can distinguish channel
/// failures from legitimate empty responses.
///
/// ```ignore
/// let cmd = oneshot!(state_tx, CurrentCommand)?;
/// ```
macro_rules! oneshot {
    ($state_tx:expr, $event:ident) => {{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _event = $event { respond_to: tx };
        if let Err(e) = $state_tx.send(_event).await {
            tracing::error!("oneshot send failed for {}: {}", stringify!($event), e);
            Err(anyhow::anyhow!(
                "oneshot send failed for {}: {}",
                stringify!($event),
                e
            ))
        } else {
            rx.await.map_err(|e| {
                tracing::error!("oneshot receive failed for {}: {}", stringify!($event), e);
                anyhow::anyhow!("oneshot receive failed for {}: {}", stringify!($event), e)
            })
        }
    }};
}

/// Box a set of statements into a pinned async future — used for
/// `FutureEventType::State` callbacks.
macro_rules! async_fn_body {
    ($($body:stmt);*) => {
        Box::pin(async move {
            $($body)*
        })
    };
}
