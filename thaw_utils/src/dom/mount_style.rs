use cfg_if::cfg_if;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
mod context {
    use leptos::prelude::{
        expect_context, provide_context, use_context, StoredValue, UpdateValue,
    };
    use std::collections::HashSet;

    pub fn provide_mount_style_context() {
        if use_context::<MountStyleContext>().is_none() {
            provide_context(MountStyleContext::default());
        }
    }

    #[derive(Clone, Default)]
    pub struct MountStyleContext {
        hash: StoredValue<HashSet<String>>,
        #[cfg(feature = "hydrate")]
        styles: StoredValue<Vec<send_wrapper::SendWrapper<leptos::prelude::AnyView>>>,
    }

    impl MountStyleContext {
        fn update(&self, id: &String) -> bool {
            self.hash
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

        #[cfg(feature = "ssr")]
        pub fn ssr_mount_style(id: String, content: String) {
            use leptos::view;
            use leptos_meta::Style;

            let context: Self = expect_context();
            if !context.update(&id) {
                return;
            }

            let _ = view! {
                <Style id=id>
                    {content}
                </Style>
            };
        }

        #[cfg(feature = "hydrate")]
        fn insert_style(&self, style: leptos::prelude::AnyView) {
            self.styles.update_value(|styles| {
                styles.push(send_wrapper::SendWrapper::new(style));
            });
        }

        #[cfg(feature = "hydrate")]
        pub fn hydrate_mount_style(id: &String) {
            use leptos::{prelude::{IntoAny, Owner}, view};
            use leptos_meta::Style;

            let shared_context = Owner::current_shared_context();
            if shared_context
                .map(|sc| sc.get_is_hydrating())
                .unwrap_or_default()
            {
                let context: Self = expect_context();
                if context.update(id) {
                    let style = view! {
                        <Style></Style>
                    };
                    context.insert_style(style.into_any());
                }
            }
        }
    }
}

#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub use context::provide_mount_style_context;

pub fn mount_style(id: &str, content: &'static str) {
    let id = format!("thaw-id-{id}");
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            context::MountStyleContext::ssr_mount_style(id, content.to_string());
        } else {
            use leptos::prelude::document;

            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style#{id}"))
                .expect("query style element error");

            if style.is_some() {
                #[cfg(feature = "hydrate")]
                context::MountStyleContext::hydrate_mount_style(&id);
                return;
            }

            let style = document()
                .create_element("style")
                .expect("create style element error");
            let _ = style.set_attribute("id", &id);
            style.set_text_content(Some(content));
            let _ = head.prepend_with_node_1(&style);
        }
    }
}

pub fn mount_dynamic_style<T: Fn() -> String + Send + Sync + 'static>(id: String, f: T) {
    let id = format!("thaw-id-{id}");
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let content = f();
            context::MountStyleContext::ssr_mount_style(id, content);
        } else {
            use leptos::prelude::{document, Effect};
            use send_wrapper::SendWrapper;

            let head = document().head().expect("head no exist");
            let style = head
                .query_selector(&format!("style#{id}"))
                .expect("query style element error");

            #[cfg(feature = "hydrate")]
            if style.is_some() {
                context::MountStyleContext::hydrate_mount_style(&id);
            }

            let style = style.unwrap_or_else(|| {
                let style = document()
                    .create_element("style")
                    .expect("create style element error");
                let _ = style.set_attribute("id", &id);
                let _ = head.prepend_with_node_1(&style);
                style
            });
            let style = SendWrapper::new(style);
            Effect::new_isomorphic(move |_| {
                let content = f();
                style.set_text_content(Some(&content));
            });
        }
    }
}
