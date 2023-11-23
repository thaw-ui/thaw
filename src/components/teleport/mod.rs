use cfg_if::cfg_if;
use leptos::{html::AnyElement, *};

/// https://github.com/solidjs/solid/blob/main/packages/solid/web/src/index.ts#L56
#[component]
pub fn Teleport(
    #[prop(into, optional)] mount: Option<web_sys::Element>,
    #[prop(optional, into)] element: Option<HtmlElement<AnyElement>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    cfg_if! { if #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))] {
        use leptos::wasm_bindgen::JsCast;
        use leptos::leptos_dom::Mountable;
        use crate::utils::with_hydration_off;

        let mount = mount.unwrap_or_else(|| {
            document()
                .body()
                .expect("body element not to exist")
                .unchecked_into()
        });

        if let Some(element) = element {
            let render_root = element;
            let _  = mount.append_child(&render_root);
            on_cleanup(move || {
                let _ = mount.remove_child(&render_root);
            });
        } else if let Some(children) =  children {
            let container = document()
                .create_element("div")
                .expect("element creation to work");
            with_hydration_off(|| {
                let _ = container.append_child(&children().into_view().get_mountable_node());
            });

            let render_root = container;
            let _  = mount.append_child(&render_root);
            on_cleanup(move || {
                let _ = mount.remove_child(&render_root);
            });
        } else {
            return;
        };
    } else {
        let _ = mount;
        #[cfg(not(feature = "ssr"))]
        {
            let _ = element;
            let _ = children;
        }
        #[cfg(feature = "ssr")]
        if element.is_none() {
            if let Some(children) =  children {
                // Consumed hydration `id`
                let _  = children();
            }
        }
    }}
}
