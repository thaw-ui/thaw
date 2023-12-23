use crate::{
    theme::use_theme,
    utils::{class_list::class_list, mount_style},
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

    view! {
        <div
            class=class_list![
                "thaw-radio", ("thaw-radio--checked", move || value.get()), move || class.get()
            ]

            style=move || css_vars.get()
            on:click=move |_| value.set(!value.get_untracked())
        >
            <input class="thaw-radio__input" type="radio" prop:value=move || value.get()/>
            <div class="thaw-radio__dot"></div>
            <div class="thaw-radio__label">{children()}</div>
        </div>
    }
}
