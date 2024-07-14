use leptos::{
    MaybeSignal, Signal, SignalGet, SignalGetUntracked, SignalWith, SignalWithUntracked,
    StoredValue,
};

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

impl<T: Clone> SignalGetUntracked for StoredMaybeSignal<T> {
    type Value = T;

    fn get_untracked(&self) -> Self::Value {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.get_value(),
            StoredMaybeSignal::Signal(signal) => signal.get_untracked(),
        }
    }

    fn try_get_untracked(&self) -> Option<Self::Value> {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.try_get_value(),
            StoredMaybeSignal::Signal(signal) => signal.try_get_untracked(),
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

impl<T> SignalWithUntracked for StoredMaybeSignal<T> {
    type Value = T;

    fn with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.with_value(f),
            StoredMaybeSignal::Signal(signal) => signal.with_untracked(f),
        }
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.try_with_value(f),
            StoredMaybeSignal::Signal(signal) => signal.try_with_untracked(f),
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

impl<T: Clone> From<StoredMaybeSignal<T>> for MaybeSignal<T> {
    fn from(value: StoredMaybeSignal<T>) -> Self {
        match value {
            StoredMaybeSignal::StoredValue(value) => MaybeSignal::Static(value.get_value()),
            StoredMaybeSignal::Signal(singal) => MaybeSignal::Dynamic(singal),
        }
    }
}
