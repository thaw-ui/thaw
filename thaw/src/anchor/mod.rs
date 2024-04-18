mod anchor_link;
mod theme;

pub use anchor_link::AnchorLink;
pub use theme::AnchorTheme;

use crate::{use_theme, Theme};
use leptos::*;
use std::cmp::Ordering;
use thaw_utils::{add_event_listener_with_bool, mount_style, throttle};

#[component]
pub fn Anchor(
    #[prop(into, optional)] offset_target: Option<web_sys::Element>,
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
            ))
        });
        css_vars
    });
    let element_ids = RwSignal::new(Vec::<String>::new());
    let active_id = RwSignal::new(None::<String>);
    let on_scroll = move || {
        element_ids.with(|ids| {
            let offset_target_top = offset_target
                .as_ref()
                .map_or(0.0, |el| el.get_bounding_client_rect().top());

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
        <div class="thaw-anchor" style=move || css_vars.get()>
            <div class="thaw-anchor-rail">
            </div>
            <div class="thaw-anchor-background">
            </div>
            <Provider value=AnchorInjection::new(element_ids, active_id)>
                {children()}
            </Provider>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct AnchorInjection {
    element_ids: RwSignal<Vec<String>>,
    pub active_id: RwSignal<Option<String>>,
}

impl Copy for AnchorInjection {}

impl AnchorInjection {
    fn new(element_ids: RwSignal<Vec<String>>, active_id: RwSignal<Option<String>>) -> Self {
        Self {
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
}

pub(crate) fn use_anchor() -> AnchorInjection {
    expect_context()
}

struct LinkInfo {
    top: f64,
    id: String,
}
