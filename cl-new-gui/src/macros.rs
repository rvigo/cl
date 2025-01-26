#[macro_export]
macro_rules! render {
    ($frame:ident, $({ $what:expr, $_where:expr}),* $(,)?) => {
        $(
            $what.borrow_mut().render($frame, $_where);
        )+
    };
}

#[macro_export]
macro_rules! oneshot {
    ($state_tx:expr, $event:ident) => {{
        let (tx, rx) = tokio::sync::oneshot::channel();
        let _event = $event { respond_to: tx };
        $state_tx.send(_event).await.ok();
        rx.await.ok()
    }};
}

#[macro_export]
macro_rules! async_fn_body {
    ($($body:stmt);*) => {
        Box::pin(async move {
            $($body)*
        })
    };
}
