pub mod class_list;
mod dom;
mod event_listener;
mod hooks;
pub mod macros;
mod optional_prop;
mod signals;
mod throttle;
mod time;
mod callback;

pub use dom::*;
pub use callback::*;
pub use event_listener::{add_event_listener, add_event_listener_with_bool, EventListenerHandle};
pub use hooks::{use_click_position, use_lock_html_scroll, NextFrame};
pub use optional_prop::OptionalProp;
pub use signals::{ComponentRef, Model, OptionalMaybeSignal, SignalWatch, StoredMaybeSignal};
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
