use leptos::window;
use leptos_dom::ev;
use std::borrow::Cow;
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn window_event_listener<E: ev::EventDescriptor + 'static>(
    event: E,
    cb: impl Fn(E::EventType) + 'static,
) -> impl FnOnce() -> ()
where
    E::EventType: JsCast,
{
    fn wel(
        cb: Box<dyn FnMut(web_sys::Event)>,
        event_name: Cow<'static, str>,
    ) -> impl FnOnce() -> () + 'static {
        let cb = Closure::wrap(cb).into_js_value();
        _ = window().add_event_listener_with_callback(&event_name, cb.unchecked_ref());
        move || {
            _ = window().remove_event_listener_with_callback(&event_name, cb.unchecked_ref());
        }
    }

    wel(
        Box::new(move |e| cb(e.unchecked_into::<E::EventType>())),
        event.name(),
    )
}