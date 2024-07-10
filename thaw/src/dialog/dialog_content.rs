use leptos::prelude::*;

#[component]
pub fn DialogContent(children: Children) -> impl IntoView {
    view! {
        <h2 class="thaw-dialog-content">
            {children()}
        </h2>
    }
}
