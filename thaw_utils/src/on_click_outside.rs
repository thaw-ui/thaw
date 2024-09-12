use crate::BoxCallback;
use leptos::{html::Div, prelude::*};

pub fn call_on_click_outside(element: NodeRef<Div>, on_click: BoxCallback) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let handle = window_event_listener(::leptos::ev::click, move |ev| {
            let Some(displayed_el) = element.get_untracked() else {
                return;
            };
            if ev.composed_path().includes(&displayed_el, 0) {
                return;
            }
            on_click();
        });
        on_cleanup(move || handle.remove());
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        let _ = element;
        let _ = on_click;
    }
}

pub fn call_on_click_outside_with_list(refs: Vec<NodeRef<Div>>, on_click: BoxCallback) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let handle = window_event_listener(::leptos::ev::click, move |ev| {
            let composed_path = ev.composed_path();
            if refs.iter().any(|r| {
                if let Some(el) = r.get_untracked() {
                    composed_path.includes(&el, 0)
                } else {
                    false
                }
            }) {
                return;
            }
            on_click();
        });
        on_cleanup(move || handle.remove());
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        let _ = refs;
        let _ = on_click;
    }
}
