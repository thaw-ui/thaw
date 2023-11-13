mod theme;

use crate::{theme::use_theme, utils::mount_style, Theme};
use leptos::*;
pub use theme::TagTheme;

#[derive(Clone, Default)]
pub enum TagVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
}

impl TagVariant {
    pub fn theme_font_color(&self, theme: &Theme) -> String {
        match self {
            TagVariant::Default => theme.tag.default_font_color.clone(),
            TagVariant::Success => theme.common.color_success.clone(),
            TagVariant::Warning => theme.common.color_warning.clone(),
            TagVariant::Error => theme.common.color_error.clone(),
        }
    }
    pub fn theme_background_color(&self, theme: &Theme) -> String {
        match self {
            TagVariant::Default => theme.tag.default_background_color.clone(),
            TagVariant::Success => theme.tag.success_background_color.clone(),
            TagVariant::Warning => theme.tag.warning_background_color.clone(),
            TagVariant::Error => theme.tag.error_background_color.clone(),
        }
    }
    pub fn theme_border_color(&self, theme: &Theme) -> String {
        match self {
            TagVariant::Default => theme.tag.default_border_color.clone(),
            TagVariant::Success => theme.tag.success_border_color.clone(),
            TagVariant::Warning => theme.tag.warning_border_color.clone(),
            TagVariant::Error => theme.tag.error_border_color.clone(),
        }
    }
}

#[component]
pub fn Tag(
    #[prop(optional, into)] variant: MaybeSignal<TagVariant>,
    children: Children,
) -> impl IntoView {
    mount_style("tag", include_str!("./tag.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let variant = variant.get();
            css_vars.push_str(&format!(
                "--thaw-font-color: {};",
                variant.theme_font_color(theme)
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                variant.theme_background_color(theme)
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                variant.theme_border_color(theme)
            ));
        });
        css_vars
    });
    view! {
        <div class="thaw-tag" style=move || css_vars.get()>
            <span class="thaw-tag__content">{children()}</span>
        </div>
    }
}
