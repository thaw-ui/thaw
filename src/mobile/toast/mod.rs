use crate::utils::mount_style::mount_style;
use cfg_if::cfg_if;
use std::time::Duration;

pub struct ToastOptions {
    pub message: String,
    pub duration: Duration,
}

pub fn show_toast(options: ToastOptions) {
    mount_style("toast", include_str!("./toast.css"));
    cfg_if! { if #[cfg(target_arch = "wasm32")] {
        use leptos::{leptos_dom::Mountable, *};
        let mount = document().body().expect("body element not to exist");
        let children = view! { <div class="thaw-toast">{options.message}</div> };
        let node = children.into_view();
        let node = node.get_mountable_node();
        _  = mount.append_child(&node);
        set_timeout(
            move || {
                _ = mount.remove_child(&node);
            },
            options.duration,
        );
    } else {
        _ = options;
    }}
}
