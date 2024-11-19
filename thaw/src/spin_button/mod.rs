use crate::{FieldInjection, FieldValidationState, Rule};
use leptos::prelude::*;
use num_traits::Bounded;
use std::ops::{Add, Deref, Sub};
use std::str::FromStr;
use thaw_utils::{
    class_list, mount_style, with, BoxOneCallback, Model, OptionalProp,
};

/// SpinButton are used to allow numerical input bounded between minimum and maximum values
/// with buttons to increment and decrement the input value.
/// 
/// Note: SpinButton is a generic component, so the type must be specified. Example: `<SpinButton<i32> step_page=1/>`.
/// [Related issue](https://github.com/leptos-rs/leptos/issues/3200)
#[component]
pub fn SpinButton<T>(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// The rules to validate Field.
    #[prop(optional, into)]
    rules: Vec<SpinButtonRule<T>>,
    /// Current value of the control.
    #[prop(optional, into)]
    value: Model<T>,
    /// Large difference between two values. This should be greater
    /// than step and is used when users hit the Page Up or Page Down keys.
    #[prop(into)]
    step_page: Signal<T>,
    /// The minimum number that the input value can take.
    #[prop(default = T::min_value().into(), into)]
    min: Signal<T>,
    /// The maximum number that the input value can take.
    #[prop(default = T::max_value().into(), into)]
    max: Signal<T>,
    /// Placeholder of input number.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Whether the input is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Modifies the user input before assigning it to the value.
    #[prop(optional, into)]
    parser: OptionalProp<BoxOneCallback<String, Option<T>>>,
    /// Formats the value to be shown to the user.
    #[prop(optional, into)]
    format: OptionalProp<BoxOneCallback<T, String>>,
) -> impl IntoView
where
    T: Send + Sync,
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Bounded,
    T: Default + Clone + FromStr + ToString + 'static,
{
    mount_style("spin-button", include_str!("./spin-button.css"));
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);
    let initialization_value = value.get_untracked().to_string();

    let update_value = move |new_value| {
        if with!(|value| value == &new_value) {
            return;
        }
        let min = min.get_untracked();
        let max = max.get_untracked();

        if new_value < min {
            value.set(min);
        } else if new_value > max {
            value.set(max);
        } else {
            value.set(new_value);
        }
        validate.run(Some(SpinButtonRuleTrigger::Change));
    };

    let increment_disabled = Memo::new(move |_| disabled.get() || value.get() >= max.get());
    let decrement_disabled = Memo::new(move |_| disabled.get() || value.get() <= min.get());

    let on_change = move |e| {
        let target_value = event_target_value(&e);
        let v = if let Some(parser) = parser.as_ref() {
            parser(target_value)
        } else {
            target_value.parse::<T>().ok()
        };

        if let Some(value) = v {
            update_value(value);
        } else {
            value.update(|_| {});
        }
    };

    view! {
        <span class=class_list![
            "thaw-spin-button",
            ("thaw-spin-button--disabled", move || disabled.get()),
            class
        ]>
            <input
                autocomplete="off"
                role="spinbutton"
                aria-valuenow=move || value.get().to_string()
                type="text"
                disabled=move || disabled.get()
                placeholder=move || placeholder.get()
                value=initialization_value
                prop:value=move || {
                    let value = value.get();
                    if let Some(format) = format.as_ref() {
                        format(value)
                    } else {
                        value.to_string()
                    }
                }
                class="thaw-spin-button__input"
                id=id
                name=name
                on:change=on_change
            />
            <button
                tabindex="-1"
                aria-label="Increment value"
                type="button"
                class="thaw-spin-button__increment-button"
                class=(
                    "thaw-spin-button__increment-button--disabled",
                    move || increment_disabled.get(),
                )
                disabled=move || disabled.get()
                on:click=move |_| {
                    if !increment_disabled.get_untracked() {
                        update_value(value.get_untracked() + step_page.get_untracked());
                    }
                }
            >
                <svg
                    fill="currentColor"
                    aria-hidden="true"
                    width="16"
                    height="16"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M3.15 10.35c.2.2.5.2.7 0L8 6.21l4.15 4.14a.5.5 0 0 0 .7-.7l-4.5-4.5a.5.5 0 0 0-.7 0l-4.5 4.5a.5.5 0 0 0 0 .7Z"
                        fill="currentColor"
                    ></path>
                </svg>
            </button>
            <button
                tabindex="-1"
                aria-label="Decrement value"
                type="button"
                class="thaw-spin-button__decrement-button"
                disabled=move || disabled.get()
                class=(
                    "thaw-spin-button__decrement-button--disabled",
                    move || decrement_disabled.get(),
                )
                on:click=move |_| {
                    if !decrement_disabled.get_untracked() {
                        update_value(value.get_untracked() - step_page.get_untracked());
                    }
                }
            >
                <svg
                    fill="currentColor"
                    aria-hidden="true"
                    width="16"
                    height="16"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M3.15 5.65c.2-.2.5-.2.7 0L8 9.79l4.15-4.14a.5.5 0 0 1 .7.7l-4.5 4.5a.5.5 0 0 1-.7 0l-4.5-4.5a.5.5 0 0 1 0-.7Z"
                        fill="currentColor"
                    ></path>
                </svg>
            </button>
        </span>
    }
}

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
