mod theme;

pub use theme::SwitchTheme;

use crate::{theme::use_theme, Theme};
use leptos::*;
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[component]
pub fn Switch(
    #[prop(optional, into)] value: Model<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
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
                "thaw-switch", ("thaw-switch--active", move || value.get()), class.map(| c | move ||
                c.get())
            ]

            style=move || css_vars.get()
            on:click=move |_| value.set(!value.get_untracked())
            role="switch"
            aria-checked=move || if value.get() { "true" } else { "false" }
        >
            <div class="thaw-switch__button"></div>
        </div>
    }
}
