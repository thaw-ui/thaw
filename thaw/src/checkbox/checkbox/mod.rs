mod types;

pub use types::*;

use super::checkbox_group::CheckboxGroupInjection;
use crate::{Checkmark12FilledIcon, Checkmark16FilledIcon};
use leptos::{either::Either, html, prelude::*};
use thaw_utils::{class_list, Model};

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/checkbox/checkbox/checkbox.css");

#[component]
pub fn Checkbox(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The controlled value for the checkbox.
    #[prop(optional, into)]
    checked: Model<bool>,
    /// The value of the checkbox to be used in a checkbox group.us
    #[prop(optional, into)]
    value: Option<String>,
    /// The Checkbox's label.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// The size of the checkbox indicator.
    #[prop(optional, into)]
    size: Signal<CheckboxSize>,
    /// Whether the input is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("checkbox", include_str!("./checkbox.css"));

    let id = uuid::Uuid::new_v4().to_string();
    let input_ref = NodeRef::<html::Input>::new();
    let group = CheckboxGroupInjection::use_context();
    let item_value = StoredValue::new(value);

    let group_checked = Memo::new(move |_| {
        let Some(group) = group.as_ref() else {
            return None;
        };
        group.value.with(|group_value| {
            item_value.with_value(|value| {
                let Some(value) = value else {
                    return None;
                };
                Some(group_value.contains(value))
            })
        })
    });

    let on_change = move |_| {
        let input = input_ref.get_untracked().unwrap();
        if group_checked.get_untracked().is_some() {
            if input.checked() {
                group.as_ref().unwrap().value.update(move |group_value| {
                    group_value.insert(item_value.get_value().unwrap());
                });
            } else {
                group.as_ref().unwrap().value.update(move |group_value| {
                    item_value.with_value(|value| {
                        group_value.remove(value.as_ref().unwrap());
                    });
                });
            }
        } else {
            checked.set(input.checked())
        }
    };

    let checked = move || group_checked.get().unwrap_or_else(|| checked.get());
    let disabled = move || disabled.get();

    view! {
        <span class=class_list![
            "thaw-checkbox",
            ("thaw-checkbox--checked", checked),
            ("thaw-checkbox--disabled", disabled),
            move || format!("thaw-checkbox--{}", size.get().as_str()),
            class
        ]>
            <input
                class="thaw-checkbox__input"
                type="checkbox"
                id=id.clone()
                name=move || group.map(|g| g.name.get()).flatten()
                value=item_value.get_value()
                checked=checked
                node_ref=input_ref
                disabled=disabled
                on:change=on_change
            />
            <div aria-hidden="true" class="thaw-checkbox__indicator">
                {move || {
                    if checked() {
                        match size.get() {
                            CheckboxSize::Medium => {
                                Either::Left(
                                    view! {
                                        <Checkmark12FilledIcon attr:style="display: inline;line-height: 0" />
                                    },
                                )
                            }
                            CheckboxSize::Large => {
                                Either::Right(
                                    view! {
                                        <Checkmark16FilledIcon attr:style="display: inline;line-height: 0" />
                                    },
                                )
                            }
                        }
                            .into()
                    } else {
                        None
                    }
                }}
            </div>
            {move || {
                if let Some(label) = label.get() {
                    view! {
                        <label class="thaw-checkbox__label" for=id.clone()>
                            {label}
                        </label>
                    }
                        .into()
                } else {
                    None
                }
            }}
        </span>
    }
}
