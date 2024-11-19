use crate::{icon::ChevronDownRegularIcon, FieldInjection, FieldValidationState, Rule};
use leptos::{html, prelude::*};
use std::ops::Deref;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn Select(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<SelectRule>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] default_value: Option<String>,
    /// Whether the select is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("select", include_str!("./select.css"));
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);
    let select_ref = NodeRef::<html::Select>::new();
    Effect::new(move |prev: Option<bool>| {
        let Some(el) = select_ref.get() else {
            return false;
        };

        let el_value = el.value();
        if !prev.unwrap_or_default() {
            if let Some(default_value) = default_value.as_ref() {
                el.set_value(default_value);
                value.set(default_value.clone());
            } else {
                value.set(el_value);
            }
            value.with(|_| {});
            return true;
        }
        value.with(|value| {
            if value != &el_value {
                el.set_value(value);
            }
        });
        true
    });

    let on_change = move |_| {
        let Some(el) = select_ref.get() else {
            return;
        };
        value.set(el.value());
        validate.run(Some(SelectRuleTrigger::Change));
    };

    view! {
        <span class=class_list![
            "thaw-select",
                ("thaw-select--disabled", move || disabled.get()),
                class
        ]>
            <select
                class="thaw-select__select"
                id=id
                name=name
                node_ref=select_ref
                disabled=disabled
                on:change=on_change
            >
                {children()}
            </select>
            <span class="thaw-select__icon">
                <ChevronDownRegularIcon />
            </span>
        </span>
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectRuleTrigger {
    #[default]
    Change,
}

pub struct SelectRule(Rule<String, SelectRuleTrigger>);

impl SelectRule {
    pub fn required(required: Signal<bool>) -> Self {
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
        required: Signal<bool>,
        message: Signal<String>,
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
        f: impl Fn(&String, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: SelectRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for SelectRule {
    type Target = Rule<String, SelectRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
