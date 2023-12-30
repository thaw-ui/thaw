mod theme;

use crate::{
    theme::use_theme,
    utils::{class_list::class_list, mount_style},
    Theme,
};
use leptos::*;
pub use theme::SwitchTheme;

#[component]
pub fn Switch(
    #[prop(optional, into)] value: RwSignal<bool>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    mount_style("switch", include_str!("./switch.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.switch.background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-active: {};",
                theme.common.color_primary
            ));
        });
        css_vars
    });

    view! {
        <div
            class=class_list![
                "thaw-switch", ("thaw-switch--active", move || value.get()), move || class.get()
            ]

            style=move || css_vars.get()
            on:click=move |_| value.set(!value.get_untracked())
        >
            <div class="thaw-switch__button"></div>
        </div>
    }
}
