mod types;

pub use types::*;

use crate::TreeItemInjection;
use leptos::{context::Provider, either::Either, prelude::*};
use std::collections::HashSet;
use thaw_components::CSSTransition;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn Tree(
    #[prop(into, optional)] open_items: Model<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("tree", include_str!("./tree.css"));

    if let Some(subtree_injection) = SubtreeInjection::use_context() {
        let level = subtree_injection.level + 1;
        Either::Left(view! { <Subtree level children /> })
    } else {
        Either::Right(view! { <RootTree open_items children /> })
    }
}

#[component]
fn RootTree(open_items: Model<HashSet<String>>, children: Children) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-tree"] role="tree">
            <Provider value=TreeInjection { open_items }>
                <Provider value=SubtreeInjection { level: 1 }>{children()}</Provider>
            </Provider>
        </div>
    }
}

#[component]
fn Subtree(level: usize, children: Children) -> impl IntoView {
    mount_style("motion", include_str!("../../_motion/index.css"));

    let tree_item_injection = TreeItemInjection::expect_context();
    let open = tree_item_injection.open;
    let subtree_ref = tree_item_injection.subtree_ref;

    view! {
        <CSSTransition show=open name="thaw-motion-collapse">
            <div class=class_list!["thaw-tree", "thaw-subtree"] role="group" node_ref=subtree_ref>
                <Provider value=SubtreeInjection { level }>{children()}</Provider>
            </div>
        </CSSTransition>
    }
}
