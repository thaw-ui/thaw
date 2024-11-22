use super::{ReadModel, WriteModel};
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use reactive_stores::{ArcField, Field, StoreField, Subfield};

pub enum OptionModel<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T> + Storage<Option<T>>,
{
    T(ReadModel<T, S>, WriteModel<T, S>, Option<WriteSignal<T, S>>),
    Option(
        ReadModel<Option<T>, S>,
        WriteModel<Option<T>, S>,
        Option<WriteSignal<Option<T>, S>>,
    ),
}

impl<T: Default + Send + Sync> Default for OptionModel<T> {
    fn default() -> Self {
        Self::new_option(None)
    }
}

impl<T, S> Clone for OptionModel<T, S>
where
    S: Storage<T> + Storage<Option<T>>,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, S> Copy for OptionModel<T, S> where S: Storage<T> + Storage<Option<T>> {}

impl<T: Send + Sync> OptionModel<T> {
    fn new(value: T) -> Self {
        let rw_signal = RwSignal::new(Some(value));
        rw_signal.into()
    }

    fn new_option(value: Option<T>) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }

    pub fn with<O>(&self, fun: impl FnOnce(OptionModelWithValue<T>) -> O) -> O {
        match self {
            Self::T(read, _, _) => read.with(|value| fun(OptionModelWithValue::T(value))),
            Self::Option(read, _, _) => read.with(|value| fun(OptionModelWithValue::Option(value))),
        }
    }

    pub fn with_untracked<O>(&self, fun: impl FnOnce(OptionModelWithValue<T>) -> O) -> O {
        match self {
            Self::T(read, _, _) => read.with_untracked(|value| fun(OptionModelWithValue::T(value))),
            Self::Option(read, _, _) => {
                read.with_untracked(|value| fun(OptionModelWithValue::Option(value)))
            }
        }
    }
}

pub enum OptionModelWithValue<'a, T> {
    T(&'a T),
    Option(&'a Option<T>),
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
        Self::T(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T, S> From<Field<T, S>> for OptionModel<T, S>
where
    S: Storage<T> + Storage<Option<T>>,
{
    fn from(field: Field<T, S>) -> Self {
        Self::T(field.clone().into(), field.into(), None)
    }
}

impl<T, S, Inner, Prev> From<Subfield<Inner, Prev, T>> for OptionModel<T, S>
where
    T: Send + Sync,
    S: Storage<T> + Storage<ArcField<T>> + Storage<Option<T>>,
    Subfield<Inner, Prev, T>: Clone,
    Inner: StoreField<Value = Prev> + Send + Sync + 'static,
    Prev: 'static,
{
    fn from(subfield: Subfield<Inner, Prev, T>) -> Self {
        let field: Field<T, S> = subfield.into();
        Self::T(field.clone().into(), field.into(), None)
    }
}

impl<T: Send + Sync> From<RwSignal<Option<T>>> for OptionModel<T> {
    fn from(rw_signal: RwSignal<Option<T>>) -> Self {
        let (read, write) = rw_signal.split();
        Self::Option(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T, S> From<Field<Option<T>, S>> for OptionModel<T, S>
where
    S: Storage<T> + Storage<Option<T>>,
{
    fn from(field: Field<Option<T>, S>) -> Self {
        Self::Option(field.clone().into(), field.into(), None)
    }
}

impl<T, S, Inner, Prev> From<Subfield<Inner, Prev, Option<T>>> for OptionModel<T, S>
where
    T: Send + Sync,
    S: Storage<T> + Storage<ArcField<Option<T>>> + Storage<Option<T>>,
    Subfield<Inner, Prev, Option<T>>: Clone,
    Inner: StoreField<Value = Prev> + Send + Sync + 'static,
    Prev: 'static,
{
    fn from(subfield: Subfield<Inner, Prev, Option<T>>) -> Self {
        let field: Field<Option<T>, S> = subfield.into();
        Self::Option(field.clone().into(), field.into(), None)
    }
}

impl<T, S> From<(Signal<T, S>, WriteSignal<T, S>)> for OptionModel<T, S>
where
    S: Storage<T> + Storage<Option<T>>,
{
    fn from((read, write): (Signal<T, S>, WriteSignal<T, S>)) -> Self {
        Self::T(read.into(), write.into(), None)
    }
}

impl<T, S> From<(Signal<T, S>, SignalSetter<T, S>)> for OptionModel<T, S>
where
    S: Storage<T> + Storage<Option<T>>,
{
    fn from((read, write): (Signal<T, S>, SignalSetter<T, S>)) -> Self {
        Self::T(
            ReadModel::Signal(read.clone()),
            WriteModel::SignalSetter(ReadModel::Signal(read), write),
            None,
        )
    }
}

impl<T, S> From<(Signal<Option<T>, S>, WriteSignal<Option<T>, S>)> for OptionModel<T, S>
where
    S: Storage<T> + Storage<Option<T>>,
{
    fn from((read, write): (Signal<Option<T>, S>, WriteSignal<Option<T>, S>)) -> Self {
        Self::Option(read.into(), write.into(), None)
    }
}

impl<T, S> From<(Signal<Option<T>, S>, SignalSetter<Option<T>, S>)> for OptionModel<T, S>
where
    S: Storage<T> + Storage<Option<T>>,
{
    fn from((read, write): (Signal<Option<T>, S>, SignalSetter<Option<T>, S>)) -> Self {
        Self::Option(
            read.clone().into(),
            WriteModel::SignalSetter(ReadModel::Signal(read), write),
            None,
        )
    }
}

impl<T: Send + Sync> From<(ReadSignal<T>, WriteSignal<T>)> for OptionModel<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::T(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<Option<T>>, WriteSignal<Option<T>>)> for OptionModel<T> {
    fn from((read, write): (ReadSignal<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(Memo<T>, WriteSignal<T>)> for OptionModel<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self::T(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(Memo<Option<T>>, WriteSignal<Option<T>>)> for OptionModel<T> {
    fn from((read, write): (Memo<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(ReadModel::Signal(read.into()), write.into(), None)
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

#[cfg(test)]
mod test {
    use super::OptionModel;
    use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
    use reactive_stores::Store;

    #[derive(Debug, Store, Clone)]
    struct StoreInfo {
        my_str: String,
    }

    #[test]
    fn from() {
        let owner = Owner::new();
        owner.set();

        // Signal SignalSetter
        let (read, write) = signal(0);
        let signal = Signal::from(read);
        let setter = SignalSetter::map(move |v: i32| {
            write.set(v);
        });
        let model: OptionModel<i32> = (signal, setter).into();
        assert_eq!(model.get_untracked(), Some(0));
        model.set(Some(1));
        assert_eq!(model.get_untracked(), Some(1));

        // Store Subfield
        let store = Store::new(StoreInfo {
            my_str: "old".to_string(),
        });
        let model: OptionModel<String> = store.my_str().into();
        assert_eq!(model.get_untracked(), Some("old".to_string()));
        model.set(Some("new".to_string()));
        assert_eq!(model.get_untracked(), Some("new".to_string()));
    }
}
