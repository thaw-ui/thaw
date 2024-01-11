mod checkbox_group;
mod checkbox_item;

use crate::{
    components::*,
    icon::*,
    theme::use_theme,
    utils::{class_list::class_list, mount_style},
    Theme,
};
pub use checkbox_group::CheckboxGroup;
pub use checkbox_item::CheckboxItem;
use icondata::AiIcon;
use leptos::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] value: RwSignal<bool>,
    #[prop(optional, into)] initial: RwSignal<bool>,
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

    let is_checked = Signal::derive(move || value.get() || initial.get());

    view! {
        <div
            class=class_list![
                "thaw-checkbox", ("thaw-checkbox--checked", move || value.get() || initial.get()), move || class
                .get()
            ]

            style=move || css_vars.get()
            on:click=move |_| {
                if initial.get() {
                    value.set(false);
                } else {
                    value.set(!value.get_untracked())
                }
                initial.set(false); 
            }
        >
            <input class="thaw-checkbox__input" type="checkbox"/>
            <div class="thaw-checkbox__dot">
                <If cond=is_checked>
                    <Then slot>
                        <Icon icon=Icon::from(AiIcon::AiCheckOutlined) style="color: white"/>
                    </Then>
                </If>
            </div>
            <div class="thaw-checkbox__label">{children()}</div>
        </div>
    }
}
