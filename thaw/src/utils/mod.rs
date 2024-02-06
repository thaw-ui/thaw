// mod callback;
pub(crate) mod class_list;
mod component_ref;
mod event_listener;
mod model;
mod mount_style;
mod optional_prop;
mod signal;
mod stored_maybe_signal;
mod time;
mod use_lock_html_scroll;

// pub use callback::AsyncCallback;
pub use component_ref::{create_component_ref, ComponentRef};
pub(crate) use event_listener::*;
pub(crate) use model::Model;
pub(crate) use mount_style::mount_style;
pub(crate) use optional_prop::OptionalProp;
pub use signal::SignalWatch;
pub(crate) use stored_maybe_signal::*;
pub(crate) use time::*;
pub(crate) use use_lock_html_scroll::*;

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
