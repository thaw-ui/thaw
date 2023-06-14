use crate::utils::mount_style::mount_style;
use leptos::*;
use std::time::Duration;
use stylers::style_sheet_str;
use web_sys::Element;

pub struct ToastOptions {
    pub message: String,
    pub duration: Duration,
}

pub fn show_toast(cx: Scope, options: ToastOptions) {
    let class_name = mount_style("toast", || style_sheet_str!("./src/mobile/toast/toast.css"));

    let parent = Element::from(document().body().expect("body element not to exist"));
    let children = view! {cx, class=class_name,
        <div class="melt-toast">
            { options.message }
        </div>
    };
    let node = children.into_view(cx);

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
}
