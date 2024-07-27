use leptos::reactive_graph::{
    effect::Effect,
    owner::Storage,
    signal::{ArcReadSignal, ArcRwSignal, RwSignal},
    traits::{Dispose, With},
    untrack,
};

pub trait SignalWatch {
    type Value;

    fn watch(&self, f: impl Fn(&Self::Value) + 'static) -> Box<dyn FnOnce()>;
}

impl<T, S> SignalWatch for RwSignal<T, S>
where
    T: 'static,
    S: Storage<ArcRwSignal<T>> + Storage<ArcReadSignal<T>>,
{
    type Value = T;
    ///
    /// ## Usage
    ///
    /// ```rust
    /// use leptos::*;
    /// use thaw::*;
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

        let effect = Effect::new(move |prev| {
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
