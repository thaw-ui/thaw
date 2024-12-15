use leptos::{html, prelude::*};

#[derive(Clone)]
pub struct TextareaRef {
    pub(super) textarea_ref: NodeRef<html::Textarea>,
}

impl TextareaRef {
    /// Focus the input element.
    pub fn focus(&self) {
        if let Some(textarea_el) = self.textarea_ref.get_untracked() {
            _ = textarea_el.focus();
        }
    }

    /// Blur the input element.
    pub fn blur(&self) {
        if let Some(textarea_el) = self.textarea_ref.get_untracked() {
            _ = textarea_el.blur();
        }
    }
}

#[derive(Clone, Default)]
pub enum TextareaResize {
    #[default]
    None,
    Both,
    Horizontal,
    Vertical,
}

impl TextareaResize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaResize::None => "none",
            TextareaResize::Both => "both",
            TextareaResize::Horizontal => "horizontal",
            TextareaResize::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TextareaSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TextareaSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
