use leptos::prelude::*;
use thaw_utils::{class_list, mount_style, ArcOneCallback};

#[component]
pub fn InteractionTag(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("interaction-tag", include_str!("./interaction-tag.css"));

    view! { <div class=class_list!["thaw-interaction-tag", class]>{children()}</div> }
}

#[component]
pub fn InteractionTagPrimary(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <button class=class_list!["thaw-interaction-tag-primary", class]>
            <span class="thaw-interaction-tag-primary__primary-text">{children()}</span>
        </button>
    }
}
