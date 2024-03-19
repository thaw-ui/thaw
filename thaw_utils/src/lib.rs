pub mod class_list;
mod component_ref;
mod event_listener;
mod model;
mod mount_style;
mod optional_maybe_signal;
mod optional_prop;
mod signal;
mod stored_maybe_signal;
mod time;
mod use_click_position;
mod use_lock_html_scroll;

pub use component_ref::{create_component_ref, ComponentRef};
pub use event_listener::*;
pub use model::Model;
pub use mount_style::mount_style;
pub use optional_maybe_signal::OptionalMaybeSignal;
pub use optional_prop::OptionalProp;
pub use signal::SignalWatch;
pub use stored_maybe_signal::*;
pub use time::*;
pub use use_click_position::*;
pub use use_lock_html_scroll::*;

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
