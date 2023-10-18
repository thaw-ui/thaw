use leptos::{create_effect, untrack, RwSignal, SignalDispose, SignalWith};

pub trait SignalWatch {
    type Value;

    fn watch(&self, f: impl Fn(&Self::Value) + 'static) -> Box<dyn FnOnce()>;
}

impl<T> SignalWatch for RwSignal<T> {
    type Value = T;

    /// Listens for RwSignal changes and is not executed immediately
    ///
    /// ## Usage
    ///
    /// ```rust
    /// use leptos::*;
    /// use melt_ui::*;
    ///
    /// let count = create_rw_signal(0);
    /// let stop = count.watch(|count| {
    ///     assert_eq!(count, &1);
    /// });
    ///
    /// count.set(1); // assert_eq!(count, &1);
    ///
    /// stop(); // stop watching
    ///
    /// count.set(2); // nothing happens
    /// ```
    fn watch(&self, f: impl Fn(&Self::Value) + 'static) -> Box<dyn FnOnce()> {
        let signal = *self;

        let effect = create_effect(move |prev| {
            signal.with(|value| {
                if prev.is_some() {
                    untrack(|| f(value));
                }
            });
        });

        Box::new(move || {
            effect.dispose();
        })
    }
}
