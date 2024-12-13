use super::{SelectRule, SelectSize};

use crate::{icon::ChevronDownRegularIcon, FieldInjection, Rule, SelectRuleTrigger};
use leptos::{html, prelude::*};
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
    /// Matches the Input sizes.
    #[prop(optional, into)]
    size: Signal<SelectSize>,
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
            move || format!("thaw-select--{}", size.get().as_str()),
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
