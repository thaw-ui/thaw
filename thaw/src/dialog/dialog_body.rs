use leptos::prelude::*;

#[component]
pub fn DialogBody(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-dialog-body">
            {children()}
        </div>
    }
}