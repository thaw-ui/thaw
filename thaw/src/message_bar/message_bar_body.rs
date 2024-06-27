use leptos::*;

#[component]
pub fn MessageBarBody(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-message-bar-body">
            {children()}
        </div>
    }
}
