mod types;

pub use types::*;

use leptos::{context::Provider, either::Either, prelude::*};
use std::collections::HashSet;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn Tree(
    #[prop(into, optional)] open_items: Model<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("tree", include_str!("./tree.css"));

    let subtree_injection = SubtreeInjection::use_context();
    let is_root = subtree_injection.is_none();

    let level = if let Some(subtree_injection) = subtree_injection {
        subtree_injection.level + 1
    } else {
        1
    };

    let role = if is_root {
        "tree"
    } else {
        "group"
    };

    view! {
        <div class=class_list!["thaw-tree"] role=role>
            {if is_root {
                Either::Left(view! {
                    <Provider value=TreeInjection { open_items }>
                        <Provider value=SubtreeInjection { level }>
                            {children()}
                        </Provider>
                    </Provider>
                })
            } else {
                Either::Right(view! {
                    <Provider value=SubtreeInjection { level }>
                        {children()}
                    </Provider>
                })
            }}
        </div>
    }
}
