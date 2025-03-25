mod types;

pub use types::*;

use leptos::{context::Provider, prelude::*};
use std::collections::HashSet;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn Tree(
    #[prop(into, optional)] open_items: Model<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("tree", include_str!("./tree.css"));
    view! {
        <div class=class_list!["thaw-tree"] role="tree">
            <Provider value=TreeInjection { open_items }>{children()}</Provider>
        </div>
    }
}
