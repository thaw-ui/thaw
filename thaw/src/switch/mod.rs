use leptos::*;
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[component]
pub fn Switch(
    #[prop(optional, into)] checked: Model<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    mount_style("switch", include_str!("./switch.css"));

    let id = uuid::Uuid::new_v4().to_string();
    let input_ref = NodeRef::<html::Input>::new();
    let on_change = move |_| {
        let input = input_ref.get_untracked().unwrap();
        checked.set(input.checked());
    };

    view! {
        <div
            class=class_list![
                "thaw-switch", class.map(| c | move ||
                c.get())
            ]
        >
            <input
                class="thaw-switch__input"
                role="switch"
                type="checkbox"
                id=id.clone()
                checked=checked.get_untracked()
                ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" class="thaw-switch__indicator thaw-switch__button">
                <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                    <path d="M10 2a8 8 0 1 0 0 16 8 8 0 0 0 0-16Z" fill="currentColor"></path>
                </svg>
            </div>
            {
                move || if let Some(label) = label.get() {
                    view! {
                        <label class="thaw-switch__label" for=id.clone()>{label}</label>
                    }.into()
                } else {
                    None
                }
            }
        </div>
    }
}
