pub mod answer_state;
pub mod field_state;
pub mod namespace_state;
pub mod ui_state;

pub trait State {
    type Output;

    fn select(&mut self, selected: Self::Output);

    fn selected(&self) -> Self::Output;
}
