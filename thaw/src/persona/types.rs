use leptos::prelude::*;

#[slot]
pub struct PersonaPrimaryText {
    children: Children,
}

#[slot]
pub struct PersonaSecondaryText {
    children: Children,
}

#[slot]
pub struct PersonaTertiaryText {
    children: Children,
}

#[slot]
pub struct PersonaQuaternaryText {
    children: Children,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PersonaTextAlignment {
    #[default]
    Start,
    Center,
}

impl PersonaTextAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PersonaTextPosition {
    Before,
    #[default]
    After,
    Below,
}

impl PersonaTextPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Before => "before",
            Self::After => "after",
            Self::Below => "below",
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum PersonaSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    Huge,
}

impl PersonaSize {
    pub(crate) fn as_avatar_size(&self) -> u8 {
        match self {
            Self::ExtraSmall => 20,
            Self::Small => 28,
            Self::Medium => 32,
            Self::Large => 36,
            Self::ExtraLarge => 40,
            Self::Huge => 56,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExtraSmall => "extra-small",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
            Self::Huge => "huge",
        }
    }
}
