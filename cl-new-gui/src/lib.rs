mod component;
mod macros;
mod observer;

pub mod crossterm;
pub mod screen;
pub mod state;
pub mod ui;
pub mod signal_handler;
pub mod clipboard;
mod fuzzy;

pub trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R;
}

impl<T> Pipe for T {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}
