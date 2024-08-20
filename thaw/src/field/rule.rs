use super::{FieldContextInjection, FieldInjection, FieldValidationState};
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use std::ops::Deref;

enum RuleValidator<T> {
    Required(MaybeSignal<bool>),
    RequiredMessage(MaybeSignal<bool>, MaybeSignal<String>),
    Validator(Box<dyn Fn(&T) -> Result<(), FieldValidationState>>),
}

pub struct Rule<T, Trigger> {
    validator: RuleValidator<T>,
    pub trigger: Trigger,
}

impl<T, Trigger> Rule<T, Trigger>
where
    Trigger: Default,
{
    pub fn required(required: MaybeSignal<bool>) -> Self {
        Self {
            trigger: Default::default(),
            validator: RuleValidator::Required(required),
        }
    }

    pub fn required_with_message(
        required: MaybeSignal<bool>,
        message: MaybeSignal<String>,
    ) -> Self {
        Self {
            trigger: Default::default(),
            validator: RuleValidator::RequiredMessage(required, message),
        }
    }

    pub fn validator(
        f: impl Fn(&T) -> Result<(), FieldValidationState> + Send + Sync + 'static,
    ) -> Self {
        Self {
            trigger: Default::default(),
            validator: RuleValidator::Validator(Box::new(f)),
        }
    }
}

impl<T, Trigger> Rule<T, Trigger> {
    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.trigger = trigger;

        self
    }
}

impl<T: RuleValueIsEmpty, Trigger> Rule<T, Trigger> {
    pub fn call_validator<V: WithUntracked<Value = T>>(
        &self,
        value: V,
        name: Signal<Option<String>>,
    ) -> Result<(), FieldValidationState> {
        value.with_untracked(|value| match &self.validator {
            RuleValidator::Required(required) => {
                if required.get_untracked() && RuleValueIsEmpty::is_empty(value) {
                    let message = name.get_untracked().map_or_else(
                        || String::from("Please input!"),
                        |name| format!("Please input {name}!"),
                    );
                    Err(FieldValidationState::Error(message))
                } else {
                    Ok(())
                }
            }
            RuleValidator::RequiredMessage(required, message) => {
                if required.get_untracked() && RuleValueIsEmpty::is_empty(value) {
                    Err(FieldValidationState::Error(message.get_untracked()))
                } else {
                    Ok(())
                }
            }
            RuleValidator::Validator(f) => f(value),
        })
    }

    pub fn validate<V, R>(rules: Vec<R>, value: V, name: Signal<Option<String>>) -> Callback<Option<Trigger>, bool>
    where
        V: WithUntracked<Value = T>,
        V: Send + Sync + Copy + 'static,
        R: Deref<Target = Rule<T, Trigger>> + 'static,
        Trigger: PartialEq + 'static,
    {
        let field_injection = FieldInjection::use_context();
        let rules = StoredValue::new(SendWrapper::new(rules));
        let validate = Callback::new(move |trigger: Option<Trigger>| {
            let state = rules.with_value(move |rules| {
                if rules.is_empty() {
                    return Some(Ok(()));
                }

                let mut rules_iter = rules.iter();
                let mut call_count = 0;
                loop {
                    let Some(rule) = rules_iter.next() else {
                        if call_count == 0 {
                            return None;
                        } else {
                            return Some(Ok(()));
                        }
                    };

                    if let Some(trigger) = trigger.as_ref() {
                        if &rule.trigger != trigger {
                            continue;
                        }
                    }
                    call_count += 1;

                    let state = rule.call_validator(value, name);
                    if state.is_err() {
                        return Some(state);
                    }
                }
            });

            let Some(state) = state else {
                return true;
            };

            let rt = state.is_ok();
            if let Some(field_injection) = field_injection.as_ref() {
                field_injection.update_validation_state(state);
            };
            rt
        });
        if let Some(field_context) = FieldContextInjection::use_context() {
            field_context.register_field(name, move || validate.run(None));
        }

        validate
    }
}

pub trait RuleValueIsEmpty {
    fn is_empty(&self) -> bool;
}

impl RuleValueIsEmpty for String {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}