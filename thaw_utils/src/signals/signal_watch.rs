use leptos::prelude::*;

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
    /// ```rust,no_run
    /// use leptos::prelude::*;
    /// use thaw_utils::SignalWatch;
    ///
    /// let count = RwSignal::new(0);
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

        let effect = Effect::new(move |prev: Option<()>| {
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
