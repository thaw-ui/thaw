use crate::{
    theme::use_theme,
    utils::{maybe_rw_signal::MaybeRwSignal, mount_style::mount_style},
    Theme,
};
use leptos::*;

#[component]
pub fn Radio(
    #[prop(optional, into)] value: MaybeRwSignal<bool>,
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

    view! {
        <div
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
