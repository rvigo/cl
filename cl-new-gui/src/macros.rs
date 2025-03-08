#[macro_export]
macro_rules! render {
     ($frame:ident, $theme:expr, $({ $what:expr , $_where:expr}),* $(,)?) => {
        $(
            $what.render($frame, $_where, $theme);
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

#[macro_export]
macro_rules! event {
    ($type_:ty, $event:expr) => {
        ScreenCommand::Notify((std::any::TypeId::of::<$type_>(), $event))
    };
}

#[macro_export]
macro_rules! run_if_some {
    (
        $option:expr,
        $callback:expr
    ) => {
        if let Some(value) = $option {
            $callback(value)
        }
        else { 
            None
        }
    };
}
