use cfg_if::cfg_if;

pub fn mount_style(id: &str, content: &'static str) {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::{tachys::view::Render, view};
            use leptos_meta::Style;

            let _ = view! {
                <Style attr:data-thaw-id=id>
                    {content}
                </Style>
            };
        } else {
            use leptos::prelude::document;
            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style[data-thaw-id=\"{id}\"]"))
                .expect("query style element error");

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

pub fn mount_dynamic_style<T: Fn() -> String + Send + Sync + 'static>(id: String, f: T) {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::{tachys::view::Render, view};
            use leptos_meta::Style;

            let _ = view! {
                <Style attr:data-thaw-id=id>
                    {f()}
                </Style>
            };
        } else {
            use leptos::prelude::document;
            use send_wrapper::SendWrapper;

            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style[data-thaw-id=\"{id}\"]"))
                .expect("query style element error").unwrap_or_else(|| {
                    let style = document()
                        .create_element("style")
                        .expect("create style element error");
                    _ = style.set_attribute("data-thaw-id", &id);
                    _ = head.prepend_with_node_1(&style);

                    style
                });

            let style = SendWrapper::new(style);
            leptos::prelude::Effect::new_isomorphic(move |_| {
                let content = f();

                style.set_text_content(Some(&content));
            });
        }
    }
}
