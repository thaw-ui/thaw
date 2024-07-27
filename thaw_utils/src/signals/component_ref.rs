use leptos::{
    logging::debug_warn,
    reactive_graph::{
        owner::{Storage, SyncStorage},
        signal::{ArcReadSignal, ArcRwSignal, ArcWriteSignal, RwSignal},
        traits::{Get, GetUntracked, Update},
    },
};

pub struct ComponentRef<T, S = SyncStorage>(RwSignal<Option<T>, S>);

impl<T> Default for ComponentRef<T>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        Self(RwSignal::new(None))
    }
}

impl<T, S> Clone for ComponentRef<T, S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, S> Copy for ComponentRef<T, S> {}

impl<T> ComponentRef<T>
where
    T: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, S> ComponentRef<T, S>
where
    T: Clone + 'static,
    S: Storage<ArcRwSignal<Option<T>>> + Storage<ArcReadSignal<Option<T>>>,
{
    pub fn get(&self) -> Option<T> {
        self.0.get()
    }

    pub fn get_untracked(&self) -> Option<T> {
        self.0.get_untracked()
    }
}

impl<T, S> ComponentRef<T, S>
where
    T: 'static,
    S: Storage<ArcRwSignal<Option<T>>> + Storage<ArcWriteSignal<Option<T>>>,
{
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

    // pub fn on_load<F>(self, f: F)
    // where
    //     T: Clone,
    //     F: FnOnce(T) + 'static,
    // {
    //     let f = Cell::new(Some(f));

    //     RenderEffect::new(move |_| {
    //         if let Some(comp) = self.get() {
    //             f.take().unwrap()(comp);
    //         }
    //     });
    // }
}
