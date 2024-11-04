use cfg_if::cfg_if;

pub fn mount_style_provider() {
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::context::{provide_context, use_context};
            if use_context::<MountStyleInjection>().is_none() {
                provide_context(MountStyleInjection::default());
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::collections::HashSet;
        use leptos::prelude::{StoredValue, expect_context, UpdateValue};

        #[derive(Clone, Copy, Default)]
        struct MountStyleInjection(StoredValue<HashSet<String>>);

        impl MountStyleInjection {
            fn expect_context() -> Self {
                expect_context()
            }

            fn update(&self, id: &String) -> bool {
                self.0
                    .try_update_value(|set| {
                        if set.contains(id) {
                            false
                        } else {
                            set.insert(id.to_string());
                            true
                        }
                    })
                    .unwrap_or_default()
            }
        }
    }
}

pub fn mount_style(id: &str, content: &'static str) {
    let id = format!("thaw-id-{id}");
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            use leptos::view;
            use leptos_meta::Style;

            let set = MountStyleInjection::expect_context();
            if !set.update(&id) {
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
            use leptos::view;
            use leptos_meta::Style;

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
