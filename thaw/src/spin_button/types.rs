#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SpinButtonSize {
    Small,
    #[default]
    Medium,
}

impl SpinButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
        }
    }
}
