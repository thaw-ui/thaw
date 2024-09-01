use leptos::reactive_graph::wrappers::read::MaybeSignal;

pub fn use_lock_html_scroll(is_lock: MaybeSignal<bool>) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::prelude::{document, on_cleanup, Get, RenderEffect, StoredValue};
        use send_wrapper::SendWrapper;

        let style_el = StoredValue::new(SendWrapper::new(None::<web_sys::Element>));
        let remove_style_el = move || {
            style_el.update_value(move |el| {
                if let Some(el) = Option::take(el) {
                    el.remove();
                }
            });
        };

        let effect = RenderEffect::new(move |prev| {
            let is_lock = is_lock.get();
            let prev: bool = prev.unwrap_or_default();

            if is_lock && !prev {
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
            } else if !is_lock && prev {
                remove_style_el();
            }

            is_lock
        });

        on_cleanup(move || {
            drop(effect);
            remove_style_el();
        });
    }

    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        _ = is_lock;
    }
}
