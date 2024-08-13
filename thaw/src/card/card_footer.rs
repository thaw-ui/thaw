use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn CardFooter(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("card-footer", include_str!("./card-footer.css"));
    view! { <div class=class_list!["thaw-card-footer", class]>{children()}</div> }
}
