mod interaction_tag;
mod tag_group;

pub use interaction_tag::*;
pub use tag_group::*;

use crate::DismissRegularIcon;
use leptos::{either::Either, ev, prelude::*};
use thaw_utils::{class_list, mount_style, ArcOneCallback};

#[component]
pub fn Tag(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Whether the tag is disabled.
    #[prop(optional, into)]
    disabled: Option<Signal<bool>>,
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
    let (group_disabled, group_size, group_on_dismiss, group_dismissible) =
        TagGroupInjection::use_context()
            .map(
                |TagGroupInjection {
                     disabled,
                     size,
                     on_dismiss,
                     dismissible,
                 }| {
                    if value.is_none() {
                        (Some(disabled), Some(size), None, None)
                    } else {
                        (Some(disabled), Some(size), on_dismiss, Some(dismissible))
                    }
                },
            )
            .unwrap_or_default();

    let disabled = {
        if let Some(disabled) = disabled {
            Some(disabled)
        } else if let Some(disabled) = group_disabled {
            Some(disabled)
        } else {
            None
        }
    };

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
                ("thaw-tag--disabled", move || disabled.map_or(false, |d| d.get())),
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
                        if disabled.map_or(false, |d| d.get()) {
                            return;
                        }
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
                                <DismissRegularIcon />
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
