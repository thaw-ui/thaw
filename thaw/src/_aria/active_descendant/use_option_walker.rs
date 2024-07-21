use leptos::{prelude::document, reactive_graph::owner::StoredValue};
use send_wrapper::SendWrapper;
use std::sync::Arc;
use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use web_sys::{HtmlElement, Node, NodeFilter, TreeWalker};

pub fn use_option_walker<MF>(match_option: MF) -> (Box<dyn Fn(&Node)>, OptionWalker)
where
    MF: Fn(HtmlElement) -> bool + 'static,
{
    let tree_walker = StoredValue::new(
        None::<(
            SendWrapper<TreeWalker>,
            SendWrapper<Closure<dyn Fn(Node) -> u32>>,
        )>,
    );
    let option_walker = OptionWalker(tree_walker);
    let match_option = Arc::new(match_option);
    let set_listbox = move |el: &Node| {
        let match_option = match_option.clone();
        let cb: Closure<dyn Fn(Node) -> u32> = Closure::new(move |node: Node| {
            if let Ok(html_element) = node.dyn_into() {
                if match_option(html_element) {
                    return 1u32;
                }
            }

            3u32
        });
        let mut node_filter = NodeFilter::new();
        node_filter.accept_node(cb.as_ref().unchecked_ref());

        let tw = document()
            .create_tree_walker_with_what_to_show_and_filter(el, 0x1, Some(&node_filter))
            .unwrap_throw();
        tree_walker.set_value(Some((SendWrapper::new(tw), SendWrapper::new(cb))));
    };

    (Box::new(set_listbox), option_walker)
}

#[derive(Clone)]
pub struct OptionWalker(
    StoredValue<
        Option<(
            SendWrapper<TreeWalker>,
            SendWrapper<Closure<dyn Fn(Node) -> u32>>,
        )>,
    >,
);

impl OptionWalker {
    pub fn first(&self) -> Option<HtmlElement> {
        self.0.with_value(|tree_walker| {
            let Some((tree_walker, _)) = tree_walker.as_ref() else {
                return None;
            };
            tree_walker.set_current_node(&tree_walker.root());
            tree_walker.first_child().unwrap_throw()?.dyn_into().ok()
        })
    }

    pub fn last(&self) -> Option<HtmlElement> {
        self.0.with_value(|tree_walker| {
            let Some((tree_walker, _)) = tree_walker.as_ref() else {
                return None;
            };
            tree_walker.set_current_node(&tree_walker.root());
            tree_walker.last_child().unwrap_throw()?.dyn_into().ok()
        })
    }

    pub fn next(&self) -> Option<HtmlElement> {
        self.0.with_value(|tree_walker| {
            let Some((tree_walker, _)) = tree_walker.as_ref() else {
                return None;
            };
            tree_walker.next_node().unwrap_throw()?.dyn_into().ok()
        })
    }

    pub fn prev(&self) -> Option<HtmlElement> {
        self.0.with_value(|tree_walker| {
            let Some((tree_walker, _)) = tree_walker.as_ref() else {
                return None;
            };
            tree_walker.previous_node().unwrap_throw()?.dyn_into().ok()
        })
    }
}
