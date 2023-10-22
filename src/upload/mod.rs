use leptos::*;
use crate::mount_style;
// TODO

#[component]
pub fn Upload(
    #[prop(optional, into)] accept: MaybeSignal<String>,
    #[prop(optional, into)] multiple: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("upload", include_str!("./upload.css"));
    let on_file_addition = move || {

    };
    let input_ref = create_node_ref::<html::Input>();
    let on_change = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
        }
    };
    let on_click= move |_| {
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
