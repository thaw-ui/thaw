use leptos::{MaybeSignal, Signal, SignalGet, SignalWith, StoredValue};

#[derive(Clone)]
pub enum StoredMaybeSignal<T>
where
    T: 'static,
{
    StoredValue(StoredValue<T>),
    Signal(Signal<T>),
}

impl<T: Clone> Copy for StoredMaybeSignal<T> {}

impl<T: Clone> SignalGet for StoredMaybeSignal<T> {
    type Value = T;

    fn get(&self) -> Self::Value {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.get_value(),
            StoredMaybeSignal::Signal(signal) => signal.get(),
        }
    }

    fn try_get(&self) -> Option<Self::Value> {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.try_get_value(),
            StoredMaybeSignal::Signal(signal) => signal.try_get(),
        }
    }
}

impl<T> SignalWith for StoredMaybeSignal<T> {
    type Value = T;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.with_value(f),
            StoredMaybeSignal::Signal(signal) => signal.with(f),
        }
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.try_with_value(f),
            StoredMaybeSignal::Signal(signal) => signal.try_with(f),
        }
    }
}

impl<T> From<MaybeSignal<T>> for StoredMaybeSignal<T> {
    fn from(value: MaybeSignal<T>) -> Self {
        match value {
            MaybeSignal::Static(value) => Self::StoredValue(StoredValue::new(value)),
            MaybeSignal::Dynamic(signal) => Self::Signal(signal),
        }
    }
}
