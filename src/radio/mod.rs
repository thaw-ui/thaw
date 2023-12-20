#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::{
    theme::use_theme,
    utils::{mount_style, ssr_class},
    Theme,
};
use leptos::*;

#[component]
pub fn Radio(
    #[prop(optional, into)] value: RwSignal<bool>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("radio", include_str!("./radio.css"));

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let bg_color = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-background-color-checked: {bg_color};"));
        });

        css_vars
    });

    let ssr_class = ssr_class(&class);
    view! {
        <div
            class=ssr_class
            use:dyn_classes=class
            class="thaw-radio"
            class=("thaw-radio--checked", move || value.get())
            style=move || css_vars.get()
            on:click=move |_| value.set(!value.get_untracked())
        >
            <input class="thaw-radio__input" type="radio" prop:value=move || value.get()/>
            <div class="thaw-radio__dot"></div>
            <div class="thaw-radio__label">{children()}</div>
        </div>
    }
}
