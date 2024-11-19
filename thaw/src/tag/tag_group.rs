use super::TagSize;
use leptos::{context::Provider, prelude::*};
use thaw_utils::{class_list, mount_style, ArcOneCallback};

#[component]
pub fn TagGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Size of the tag.
    #[prop(optional, into)]
    size: Signal<TagSize>,
    /// Callback for when a tag is dismissed.
    #[prop(optional, into)]
    on_dismiss: Option<ArcOneCallback<String>>,
    /// A Tag can be dismissible.
    #[prop(optional, into)]
    dismissible: Signal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("tag-group", include_str!("./tag-group.css"));

    view! {
        <div class=class_list!["thaw-tag-group", class]>
            <Provider value=TagGroupInjection {
                size,
                on_dismiss,
                dismissible,
            }>{children()}</Provider>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct TagGroupInjection {
    pub size: Signal<TagSize>,
    pub on_dismiss: Option<ArcOneCallback<String>>,
    pub dismissible: Signal<bool>,
}

impl TagGroupInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
