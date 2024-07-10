use leptos::prelude::*;

#[component]
pub fn DialogTitle(children: Children) -> impl IntoView {
    view! {
        <h2 class="thaw-dialog-title">
            {children()}
        </h2>
    }
}
