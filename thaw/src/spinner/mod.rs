mod theme;

use crate::{
    theme::use_theme,
    utils::{class_list::class_list, mount_style},
    Theme,
};
use leptos::*;
pub use theme::SpinnerTheme;

#[derive(Default, Clone)]
pub enum SpinnerSize {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
}

impl SpinnerSize {
    fn theme_height(&self, theme: &Theme) -> String {
        match self {
            SpinnerSize::Tiny => theme.common.height_tiny.clone(),
            SpinnerSize::Small => theme.common.height_small.clone(),
            SpinnerSize::Medium => theme.common.height_medium.clone(),
            SpinnerSize::Large => theme.common.height_large.clone(),
        }
    }
}

#[component]
pub fn Spinner(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(optional, into)] size: MaybeSignal<SpinnerSize>,
) -> impl IntoView {
    mount_style("spinner", include_str!("./spinner.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-height: {};",
                size.get().theme_height(theme)
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                &theme.spinner.background_color
            ));
            css_vars.push_str(&format!("--thaw-color: {};", &theme.common.color_primary));
        });
        css_vars
    });

    view! {
        <div
            class=class_list!["thaw-spinner", move || class.get()]
            style=move || css_vars.get()
        ></div>
    }
}
