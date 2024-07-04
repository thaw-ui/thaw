use leptos::prelude::*;
use thaw_utils::mount_style;

#[component]
pub fn SkeletonItem() -> impl IntoView {
    mount_style("skeleton-item", include_str!("./skeleton-item.css"));

    view! {
        <div class="thaw-skeleton-item">
        </div>
    }
}
