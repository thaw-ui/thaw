use crate::{FieldInjection, FieldValidationState, Rule};
use leptos::{context::Provider, prelude::*};
use std::ops::Deref;
use thaw_utils::{class_list, OptionModel};

#[component]
pub fn RadioGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<RadioGroupRule>,
    /// The selected Radio item in this group.
    #[prop(optional, into)]
    value: OptionModel<String>,
    /// The name of this radio group.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);

    Effect::new(move |prev: Option<()>| {
        value.with(|_| {});
        if prev.is_some() {
            validate.run(Some(RadioGroupRuleTrigger::Change));
        }
    });

    let name = Signal::derive(move || {
        name.get()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
    });

    view! {
        <Provider value=RadioGroupInjection { value, name }>
            <div class=class_list!["thaw-radio-group", class] id=id role="radiogroup">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct RadioGroupInjection {
    pub value: OptionModel<String>,
    pub name: Signal<String>,
}

impl RadioGroupInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum RadioGroupRuleTrigger {
    #[default]
    Change,
}

pub struct RadioGroupRule(Rule<Option<String>, RadioGroupRuleTrigger>);

impl RadioGroupRule {
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
        f: impl Fn(&Option<String>, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: RadioGroupRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for RadioGroupRule {
    type Target = Rule<Option<String>, RadioGroupRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
