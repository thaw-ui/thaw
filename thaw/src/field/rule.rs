use super::{FieldContextInjection, FieldInjection, FieldValidationState};
use chrono::{NaiveDate, NaiveTime};
use leptos::prelude::*;
use std::ops::Deref;
use thaw_utils::{Model, OptionModel, OptionModelWithValue, VecModel, VecModelWithValue};

type RuleValidator<T> =
    Box<dyn Fn(&T, Signal<Option<String>>) -> Result<(), FieldValidationState> + Send + Sync>;

pub struct Rule<T, Trigger> {
    validator: RuleValidator<T>,
    trigger: Trigger,
}

impl<T, Trigger> Rule<T, Trigger> {
    pub fn validator(
        f: impl Fn(&T, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Trigger: Default,
    {
        Self {
            trigger: Default::default(),
            validator: Box::new(f),
        }
    }

    pub fn with_trigger(mut self, trigger: Trigger) -> Self {
        self.trigger = trigger;

        self
    }

    pub fn validate<V, R>(
        rules: Vec<R>,
        value: V,
        name: Signal<Option<String>>,
    ) -> Callback<Option<Trigger>, bool>
    where
        V: RuleValueWithUntracked<T>,
        // V: WithUntracked<Value = T>,
        V: Send + Sync + Copy + 'static,
        R: Deref<Target = Rule<T, Trigger>> + Send + Sync + 'static,
        Trigger: PartialEq + 'static,
    {
        if rules.is_empty() {
            return Callback::new(move |_trigger: Option<Trigger>| {
                return true;
            });
        }

        let field_injection = FieldInjection::use_context();
        // let rules = StoredValue::new(SendWrapper::new(rules));
        let validate = Callback::new(move |trigger: Option<Trigger>| {
            let state = {
                let mut rules_iter = rules.iter();
                let mut call_count = 0;
                loop {
                    let Some(rule) = rules_iter.next() else {
                        if call_count == 0 {
                            break None;
                        } else {
                            break Some(Ok(()));
                        }
                    };

                    if let Some(trigger) = trigger.as_ref() {
                        if &rule.trigger != trigger {
                            continue;
                        }
                    }
                    call_count += 1;

                    let state = value.value_with_untracked(|value| (rule.validator)(value, name));
                    if state.is_err() {
                        break Some(state);
                    }
                }
            };

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

pub trait RuleValueWithUntracked<T> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&T) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState>;
}

impl<T: Send + Sync> RuleValueWithUntracked<T> for Model<T> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&T) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        self.with_untracked(move |v| f(v))
    }
}

impl RuleValueWithUntracked<Option<f32>> for OptionModel<f32> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&Option<f32>) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        self.with_untracked(move |v| match v {
            OptionModelWithValue::T(v) => f(&Some(v.clone())),
            OptionModelWithValue::Option(v) => f(v),
        })
    }
}

impl RuleValueWithUntracked<Option<String>> for OptionModel<String> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&Option<String>) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        self.with_untracked(move |v| match v {
            OptionModelWithValue::T(v) => {
                if v.is_empty() {
                    f(&None)
                } else {
                    f(&Some(v.clone()))
                }
            }
            OptionModelWithValue::Option(v) => f(v),
        })
    }
}

impl RuleValueWithUntracked<Option<NaiveDate>> for OptionModel<NaiveDate> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&Option<NaiveDate>) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        self.with_untracked(move |v| match v {
            OptionModelWithValue::T(v) => f(&Some(v.clone())),
            OptionModelWithValue::Option(v) => f(v),
        })
    }
}

impl RuleValueWithUntracked<Option<NaiveTime>> for OptionModel<NaiveTime> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&Option<NaiveTime>) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        self.with_untracked(move |v| match v {
            OptionModelWithValue::T(v) => f(&Some(v.clone())),
            OptionModelWithValue::Option(v) => f(v),
        })
    }
}

impl RuleValueWithUntracked<Vec<String>> for VecModel<String> {
    fn value_with_untracked(
        &self,
        f: impl FnOnce(&Vec<String>) -> Result<(), FieldValidationState>,
    ) -> Result<(), FieldValidationState> {
        self.with_untracked(move |v| match v {
            VecModelWithValue::T(v) => {
                if v.is_empty() {
                    f(&vec![])
                } else {
                    f(&vec![v.clone()])
                }
            }
            VecModelWithValue::Option(v) => {
                if let Some(v) = v {
                    f(&vec![v.clone()])
                } else {
                    f(&vec![])
                }
            }
            VecModelWithValue::Vec(v) => f(v),
        })
    }
}
