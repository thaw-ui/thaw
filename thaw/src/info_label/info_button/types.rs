#[derive(Debug, Default, Clone)]
pub enum InfoButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl InfoButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}