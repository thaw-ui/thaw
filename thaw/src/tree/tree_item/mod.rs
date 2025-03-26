mod types;

pub use types::*;

use crate::TreeInjection;
use leptos::{context::Provider, prelude::*};
use thaw_utils::class_list;

#[component]
pub fn TreeItem(
    /// A tree item can be a leaf or a branch
    #[prop(optional)]
    item_type: TreeItemType,
    /// A tree item should have a well defined value, in case one is not provided by the user by this prop
    /// one will be inferred internally.
    #[prop(into, optional)]
    value: Option<String>,
    children: Children,
) -> impl IntoView {
    let tree_injection = TreeInjection::expect_context();
    let value = value.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let open = Memo::new(move |_| {
        tree_injection
            .open_items
            .with(|open_items| open_items.contains(&value))
    });

    // --thaw-tree-item--level

    view! {
        <div class=class_list!["thaw-tree-item"] role="treeitem">
            <Provider value=TreeItemInjection { open }>{children()}</Provider>
        </div>
    }
}
