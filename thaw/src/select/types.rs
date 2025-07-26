#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
