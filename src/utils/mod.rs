// mod callback;
pub(crate) mod class_list;
mod component_ref;
mod dyn_classes;
mod event_listener;
mod mount_style;
mod signal;
mod stored_maybe_signal;
mod time;

// pub use callback::AsyncCallback;
pub use component_ref::{create_component_ref, ComponentRef};
pub(crate) use dyn_classes::*;
pub(crate) use event_listener::*;
pub(crate) use mount_style::mount_style;
pub use signal::SignalWatch;
pub(crate) use stored_maybe_signal::*;
pub(crate) use time::*;

pub(crate) fn with_hydration_off<T>(f: impl FnOnce() -> T) -> T {
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
