mod theme;

use crate::{mount_style, theme::use_theme, utils::maybe_rw_signal::MaybeRwSignal, Theme};
use leptos::*;
pub use theme::SwitchTheme;

#[component]
pub fn Switch(#[prop(optional, into)] value: MaybeRwSignal<bool>) -> impl IntoView {
    mount_style("switch", include_str!("./switch.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--melt-background-color: {};",
                theme.switch.background_color
            ));
            css_vars.push_str(&format!(
                "--melt-background-color-active: {};",
                theme.common.color_primary
            ));
        });
        css_vars
    });
    view! {
        <div
            class="melt-switch"
            class=("melt-switch--active", move || value.get())
            style=move || css_vars.get()
            on:click=move |_| value.set(!value.get_untracked())
        >
            <div class="melt-switch__button"></div>
        </div>
    }
}
