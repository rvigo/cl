use super::State;

#[derive(Clone, Default)]
pub struct AliasListState {
    selected_idx: usize,
}

impl State for AliasListState {
    type Output = usize;

    fn select(&mut self, selected: Self::Output) {
        self.selected_idx = selected
    }

    fn selected(&self) -> Self::Output {
        self.selected_idx
    }
}
