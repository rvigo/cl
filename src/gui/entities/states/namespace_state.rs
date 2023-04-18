use super::State;

#[derive(Debug, Clone, Default)]
pub struct NamespaceState {
    pub selected: usize,
}

impl State for NamespaceState {
    type Output = usize;
    fn selected(&self) -> usize {
        self.selected
    }

    fn select(&mut self, index: usize) {
        self.selected = index;
    }
}
