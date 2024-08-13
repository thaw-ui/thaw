use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn MessageBarBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <div class=class_list!["thaw-message-bar-body", class]>{children()}</div> }
}
