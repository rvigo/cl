#[derive(Default, PartialEq, Debug, Clone, Eq)]
pub struct SelectedNamespace {
    pub idx: usize,
    pub name: String,
}

impl SelectedNamespace {
    pub fn new(idx: usize, name: String) -> Self {
        Self { idx, name }
    }
}
