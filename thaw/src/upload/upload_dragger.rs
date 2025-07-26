use leptos::prelude::*;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/upload/upload-dragger.css");

#[component]
pub fn UploadDragger(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("upload-dragger", include_str!("./upload-dragger.css"));

    view! { <div class=class_list!["thaw-upload-dragger", class]>{children()}</div> }
}
