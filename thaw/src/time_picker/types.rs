use crate::InputSize;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TimePickerSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TimePickerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

impl From<TimePickerSize> for InputSize {
    fn from(value: TimePickerSize) -> Self {
        match value {
            TimePickerSize::Small => Self::Small,
            TimePickerSize::Medium => Self::Medium,
            TimePickerSize::Large => Self::Large,
        }
    }
}
