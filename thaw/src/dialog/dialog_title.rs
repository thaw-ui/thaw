use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn DialogTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <h2 class=class_list!["thaw-dialog-title", class]>
            {children()}
        </h2>
    }
}
