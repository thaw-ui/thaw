use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn SkeletonItem(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    mount_style("skeleton-item", include_str!("./skeleton-item.css"));

    view! { <div class=class_list!["thaw-skeleton-item", class]></div> }
}
