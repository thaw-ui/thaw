use cfg_if::cfg_if;

pub fn mount_style(id: &str, content: &'static str) {
    let id = format!("thaw-id-{id}");
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::view;
            use leptos_meta::Style;
            use super::SSRMountStyleContext;

            if let Some(context) = SSRMountStyleContext::use_context() {
                context.push_style(id, content.to_string());
                return;
            }

            let _ = view! {
                <Style id=id>
                    {content}
                </Style>
            };
        } else {
            use leptos::prelude::document;
            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style#{id}"))
                .expect("query style element error");

            if style.is_some() {
                return;
            }

            let style = document()
                .create_element("style")
                .expect("create style element error");
            _ = style.set_attribute("id", &id);
            style.set_text_content(Some(content));
            _ = head.prepend_with_node_1(&style);
        }
    }
}

pub fn mount_dynamic_style<T: Fn() -> String + Send + Sync + 'static>(id: String, f: T) {
    let id = format!("thaw-id-{id}");
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::{view, prelude::untrack};
            use leptos_meta::Style;
            use super::SSRMountStyleContext;

            if let Some(context) = SSRMountStyleContext::use_context() {
                context.push_style(id, untrack(f));
                return;
            }

            let _ = view! {
                <Style id=id>
                    {f()}
                </Style>
            };
        } else {
            use leptos::prelude::document;
            use send_wrapper::SendWrapper;

            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style#{id}"))
                .expect("query style element error").unwrap_or_else(|| {
                    let style = document()
                        .create_element("style")
                        .expect("create style element error");
                    _ = style.set_attribute("id", &id);
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
