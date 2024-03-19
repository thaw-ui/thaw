use crate::{use_theme, Theme};
use leptos::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn Text(
    #[prop(optional)] code: bool,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("text", include_str!("./text.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {}",
                theme.typograph.code_background_color
            ));
        });
        css_vars
    });

    if code {
        view! {
            <code
                class=class_list!["thaw-text thaw-text--code", class.map(| c | move || c.get())]
                style=move || css_vars.get()
            >
                {children()}
            </code>
        }
        .into_any()
    } else {
        view! { <span class=class_list!["thaw-text", class.map(| c | move || c.get())]>{children()}</span> }
            .into_any()
    }
}
