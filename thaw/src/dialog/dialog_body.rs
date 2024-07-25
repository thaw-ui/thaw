use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn DialogBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-dialog-body", class]>
            {children()}
        </div>
    }
}
