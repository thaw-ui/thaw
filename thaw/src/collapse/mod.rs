mod collapse_item;

pub use collapse_item::CollapseItem;

use crate::utils::{class_list::class_list, mount_style};
use leptos::*;
use std::collections::HashSet;

#[component]
pub fn Collapse(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(optional, into)] value: RwSignal<HashSet<String>>,
    #[prop(optional)] accordion: bool,
    children: Children,
) -> impl IntoView {
    mount_style("collapser", include_str!("./collapse.css"));

    view! {
        <Provider value=CollapseInjection{ value, accordion }>
            <div class=class_list!["thaw-collapse", move || class.get()]>{children()}</div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct CollapseInjection {
    pub value: RwSignal<HashSet<String>>,
    pub accordion: bool,
}

pub(crate) fn use_collapse() -> CollapseInjection {
    expect_context()
}
