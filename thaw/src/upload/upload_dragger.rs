use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn UploadDragger(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("upload-dragger", include_str!("./upload-dragger.css"));

    view! {
        <div class=class_list!["thaw-upload-dragger", class]>
            {children()}
        </div>
    }
}
