mod collapse_item;

pub use collapse_item::CollapseItem;

use crate::utils::{class_list::class_list, mount_style};
use leptos::*;

#[component]
pub fn Collapse(
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("collapser", include_str!("./collapse.css"));

    view! { <div class=class_list!["thaw-collapse", move || class.get()]>{children()}</div> }
}
