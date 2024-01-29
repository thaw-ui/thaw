mod checkbox_group;
mod checkbox_item;

use crate::{
    components::*,
    icon::*,
    theme::use_theme,
    utils::{class_list::class_list, mount_style, Model},
    Theme,
};
pub use checkbox_group::CheckboxGroup;
pub use checkbox_item::CheckboxItem;
use leptos::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] value: Model<bool>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("checkbox", include_str!("./checkbox.css"));

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color-checked: {};",
                theme.common.color_primary
            ));
        });
        css_vars
    });

    view! {
        <div
            class=class_list![
                "thaw-checkbox", ("thaw-checkbox--checked", move || value.get()), move || class
                .get()
            ]

            style=move || css_vars.get()
            on:click=move |_| value.set(!value.get_untracked())
        >
            <input class="thaw-checkbox__input" type="checkbox"/>
            <div class="thaw-checkbox__dot">
                <If cond=value.signal()>
                    <Then slot>
                        <Icon icon=icondata::AiCheckOutlined style="color: white"/>
                    </Then>
                </If>
            </div>
            <div class="thaw-checkbox__label">{children()}</div>
        </div>
    }
}
