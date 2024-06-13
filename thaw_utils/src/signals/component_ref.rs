use leptos::{
    logging::debug_warn,
    reactive_graph::{
        effect::RenderEffect,
        signal::RwSignal,
        traits::{Get, GetUntracked, Update},
    },
};
use std::cell::Cell;

pub struct ComponentRef<T: 'static>(RwSignal<Option<T>>);

impl<T: Send + Sync> Default for ComponentRef<T> {
    fn default() -> Self {
        Self(RwSignal::new(None))
    }
}

impl<T> Clone for ComponentRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: 'static> Copy for ComponentRef<T> {}

// TODO
impl<T: Send + Sync> ComponentRef<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> ComponentRef<T> {
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

        RenderEffect::new(move |_| {
            if let Some(comp) = self.get() {
                f.take().unwrap()(comp);
            }
        });
    }
}
