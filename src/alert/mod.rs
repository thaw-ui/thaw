mod theme;

#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::{
    theme::use_theme,
    utils::{mount_style, ssr_class},
    Icon, Theme,
};
use icondata::AiIcon;
use leptos::*;
pub use theme::AlertTheme;

#[derive(Clone)]
pub enum AlertVariant {
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    fn theme_icon_color(&self, theme: &Theme) -> String {
        match self {
            AlertVariant::Success => theme.common.color_success.clone(),
            AlertVariant::Warning => theme.common.color_warning.clone(),
            AlertVariant::Error => theme.common.color_error.clone(),
        }
    }
    fn theme_background_color(&self, theme: &Theme) -> String {
        match self {
            AlertVariant::Success => theme.alert.success_background_color.clone(),
            AlertVariant::Warning => theme.alert.warning_background_color.clone(),
            AlertVariant::Error => theme.alert.error_background_color.clone(),
        }
    }
    fn theme_border_color(&self, theme: &Theme) -> String {
        match self {
            AlertVariant::Success => theme.alert.success_border_color.clone(),
            AlertVariant::Warning => theme.alert.warning_border_color.clone(),
            AlertVariant::Error => theme.alert.error_border_color.clone(),
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(into)] variant: MaybeSignal<AlertVariant>,
    children: Children,
) -> impl IntoView {
    mount_style("alert", include_str!("./alert.css"));
    let theme = use_theme(Theme::light);

    let css_vars = create_memo({
        let variant = variant.clone();

        move |_| {
            let mut css_vars = String::new();

            theme.with(|theme| {
                let variant = variant.get();
                css_vars.push_str(&format!(
                    "--thaw-icon-color: {};",
                    variant.theme_icon_color(theme)
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
        }
    });
    let icon = create_memo(move |_| {
        match variant.get() {
            AlertVariant::Success => AiIcon::AiCheckCircleFilled,
            AlertVariant::Warning => AiIcon::AiExclamationCircleFilled,
            AlertVariant::Error => AiIcon::AiCloseCircleFilled,
        }
        .into()
    });

    let ssr_class = ssr_class(&class);
    view! {
        <div
            class=ssr_class
            use:dyn_classes=class
            class:thaw-alert=true
            style=move || css_vars.get()
        >
            <Icon icon class="thaw-alert__icon"/>
            <div>

                {move || {
                    let title = title.get();
                    if title.is_empty() {
                        None
                    } else {
                        view! { <div class="thaw-alert__header">{title}</div> }.into()
                    }
                }}
                <div class="thaw-alert__content">{children()}</div>
            </div>
        </div>
    }
}
