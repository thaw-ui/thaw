use leptos::*;
use thaw_utils::mount_style;

#[component]
pub fn UploadDragger(children: Children) -> impl IntoView {
    mount_style("upload-dragger", include_str!("./upload-dragger.css"));

    view! {
        <div class="thaw-upload-dragger">
            {children()}
        </div>
    }
}
