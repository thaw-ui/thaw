use cfg_if::cfg_if;
use leptos::*;
/// https://github.com/solidjs/solid/blob/main/packages/solid/web/src/index.ts#L56
#[component]
pub fn Teleport(
    #[prop(into, optional)] mount: Option<web_sys::Element>,
    children: Children,
) -> impl IntoView {
    cfg_if! { if #[cfg(target_arch = "wasm32")] {
        use leptos::{
            wasm_bindgen::JsCast,
            leptos_dom::Mountable
        };
        let mount = mount.unwrap_or_else(|| {
            document()
                .body()
                .expect("body element not to exist")
                .unchecked_into()
        });
        let node = children().into_view();
        let node = node.get_mountable_node();
        _  = mount.append_child(&node);
        on_cleanup(move || {
            _ = mount.remove_child(&node);
        });
    } else {
        _ = mount;
        _ = children;
    }}
}
