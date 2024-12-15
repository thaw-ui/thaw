#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ColorPickerSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ColorPickerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
