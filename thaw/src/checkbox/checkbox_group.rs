use crate::{FieldInjection, FieldValidationState, Rule};
use leptos::{context::Provider, prelude::*};
use std::{collections::HashSet, ops::Deref};
use thaw_utils::{class_list, Model};

#[component]
pub fn CheckboxGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<CheckboxGroupRule>,
    /// Sets the value of the checkbox group.
    #[prop(optional, into)]
    value: Model<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);
    Effect::new(move |prev: Option<()>| {
        value.with(|_| {});
        if prev.is_some() {
            validate.run(Some(CheckboxGroupRuleTrigger::Change));
        }
    });

    view! {
        <Provider value=CheckboxGroupInjection {
            value,
            name,
        }>
            <div class=class_list!["thaw-checkbox-group", class] id=id role="group">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct CheckboxGroupInjection {
    pub value: Model<HashSet<String>>,
    pub name: Signal<Option<String>>,
}

impl Copy for CheckboxGroupInjection {}

impl CheckboxGroupInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum CheckboxGroupRuleTrigger {
    #[default]
    Change,
}

pub struct CheckboxGroupRule(Rule<HashSet<String>, CheckboxGroupRuleTrigger>);

impl CheckboxGroupRule {
    pub fn required(required: MaybeSignal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_empty() {
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

    pub fn required_with_message(
        required: MaybeSignal<bool>,
        message: MaybeSignal<String>,
    ) -> Self {
        Self::validator(move |value, _| {
            if required.get_untracked() && value.is_empty() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&HashSet<String>, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: CheckboxGroupRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for CheckboxGroupRule {
    type Target = Rule<HashSet<String>, CheckboxGroupRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
