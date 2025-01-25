use std::ops::Deref;

use crate::{FieldValidationState, Rule};
use leptos::prelude::*;
use thaw_utils::OptionModel;

#[derive(Clone, Copy)]
pub(crate) struct RatingInjection {
    pub value: OptionModel<f32>,
    pub hovered_value: RwSignal<Option<f32>>,
    pub name: Memo<String>,
    pub step: Signal<f32>,
    pub size: Signal<RatingSize>,
    pub color: Signal<RatingColor>,
    pub interactive: bool,
}

impl RatingInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Default, Clone)]
pub enum RatingColor {
    Brand,
    // TODO v0.5 Marigold,
    #[default]
    Neutral,
}

impl RatingColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            // RatingColor::Marigold => "marigold",
            Self::Neutral => "neutral",
        }
    }
}

#[derive(Clone)]
pub enum RatingSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl RatingSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum RatingRuleTrigger {
    #[default]
    Change,
}

pub struct RatingRule(Rule<Option<f32>, RatingRuleTrigger>);

impl RatingRule {
    pub fn required(required: Signal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_none() {
                let message = name.get_untracked().map_or_else(
                    || String::from("Please select!"),
                    |name| format!("Please select {name}!"),
                );
                Err(FieldValidationState::Error(message))
            } else {
                Ok(())
            }
        })
    }

    pub fn required_with_message(required: Signal<bool>, message: Signal<String>) -> Self {
        Self::validator(move |value, _| {
            if required.get_untracked() && value.is_none() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&Option<f32>, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: RatingRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for RatingRule {
    type Target = Rule<Option<f32>, RatingRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
