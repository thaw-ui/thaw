use crate::{FieldInjection, FieldValidationState, Rule, RuleValueIsEmpty};
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
        <Provider value=CheckboxGroupInjection{ value, name }>
            <div
                class=class_list!["thaw-checkbox-group", class]
                id=id
                role="group"
            >
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
        Self(Rule::required(required))
    }

    pub fn required_with_message(
        required: MaybeSignal<bool>,
        message: MaybeSignal<String>,
    ) -> Self {
        Self(Rule::required_with_message(required, message))
    }

    pub fn validator(
        f: impl Fn(&HashSet<String>) -> Result<(), FieldValidationState> + Send + Sync + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(mut self, trigger: CheckboxGroupRuleTrigger) -> Self {
        self.0.trigger = trigger;

        self
    }
}

impl Deref for CheckboxGroupRule {
    type Target = Rule<HashSet<String>, CheckboxGroupRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RuleValueIsEmpty for HashSet<String> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}
