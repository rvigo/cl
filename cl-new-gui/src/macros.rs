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
