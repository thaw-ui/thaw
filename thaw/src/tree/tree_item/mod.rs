mod types;

pub use types::*;

use crate::{SubtreeInjection, TreeInjection};
use leptos::{context::Provider, ev, html, prelude::*};
use thaw_utils::class_list;

#[component]
pub fn TreeItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A tree item can be a leaf or a branch.
    #[prop(optional)]
    item_type: TreeItemType,
    /// A tree item should have a well defined value, in case one is not provided by the user by this prop
    /// one will be inferred internally.
    #[prop(into, optional)]
    value: Option<String>,
    children: Children,
) -> impl IntoView {
    let tree_injection = TreeInjection::expect_context();
    let subtree_injection = SubtreeInjection::expect_context();
    let value = StoredValue::new(value.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()));
    let subtree_ref = NodeRef::<html::Div>::new();
    let tree_open_items = tree_injection.open_items;

    let open = Memo::new(move |_| {
        tree_open_items.with(|open_items| value.with_value(|value| open_items.contains(value)))
    });

    let style = move || {
        let level = subtree_injection.level;
        format!("--thaw-tree-item--level: {level}")
    };

    let on_click = move |event: ev::MouseEvent| {
        let composed_path = event.composed_path();

        let is_event_from_subtree = move || {
            subtree_ref.with_untracked(|el| {
                if let Some(el) = el {
                    composed_path.includes(el, 0)
                } else {
                    false
                }
            })
        };

        if is_event_from_subtree() {
            return;
        }

        if event.default_prevented() || item_type == TreeItemType::Leaf {
            return;
        }

        tree_open_items.update(move |open_items| {
            let value = value.get_value();
            if open.get_untracked() {
                open_items.remove(&value);
            } else {
                open_items.insert(value);
            }
        });
    };

    view! {
        <div
            class=class_list![
                "thaw-tree-item", format!("thaw-tree-item--{}", item_type.as_str()), class
            ]
            role="treeitem"
            style=style
            on:click=on_click
        >
            <Provider value=TreeItemInjection {
                open,
                item_type,
                subtree_ref,
            }>{children()}</Provider>
        </div>
    }
}
