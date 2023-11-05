use leptos::*;

use crate::mount_style;

#[component]
pub fn Divider() -> impl IntoView {
    mount_style("divider", include_str!("./divider.css"));
    view! {
        <div class="thaw-divider">
            <div class="thaw-divider__line"></div>
        </div>
    }
}
