use super::TagSize;
use leptos::{context::Provider, prelude::*};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn TagGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Size of the tag.
    #[prop(optional, into)]
    size: MaybeSignal<TagSize>,
    children: Children,
) -> impl IntoView {
    mount_style("tag-group", include_str!("./tag-group.css"));

    view! {
        <div class=class_list!["thaw-tag-group", class]>
            <Provider value=TagGroupInjection { size }>
                {children()}
            </Provider>
        </div>
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TagGroupInjection {
    pub size: MaybeSignal<TagSize>,
}

impl TagGroupInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
