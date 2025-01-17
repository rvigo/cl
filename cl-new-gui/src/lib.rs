mod component;
mod macros;
mod observer;

pub mod crossterm;
pub mod screen;
pub mod state;
pub mod termination;
pub mod ui;

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
