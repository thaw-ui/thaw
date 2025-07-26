#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum CheckboxSize {
    #[default]
    Medium,
    Large,
}

impl CheckboxSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
