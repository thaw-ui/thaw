use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn DialogContent(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <h2 class=class_list!["thaw-dialog-content", class]>{children()}</h2> }
}
