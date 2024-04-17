mod anchor_link;
mod theme;

pub use anchor_link::AnchorLink;
pub use theme::AnchorTheme;

use leptos::*;
use thaw_utils::{add_event_listener_with_bool, mount_style, throttle};

use crate::{use_theme, Theme};

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
    let on_scroll = move || {
        element_ids.with(|ids| {
            let offset_target_top = offset_target
                .as_ref()
                .map_or(0.0, |el| el.get_bounding_client_rect().top());

            let links: Vec<(f64, f64, f64)> = vec![];
            for id in ids.iter() {
                if let Some(link_el) = document().get_element_by_id(id) {
                    let link_rect = link_el.get_bounding_client_rect();
                    let top = link_rect.top() - offset_target_top;
                    let height = link_rect.height();
                }
            }
            links
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
            <Provider value=AnchorInjection(element_ids)>
                {children()}
            </Provider>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct AnchorInjection(pub RwSignal<Vec<String>>);

pub(crate) fn use_anchor() -> AnchorInjection {
    expect_context()
}
