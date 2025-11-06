use crate::SpinnerSize;
use leptos::{html, prelude::*};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonAppearance {
    /// Gives emphasis to the button in such a way that it indicates a secondary action.
    #[default]
    Secondary,
    /// Emphasizes the button as a primary action.
    Primary,
    /// Minimizes emphasis to blend into the background until hovered or focused.
    Subtle,
    /// Removes background and border styling.
    Transparent,
}

impl ButtonAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonAppearance::Secondary => "secondary",
            ButtonAppearance::Primary => "primary",
            ButtonAppearance::Subtle => "subtle",
            ButtonAppearance::Transparent => "transparent",
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonShape {
    #[default]
    Rounded,
    Circular,
    Square,
}

impl ButtonShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonShape::Rounded => "rounded",
            ButtonShape::Circular => "circular",
            ButtonShape::Square => "square",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Medium => "medium",
            ButtonSize::Large => "large",
        }
    }
}

impl From<ButtonSize> for SpinnerSize {
    fn from(value: ButtonSize) -> Self {
        match value {
            ButtonSize::Small => Self::Tiny,
            ButtonSize::Medium => Self::Tiny,
            ButtonSize::Large => Self::ExtraSmall,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonSizeInjection(pub ButtonSize);

impl ButtonSizeInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}

/// The default behavior of the button.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#type)
#[derive(Debug, Clone)]
pub enum ButtonType {
    /// The button submits the form data to the server.
    /// This is the default if the attribute is not specified for buttons associated with a <form>,
    /// or if the attribute is an empty or invalid value.
    Submit,
    /// The button resets all the controls to their initial values,
    /// like <input type="reset">. (This behavior tends to annoy users.)
    Reset,
    /// The button has no default behavior, and does nothing when pressed by default.
    /// It can have client-side scripts listen to the element's events,
    /// which are triggered when the events occur.
    Button,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
        }
    }
}

#[derive(Clone)]
pub struct ButtonRef {
    pub(super) button_ref: NodeRef<html::Button>,
}

impl ButtonRef {
    /// Click the button element.
    pub fn click(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            _ = button_el.click();
        }
    }

    /// Focus the button element.
    pub fn focus(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            _ = button_el.focus();
        }
    }

    /// Blur the button element
    pub fn blur(&self) {
        if let Some(button_el) = self.button_ref.get_untracked() {
            _ = button_el.blur()
        }
    }
}
