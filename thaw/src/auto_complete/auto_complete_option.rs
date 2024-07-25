use super::AutoCompleteInjection;
use crate::combobox::listbox::ListboxInjection;
use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn AutoCompleteOption(
    #[prop(optional, into)] class: MaybeProp<String>,
    value: String,
    children: Children,
) -> impl IntoView {
    let auto_complete = AutoCompleteInjection::expect_context();
    let listbox = ListboxInjection::expect_context();
    let is_selected = Memo::new({
        let value = value.clone();
        move |_| auto_complete.is_selected(&value)
    });
    let id = uuid::Uuid::new_v4().to_string();
    {
        auto_complete.insert_option(id.clone(), value.clone());
        let id = id.clone();
        listbox.trigger();
        on_cleanup(move || {
            auto_complete.remove_option(&id);
            listbox.trigger();
        });
    }

    view! {
        <div
            class=class_list!["thaw-auto-complete-option", class]
            role="option"
            id=id
            aria-selected=move || if is_selected.get() { "true" } else { "false" }
            on:click=move |_| auto_complete.select_option(value.clone())
        >
            {children()}
        </div>
    }
}
