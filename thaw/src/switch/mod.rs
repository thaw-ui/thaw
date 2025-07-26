use crate::{FieldInjection, FieldValidationState, Rule};
use leptos::{html, prelude::*};
use std::ops::Deref;
use thaw_utils::{class_list, Model};

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/switch/switch.css");

#[component]
pub fn Switch(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// A String specifying the value of the input control.
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// The rules to validate Field.
    #[prop(optional, into)]
    rules: Vec<SwitchRule>,
    /// Defines the controlled checked state of the Switch.
    #[prop(optional, into)]
    checked: Model<bool>,
    /// The Switch's label.
    #[prop(optional, into)]
    label: MaybeProp<String>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("switch", include_str!("./switch.css"));

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, checked, name);
    let id = Signal::derive(move || id.get().unwrap_or_else(|| uuid::Uuid::new_v4().to_string()));
    let input_ref = NodeRef::<html::Input>::new();
    let on_change = move |_| {
        let input = input_ref.get_untracked().unwrap();
        let did_update = checked.try_maybe_update(|checked| {
            if *checked != input.checked() {
                *checked = input.checked();
                (true, true)
            } else {
                (false, false)
            }
        });
        if did_update.unwrap_or_default() {
            validate.run(Some(SwitchRuleTrigger::Change));
        }
    };

    view! {
        <div class=class_list!["thaw-switch", class]>
            <input
                class="thaw-switch__input"
                role="switch"
                type="checkbox"
                id=id
                name=name
                value=move || value.get()
                checked=checked.get_untracked()
                prop:checked=move || { checked.get() }
                node_ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" class="thaw-switch__indicator thaw-switch__button">
                <svg
                    fill="currentColor"
                    aria-hidden="true"
                    width="1em"
                    height="1em"
                    viewBox="0 0 20 20"
                >
                    <path d="M10 2a8 8 0 1 0 0 16 8 8 0 0 0 0-16Z" fill="currentColor"></path>
                </svg>
            </div>
            {move || {
                if let Some(label) = label.get() {
                    view! {
                        <label class="thaw-switch__label" for=id>
                            {label}
                        </label>
                    }
                        .into()
                } else {
                    None
                }
            }}
        </div>
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SwitchRuleTrigger {
    #[default]
    Change,
}

pub struct SwitchRule(Rule<bool, SwitchRuleTrigger>);

impl SwitchRule {
    pub fn validator(
        f: impl Fn(&bool, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: SwitchRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for SwitchRule {
    type Target = Rule<bool, SwitchRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
