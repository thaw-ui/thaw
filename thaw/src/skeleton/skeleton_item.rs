use leptos::prelude::*;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/skeleton/skeleton-item.css");

#[component]
pub fn SkeletonItem(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("skeleton-item", include_str!("./skeleton-item.css"));

    view! { <div class=class_list!["thaw-skeleton-item", class]></div> }
}
