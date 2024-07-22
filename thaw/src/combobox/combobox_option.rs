use crate::ComboboxInjection;
use leptos::prelude::*;
use thaw_components::{Fallback, If, OptionComp, Then};
use thaw_utils::class_list;

#[component]
pub fn ComboboxOption(
    #[prop(optional, into)] value: Option<String>,
    #[prop(into)] text: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let combobox = ComboboxInjection::expect_context();
    let value = StoredValue::new(value.unwrap_or_else(|| text.clone()));
    let text = StoredValue::new(text);
    let is_selected = Memo::new(move |_| value.with_value(|value| combobox.is_selected(&value)));
    let id = uuid::Uuid::new_v4().to_string();

    let on_click = move |_| {
        text.with_value(|text| {
            value.with_value(|value| {
                combobox.select_option(value, text);
            });
        });
    };

    {
        combobox.insert_option(id.clone(), (value.get_value(), text.get_value()));
        let id = id.clone();
        on_cleanup(move || {
            combobox.remove_option(&id);
        });
    }

    view! {
        <div
            role="option"
            aria-selected=move || if is_selected.get() { "true" } else { "false" }
            id=id
            class=class_list![
                "thaw-combobox-option",
                ("thaw-combobox-option--selected", move || is_selected.get())
            ]
            on:click=on_click
        >
            {
                if combobox.multiselect {
                    view! {
                        <span aria-hidden="true" class="thaw-combobox-option__check-icon--multiselect">
                            <If cond=is_selected>
                                <Then slot>
                                    <svg fill="currentColor" aria-hidden="true" width="12" height="12" viewBox="0 0 12 12">
                                        <path d="M9.76 3.2c.3.29.32.76.04 1.06l-4.25 4.5a.75.75 0 0 1-1.08.02L2.22 6.53a.75.75 0 0 1 1.06-1.06l1.7 1.7L8.7 3.24a.75.75 0 0 1 1.06-.04Z" fill="currentColor"></path>
                                    </svg>
                                </Then>
                            </If>
                        </span>
                    }.into_any()
                } else {
                    view! {
                        <span aria-hidden="true" class="thaw-combobox-option__check-icon">
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="M7.03 13.9 3.56 10a.75.75 0 0 0-1.12 1l4 4.5c.29.32.79.34 1.09.03l10.5-10.5a.75.75 0 0 0-1.06-1.06l-9.94 9.94Z" fill="currentColor"></path>
                            </svg>
                        </span>
                    }.into_any()
                }
            }
            <OptionComp value=children let:children>
                <Fallback slot>
                    {text.get_value()}
                </Fallback>
                {children()}
            </OptionComp>
        </div>
    }
}
