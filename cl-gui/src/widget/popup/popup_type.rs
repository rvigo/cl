#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Type {
    #[default]
    Error,
    Warning,
    Help,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Error => " Error ".to_owned(),
            Type::Warning => " Warning ".to_owned(),
            Type::Help => " Help ".to_owned(),
        }
    }
}
