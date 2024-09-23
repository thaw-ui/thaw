use leptos::{either::Either, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::class_list;

use crate::ToastIntent;

#[component]
pub fn ToastTitle(
    #[prop(optional)] toast_title_media: Option<ToastTitleMedia>,
    children: Children,
    #[prop(optional)] toast_title_action: Option<ToastTitleAction>,
) -> impl IntoView {
    let intent: ToastIntent = expect_context();

    view! {
        <div class=class_list![
            "thaw-toast-title__media", format!("thaw-toast-title__{:?}", intent).to_lowercase()
        ]>
            {if let Some(media) = toast_title_media {
                Either::Left((media.children)())
            } else {
                {
                    Either::Right(
                        view! {
                            <svg
                                fill="currentColor"
                                aria-hidden="true"
                                width="1em"
                                height="1em"
                                viewBox="0 0 20 20"
                            >
                                {match intent {
                                    ToastIntent::Info => {
                                        view! {
                                            <path
                                                d="M18 10a8 8 0 1 0-16 0 8 8 0 0 0 16 0ZM9.5 8.91a.5.5 0 0 1 1 0V13.6a.5.5 0 0 1-1 0V8.9Zm-.25-2.16a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0Z"
                                                fill="currentColor"
                                            ></path>
                                        }
                                    }
                                    ToastIntent::Success => {
                                        view! {
                                            <path
                                                d="M10 2a8 8 0 1 1 0 16 8 8 0 0 1 0-16Zm3.36 5.65a.5.5 0 0 0-.64-.06l-.07.06L9 11.3 7.35 9.65l-.07-.06a.5.5 0 0 0-.7.7l.07.07 2 2 .07.06c.17.11.4.11.56 0l.07-.06 4-4 .07-.08a.5.5 0 0 0-.06-.63Z"
                                                fill="currentColor"
                                            ></path>
                                        }
                                    }
                                    ToastIntent::Warning => {
                                        view! {
                                            <path
                                                d="M8.68 2.79a1.5 1.5 0 0 1 2.64 0l6.5 12A1.5 1.5 0 0 1 16.5 17h-13a1.5 1.5 0 0 1-1.32-2.21l6.5-12ZM10.5 7.5a.5.5 0 0 0-1 0v4a.5.5 0 0 0 1 0v-4Zm.25 6.25a.75.75 0 1 0-1.5 0 .75.75 0 0 0 1.5 0Z"
                                                fill="currentColor"
                                            ></path>
                                        }
                                    }
                                    ToastIntent::Error => {
                                        view! {
                                            <path
                                                d="M10 2a8 8 0 1 1 0 16 8 8 0 0 1 0-16ZM7.8 7.11a.5.5 0 0 0-.63.06l-.06.07a.5.5 0 0 0 .06.64L9.3 10l-2.12 2.12-.06.07a.5.5 0 0 0 .06.64l.07.06c.2.13.47.11.64-.06L10 10.7l2.12 2.12.07.06c.2.13.46.11.64-.06l.06-.07a.5.5 0 0 0-.06-.64L10.7 10l2.12-2.12.06-.07a.5.5 0 0 0-.06-.64l-.07-.06a.5.5 0 0 0-.64.06L10 9.3 7.88 7.17l-.07-.06Z"
                                                fill="currentColor"
                                            ></path>
                                        }
                                    }
                                }}
                            </svg>
                        },
                    )
                }
            }}
        </div>
        <div class="thaw-toast-title">{children()}</div>
        <OptionComp value=toast_title_action let:action>
            <div class="thaw-toast-title__action">{(action.children)()}</div>
        </OptionComp>
    }
}

#[slot]
pub struct ToastTitleMedia {
    children: Children,
}

#[slot]
pub struct ToastTitleAction {
    children: Children,
}
