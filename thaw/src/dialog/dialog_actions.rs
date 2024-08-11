use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn DialogActions(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-dialog-actions", class]>
            {children()}
        </div>
    }
}
