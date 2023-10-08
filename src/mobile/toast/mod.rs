use crate::utils::mount_style::mount_style;
use leptos::*;
use std::time::Duration;
use web_sys::Element;

pub struct ToastOptions {
    pub message: String,
    pub duration: Duration,
}

pub fn show_toast(options: ToastOptions) {
    mount_style("toast", include_str!("./toast.css"));

    let parent = Element::from(document().body().expect("body element not to exist"));
    let children = view! { <div class="melt-toast">{options.message}</div> };
    let node = children.into_view();

    #[cfg(all(target_arch = "wasm32"))]
    {
        use leptos_dom::Mountable;
        let node = node.get_mountable_node();
        parent.append_child(&node).unwrap();
        set_timeout(
            move || {
                _ = parent.remove_child(&node);
            },
            options.duration,
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        _ = parent;
        _ = node;
    }
}
