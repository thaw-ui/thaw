mod upload_dragger;

pub use upload_dragger::UploadDragger;
pub use web_sys::FileList;

use leptos::{ev, html, prelude::*};
use thaw_utils::{add_event_listener, class_list, mount_style, ArcOneCallback};

#[component]
pub fn Upload(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// The accept type of upload.
    #[prop(optional, into)]
    accept: MaybeSignal<String>,
    /// Allow multiple files to be selected.
    #[prop(optional, into)]
    multiple: MaybeSignal<bool>,
    /// Customize upload request.
    #[prop(optional, into)]
    custom_request: Option<ArcOneCallback<FileList>>,
    children: Children,
) -> impl IntoView {
    mount_style("upload", include_str!("./upload.css"));

    let input_ref = NodeRef::<html::Input>::new();
    let trigger_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {
        let Some(trigger_el) = trigger_ref.get() else {
            return;
        };
        let handle = add_event_listener(trigger_el.into(), ev::click, move |_| {
            if let Some(input_ref) = input_ref.get_untracked() {
                input_ref.click();
            }
        });
        on_cleanup(move || {
            handle.remove();
        });
    });

    let on_file_addition = move |files: FileList| {
        if let Some(custom_request) = custom_request.as_ref() {
            custom_request(files);
        }
    };

    let on_change = {
        let on_file_addition = on_file_addition.clone();
        move |_| {
            if let Some(input_ref) = input_ref.get_untracked() {
                if let Some(files) = input_ref.files() {
                    on_file_addition(files);
                }
                input_ref.set_value("");
            }
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
        <div class=class_list![
            "thaw-upload",
                ("thaw-upload--drag-over", move || is_trigger_dragover.get()),
                class
        ]>
            <input
                class="thaw-upload__input"
                id=move || id.get()
                name=move || name.get()
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
