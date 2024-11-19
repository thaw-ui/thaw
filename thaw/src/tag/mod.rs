mod interaction_tag;
mod tag_group;

pub use interaction_tag::*;
pub use tag_group::*;

use leptos::{either::Either, ev, prelude::*};
use thaw_utils::{class_list, mount_style, ArcOneCallback};

#[component]
pub fn Tag(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Size of the tag.
    #[prop(optional, into)]
    size: Option<Signal<TagSize>>,
    /// A Tag can be dismissible.
    #[prop(optional, into)]
    dismissible: Signal<bool>,
    /// Callback for when a tag is dismissed.
    #[prop(optional, into)]
    on_dismiss: Option<ArcOneCallback<ev::MouseEvent>>,
    /// Unique value identifying the tag within a TagGroup.
    #[prop(optional, into)]
    value: Option<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tag", include_str!("./tag.css"));
    let (group_size, group_on_dismiss, group_dismissible) = TagGroupInjection::use_context()
        .map(
            |TagGroupInjection {
                 size,
                 on_dismiss,
                 dismissible,
             }| {
                if value.is_none() {
                    (Some(size), None, None)
                } else {
                    (Some(size), on_dismiss, Some(dismissible))
                }
            },
        )
        .unwrap_or_default();

    let size_class = {
        if let Some(size) = size {
            Some(size)
        } else if let Some(group_size) = group_size {
            Some(group_size)
        } else {
            None
        }
    };

    view! {
        <span class=class_list![
            "thaw-tag",
                ("thaw-tag--dismissible", move || group_dismissible.map_or_else(|| dismissible.get(), |d| d.get())),
                size_class.map(|size| move || format!("thaw-tag--{}", size.get().as_str())),
                class
        ]>

            <span class="thaw-tag__primary-text">{children()}</span>

            {move || {
                if group_dismissible.map_or_else(|| dismissible.get(), |d| d.get()) {
                    let on_dismiss = on_dismiss.clone();
                    let group_on_dismiss = group_on_dismiss.clone();
                    let value = value.clone();
                    let on_dismiss = move |event: ev::MouseEvent| {
                        if let Some(on_dismiss) = group_on_dismiss.as_ref() {
                            event.prevent_default();
                            on_dismiss(value.clone().unwrap());
                        }
                        let Some(on_dismiss) = on_dismiss.as_ref() else {
                            return;
                        };
                        on_dismiss(event);
                    };
                    Either::Left(
                        view! {
                            <button class="thaw-tag__dismiss" on:click=on_dismiss>
                                <svg
                                    fill="currentColor"
                                    aria-hidden="true"
                                    width="1em"
                                    height="1em"
                                    viewBox="0 0 20 20"
                                >
                                    <path
                                        d="m4.09 4.22.06-.07a.5.5 0 0 1 .63-.06l.07.06L10 9.29l5.15-5.14a.5.5 0 0 1 .63-.06l.07.06c.18.17.2.44.06.63l-.06.07L10.71 10l5.14 5.15c.18.17.2.44.06.63l-.06.07a.5.5 0 0 1-.63.06l-.07-.06L10 10.71l-5.15 5.14a.5.5 0 0 1-.63.06l-.07-.06a.5.5 0 0 1-.06-.63l.06-.07L9.29 10 4.15 4.85a.5.5 0 0 1-.06-.63l.06-.07-.06.07Z"
                                        fill="currentColor"
                                    ></path>
                                </svg>
                            </button>
                        },
                    )
                } else {
                    Either::Right(())
                }
            }}

        </span>
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TagSize {
    #[default]
    Medium,
    Small,
    ExtraSmall,
}

impl TagSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Small => "small",
            Self::ExtraSmall => "extra-small",
        }
    }
}
