pub mod class_list;
mod dom;
mod event_listener;
mod hooks;
mod on_click_outside;
mod optional_prop;
mod signals;
mod throttle;
mod time;

pub use dom::{get_scroll_parent, mount_style};
pub use event_listener::{
    add_event_listener, add_event_listener_with_bool, EventListenerHandle, IntoEventTarget,
};
pub use hooks::{use_click_position, use_lock_html_scroll, use_next_frame, NextFrame};
pub use on_click_outside::call_on_click_outside;
pub use optional_prop::OptionalProp;
pub use signals::{
    create_component_ref, ComponentRef, Model, OptionalMaybeSignal, SignalWatch, StoredMaybeSignal,
};
pub use throttle::throttle;
pub use time::now_date;

pub fn with_hydration_off<T>(f: impl FnOnce() -> T) -> T {
    #[cfg(feature = "hydrate")]
    {
        use leptos::leptos_dom::HydrationCtx;
        HydrationCtx::with_hydration_off(f)
    }
    #[cfg(not(feature = "hydrate"))]
    {
        f()
    }
}
