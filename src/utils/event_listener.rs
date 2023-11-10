use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};

pub fn add_event_listener<E: ev::EventDescriptor + 'static>(
    target: &web_sys::Element,
    event: E,
    cb: impl Fn(E::EventType) + 'static,
) -> EventListenerHandle
where
    E::EventType: JsCast,
{
    add_event_listener_untyped(target, &event.name(), move |e| {
        cb(e.unchecked_into::<E::EventType>())
    })
}

pub struct EventListenerHandle(Box<dyn FnOnce()>);

impl std::fmt::Debug for EventListenerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EventListenerHandle").finish()
    }
}

impl EventListenerHandle {
    pub fn remove(self) {
        (self.0)()
    }
}

fn add_event_listener_untyped(
    target: &web_sys::Element,
    event_name: &str,
    cb: impl Fn(web_sys::Event) + 'static,
) -> EventListenerHandle {
    fn wel(
        target: &web_sys::Element,
        cb: Box<dyn FnMut(web_sys::Event)>,
        event_name: &str,
    ) -> EventListenerHandle {
        let cb = Closure::wrap(cb).into_js_value();
        _ = target.add_event_listener_with_callback(event_name, cb.unchecked_ref());
        let event_name = event_name.to_string();
        let target = target.clone();
        EventListenerHandle(Box::new(move || {
            _ = target.remove_event_listener_with_callback(&event_name, cb.unchecked_ref());
        }))
    }

    wel(target, Box::new(cb), event_name)
}
