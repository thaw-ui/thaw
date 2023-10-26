mod upload_dragger;

use crate::mount_style;
use leptos::*;
pub use upload_dragger::UploadDragger;
pub use web_sys::FileList;

#[component]
pub fn Upload(
    #[prop(optional, into)] accept: MaybeSignal<String>,
    #[prop(optional, into)] multiple: MaybeSignal<bool>,
    #[prop(optional, into)] custom_request: Option<Callback<FileList, ()>>,
    children: Children,
) -> impl IntoView {
    mount_style("upload", include_str!("./upload.css"));

    let on_file_addition = move |files: FileList| {
        if let Some(custom_request) = custom_request {
            custom_request.call(files);
        }
    };
    let input_ref = create_node_ref::<html::Input>();
    let on_change = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
            if let Some(files) = input_ref.files() {
                on_file_addition(files);
            }
        }
    };
    let on_click = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
            input_ref.click();
        }
    };
    view! {
        <div class="melt-upload">
            <input
                class="melt-upload__input"
                ref=input_ref
                type="file"
                accept=move || accept.get()
                multiple=move || multiple.get()
                on:change=on_change
            />
            <div class="melt-upload__trigger" on:click=on_click>
                {children()}
            </div>
        </div>
    }
}
