use crate::{
    use_theme,
    utils::{class_list::class_list, mount_style},
    Theme,
};
use leptos::*;

#[component]
pub fn Text(
    #[prop(optional)] code: bool,
    #[prop(optional, into)] class: MaybeSignal<String>,
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
                class=class_list!["thaw-text thaw-text--code", move || class.get()]
                style=move || css_vars.get()
            >
                {children()}
            </code>
        }
        .into_any()
    } else {
        view! { <span class=class_list!["thaw-text", move || class.get()]>{children()}</span> }
        .into_any()
    }
}
