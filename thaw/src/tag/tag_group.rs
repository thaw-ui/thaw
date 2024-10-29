use super::TagSize;
use leptos::{context::Provider, html, prelude::*};
use thaw_tabster::{use_arrow_navigation_group, Axis, UseArrowNavigationGroupOptions};
use thaw_utils::{class_list, mount_style, ArcOneCallback};

#[component]
pub fn TagGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Size of the tag.
    #[prop(optional, into)]
    size: MaybeSignal<TagSize>,
    /// Callback for when a tag is dismissed.
    #[prop(optional, into)]
    on_dismiss: Option<ArcOneCallback<String>>,
    /// A Tag can be dismissible.
    #[prop(optional, into)]
    dismissible: MaybeSignal<bool>,
    #[prop(optional)] node_ref: Option<NodeRef<html::Div>>,
    children: Children,
) -> impl IntoView {
    mount_style("tag-group", include_str!("./tag-group.css"));
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    let arrowNavigationProps = use_arrow_navigation_group(UseArrowNavigationGroupOptions {
        circular: Some(true),
        axis: Some(Axis::Both),
        memorize_current: Some(true),
        ..Default::default()
    });
    view! {
        <div class=class_list!["thaw-tag-group", class] node_ref=node_ref data-tabster=arrowNavigationProps.1>
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
    pub size: MaybeSignal<TagSize>,
    pub on_dismiss: Option<ArcOneCallback<String>>,
    pub dismissible: MaybeSignal<bool>,
}

impl TagGroupInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
