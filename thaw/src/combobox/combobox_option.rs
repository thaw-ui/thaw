use crate::ComboboxInjection;
use leptos::*;
use thaw_components::{If, Then};
use thaw_utils::class_list;

#[component]
pub fn ComboboxOption(#[prop(into)] key: String, children: Children) -> impl IntoView {
    let combobox = ComboboxInjection::use_();
    let key = StoredValue::new(key);

    view! {
        <div
            role="option"
            aria-selected="true"
            class=class_list![
                "thaw-combobox-option",
                ("thaw-combobox-option--selected", move || key.with_value(|key| combobox.is_selected(&key)))
            ]
            on:click=move |_| {
                key.with_value(|key| combobox.on_option_select(key));
            }
        >
            {
                if combobox.multiselect {
                    view! {
                        <span aria-hidden="true" class="thaw-combobox-option__check-icon--multiselect">
                            <If cond=Signal::derive(move || key.with_value(|key| combobox.is_selected(&key)))>
                                <Then slot>
                                    <svg fill="currentColor" aria-hidden="true" width="12" height="12" viewBox="0 0 12 12">
                                        <path d="M9.76 3.2c.3.29.32.76.04 1.06l-4.25 4.5a.75.75 0 0 1-1.08.02L2.22 6.53a.75.75 0 0 1 1.06-1.06l1.7 1.7L8.7 3.24a.75.75 0 0 1 1.06-.04Z" fill="currentColor"></path>
                                    </svg>
                                </Then>
                            </If>
                        </span>
                    }
                } else {
                    view! {
                        <span aria-hidden="true" class="thaw-combobox-option__check-icon">
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="M7.03 13.9 3.56 10a.75.75 0 0 0-1.12 1l4 4.5c.29.32.79.34 1.09.03l10.5-10.5a.75.75 0 0 0-1.06-1.06l-9.94 9.94Z" fill="currentColor"></path>
                            </svg>
                        </span>
                    }
                }
            }
            {children()}
        </div>
    }
}
