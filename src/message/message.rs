use crate::{theme::use_theme, Icon, Theme};
use icondata::*;
use leptos::*;

#[derive(Default, Clone)]
pub enum MessageVariant {
    #[default]
    Success,
    Warning,
    Error,
}

impl MessageVariant {
    fn icon(&self) -> Icon {
        match self {
            MessageVariant::Success => icondata::Icon::Ai(AiCloseCircleFilled),
            MessageVariant::Warning => icondata::Icon::Ai(AiExclamationCircleFilled),
            MessageVariant::Error => icondata::Icon::Ai(AiCheckCircleFilled),
        }
    }
    fn theme_color(&self, theme: &Theme) -> String {
        match self {
            MessageVariant::Success => theme.common.color_success.clone(),
            MessageVariant::Warning => theme.common.color_warning.clone(),
            MessageVariant::Error => theme.common.color_error.clone(),
        }
    }
}

#[component]
pub(crate) fn Message(variant: MessageVariant, content: String) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let style = theme.with_untracked(|theme| format!("color: {};", variant.theme_color(theme)));
    view! {
        <div class="melt-message-wrapper">
            <div class="melt-message">
                <div class="melt-message__icon">
                    <Icon icon=variant.icon() style/>
                </div>
                <div class="melt-message__content">{content}</div>
            </div>
        </div>
    }
}
