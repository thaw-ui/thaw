mod anchor_link;

pub use anchor_link::AnchorLink;

use leptos::*;
use thaw_utils::{add_event_listener_with_bool, mount_style, throttle};

#[component]
pub fn Anchor(children: Children) -> impl IntoView {
    mount_style("anchor", include_str!("./anchor.css"));
    let on_scroll = move || {};
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
        <div class="thaw-anchor">
            <div class="thaw-anchor-rail">
            </div>
            <div class="thaw-anchor-background">
            </div>
            {children()}
        </div>
    }
}
