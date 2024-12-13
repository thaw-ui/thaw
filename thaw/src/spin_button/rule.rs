
use crate::{FieldValidationState, Rule};
use leptos::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SpinButtonRuleTrigger {
    #[default]
    Change,
}

pub struct SpinButtonRule<T>(Rule<T, SpinButtonRuleTrigger>);

impl<T> SpinButtonRule<T> {
    pub fn validator(
        f: impl Fn(&T, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: SpinButtonRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl<T> Deref for SpinButtonRule<T> {
    type Target = Rule<T, SpinButtonRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}