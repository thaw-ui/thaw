mod theme;

use crate::{mount_style, use_theme, Theme};
use leptos::*;
pub use theme::AvatarTheme;

#[component]
pub fn Avatar(
    #[prop(optional, into)] src: MaybeSignal<String>,
    #[prop(optional, into)] circle: MaybeSignal<bool>,
    #[prop(default = MaybeSignal::Static(30), into)] size: MaybeSignal<i32>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        css_vars.push_str(&format!("--melt-size: {}px;", size.get()));
        css_vars.push_str(&format!(
            "--melt-border-radius: {};",
            if circle.get() { "50%" } else { "3px" }
        ));
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--melt-background-color: {}",
                theme.avatar.background_color
            ));
        });
        css_vars
    });
    mount_style("avatar", include_str!("./avatar.css"));
    view! {
        <span class="melt-avatar" style=move || css_vars.get()>
            {move || {
                let src = src.get();
                (!src.is_empty()).then(|| view! {
                    <img src=src />
                })
            }}
        </span>
    }
}
