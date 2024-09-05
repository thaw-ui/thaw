use leptos::prelude::*;

#[component]
pub fn TagPickerGroup(children: Children) -> impl IntoView {
    view! {
        <div role="listbox" class="thaw-tag-picker-group">
            {children()}
        </div>
    }
}
