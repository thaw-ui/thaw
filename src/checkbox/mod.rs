mod checkbox_group;
mod checkbox_item;

#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::{
    components::*,
    icon::*,
    theme::use_theme,
    utils::{mount_style, ssr_class},
    Theme,
};
pub use checkbox_group::CheckboxGroup;
pub use checkbox_item::CheckboxItem;
use icondata::AiIcon;
use leptos::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] value: RwSignal<bool>,
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

    let ssr_class = ssr_class(&class);
    view! {
        <div
            class=ssr_class
            use:dyn_classes=class
            class:thaw-checkbox=true
            class=("thaw-checkbox--checked", move || value.get())
            style=move || css_vars.get()
            on:click=move |_| value.set(!value.get_untracked())
        >
            <input class="thaw-checkbox__input" type="checkbox"/>
            <div class="thaw-checkbox__dot">
                <If cond=value>
                    <Then slot>
                        <Icon icon=Icon::from(AiIcon::AiCheckOutlined) style="color: white"/>
                    </Then>
                </If>
            </div>
            <div class="thaw-checkbox__label">{children()}</div>
        </div>
    }
}
