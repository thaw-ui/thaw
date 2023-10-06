use leptos::RwSignal;
use std::ops::Deref;

pub struct MaybeRwSignal<T: Default + 'static>(RwSignal<T>);

impl<T: Default> Default for MaybeRwSignal<T> {
    fn default() -> Self {
        Self(RwSignal::new(Default::default()))
    }
}

impl<T: Default> Clone for MaybeRwSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Default> Copy for MaybeRwSignal<T> {}

impl<T: Default> Deref for MaybeRwSignal<T> {
    type Target = RwSignal<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default> From<RwSignal<T>> for MaybeRwSignal<T> {
    fn from(value: RwSignal<T>) -> Self {
        Self(value)
    }
}
