use leptos::prelude::Signal;

pub fn use_lock_html_scroll(is_lock: Signal<bool>) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::prelude::{document, on_cleanup, Get, RenderEffect, StoredValue, UpdateValue};
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
                _ = style.set_attribute("data-id", "thaw-lock-html-scroll");
                style.set_text_content(Some(&lock_scroll_css()));
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

/// Build the CSS rule that locks scrolling and compensates for the scrollbar.
///
/// When `overflow: hidden` removes the scrollbar, the viewport widens by
/// the scrollbar's width, causing `margin: auto` centered content to shift.
/// Adding `padding-right` equal to the scrollbar width keeps the layout stable.
#[cfg(any(feature = "csr", feature = "hydrate"))]
fn lock_scroll_css() -> String {
    use leptos::prelude::document;

    let scrollbar_width = document()
        .document_element()
        .map(|html| {
            let client_w = html.client_width();
            let window_w = web_sys::window()
                .and_then(|w| w.inner_width().ok())
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as i32;
            window_w - client_w
        })
        .unwrap_or(0);

    if scrollbar_width > 0 {
        format!("html {{ overflow: hidden; padding-right: {scrollbar_width}px; }}")
    } else {
        "html { overflow: hidden; }".to_string()
    }
}
