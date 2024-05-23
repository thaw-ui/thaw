use cfg_if::cfg_if;

pub fn mount_style(id: &str, content: &'static str) {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::html::style;
            use leptos_meta::use_head;
            let meta = use_head();
            let style_el = style().attr("data-thaw-id", id).child(content);
            meta.tags.register(format!("leptos-thaw-{id}").into(), style_el.into_any());
        } else {
            use leptos::document;
            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style[data-thaw-id=\"{id}\"]"))
                .expect("query style element error");

            #[cfg(feature = "hydrate")]
            let _ = leptos::leptos_dom::HydrationCtx::id();

            if style.is_some() {
                return;
            }

            let style = document()
                .create_element("style")
                .expect("create style element error");
            _ = style.set_attribute("data-thaw-id", id);
            style.set_text_content(Some(content));
            _ = head.prepend_with_node_1(&style);
        }
    }
}

pub fn mount_dynamic_style<T: Fn() -> String + 'static>(id: String, f: T) {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::html::style;
            use leptos_meta::use_head;
            let meta = use_head();
            let content = leptos::untrack(|| f());
            let style_el = style().attr("data-thaw-id", id).child(content);
            meta.tags.register(format!("leptos-thaw-{id}").into(), style_el.into_any());
        } else {
            use leptos::document;
            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style[data-thaw-id=\"{id}\"]"))
                .expect("query style element error");

            #[cfg(feature = "hydrate")]
            let _ = leptos::leptos_dom::HydrationCtx::id();


            leptos::Effect::new_isomorphic(move |prev: Option<Option<web_sys::Element>>| {
                let content = f();

                if let Some(style) = style.as_ref() {
                    style.set_text_content(Some(&content));
                    None
                } else  if let Some(style) = prev.flatten() {
                    style.set_text_content(Some(&content));
                    Some(style)
                } else {
                    let style = document()
                        .create_element("style")
                        .expect("create style element error");
                    _ = style.set_attribute("data-thaw-id", &id);
                    style.set_text_content(Some(&content));
                    _ = head.prepend_with_node_1(&style);

                    Some(style)
                }
            });
        }
    }
}
