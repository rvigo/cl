mod clipboard_state;
mod field_state;
mod list_state;

pub use clipboard_state::ClipboardState;
pub use field_state::FieldState;
pub use list_state::CommandListState;

pub trait State {
    type Output;

    fn select(&mut self, selected: Self::Output);

    fn selected(&self) -> Self::Output;
}
