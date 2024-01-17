mod message_environment;
mod message_provider;
mod theme;

use crate::{theme::use_theme, Icon, Theme, components::{If, Then}};
use icondata::*;
use leptos::*;
use uuid::Uuid;
pub use message_provider::*;
pub use theme::MessageTheme;

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
pub(crate) fn Message(
    variant: MessageVariant,
    content: String,
    can_close: bool,
    id: Uuid,
    #[prop(into)] on_close: Callback<Uuid, ()>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {}",
                theme.message.background_color
            ))
        });
        css_vars
    });
    let style = theme.with_untracked(|theme| format!("color: {};", variant.theme_color(theme)));
    view! {
        <div class="thaw-message-wrapper">
            <div class="thaw-message" style=move || css_vars.get()>
                <div class="thaw-message__icon">
                    <Icon icon=variant.icon() style/>
                </div>
                <div class="thaw-message__content">{content}</div>
                <If cond=can_close>
                    <Then slot>
                        <div class="thaw-message__close"  on:click=move |_| on_close.call(id)>
                            <Icon icon=icondata::Icon::Ai(AiCloseCircleFilled)/>
                        </div>
                    </Then>
                </If>
            </div>
        </div>
    }
}


