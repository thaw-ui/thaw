mod types;

pub use types::*;

use crate::{TreeItemInjection, _motion::CollapseTransition};
use leptos::{context::Provider, either::Either, prelude::*};
use std::collections::HashSet;
use thaw_utils::{class_list, Model};

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/tree/tree/tree.css");

#[component]
pub fn Tree(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// This refers to a list of ids of opened tree items.
    /// Controls the state of the open tree items. These property is ignored for subtrees.
    #[prop(into, optional)]
    open_items: Model<HashSet<String>>,
    /// Size of the tree item.
    #[prop(optional, into)]
    size: Signal<TreeSize>,
    children: Children,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("tree", include_str!("./tree.css"));

    if let Some(subtree_injection) = SubtreeInjection::use_context() {
        let level = subtree_injection.level + 1;
        Either::Left(view! { <Subtree class size level children /> })
    } else {
        Either::Right(view! { <RootTree class open_items size children /> })
    }
}

#[component]
fn RootTree(
    class: MaybeProp<String>,
    open_items: Model<HashSet<String>>,
    size: Signal<TreeSize>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class_list![
                "thaw-tree", move || format!("thaw-tree--{}", size.get().as_str()), class
            ]
            role="tree"
        >
            <Provider value=TreeInjection { open_items }>
                <Provider value=SubtreeInjection { level: 1 }>{children()}</Provider>
            </Provider>
        </div>
    }
}

#[component]
fn Subtree(
    class: MaybeProp<String>,
    level: usize,
    size: Signal<TreeSize>,
    children: Children,
) -> impl IntoView {
    let tree_item_injection = TreeItemInjection::expect_context();
    let open = tree_item_injection.open;
    let subtree_ref = tree_item_injection.subtree_ref;

    view! {
        <CollapseTransition show=open>
            <div
                class=class_list![
                    "thaw-tree", "thaw-subtree", move || format!("thaw-tree--{}", size.get().as_str()), class
                ]
                role="group"
                node_ref=subtree_ref
            >
                <Provider value=SubtreeInjection { level }>{children()}</Provider>
            </div>
        </CollapseTransition>
    }
}
