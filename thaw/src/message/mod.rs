mod message_provider;
mod theme;

pub use message_provider::*;
pub use theme::MessageTheme;

use crate::{theme::use_theme, Icon, Theme};
use leptos::*;
use thaw_components::{CSSTransition, If, Then};
use uuid::Uuid;

#[derive(Default, Clone)]
pub enum MessageVariant {
    #[default]
    Success,
    Warning,
    Error,
}

impl MessageVariant {
    fn icon(&self) -> icondata_core::Icon {
        match self {
            MessageVariant::Success => icondata_ai::AiCloseCircleFilled,
            MessageVariant::Warning => icondata_ai::AiExclamationCircleFilled,
            MessageVariant::Error => icondata_ai::AiCheckCircleFilled,
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
fn Message(message: MessageType, #[prop(into)] on_close: Callback<Uuid, ()>) -> impl IntoView {
    let (id, content, variant, MessageOptions { duration, closable }) = message;
    let is_show = RwSignal::new(true);
    let message_ref = NodeRef::<html::Div>::new();

    if !duration.is_zero() {
        set_timeout(
            move || {
                is_show.set(false);
            },
            duration,
        );
    }

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

    let on_before_leave = move |_| {
        let Some(node_el) = message_ref.get() else {
            return;
        };
        use std::ops::Deref;
        let any_el = node_el.into_any();
        let el = any_el.deref();
        let style = el.style();
        let _ = style.set_property("max-height", &format!("{}px", el.offset_height()));
    };
    let on_after_leave = Callback::new(move |_| {
        queue_microtask(move || on_close.call(id));
    });

    view! {
        <CSSTransition
            node_ref=message_ref
            name="fade-in-height-expand-transition"
            show=is_show
            appear=true
            on_before_leave=on_before_leave
            on_after_leave=on_after_leave
            let:_
        >
            <div class="thaw-message-wrapper" ref=message_ref>
                <div class="thaw-message" style=move || css_vars.get()>
                    <div class="thaw-message__icon">
                        <Icon icon=variant.icon() style/>
                    </div>
                    <div class="thaw-message__content">{content}</div>
                    <If cond=closable>
                        <Then slot>
                            <div class="thaw-message__close" on:click=move |_| is_show.set(false)>
                                <Icon icon=icondata_ai::AiCloseOutlined/>
                            </div>
                        </Then>
                    </If>
                </div>
            </div>
        </CSSTransition>
    }
}
