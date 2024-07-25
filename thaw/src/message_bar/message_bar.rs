use leptos::prelude::*;
use thaw_utils::{class_list, mount_style, StoredMaybeSignal};

#[component]
pub fn MessageBar(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] layout: MaybeSignal<MessageBarLayout>,
    #[prop(optional, into)] intent: MaybeSignal<MessageBarIntent>,
    children: Children,
) -> impl IntoView {
    mount_style("message-bar", include_str!("./message-bar.css"));
    let intent: StoredMaybeSignal<_> = intent.into();

    view! {
        <div
            class=class_list![
                "thaw-message-bar",
                move || format!("thaw-message-bar--{}", intent.get().as_str()),
                move || format!("thaw-message-bar--{}", layout.get().as_str()),
                class
            ]
            role="group"
        >
            <div class="thaw-message-bar__icon">
                {
                    move || match intent.get() {
                        MessageBarIntent::Info => view! {
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="M18 10a8 8 0 1 0-16 0 8 8 0 0 0 16 0ZM9.5 8.91a.5.5 0 0 1 1 0V13.6a.5.5 0 0 1-1 0V8.9Zm-.25-2.16a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0Z" fill="currentColor"></path>
                            </svg>
                        },
                        MessageBarIntent::Success => view! {
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="M10 2a8 8 0 1 1 0 16 8 8 0 0 1 0-16Zm3.36 5.65a.5.5 0 0 0-.64-.06l-.07.06L9 11.3 7.35 9.65l-.07-.06a.5.5 0 0 0-.7.7l.07.07 2 2 .07.06c.17.11.4.11.56 0l.07-.06 4-4 .07-.08a.5.5 0 0 0-.06-.63Z" fill="currentColor">
                                </path>
                            </svg>
                        },
                        MessageBarIntent::Warning => view! {
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="M8.68 2.79a1.5 1.5 0 0 1 2.64 0l6.5 12A1.5 1.5 0 0 1 16.5 17h-13a1.5 1.5 0 0 1-1.32-2.21l6.5-12ZM10.5 7.5a.5.5 0 0 0-1 0v4a.5.5 0 0 0 1 0v-4Zm.25 6.25a.75.75 0 1 0-1.5 0 .75.75 0 0 0 1.5 0Z" fill="currentColor">
                                </path>
                            </svg>
                        },
                        MessageBarIntent::Error => view! {
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="M10 2a8 8 0 1 1 0 16 8 8 0 0 1 0-16ZM7.8 7.11a.5.5 0 0 0-.63.06l-.06.07a.5.5 0 0 0 .06.64L9.3 10l-2.12 2.12-.06.07a.5.5 0 0 0 .06.64l.07.06c.2.13.47.11.64-.06L10 10.7l2.12 2.12.07.06c.2.13.46.11.64-.06l.06-.07a.5.5 0 0 0-.06-.64L10.7 10l2.12-2.12.06-.07a.5.5 0 0 0-.06-.64l-.07-.06a.5.5 0 0 0-.64.06L10 9.3 7.88 7.17l-.07-.06Z" fill="currentColor">
                                </path>
                            </svg>
                        },
                    }
                }

            </div>
            {children()}
        </div>
    }
}

#[derive(Default, Clone)]
pub enum MessageBarIntent {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl MessageBarIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageBarIntent::Info => "into",
            MessageBarIntent::Success => "success",
            MessageBarIntent::Warning => "warning",
            MessageBarIntent::Error => "error",
        }
    }
}

#[derive(Default, Clone)]
pub enum MessageBarLayout {
    #[default]
    Singleline,
    Multiline,
}

impl MessageBarLayout {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageBarLayout::Singleline => "singleline",
            MessageBarLayout::Multiline => "multiline",
        }
    }
}
