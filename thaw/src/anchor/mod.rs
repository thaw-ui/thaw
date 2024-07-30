mod anchor_link;

pub use anchor_link::AnchorLink;

use leptos::{context::Provider, html, prelude::*};
use thaw_utils::{class_list, mount_style};
use web_sys::{DomRect, Element};

#[component]
pub fn Anchor(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The element or selector used to calc offset of link elements.
    /// If you are not scrolling the entire document but only a part of it, you may need to set this.
    #[prop(into, optional)]
    offset_target: Option<OffsetTarget>,
    children: Children,
) -> impl IntoView {
    mount_style("anchor", include_str!("./anchor.css"));
    let anchor_ref = NodeRef::new();
    let bar_ref = NodeRef::new();
    let element_ids = RwSignal::new(Vec::<String>::new());
    let active_id = RwSignal::new(None::<String>);

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::ev;
        use std::cmp::Ordering;
        use thaw_utils::{add_event_listener_with_bool, throttle};

        struct LinkInfo {
            top: f64,
            id: String,
        }

        impl OffsetTarget {
            fn get_bounding_client_rect(&self) -> Option<DomRect> {
                match self {
                    OffsetTarget::Selector(selector) => {
                        let el = document().query_selector(selector).ok().flatten()?;
                        Some(el.get_bounding_client_rect())
                    }
                    OffsetTarget::Element(el) => Some(el.get_bounding_client_rect()),
                }
            }
        }

        let offset_target = send_wrapper::SendWrapper::new(offset_target);

        let on_scroll = move || {
            element_ids.with(|ids| {
                let offset_target_top = if let Some(offset_target) = offset_target.as_ref() {
                    if let Some(rect) = offset_target.get_bounding_client_rect() {
                        rect.top()
                    } else {
                        return;
                    }
                } else {
                    0.0
                };

                let mut links: Vec<LinkInfo> = vec![];
                for id in ids.iter() {
                    if let Some(link_el) = document().get_element_by_id(id) {
                        let link_rect = link_el.get_bounding_client_rect();
                        links.push(LinkInfo {
                            top: link_rect.top() - offset_target_top,
                            id: id.clone(),
                        });
                    }
                }
                links.sort_by(|a, b| {
                    if a.top > b.top {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                });

                let mut temp_link = None::<LinkInfo>;
                for link in links.into_iter() {
                    if link.top >= 0.0 {
                        if link.top <= 12.0 {
                            temp_link = Some(link);
                            break;
                        } else if temp_link.is_some() {
                            break;
                        } else {
                            temp_link = None;
                        }
                    } else {
                        temp_link = Some(link);
                    }
                }
                active_id.set(temp_link.map(|link| link.id));
            });
        };
        let cb = throttle(
            move || {
                on_scroll();
            },
            std::time::Duration::from_millis(200),
        );
        let scroll_handle = add_event_listener_with_bool(
            document(),
            ev::scroll,
            move |_| {
                cb();
            },
            true,
        );
        on_cleanup(move || {
            scroll_handle.remove();
        });
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        let _ = offset_target;
    }

    view! {
        <div
            class=class_list!["thaw-anchor", class]
            node_ref=anchor_ref
        >
            <div class="thaw-anchor-rail">
                <div
                    class="thaw-anchor-rail__bar"
                    class=(
                        "thaw-anchor-rail__bar--active",
                        move || active_id.with(|id| id.is_some()),
                    )

                    node_ref=bar_ref
                ></div>
            </div>
            <Provider value=AnchorInjection::new(
                anchor_ref,
                bar_ref,
                element_ids,
                active_id,
            )>{children()}</Provider>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct AnchorInjection {
    anchor_ref: NodeRef<html::Div>,
    bar_ref: NodeRef<html::Div>,
    element_ids: RwSignal<Vec<String>>,
    pub active_id: RwSignal<Option<String>>,
}

impl Copy for AnchorInjection {}

impl AnchorInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    fn new(
        anchor_ref: NodeRef<html::Div>,
        bar_ref: NodeRef<html::Div>,
        element_ids: RwSignal<Vec<String>>,
        active_id: RwSignal<Option<String>>,
    ) -> Self {
        Self {
            anchor_ref,
            bar_ref,
            element_ids,
            active_id,
        }
    }

    pub fn scroll_into_view(&self, id: &String) {
        let Some(link_el) = document().get_element_by_id(id) else {
            return;
        };
        link_el.scroll_into_view();
    }

    pub fn append_id(&self, id: String) {
        self.element_ids.update(|ids| {
            ids.push(id);
        });
    }

    pub fn remove_id(&self, id: &String) {
        self.element_ids.update(|ids| {
            if let Some(index) = ids.iter().position(|item_id| item_id == id) {
                ids.remove(index);
            }
        });
    }

    pub fn update_background_position(&self, title_rect: DomRect) {
        if let Some(anchor_el) = self.anchor_ref.get_untracked() {
            let bar_el = self.bar_ref.get_untracked().unwrap();
            let anchor_rect = anchor_el.get_bounding_client_rect();

            let offset_top = title_rect.top() - anchor_rect.top();
            // let offset_left = title_rect.left() - anchor_rect.left();

            bar_el.style(("top", format!("{}px", offset_top)));
            bar_el.style(("height", format!("{}px", title_rect.height())));
        }
    }
}

pub enum OffsetTarget {
    Selector(String),
    Element(Element),
}



impl From<&'static str> for OffsetTarget {
    fn from(value: &'static str) -> Self {
        Self::Selector(value.to_string())
    }
}

impl From<String> for OffsetTarget {
    fn from(value: String) -> Self {
        Self::Selector(value)
    }
}

impl From<Element> for OffsetTarget {
    fn from(value: Element) -> Self {
        Self::Element(value)
    }
}
