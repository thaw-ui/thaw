use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn CardPreview(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-card-preview", class] style="position: relative">
            {children()}
        </div>
    }
}
