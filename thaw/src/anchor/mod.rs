mod anchor_link;
mod theme;

pub use anchor_link::AnchorLink;
pub use theme::AnchorTheme;

use crate::{use_theme, Theme};
use leptos::*;
use std::cmp::Ordering;
use thaw_utils::{add_event_listener_with_bool, class_list, mount_style, throttle, OptionalProp};
use web_sys::{DomRect, Element};

#[component]
pub fn Anchor(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into, optional)] offset_target: Option<OffsetTarget>,
    children: Children,
) -> impl IntoView {
    mount_style("anchor", include_str!("./anchor.css"));
    let theme = use_theme(Theme::light);
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-rail-background-color: {};",
                theme.anchor.rail_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-title-color-hover: {};",
                theme.common.color_primary_hover
            ));
            css_vars.push_str(&format!(
                "--thaw-title-color-active: {};",
                theme.common.color_primary
            ));
            css_vars.push_str(&format!(
                "--thaw-link-background-color: {}1a;",
                theme.common.color_primary
            ));
        });
        css_vars
    });
    let anchor_ref = NodeRef::new();
    let background_ref = NodeRef::<html::Div>::new();
    let bar_ref = NodeRef::new();
    let element_ids = RwSignal::new(Vec::<String>::new());
    let active_id = RwSignal::new(None::<String>);

    let _ = watch(
        move || active_id.get(),
        move |id, _, _| {
            if id.is_none() {
                if let Some(background_el) = background_ref.get_untracked() {
                    let _ = background_el.style("max-width", "0");
                }
            }
        },
        false,
    );
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
    view! {
        <div
            class=class_list!["thaw-anchor", class.map(| c | move || c.get())]
            ref=anchor_ref
            style=move || css_vars.get()
        >
            <div class="thaw-anchor-rail">
                <div
                    class="thaw-anchor-rail__bar"
                    class=(
                        "thaw-anchor-rail__bar--active",
                        move || active_id.with(|id| id.is_some()),
                    )

                    ref=bar_ref
                ></div>
            </div>
            <div class="thaw-anchor-background" ref=background_ref></div>
            <Provider value=AnchorInjection::new(
                anchor_ref,
                background_ref,
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
    background_ref: NodeRef<html::Div>,
    bar_ref: NodeRef<html::Div>,
    element_ids: RwSignal<Vec<String>>,
    pub active_id: RwSignal<Option<String>>,
}

impl Copy for AnchorInjection {}

impl AnchorInjection {
    fn new(
        anchor_ref: NodeRef<html::Div>,
        background_ref: NodeRef<html::Div>,
        bar_ref: NodeRef<html::Div>,
        element_ids: RwSignal<Vec<String>>,
        active_id: RwSignal<Option<String>>,
    ) -> Self {
        Self {
            anchor_ref,
            background_ref,
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
            let background_el = self.background_ref.get_untracked().unwrap();
            let bar_el = self.bar_ref.get_untracked().unwrap();
            let anchor_rect = anchor_el.get_bounding_client_rect();

            let offset_top = title_rect.top() - anchor_rect.top();
            let offset_left = title_rect.left() - anchor_rect.left();
            let _ = background_el
                .style("top", format!("{}px", offset_top))
                .style("height", format!("{}px", title_rect.height()))
                .style(
                    "max-width",
                    format!("{}px", title_rect.width() + offset_left),
                );
            let _ = bar_el
                .style("top", format!("{}px", offset_top))
                .style("height", format!("{}px", title_rect.height()));
        }
    }
}

pub(crate) fn use_anchor() -> AnchorInjection {
    expect_context()
}

struct LinkInfo {
    top: f64,
    id: String,
}

pub enum OffsetTarget {
    Selector(String),
    Element(Element),
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
