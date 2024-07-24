use leptos::reactive_graph::{
    computed::Memo,
    signal::{ReadSignal, RwSignal, WriteSignal},
    traits::{Get, GetUntracked, Set, With, WithUntracked},
    wrappers::read::Signal,
};

pub enum OptionModel<T>
where
    T: 'static,
{
    T(Signal<T>, WriteSignal<T>, Option<WriteSignal<T>>),
    Option(
        Signal<Option<T>>,
        WriteSignal<Option<T>>,
        Option<WriteSignal<Option<T>>>,
    ),
}

impl<T: Default + Send + Sync> Default for OptionModel<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> Clone for OptionModel<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for OptionModel<T> {}

impl<T: Send + Sync> OptionModel<T> {
    fn new(value: T) -> Self {
        let rw_signal = RwSignal::new(Some(value));
        rw_signal.into()
    }

    fn new_option(value: Option<T>) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }

    pub fn with<O>(&self, fun: impl FnOnce(Option<&T>) -> O) -> O {
        match self {
            Self::T(read, _, _) => read.with(|value| fun(Some(value))),
            Self::Option(read, _, _) => read.with(|value| fun(value.as_ref())),
        }
    }

    pub fn with_untracked<O>(&self, fun: impl FnOnce(Option<&T>) -> O) -> O {
        match self {
            Self::T(read, _, _) => read.with_untracked(|value| fun(Some(value))),
            Self::Option(read, _, _) => read.with_untracked(|value| fun(value.as_ref())),
        }
    }
}

impl<T: Send + Sync + Clone> OptionModel<T> {
    pub fn get(&self) -> Option<T> {
        match self {
            Self::T(read, _, _) => Some(read.get()),
            Self::Option(read, _, _) => read.get(),
        }
    }

    pub fn get_untracked(&self) -> Option<T> {
        match self {
            Self::T(read, _, _) => Some(read.get_untracked()),
            Self::Option(read, _, _) => read.get_untracked(),
        }
    }
}

impl<T: Send + Sync + Clone + Default> OptionModel<T> {
    pub fn set(&self, value: Option<T>) {
        match self {
            Self::T(read, write, on_write) => {
                write.set(value.unwrap_or_default());
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
            Self::Option(read, write, on_write) => {
                write.set(value);
                if let Some(on_write) = on_write.as_ref() {
                    on_write.set(read.get_untracked());
                }
            }
        }
    }
}

impl<T: Send + Sync> From<T> for OptionModel<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Send + Sync> From<RwSignal<T>> for OptionModel<T> {
    fn from(rw_signal: RwSignal<T>) -> Self {
        let (read, write) = rw_signal.split();
        Self::T(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<RwSignal<Option<T>>> for OptionModel<T> {
    fn from(rw_signal: RwSignal<Option<T>>) -> Self {
        let (read, write) = rw_signal.split();
        Self::Option(read.into(), write, None)
    }
}

impl<T> From<(Signal<T>, WriteSignal<T>)> for OptionModel<T> {
    fn from((read, write): (Signal<T>, WriteSignal<T>)) -> Self {
        Self::T(read, write, None)
    }
}

impl<T> From<(Signal<Option<T>>, WriteSignal<Option<T>>)> for OptionModel<T> {
    fn from((read, write): (Signal<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(read, write, None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<T>, WriteSignal<T>)> for OptionModel<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::T(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<Option<T>>, WriteSignal<Option<T>>)> for OptionModel<T> {
    fn from((read, write): (ReadSignal<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(Memo<T>, WriteSignal<T>)> for OptionModel<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self::T(read.into(), write, None)
    }
}

impl<T: Send + Sync> From<(Memo<Option<T>>, WriteSignal<Option<T>>)> for OptionModel<T> {
    fn from((read, write): (Memo<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(read.into(), write, None)
    }
}

impl<T: Default + Send + Sync> From<(Option<T>, WriteSignal<T>)> for OptionModel<T> {
    fn from((read, write): (Option<T>, WriteSignal<T>)) -> Self {
        let mut model = Self::new(read.unwrap_or_default());
        if let OptionModel::T(_, _, on_write) = &mut model {
            *on_write = Some(write);
        }
        model
    }
}

impl<T: Default + Send + Sync> From<(Option<Option<T>>, WriteSignal<Option<T>>)>
    for OptionModel<T>
{
    fn from((read, write): (Option<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        let mut model = Self::new_option(read.unwrap_or_default());
        if let OptionModel::Option(_, _, on_write) = &mut model {
            *on_write = Some(write);
        }
        model
    }
}
