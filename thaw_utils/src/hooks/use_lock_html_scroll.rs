use leptos::reactive_graph::wrappers::read::MaybeSignal;

pub fn use_lock_html_scroll(is_lock: MaybeSignal<bool>) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        // use leptos::{create_render_effect, document, on_cleanup, SignalGet, StoredValue};
        use leptos::prelude::{
            document, effect::RenderEffect, on_cleanup, traits::Get, StoredValue,
        };
        use send_wrapper::SendWrapper;

        let style_el = StoredValue::new(SendWrapper::new(None::<web_sys::Element>));
        let remove_style_el = move || {
            style_el.update_value(move |el| {
                if let Some(el) = Option::take(el) {
                    let head = document().head().expect("head no exist");
                    _ = head.remove_child(&el);
                }
            });
        };

        RenderEffect::new(move |_| {
            if is_lock.get() {
                let head = document().head().expect("head no exist");
                let style = document()
                    .create_element("style")
                    .expect("create style element error");
                _ = style.set_attribute("data-id", &format!("thaw-lock-html-scroll"));
                style.set_text_content(Some("html { overflow: hidden; }"));
                _ = head.append_child(&style);
                style_el.update_value(move |el| {
                    *el = SendWrapper::new(Some(style));
                });
            } else {
                remove_style_el();
            }
        });

        on_cleanup(remove_style_el)
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        _ = is_lock;
    }
}
