use leptos::{
    html::{AnyElement, ToHtmlElement},
    *,
};

pub fn get_scroll_parent(element: &HtmlElement<AnyElement>) -> Option<HtmlElement<AnyElement>> {
    let parent_element = get_parent_element(element)?;

    if parent_element.node_type() == 9 {
        return Some(parent_element);
    }

    if parent_element.node_type() == 1 {
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
    }

    get_scroll_parent(&parent_element)
}

fn get_parent_element(element: &HtmlElement<AnyElement>) -> Option<HtmlElement<AnyElement>> {
    if element.node_type() == 9 {
        None
    } else {
        element.parent_element().map(|ele| ele.to_leptos_element())
    }
}

fn get_overflow(parent_element: &HtmlElement<AnyElement>) -> Option<(String, String, String)> {
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
