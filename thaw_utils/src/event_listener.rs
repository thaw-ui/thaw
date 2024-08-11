use ::wasm_bindgen::{prelude::Closure, JsCast};
use leptos::ev;
use web_sys::{Element, EventTarget};

pub fn add_event_listener<E>(
    target: Element,
    event: E,
    cb: impl Fn(E::EventType) + 'static,
) -> EventListenerHandle
where
    E: ev::EventDescriptor + 'static,
    E::EventType: JsCast,
{
    add_event_listener_untyped(target, &event.name(), move |e| {
        cb(e.unchecked_into::<E::EventType>())
    })
}

pub struct EventListenerHandle(Box<dyn FnOnce() + Send + Sync>);

impl std::fmt::Debug for EventListenerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EventListenerHandle").finish()
    }
}

impl EventListenerHandle {
    pub fn remove(self) {
        (self.0)();
    }
}

fn add_event_listener_untyped(
    target: Element,
    event_name: &str,
    cb: impl Fn(web_sys::Event) + 'static,
) -> EventListenerHandle {
    fn wel(
        target: Element,
        cb: Box<dyn FnMut(web_sys::Event)>,
        event_name: &str,
    ) -> EventListenerHandle {
        let cb = Closure::wrap(cb);
        _ = target.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref());

        EventListenerHandle({
            let event_name = event_name.to_string();
            let cb = send_wrapper::SendWrapper::new(cb);
            let target = send_wrapper::SendWrapper::new(target);
            Box::new(move || {
                let _ = target
                    .remove_event_listener_with_callback(&event_name, cb.as_ref().unchecked_ref());
            })
        })
    }

    wel(target, Box::new(cb), event_name)
}

pub fn add_event_listener_with_bool<E: ev::EventDescriptor + 'static>(
    target: impl IntoEventTarget,
    event: E,
    cb: impl Fn(E::EventType) + 'static,
    use_capture: bool,
) -> EventListenerHandle
where
    E::EventType: JsCast,
{
    add_event_listener_untyped_with_bool(
        target.into_event_target(),
        &event.name(),
        move |e| cb(e.unchecked_into::<E::EventType>()),
        use_capture,
    )
}

fn add_event_listener_untyped_with_bool(
    target: EventTarget,
    event_name: &str,
    cb: impl Fn(web_sys::Event) + 'static,
    use_capture: bool,
) -> EventListenerHandle {
    fn wel(
        target: EventTarget,
        cb: Box<dyn FnMut(web_sys::Event)>,
        event_name: &str,
        use_capture: bool,
    ) -> EventListenerHandle {
        let cb = Closure::wrap(cb).into_js_value();
        _ = target.add_event_listener_with_callback_and_bool(
            event_name,
            cb.unchecked_ref(),
            use_capture,
        );

        EventListenerHandle({
            let event_name = event_name.to_string();
            let cb = send_wrapper::SendWrapper::new(cb);
            let target = send_wrapper::SendWrapper::new(target);
            Box::new(move || {
                let _ = target.remove_event_listener_with_callback_and_bool(
                    &event_name,
                    cb.unchecked_ref(),
                    use_capture,
                );
            })
        })
    }

    wel(target, Box::new(cb), event_name, use_capture)
}

pub trait IntoEventTarget {
    fn into_event_target(self) -> EventTarget;
}

impl IntoEventTarget for EventTarget {
    fn into_event_target(self) -> EventTarget {
        self
    }
}

impl IntoEventTarget for web_sys::Document {
    fn into_event_target(self) -> EventTarget {
        self.into()
    }
}

// impl IntoEventTarget for HtmlElement<AnyElement> {
//     fn into_event_target(self) -> EventTarget {
//         self.deref().deref().deref().deref().clone()
//     }
// }
