use super::{TagGroupInjection, TagSize};
use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn InteractionTag(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Size of the tag.
    #[prop(optional, into)]
    size: Option<MaybeSignal<TagSize>>,
    children: Children,
) -> impl IntoView {
    mount_style("interaction-tag", include_str!("./interaction-tag.css"));
    let tag_group = TagGroupInjection::use_context();
    let size_class = {
        if let Some(size) = size {
            Some(size)
        } else if let Some(tag_group) = tag_group {
            Some(tag_group.size)
        } else {
            None
        }
    };

    view! {
        <div class=class_list![
            "thaw-interaction-tag",
                size_class.map(|size| move || format!("thaw-interaction-tag--{}", size.get().as_str())),
                class
        ]>{children()}</div>
    }
}

#[component]
pub fn InteractionTagPrimary(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <button class=class_list!["thaw-interaction-tag-primary", class]>
            <span class="thaw-interaction-tag-primary__primary-text">{children()}</span>
        </button>
    }
}
