use leptos::prelude::*;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/card/card-footer.css");

#[component]
pub fn CardFooter(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("card-footer", include_str!("./card-footer.css"));

    view! { <div class=class_list!["thaw-card-footer", class]>{children()}</div> }
}
