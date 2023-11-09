use crate::teleport::Teleport;
use leptos::{
    html::{AnyElement, ToHtmlElement},
    *,
};

#[slot]
pub struct Follower {
    show: MaybeSignal<bool>,
    children: Children,
}

#[component]
pub fn Binder(
    target: NodeRef<AnyElement>,
    follower: Follower,
    children: Children,
) -> impl IntoView {
    let scrollable_element_vec = store_value::<Vec<HtmlElement<AnyElement>>>(vec![]);
    let ensure_scroll_listener = move || {
        let mut cursor = target.get_untracked();
        loop {
            cursor = get_scroll_parent(cursor);
            if let Some(cursor) = cursor.take() {
                scrollable_element_vec.update_value(|vec| vec.push(cursor));
            } else {
                break;
            }
        }
        scrollable_element_vec.with_value(|vec| {
            vec.iter().for_each(|ele| {
                _ = ele.clone().on(ev::scroll, move |_| {});
            })
        });
    };

    view! {
        {children()}
        <Teleport>
            <div class="thaw-binder-follower-container">
                <div class="thaw-binder-follower-content">
                    {(follower.children)()}
                </div>
            </div>
        </Teleport>
    }
}

fn get_scroll_parent(element: Option<HtmlElement<AnyElement>>) -> Option<HtmlElement<AnyElement>> {
    let Some(element) = element else {
        return None;
    };

    fn get_parent_element(element: HtmlElement<AnyElement>) -> Option<HtmlElement<AnyElement>> {
        if element.node_type() == 9 {
            None
        } else {
            element.parent_element().map(|ele| ele.to_leptos_element())
        }
    }
    let Some(parent_element) = get_parent_element(element) else {
        return None;
    };

    if parent_element.node_type() == 9 {
        return Some(parent_element);
    }

    if parent_element.node_type() == 1 {
        fn get_overflow(
            parent_element: &HtmlElement<AnyElement>,
        ) -> Option<(String, String, String)> {
            let Ok(Some(css_style_declaration)) = window().get_computed_style(parent_element)
            else {
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

    get_scroll_parent(Some(parent_element))
}
