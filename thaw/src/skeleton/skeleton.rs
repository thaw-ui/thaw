use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn Skeleton(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="progressbar"
            aria-busy="true"
            class=class_list!["thaw-skeleton", class]
        >
            {children()}
        </div>
    }
}
