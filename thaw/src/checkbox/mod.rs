mod checkbox_group;
mod checkbox_item;

pub use checkbox_group::CheckboxGroup;
pub use checkbox_item::CheckboxItem;

use crate::icon::*;
use leptos::*;
use thaw_components::*;
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[component]
pub fn Checkbox(
    #[prop(optional, into)] value: Model<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("checkbox", include_str!("./checkbox.css"));

    view! {
        <div
            class=class_list![
                "thaw-checkbox", ("thaw-checkbox--checked", move || value.get()), class.map(| c |
                move || c.get())
            ]

            on:click=move |_| value.set(!value.get_untracked())
        >
            <input class="thaw-checkbox__input" type="checkbox"/>
            <div class="thaw-checkbox__indicator">
                <If cond=value.signal()>
                    <Then slot>
                        <Icon icon=icondata_ai::AiCheckOutlined style="color: white"/>
                    </Then>
                </If>
            </div>
            <OptionComp value=children let:children>
                <label class="thaw-checkbox__label">{children()}</label>
            </OptionComp>
        </div>
    }
}
