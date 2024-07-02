use super::AutoCompleteInjection;
use leptos::*;

#[component]
pub fn AutoCompleteOption(key: String, children: Children) -> impl IntoView {
    let value = key.clone();
    let auto_complete = AutoCompleteInjection::use_();
    let is_selected = Memo::new(move |_| auto_complete.is_selected(&key));

    view! {
        <div
            class="thaw-auto-complete-option"
            role="option"
            aria-selected=move || if is_selected.get() { "true" } else { "false" }
            on:click=move |_| auto_complete.select_value(value.clone())
        >
            {children()}
        </div>
    }
}
