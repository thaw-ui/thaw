use leptos::{html::Div, *};

pub fn call_on_click_outside(element: NodeRef<Div>, on_click: Callback<()>) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let handle = window_event_listener(ev::click, move |ev| {
            use leptos::wasm_bindgen::__rt::IntoJsResult;
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                let Some(dropdown_el) = element.get_untracked() else {
                    break;
                };
                if current_el == ***dropdown_el {
                    return;
                }
                el = current_el.parent_element();
            }
            on_click.call(());
        });
        on_cleanup(move || handle.remove());
    }
}
