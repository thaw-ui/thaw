mod theme;

#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::{
    use_theme,
    utils::{mount_style, ssr_class},
    Theme,
};
use leptos::*;
pub use theme::AvatarTheme;

#[component]
pub fn Avatar(
    #[prop(optional, into)] src: MaybeSignal<String>,
    #[prop(optional, into)] round: MaybeSignal<bool>,
    #[prop(default = MaybeSignal::Static(30), into)] size: MaybeSignal<u16>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        css_vars.push_str(&format!("--thaw-size: {}px;", size.get()));
        css_vars.push_str(&format!(
            "--thaw-border-radius: {};",
            if round.get() { "50%" } else { "3px" }
        ));
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {}",
                theme.avatar.background_color
            ));
        });
        css_vars
    });
    mount_style("avatar", include_str!("./avatar.css"));

    let ssr_class = ssr_class(&class);
    view! {
        <span class=ssr_class use:dyn_classes=class class:thaw-avatar=true style=move || css_vars.get()>
            {move || {
                let src = src.get();
                (!src.is_empty()).then(|| view! { <img src=src/> })
            }}

        </span>
    }
}
