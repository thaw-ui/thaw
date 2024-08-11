mod checkbox_group;

pub use checkbox_group::CheckboxGroup;

use checkbox_group::CheckboxGroupInjection;
use leptos::{html, prelude::*};
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn Checkbox(
    /// The controlled value for the checkbox.
    #[prop(optional, into)] checked: Model<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The value of the checkbox to be used in a checkbox group.
    #[prop(optional, into)] value: Option<String>,
    /// The Checkbox's label.
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    mount_style("checkbox", include_str!("./checkbox.css"));

    let id = uuid::Uuid::new_v4().to_string();
    let input_ref = NodeRef::<html::Input>::new();
    let group = CheckboxGroupInjection::use_context();
    let item_value = StoredValue::new(value);

    let group_checked = Memo::new(move |_| {
        let Some(group) = group.as_ref() else {
            return None;
        };
        group.0.with(|group_value| {
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
                group.as_ref().unwrap().0.update(move |group_value| {
                    group_value.insert(item_value.get_value().unwrap());
                });
            } else {
                group.as_ref().unwrap().0.update(move |group_value| {
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

    view! {
        <span
            class=class_list![
                "thaw-checkbox", ("thaw-checkbox--checked", checked),
                class
            ]
        >
            <input
                class="thaw-checkbox__input"
                type="checkbox"
                id=id.clone()
                checked=checked
                node_ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" class="thaw-checkbox__indicator">
                {
                    move || if checked() {
                        view! {
                            <svg fill="currentColor" aria-hidden="true" width="12" height="12" viewBox="0 0 12 12" style="display: inline;line-height: 0">
                                <path d="M9.76 3.2c.3.29.32.76.04 1.06l-4.25 4.5a.75.75 0 0 1-1.08.02L2.22 6.53a.75.75 0 0 1 1.06-1.06l1.7 1.7L8.7 3.24a.75.75 0 0 1 1.06-.04Z" fill="currentColor"></path>
                            </svg>
                        }.into()
                    } else {
                        None
                    }
                }
            </div>
            {
                move || if let Some(label) = label.get() {
                    view! {
                        <label class="thaw-checkbox__label" for=id.clone()>{label}</label>
                    }.into()
                } else {
                    None
                }
            }
        </span>
    }
}
