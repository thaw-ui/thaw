use leptos::window;
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn window_event_listener<'a>(
    event_name: &'a str,
    cb: impl Fn(web_sys::Event) + 'static,
) -> impl FnOnce() -> () + 'a {
    let handler = Box::new(cb) as Box<dyn FnMut(web_sys::Event)>;

    let cb = Closure::wrap(handler).into_js_value();
    _ = window().add_event_listener_with_callback(event_name, cb.unchecked_ref());

    move || {
        _ = window().remove_event_listener_with_callback(event_name, cb.unchecked_ref());
    }
}
