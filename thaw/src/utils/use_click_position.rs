use leptos::{ReadSignal, RwSignal};

pub fn use_click_position() -> ReadSignal<Option<(i32, i32)>> {
    let mouse_position = RwSignal::new(None);
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::{ev, on_cleanup, window_event_listener, SignalSet};
        use wasm_bindgen::JsCast;
        use web_sys::MouseEvent;

        fn click_handler(event: MouseEvent) -> Option<(i32, i32)> {
            if event.client_x() > 0 || event.client_y() > 0 {
                return Some((event.client_x(), event.client_y()));
            }
            let Some(target) = event.target() else {
                return None;
            };

            let Ok(target) = target.dyn_into::<web_sys::Element>() else {
                return None;
            };
            let rect = target.get_bounding_client_rect();
            let left = rect.left() as i32;
            let top = rect.top() as i32;
            let width = rect.width() as i32;
            let height = rect.height() as i32;
            if left > 0 || top > 0 {
                Some((left + width / 2, top + height / 2))
            } else {
                Some((0, 0))
            }
        }
        let handle = window_event_listener(ev::click, move |event| {
            let position = click_handler(event);
            mouse_position.set(position);
        });
        on_cleanup(move || {
            handle.remove();
        });
    }

    mouse_position.read_only()
}
