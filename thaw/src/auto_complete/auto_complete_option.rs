use super::AutoCompleteInjection;
use leptos::*;

#[component]
pub fn AutoCompleteOption2(key: String, children: Children) -> impl IntoView {
    let value = key.clone();
    let auto_complete = RwSignal::new(AutoCompleteInjection::use_());
    let is_selected = Memo::new(move |_| auto_complete.get().is_selected(&key));

    view! {
        <div
            class="thaw-auto-complete-option"
            role="option"
            aria-selected=move || if is_selected.get() { "true" } else { "false" }
            on:click=move |_| auto_complete.get().select_value(value.clone())
        >
            {children()}
        </div>
    }
}
