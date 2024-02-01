mod theme;

use crate::{
    components::OptionComp,
    theme::use_theme,
    utils::{class_list::class_list, mount_style, OptionalProp},
    Icon, Theme,
};
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
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] title: Option<MaybeSignal<String>>,
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
    let icon = create_memo(move |_| match variant.get() {
        AlertVariant::Success => icondata_ai::AiCheckCircleFilled,
        AlertVariant::Warning => icondata_ai::AiExclamationCircleFilled,
        AlertVariant::Error => icondata_ai::AiCloseCircleFilled,
    });

    view! {
        <div
            class=class_list!["thaw-alert", class.map(| c | move || c.get())]
            style=move || css_vars.get()
        >
            <Icon icon class="thaw-alert__icon"/>
            <div>
                <OptionComp value=title let:title>
                    <div class="thaw-alert__header">{move || title.get()}</div>
                </OptionComp>
                <div class="thaw-alert__content">{children()}</div>
            </div>
        </div>
    }
}
