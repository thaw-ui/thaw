use leptos::*;
use web_sys::Element;

/// https://github.com/solidjs/solid/blob/main/packages/solid/web/src/index.ts#L56
#[component]
pub fn Teleport(#[prop(optional)] to: Option<&'static str>, children: Children) -> impl IntoView {
    let parent = if let Some(to) = to {
        document()
            .query_selector(to)
            .expect("element not to exist")
            .expect("element not to exist")
    } else {
        Element::from(document().body().expect("body element not to exist"))
    };

    #[cfg(all(target_arch = "wasm32"))]
    {
        use leptos::leptos_dom::Mountable;
        let node = children().into_view();
        let node = node.get_mountable_node();
        parent.append_child(&node).unwrap();
        on_cleanup(move || {
            _ = parent.remove_child(&node);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        _ = parent;
        _ = children;
    }

    view! { <></> }
}
