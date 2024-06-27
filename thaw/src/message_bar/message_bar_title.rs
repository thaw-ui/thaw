use leptos::*;

#[component]
pub fn MessageBarTitle(children: Children) -> impl IntoView {
    view! {
        <span class="thaw-message-bar-title">
            {children()}
        </span>
    }
}
