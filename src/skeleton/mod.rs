mod theme;

use crate::{mount_style, theme::use_theme, Theme};
use leptos::*;
pub use theme::SkeletionTheme;

#[component]
pub fn Skeleton(
    #[prop(default = MaybeSignal::Static(1), into)] repeat: MaybeSignal<u32>,
    #[prop(optional, into)] text: MaybeSignal<bool>,
    #[prop(optional, into)] width: Option<MaybeSignal<String>>,
    #[prop(optional, into)] height: Option<MaybeSignal<String>>,
) -> impl IntoView {
    mount_style("skeleton", include_str!("./skeleton.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        if text.get() {
            css_vars.push_str("display: inline-block;");
        }

        if let Some(width) = width.as_ref() {
            css_vars.push_str(&format!("width: {};", width.get()));
        }
        if let Some(height) = height.as_ref() {
            css_vars.push_str(&format!("height: {};", height.get()));
        }

        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color-start: {};",
                theme.skeletion.background_color_start
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-end: {};",
                theme.skeletion.background_color_end
            ));
        });

        css_vars
    });
    (0..repeat.get())
        .map(|_| {
            view! { <div class="thaw-skeleton" style=move || css_vars.get()></div> }
        })
        .collect_view()
}
