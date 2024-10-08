use leptos::prelude::*;

pub enum StoredMaybeSignal<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T>,
{
    StoredValue(StoredValue<T, S>),
    Signal(Signal<T, S>),
}

impl<T, S> Clone for StoredMaybeSignal<T, S>
where
    S: Storage<T>,
{
    fn clone(&self) -> Self {
        match self {
            Self::StoredValue(v) => Self::StoredValue(v.clone()),
            Self::Signal(s) => Self::Signal(s.clone()),
        }
    }
}

impl<T: Clone, S> Copy for StoredMaybeSignal<T, S> where S: Storage<T> {}

impl<T, S> DefinedAt for StoredMaybeSignal<T, S>
where
    S: Storage<T>,
{
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.defined_at(),
            StoredMaybeSignal::Signal(signal) => signal.defined_at(),
        }
    }
}

impl<T: Send + Sync> With for StoredMaybeSignal<T> {
    type Value = T;

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.try_with_value(f),
            StoredMaybeSignal::Signal(signal) => signal.try_with(f),
        }
    }
}

impl<T: Send + Sync> WithUntracked for StoredMaybeSignal<T> {
    type Value = T;

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        match self {
            StoredMaybeSignal::StoredValue(value) => value.try_with_value(f),
            StoredMaybeSignal::Signal(signal) => signal.try_with_untracked(f),
        }
    }
}

impl<T: Send + Sync> From<MaybeSignal<T>> for StoredMaybeSignal<T> {
    fn from(value: MaybeSignal<T>) -> Self {
        match value {
            MaybeSignal::Static(value) => Self::StoredValue(StoredValue::new(value)),
            MaybeSignal::Dynamic(signal) => Self::Signal(signal),
        }
    }
}
