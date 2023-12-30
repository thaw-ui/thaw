use crate::utils::mount_style;
use leptos::*;

#[component]
pub fn Code(children: Children) -> impl IntoView {
    mount_style("code", include_str!("./code.css"));
    view! { <code class="thaw-code">{children()}</code> }
}
