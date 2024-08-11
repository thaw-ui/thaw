use leptos::{
    prelude::Update,
    reactive_graph::{
        computed::Memo,
        owner::{Storage, SyncStorage},
        signal::{ReadSignal, RwSignal, WriteSignal},
        traits::{GetUntracked, Set, With, WithUntracked},
        wrappers::read::Signal,
    },
};

pub enum VecModel<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    T(Signal<T, S>, WriteSignal<T, S>, Option<WriteSignal<T, S>>),
    Option(
        Signal<Option<T>, S>,
        WriteSignal<Option<T>, S>,
        Option<WriteSignal<Option<T>, S>>,
    ),
    Vec(
        Signal<Vec<T>, S>,
        WriteSignal<Vec<T>, S>,
        Option<WriteSignal<Vec<T>, S>>,
    ),
}

impl<T: Default + Send + Sync> Default for VecModel<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T, S> Clone for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, S> Copy for VecModel<T, S> where S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>> {}

impl<T: Send + Sync> VecModel<T> {
    fn new(value: T) -> Self {
        Self::new_option(Some(value))
    }

    fn new_option(value: Option<T>) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }

    fn new_vec(value: Vec<T>) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }

    pub fn is_vec(&self) -> bool {
        if let VecModel::Vec(_, _, _) = self {
            true
        } else {
            false
        }
    }

    pub fn with<O>(
        &self,
        fun: impl FnOnce((Option<&T>, Option<&Option<T>>, Option<&Vec<T>>)) -> O,
    ) -> O {
        match self {
            Self::T(read, _, _) => read.with(|value| fun((Some(value), None, None))),
            Self::Option(read, _, _) => read.with(|value| fun((None, Some(value), None))),
            Self::Vec(read, _, _) => read.with(|value| fun((None, None, Some(value)))),
        }
    }

    pub fn with_untracked<O>(
        &self,
        fun: impl FnOnce((Option<&T>, Option<&Option<T>>, Option<&Vec<T>>)) -> O,
    ) -> O {
        match self {
            Self::T(read, _, _) => read.with_untracked(|value| fun((Some(value), None, None))),
            Self::Option(read, _, _) => read.with_untracked(|value| fun((None, Some(value), None))),
            Self::Vec(read, _, _) => read.with_untracked(|value| fun((None, None, Some(value)))),
        }
    }
}

impl<T: Send + Sync + Clone + Default> VecModel<T> {
    pub fn set(&self, mut value: Vec<T>) {
        match self {
            Self::T(read, write, on_write) => {
                let value = if value.is_empty() {
                    Default::default()
                } else {
                    value.remove(0)
                };

                write.set(value);
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
            Self::Option(read, write, on_write) => {
                let value = if value.is_empty() {
                    None
                } else {
                    Some(value.remove(0))
                };
                write.set(value);
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
            Self::Vec(read, write, on_write) => {
                write.set(value);
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
        }
    }

    pub fn update(
        &self,
        fun: impl FnOnce((Option<&mut T>, Option<&mut Option<T>>, Option<&mut Vec<T>>)),
    ) {
        match self {
            Self::T(read, write, on_write) => {
                write.update(move |write| {
                    fun((Some(write), None, None));
                });
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
            Self::Option(read, write, on_write) => {
                write.update(move |write| {
                    fun((None, Some(write), None));
                });
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
            Self::Vec(read, write, on_write) => {
                write.update(move |write| {
                    fun((None, None, Some(write)));
                });
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
        }
    }
}

impl<T: Send + Sync> From<T> for VecModel<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Send + Sync> From<Option<T>> for VecModel<T> {
    fn from(value: Option<T>) -> Self {
        Self::new_option(value)
    }
}

impl<T: Send + Sync> From<Vec<T>> for VecModel<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new_vec(value)
    }
}

impl<T: Send + Sync> From<RwSignal<T>> for VecModel<T> {
    fn from(rw_signal: RwSignal<T>) -> Self {
        let (read, write) = rw_signal.split();
        Self::T(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<RwSignal<Option<T>>> for VecModel<T> {
    fn from(rw_signal: RwSignal<Option<T>>) -> Self {
        let (read, write) = rw_signal.split();
        Self::Option(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<RwSignal<Vec<T>>> for VecModel<T> {
    fn from(rw_signal: RwSignal<Vec<T>>) -> Self {
        let (read, write) = rw_signal.split();
        Self::Vec(read.into(), write, None)
    }
}

impl<T, S> From<(Signal<T, S>, WriteSignal<T, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<T, S>, WriteSignal<T, S>)) -> Self {
        Self::T(read, write, None)
    }
}

impl<T, S> From<(Signal<Option<T>, S>, WriteSignal<Option<T>, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<Option<T>, S>, WriteSignal<Option<T>, S>)) -> Self {
        Self::Option(read, write, None)
    }
}

impl<T, S> From<(Signal<Vec<T>, S>, WriteSignal<Vec<T>, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<Vec<T>, S>, WriteSignal<Vec<T>, S>)) -> Self {
        Self::Vec(read, write, None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<T>, WriteSignal<T>)> for VecModel<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::T(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<Option<T>>, WriteSignal<Option<T>>)> for VecModel<T> {
    fn from((read, write): (ReadSignal<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<Vec<T>>, WriteSignal<Vec<T>>)> for VecModel<T> {
    fn from((read, write): (ReadSignal<Vec<T>>, WriteSignal<Vec<T>>)) -> Self {
        Self::Vec(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(Memo<T>, WriteSignal<T>)> for VecModel<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self::T(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(Memo<Option<T>>, WriteSignal<Option<T>>)> for VecModel<T> {
    fn from((read, write): (Memo<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(Memo<Vec<T>>, WriteSignal<Vec<T>>)> for VecModel<T> {
    fn from((read, write): (Memo<Vec<T>>, WriteSignal<Vec<T>>)) -> Self {
        Self::Vec(read.into(), write, None)
    }
}

impl<T: Default + Send + Sync> From<(Option<T>, WriteSignal<T>)> for VecModel<T> {
    fn from((read, write): (Option<T>, WriteSignal<T>)) -> Self {
        let mut model = Self::new(read.unwrap_or_default());
        if let VecModel::T(_, _, on_write) = &mut model {
            *on_write = Some(write);
        }
        model
    }
}

impl<T: Default + Send + Sync> From<(Option<Option<T>>, WriteSignal<Option<T>>)> for VecModel<T> {
    fn from((read, write): (Option<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        let mut model = Self::new_option(read.unwrap_or_default());
        if let VecModel::Option(_, _, on_write) = &mut model {
            *on_write = Some(write);
        }
        model
    }
}

impl<T: Default + Send + Sync> From<(Option<Vec<T>>, WriteSignal<Vec<T>>)> for VecModel<T> {
    fn from((read, write): (Option<Vec<T>>, WriteSignal<Vec<T>>)) -> Self {
        let mut model = Self::new_vec(read.unwrap_or_default());
        if let VecModel::Vec(_, _, on_write) = &mut model {
            *on_write = Some(write);
        }
        model
    }
}
