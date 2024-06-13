use leptos::{leptos_dom::helpers::TimeoutHandle, prelude::*};
use std::time::Duration;

pub fn throttle(cb: impl Fn() + Send + Sync + 'static, duration: Duration) -> impl Fn() -> () {
    let cb = Callback::new(move |_| cb());
    let timeout_handle = StoredValue::new(None::<TimeoutHandle>);
    on_cleanup(move || {
        timeout_handle.update_value(move |handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
    });

    move || {
        if timeout_handle.with_value(|handle| handle.is_some()) {
            return;
        }
        let cb = cb.clone();
        let handle = set_timeout_with_handle(
            move || {
                cb.call(());
                timeout_handle.update_value(move |handle| {
                    *handle = None;
                });
            },
            duration,
        )
        .unwrap();
        timeout_handle.set_value(Some(handle));
    }
}
