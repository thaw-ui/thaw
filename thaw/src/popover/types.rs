use leptos::prelude::*;
use thaw_components::FollowerPlacement;

#[derive(Debug, Default, Clone)]
pub enum PopoverSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl PopoverSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Clone)]
pub enum PopoverAppearance {
    Brand,
    Inverted,
}

impl PopoverAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopoverAppearance::Brand => "brand",
            PopoverAppearance::Inverted => "inverted",
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub enum PopoverTriggerType {
    #[default]
    Hover,
    Click,
}

impl Copy for PopoverTriggerType {}

#[derive(Default)]
pub enum PopoverPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
}

impl From<PopoverPosition> for FollowerPlacement {
    fn from(value: PopoverPosition) -> Self {
        match value {
            PopoverPosition::Top => Self::Top,
            PopoverPosition::Bottom => Self::Bottom,
            PopoverPosition::Left => Self::Left,
            PopoverPosition::Right => Self::Right,
            PopoverPosition::TopStart => Self::TopStart,
            PopoverPosition::TopEnd => Self::TopEnd,
            PopoverPosition::LeftStart => Self::LeftStart,
            PopoverPosition::LeftEnd => Self::LeftEnd,
            PopoverPosition::RightStart => Self::RightStart,
            PopoverPosition::RightEnd => Self::RightEnd,
            PopoverPosition::BottomStart => Self::BottomStart,
            PopoverPosition::BottomEnd => Self::BottomEnd,
        }
    }
}

#[slot]
pub struct PopoverTrigger {
    #[prop(optional, into)]
    class: MaybeProp<String>,
    children: Children,
}
