mod theme;

pub use theme::ScrollbarTheme;

use crate::{use_theme, Theme};
use leptos::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn Scrollbar(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] style: Option<MaybeSignal<String>>,
    #[prop(default = 8)] size: u8,
    children: Children,
) -> impl IntoView {
    mount_style("scrollbar", include_str!("./scrollbar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-scrollbar-background-color: {};",
                theme.scrollbar.background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-scrollbar-background-color-hover: {};",
                theme.scrollbar.background_color_hover
            ));
            css_vars.push_str(&format!("--thaw-scrollbar-size: {}px;", size));
        });
        css_vars
    });

    view! {
        <div
            class=class_list!["thaw-scrollbar",  class.map(| c | move || c.get())]
            style=move || {
                format!("{}{}", css_vars.get(), style.as_ref().map(|s| s.get()).unwrap_or_default())
            }
        >
            <div class="thaw-scrollbar__container">
                <div class="thaw-scrollbar__content">
                    {children()}
                </div>
            </div>
            <div class="thaw-scrollbar__track--vertical">
                <div class="thaw-scrollabr__thumb"></div>
            </div>
            <div class="thaw-scrollbar__track--horizontal">
                <div class="thaw-scrollabr__thumb"></div>
            </div>
        </div>
    }
}
