use crate::{theme::use_theme, utils::mount_style::mount_style, Icon, Theme};
use icondata::AiIcon;
use leptos::*;

#[derive(Clone)]
pub enum AlertVariant {
    SUCCESS,
    WARNING,
    ERROR,
}

impl AlertVariant {
    pub fn theme_border_color(&self, theme: &Theme) -> String {
        match self {
            AlertVariant::SUCCESS => theme.common.color_success.clone(),
            AlertVariant::WARNING => theme.common.color_warning.clone(),
            AlertVariant::ERROR => theme.common.color_error.clone(),
        }
    }
}

#[component]
pub fn Alert(
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
                let border_color = variant.theme_border_color(&theme);
                css_vars.push_str(&format!("--background-color: {border_color}aa;"));
                css_vars.push_str(&format!("--border-color: {border_color};"));
            });

            css_vars
        }
    });
    let icon = create_memo(move |_| {
        match variant.get() {
            AlertVariant::SUCCESS => AiIcon::AiCheckCircleFilled,
            AlertVariant::WARNING => AiIcon::AiExclamationCircleFilled,
            AlertVariant::ERROR => AiIcon::AiCloseCircleFilled,
        }
        .into()
    });
    view! {
        <div class="melt-alert" style=move || css_vars.get()>
            <Icon icon class="melt-alert__icon"/>
            <div>
                {
                    move || {
                        let title = title.get();
                        if title.is_empty() {
                            None
                        } else {
                            view! {
                                <div class="melt-alert__header">
                                    {title}
                                </div>
                            }.into()
                        }
                    }
                }
                <div class="melt-alert-body__content">
                    { children() }
                </div>
            </div>
        </div>
    }
}
