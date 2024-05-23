mod checkbox_group;
mod checkbox_item;

pub use checkbox_group::CheckboxGroup;
pub use checkbox_item::CheckboxItem;

use leptos::*;
use thaw_components::*;
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[component]
pub fn Checkbox(
    #[prop(optional, into)] checked: Model<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("checkbox", include_str!("./checkbox.css"));

    let id = uuid::Uuid::new_v4().to_string();
    let input_ref = NodeRef::<html::Input>::new();

    let on_change = move |_| {
        let input = input_ref.get_untracked().unwrap();
        checked.set(input.checked())
    };

    view! {
        <span
            class=class_list![
                "thaw-checkbox", ("thaw-checkbox--checked", move || checked.get()), class.map(| c |
                move || c.get())
            ]
        >
            <input
                class="thaw-checkbox__input"
                type="checkbox"
                id=id.clone()
                checked=checked.get_untracked()
                ref=input_ref
                on:change=on_change
            />
            <div aria-hidden="true" class="thaw-checkbox__indicator">
                {
                    move || if checked.get() {
                        view! {
                            <svg fill="currentColor" aria-hidden="true" width="12" height="12" viewBox="0 0 12 12" style="display: inline;line-height: 0">
                                <path d="M9.76 3.2c.3.29.32.76.04 1.06l-4.25 4.5a.75.75 0 0 1-1.08.02L2.22 6.53a.75.75 0 0 1 1.06-1.06l1.7 1.7L8.7 3.24a.75.75 0 0 1 1.06-.04Z" fill="currentColor"></path>
                            </svg>
                        }.into()
                    } else {
                        None
                    }
                }
            </div>
            <OptionComp value=children let:children>
                <label class="thaw-checkbox__label" for=id>{children()}</label>
            </OptionComp>
        </span>
    }
}
