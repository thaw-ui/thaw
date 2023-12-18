#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::{
    theme::use_theme,
    utils::{mount_style, ssr_class},
    Theme,
};
use leptos::*;

#[component]
pub fn MenuGroup(
    #[prop(into)] label: String,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("menu-group", include_str!("./menu-group.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.menu.group_color));
        });
        css_vars
    });
    let ssr_class = ssr_class(&class);
    view! {
        <div
            class=ssr_class
            use:dyn_classes=class
            class="thaw-menu-group"
            style=move || css_vars.get()
        >
            {label}
        </div>
        {children()}
    }
}

