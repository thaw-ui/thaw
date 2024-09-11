use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

pub fn get_scroll_parent_node(node: &Node) -> Option<Node> {
    let parent_node = node.parent_node()?;

    let node_type = parent_node.node_type();
    if node_type == Node::ELEMENT_NODE {
        let el = parent_node.clone().dyn_into::<Element>().unwrap();
        if let Some((overflow, overflow_x, overflow_y)) = get_overflow(&el) {
            let overflow = format!("{overflow}{overflow_x}{overflow_y}");
            if overflow.contains("auto") {
                return Some(parent_node);
            }
            if overflow.contains("scroll") {
                return Some(parent_node);
            }
            if overflow.contains("overlay") {
                return Some(parent_node);
            }
        }
    } else if node_type == Node::DOCUMENT_NODE {
        return Some(document().into());
    }

    get_scroll_parent_node(&parent_node)
}

pub fn get_scroll_parent_element(element: &Element) -> Option<Element> {
    let parent_element = element.parent_element()?;

    if let Some((overflow, overflow_x, overflow_y)) = get_overflow(&parent_element) {
        let overflow = format!("{overflow}{overflow_x}{overflow_y}");
        if overflow.contains("auto") {
            return Some(parent_element);
        }
        if overflow.contains("scroll") {
            return Some(parent_element);
        }
        if overflow.contains("overlay") {
            return Some(parent_element);
        }
    }

    get_scroll_parent_element(&parent_element)
}

fn get_overflow(parent_element: &Element) -> Option<(String, String, String)> {
    let Ok(Some(css_style_declaration)) = window().get_computed_style(parent_element) else {
        return None;
    };
    let Ok(overflow) = css_style_declaration.get_property_value("overflow") else {
        return None;
    };
    let Ok(overflow_x) = css_style_declaration.get_property_value("overflowX") else {
        return None;
    };
    let Ok(overflow_y) = css_style_declaration.get_property_value("overflowY") else {
        return None;
    };
    Some((overflow, overflow_x, overflow_y))
}
