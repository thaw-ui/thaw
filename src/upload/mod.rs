use crate::{mount_style, utils::AsyncCallback};
use leptos::*;
use web_sys::FileList;

#[component]
pub fn Upload(
    #[prop(optional, into)] accept: MaybeSignal<String>,
    #[prop(optional, into)] multiple: MaybeSignal<bool>,
    #[prop(optional, into)] on_before_upload: Option<AsyncCallback<FileList, bool>>,
    children: Children,
) -> impl IntoView {
    mount_style("upload", include_str!("./upload.css"));

    let on_file_addition = move |files: FileList| {
        spawn_local(async move {
            if let Some(on_before_upload) = on_before_upload {
                let is_allow = on_before_upload.call(files).await;
                if is_allow {
                    //TODO submit
                }
            }
        });
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
