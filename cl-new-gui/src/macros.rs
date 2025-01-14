#[macro_export]
macro_rules! render {
    ($frame:ident, $({ $what:tt, $_where:expr}),* $(,)?) => {
        $(
            $frame.render_widget($what, $_where);
        )+
    };



    ($frame:ident, $({ $what:path, $_where:expr, $state:expr}),* $(,)?) => {
        $(
            $frame.render_stateful_widget($what, $_where, $state);
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