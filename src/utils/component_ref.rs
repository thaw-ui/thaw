use leptos::{create_rw_signal, RwSignal, SignalGetUntracked, SignalSet};

pub struct ComponentRef<T: 'static>(RwSignal<Option<T>>);

impl<T> Default for ComponentRef<T> {
    fn default() -> Self {
        Self(create_rw_signal(None))
    }
}

impl<T> Clone for ComponentRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for ComponentRef<T> {}

impl<T> ComponentRef<T> {
    pub fn get_untracked(&self) -> Option<T>
    where
        T: Clone,
    {
        self.0.get_untracked()
    }

    pub fn load(&self, comp_ref: T) {
        self.0.set(Some(comp_ref));
    }
}
