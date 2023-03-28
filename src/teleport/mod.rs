use leptos::*;
use web_sys::Element;

/// https://github.com/solidjs/solid/blob/main/packages/solid/web/src/index.ts#L56
#[component]
pub fn Teleport(cx: Scope, #[prop(optional)] to: Option<String>, children: Children) -> impl IntoView {
    let parent = if let Some(to) = to {
        document().query_selector(to.as_str()).expect("element not to exist").expect("element not to exist")
    } else {
        Element::from(document().body().expect("body element not to exist"))
    };

    #[cfg(all(target_arch = "wasm32"))]
    {
        use leptos_dom::Mountable;
        let node = children(cx).into_view(cx);
        parent.append_child(&node.get_mountable_node()).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        _ = cx;
        _ = parent;
        _ = children;
    }
    
    view! {
        cx,
        <></>
    }
}