use crate::InputSize;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DatePickerSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl DatePickerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

impl From<DatePickerSize> for InputSize {
    fn from(value: DatePickerSize) -> Self {
        match value {
            DatePickerSize::Small => Self::Small,
            DatePickerSize::Medium => Self::Medium,
            DatePickerSize::Large => Self::Large,
        }
    }
}
