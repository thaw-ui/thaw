pub mod class_list;
mod dom;
mod event_listener;
mod hooks;
mod optional_prop;
mod signals;
mod time;

pub use dom::{get_scroll_parent, mount_style};
pub use event_listener::{add_event_listener, EventListenerHandle};
pub use hooks::{use_click_position, use_lock_html_scroll, use_next_frame, NextFrame};
pub use optional_prop::OptionalProp;
pub use signals::{
    create_component_ref, ComponentRef, Model, OptionalMaybeSignal, SignalWatch, StoredMaybeSignal,
};
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
