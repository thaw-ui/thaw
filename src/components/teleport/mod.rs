use cfg_if::cfg_if;
use leptos::{html::AnyElement, *};
/// https://github.com/solidjs/solid/blob/main/packages/solid/web/src/index.ts#L56
#[component]
pub fn Teleport(
    #[prop(into, optional)] mount: Option<web_sys::Element>,
    #[prop(optional, into)] element: Option<HtmlElement<AnyElement>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    cfg_if! { if #[cfg(target_arch = "wasm32")] {
        use leptos::wasm_bindgen::JsCast;
        let mount = mount.unwrap_or_else(|| {
            document()
                .body()
                .expect("body element not to exist")
                .unchecked_into()
        });

        let render_root = if let Some(element) = element {
            element
        } else if let Some(children) =  children {
            html::div().child(children()).into_any()
        } else {
            return;
        };

        _  = mount.append_child(&render_root);
        on_cleanup(move || {
            _ = mount.remove_child(&render_root);
        });
    } else {
        _ = mount;
        _ = element;
        _ = children;
    }}
}
