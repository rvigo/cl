use super::State;

#[derive(Default, Clone)]
pub struct NamespacesState {
    selected_idx: usize,
}

impl State for NamespacesState {
    type Output = usize;

    fn select(&mut self, selected: Self::Output) {
        self.selected_idx = selected
    }

    fn selected(&self) -> Self::Output {
        self.selected_idx
    }
}
