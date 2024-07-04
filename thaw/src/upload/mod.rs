mod upload_dragger;

pub use upload_dragger::UploadDragger;
pub use web_sys::FileList;

use leptos::{ev, html, prelude::*};
use thaw_utils::{add_event_listener, mount_style};

#[component]
pub fn Upload(
    #[prop(optional, into)] accept: MaybeSignal<String>,
    #[prop(optional, into)] multiple: MaybeSignal<bool>,
    #[prop(optional, into)] custom_request: Option<Callback<FileList, ()>>,
    children: Children,
) -> impl IntoView {
    mount_style("upload", include_str!("./upload.css"));

    let input_ref = NodeRef::<html::Input>::new();
    let trigger_ref = NodeRef::<html::Div>::new();

    trigger_ref.on_load(move |trigger_ref| {
        let handle = add_event_listener(trigger_ref.into_any(), ev::click, move |_| {
            if let Some(input_ref) = input_ref.get_untracked() {
                input_ref.click();
            }
        });
        on_cleanup(move || {
            handle.remove();
        });
    });

    let on_file_addition = move |files: FileList| {
        if let Some(custom_request) = custom_request {
            custom_request.call(files);
        }
    };

    let on_change = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
            if let Some(files) = input_ref.files() {
                on_file_addition(files);
            }
            input_ref.set_value("");
        }
    };

    let is_trigger_dragover = RwSignal::new(false);
    let on_trigger_drop = move |event: ev::DragEvent| {
        event.prevent_default();
        if let Some(data) = event.data_transfer() {
            if let Some(files) = data.files() {
                on_file_addition(files);
            }
        }
        is_trigger_dragover.set(false);
    };
    let on_trigger_dragover = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dragover.set(true);
    };
    let on_trigger_dragenter = move |event: ev::DragEvent| {
        event.prevent_default();
    };
    let on_trigger_dragleave = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dragover.set(false);
    };

    view! {
        <div
            class="thaw-upload"
            class=("thaw-upload--drag-over", move || is_trigger_dragover.get())
        >
            <input
                class="thaw-upload__input"
                node_ref=input_ref
                type="file"
                accept=move || accept.get()
                multiple=move || multiple.get()
                on:change=on_change
            />
            <div
                class="thaw-upload__trigger"
                node_ref=trigger_ref
                on:drop=on_trigger_drop
                on:dragover=on_trigger_dragover
                on:dragenter=on_trigger_dragenter
                on:dragleave=on_trigger_dragleave
            >
                {children()}
            </div>
        </div>
    }
}
