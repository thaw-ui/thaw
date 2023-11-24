// mod callback;
mod component_ref;
mod event_listener;
mod mount_style;
mod provider;
mod signal;
mod stored_maybe_signal;

// pub use callback::AsyncCallback;
pub(crate) use component_ref::ComponentRef;
pub(crate) use event_listener::*;
pub(crate) use mount_style::mount_style;
pub(crate) use provider::Provider;
pub use signal::SignalWatch;
pub(crate) use stored_maybe_signal::*;

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
