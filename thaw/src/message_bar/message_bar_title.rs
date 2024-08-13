use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn MessageBarTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <span class=class_list!["thaw-message-bar-title", class]>{children()}</span> }
}
