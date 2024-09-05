use leptos::prelude::*;
use thaw_utils::{class_list, mount_style, ArcOneCallback};

#[component]
pub fn TagGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tag-group", include_str!("./tag-group.css"));

    view! { <div class=class_list!["thaw-tag-group", class]>{children()}</div> }
}
