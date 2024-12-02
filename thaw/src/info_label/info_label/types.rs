use super::super::info_button::InfoButtonSize;
use crate::{LabelSize, LabelWeight};
use leptos::{prelude::Children, slot};

#[derive(Debug, Default, Clone)]
pub enum InfoLabelSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl InfoLabelSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

impl From<InfoLabelSize> for LabelSize {
    fn from(value: InfoLabelSize) -> Self {
        match value {
            InfoLabelSize::Small => Self::Small,
            InfoLabelSize::Medium => Self::Medium,
            InfoLabelSize::Large => Self::Large,
        }
    }
}

impl From<InfoLabelSize> for InfoButtonSize {
    fn from(value: InfoLabelSize) -> Self {
        match value {
            InfoLabelSize::Small => Self::Small,
            InfoLabelSize::Medium => Self::Medium,
            InfoLabelSize::Large => Self::Large,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum InfoLabelWeight {
    #[default]
    Regular,
    Semibold,
}

impl From<InfoLabelWeight> for LabelWeight {
    fn from(value: InfoLabelWeight) -> Self {
        match value {
            InfoLabelWeight::Regular => Self::Regular,
            InfoLabelWeight::Semibold => Self::Semibold,
        }
    }
}

#[slot]
pub struct InfoLabelInfo {
    children: Children,
}
