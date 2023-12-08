use cfg_if::cfg_if;

use web_sys::js_sys::Array;

pub fn mount_style(id: &str, content: &'static str) {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::html::style;
            use leptos_meta::use_head;
            let meta = use_head();
            let style_el = style().attr("csr-id", format!("thaw-{id}")).child(content);
            meta.tags.register(format!("leptos-thaw-{id}").into(), style_el.into_any());
        } else {
            use leptos::document;
            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style[csr-id=\"thaw-{id}\"]"))
                .expect("query style element error");

            #[cfg(feature = "hydrate")]
            let _ = leptos::leptos_dom::HydrationCtx::id();

            if style.is_some() {
                return;
            }

            let style = document()
                .create_element("style")
                .expect("create style element error");
            _ = style.set_attribute("csr-id", &format!("thaw-{id}"));
            style.set_text_content(Some(content));
            _ = head.prepend_with_node_1(&style);
        }
    }
}
