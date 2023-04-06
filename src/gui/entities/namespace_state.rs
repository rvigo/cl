#[derive(Debug, Clone, Default)]
pub struct NamespaceState {
    pub selected: usize,
}

impl NamespaceState {
    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }
}
