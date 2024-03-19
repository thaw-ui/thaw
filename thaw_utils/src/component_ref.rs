use leptos::{
    create_render_effect, create_rw_signal, logging::debug_warn, RwSignal, SignalGet,
    SignalGetUntracked, SignalUpdate,
};
use std::cell::Cell;

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.0.get()
    }

    pub fn get_untracked(&self) -> Option<T>
    where
        T: Clone,
    {
        self.0.get_untracked()
    }

    pub fn load(&self, comp: T) {
        self.0.update(|current| {
            if current.is_some() {
                debug_warn!(
                    "You are setting a ComponentRef that has already been filled. \
                     It’s possible this is intentional."
                );
            }
            *current = Some(comp);
        });
    }

    pub fn on_load<F>(self, f: F)
    where
        T: Clone,
        F: FnOnce(T) + 'static,
    {
        let f = Cell::new(Some(f));

        create_render_effect(move |_| {
            if let Some(comp) = self.get() {
                f.take().unwrap()(comp);
            }
        });
    }
}

pub fn create_component_ref<T>() -> ComponentRef<T> {
    ComponentRef::default()
}
