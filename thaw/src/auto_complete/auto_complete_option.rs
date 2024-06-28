use super::AutoCompleteInjection;
use leptos::*;

#[component]
pub fn AutoCompleteOption2(key: String, children: Children) -> impl IntoView {
    let auto_complete = AutoCompleteInjection::use_();
    let is_selected = Memo::new(move |_| auto_complete.is_selected(&key));

    view! {
        <div
            class="thaw-auto-complete-option"
            role="option"
            aria-selected=move || if is_selected.get() { "true" } else { "false" }
        >
            {children()}
        </div>
    }
}
