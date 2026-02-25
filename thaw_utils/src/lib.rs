mod callback;
pub mod class_list;
mod dom;
mod event_listener;
mod hooks;
pub mod macros;
mod on_click_outside;
mod optional_prop;
mod signals;
mod throttle;
mod time;

pub use callback::*;
pub use dom::*;
pub use event_listener::{add_event_listener, add_event_listener_with_bool, EventListenerHandle};
pub use hooks::{use_click_position, use_lock_html_scroll};
pub use on_click_outside::*;
pub use optional_prop::OptionalProp;
pub use signals::*;
pub use throttle::throttle;
pub use time::now_date;

#[macro_export]
macro_rules! maybe_unstable {
    (INV $e:expr) => {{
        #[cfg(web_sys_unstable_apis)]
        {
            $e as f64
        }
        #[cfg(not(web_sys_unstable_apis))]
        {
            $e
        }
    }};
    ($e:expr) => {{
        #[cfg(web_sys_unstable_apis)]
        {
            $e.round() as i32
        }
        #[cfg(not(web_sys_unstable_apis))]
        {
            $e
        }
    }};
}
