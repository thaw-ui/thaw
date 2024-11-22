use super::{ReadModel, WriteModel};
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use reactive_stores::{ArcField, Field, StoreField, Subfield};

pub enum VecModel<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    T(ReadModel<T, S>, WriteModel<T, S>, Option<WriteSignal<T, S>>),
    Option(
        ReadModel<Option<T>, S>,
        WriteModel<Option<T>, S>,
        Option<WriteSignal<Option<T>, S>>,
    ),
    Vec(
        ReadModel<Vec<T>, S>,
        WriteModel<Vec<T>, S>,
        Option<WriteSignal<Vec<T>, S>>,
    ),
}

impl<T: Default + Send + Sync> Default for VecModel<T> {
    fn default() -> Self {
        Self::new_option(None)
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

    pub fn with<O>(&self, fun: impl FnOnce(VecModelWithValue<T>) -> O) -> O {
        match self {
            Self::T(read, _, _) => read.with(|value| fun(VecModelWithValue::T(value))),
            Self::Option(read, _, _) => read.with(|value| fun(VecModelWithValue::Option(value))),
            Self::Vec(read, _, _) => read.with(|value| fun(VecModelWithValue::Vec(value))),
        }
    }

    pub fn with_untracked<O>(&self, fun: impl FnOnce(VecModelWithValue<T>) -> O) -> O {
        match self {
            Self::T(read, _, _) => read.with_untracked(|value| fun(VecModelWithValue::T(value))),
            Self::Option(read, _, _) => {
                read.with_untracked(|value| fun(VecModelWithValue::Option(value)))
            }
            Self::Vec(read, _, _) => {
                read.with_untracked(|value| fun(VecModelWithValue::Vec(value)))
            }
        }
    }
}

pub enum VecModelWithValue<'a, T> {
    T(&'a T),
    Option(&'a Option<T>),
    Vec(&'a Vec<T>),
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
        Self::T(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T, S> From<Field<T, S>> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from(field: Field<T, S>) -> Self {
        Self::T(field.clone().into(), field.into(), None)
    }
}

impl<T, S, Inner, Prev> From<Subfield<Inner, Prev, T>> for VecModel<T, S>
where
    T: Send + Sync,
    S: Storage<T> + Storage<ArcField<T>> + Storage<Option<T>> + Storage<Vec<T>>,
    Subfield<Inner, Prev, T>: Clone,
    Inner: StoreField<Value = Prev> + Send + Sync + 'static,
    Prev: 'static,
{
    fn from(subfield: Subfield<Inner, Prev, T>) -> Self {
        let field: Field<T, S> = subfield.into();
        Self::T(field.clone().into(), field.into(), None)
    }
}

impl<T: Send + Sync> From<RwSignal<Option<T>>> for VecModel<T> {
    fn from(rw_signal: RwSignal<Option<T>>) -> Self {
        let (read, write) = rw_signal.split();
        Self::Option(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T, S> From<Field<Option<T>, S>> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from(field: Field<Option<T>, S>) -> Self {
        Self::Option(field.clone().into(), field.into(), None)
    }
}

impl<T, S, Inner, Prev> From<Subfield<Inner, Prev, Option<T>>> for VecModel<T, S>
where
    T: Send + Sync,
    S: Storage<T> + Storage<ArcField<Option<T>>> + Storage<Option<T>> + Storage<Vec<T>>,
    Subfield<Inner, Prev, Option<T>>: Clone,
    Inner: StoreField<Value = Prev> + Send + Sync + 'static,
    Prev: 'static,
{
    fn from(subfield: Subfield<Inner, Prev, Option<T>>) -> Self {
        let field: Field<Option<T>, S> = subfield.into();
        Self::Option(field.clone().into(), field.into(), None)
    }
}

impl<T: Send + Sync> From<RwSignal<Vec<T>>> for VecModel<T> {
    fn from(rw_signal: RwSignal<Vec<T>>) -> Self {
        let (read, write) = rw_signal.split();
        Self::Vec(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T, S> From<Field<Vec<T>, S>> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from(field: Field<Vec<T>, S>) -> Self {
        Self::Vec(field.clone().into(), field.into(), None)
    }
}

impl<T, S, Inner, Prev> From<Subfield<Inner, Prev, Vec<T>>> for VecModel<T, S>
where
    T: Send + Sync,
    S: Storage<T> + Storage<ArcField<Vec<T>>> + Storage<Option<T>> + Storage<Vec<T>>,
    Subfield<Inner, Prev, Vec<T>>: Clone,
    Inner: StoreField<Value = Prev> + Send + Sync + 'static,
    Prev: 'static,
{
    fn from(subfield: Subfield<Inner, Prev, Vec<T>>) -> Self {
        let field: Field<Vec<T>, S> = subfield.into();
        Self::Vec(field.clone().into(), field.into(), None)
    }
}

impl<T, S> From<(Signal<T, S>, WriteSignal<T, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<T, S>, WriteSignal<T, S>)) -> Self {
        Self::T(read.into(), write.into(), None)
    }
}

impl<T, S> From<(Signal<T, S>, SignalSetter<T, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<T, S>, SignalSetter<T, S>)) -> Self {
        Self::T(
            read.clone().into(),
            (ReadModel::Signal(read), write).into(),
            None,
        )
    }
}

impl<T, S> From<(Signal<Option<T>, S>, WriteSignal<Option<T>, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<Option<T>, S>, WriteSignal<Option<T>, S>)) -> Self {
        Self::Option(read.into(), write.into(), None)
    }
}

impl<T, S> From<(Signal<Option<T>, S>, SignalSetter<Option<T>, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<Option<T>, S>, SignalSetter<Option<T>, S>)) -> Self {
        Self::Option(
            read.clone().into(),
            (ReadModel::Signal(read), write).into(),
            None,
        )
    }
}

impl<T, S> From<(Signal<Vec<T>, S>, WriteSignal<Vec<T>, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<Vec<T>, S>, WriteSignal<Vec<T>, S>)) -> Self {
        Self::Vec(read.into(), write.into(), None)
    }
}

impl<T, S> From<(Signal<Vec<T>, S>, SignalSetter<Vec<T>, S>)> for VecModel<T, S>
where
    S: Storage<T> + Storage<Option<T>> + Storage<Vec<T>>,
{
    fn from((read, write): (Signal<Vec<T>, S>, SignalSetter<Vec<T>, S>)) -> Self {
        Self::Vec(
            read.clone().into(),
            (ReadModel::Signal(read), write).into(),
            None,
        )
    }
}

impl<T: Send + Sync> From<(ReadSignal<T>, WriteSignal<T>)> for VecModel<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self::T(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<Option<T>>, WriteSignal<Option<T>>)> for VecModel<T> {
    fn from((read, write): (ReadSignal<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(ReadSignal<Vec<T>>, WriteSignal<Vec<T>>)> for VecModel<T> {
    fn from((read, write): (ReadSignal<Vec<T>>, WriteSignal<Vec<T>>)) -> Self {
        Self::Vec(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(Memo<T>, WriteSignal<T>)> for VecModel<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self::T(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(Memo<Option<T>>, WriteSignal<Option<T>>)> for VecModel<T> {
    fn from((read, write): (Memo<Option<T>>, WriteSignal<Option<T>>)) -> Self {
        Self::Option(ReadModel::Signal(read.into()), write.into(), None)
    }
}

impl<T: Send + Sync> From<(Memo<Vec<T>>, WriteSignal<Vec<T>>)> for VecModel<T> {
    fn from((read, write): (Memo<Vec<T>>, WriteSignal<Vec<T>>)) -> Self {
        Self::Vec(ReadModel::Signal(read.into()), write.into(), None)
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

#[cfg(test)]
mod test {
    use super::{VecModel, VecModelWithValue};
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
        let model: VecModel<i32> = (signal, setter).into();
        model.set(vec![1]);
        model.with_untracked(|v| {
            if let VecModelWithValue::T(v) = v {
                assert_eq!(v, &1);
            } else {
                unreachable!()
            }
        });

        // Store Subfield
        let store = Store::new(StoreInfo {
            my_str: "old".to_string(),
        });
        let model: VecModel<String> = store.my_str().into();
        model.set(vec!["new".to_string()]);
        model.with_untracked(|v| {
            if let VecModelWithValue::T(v) = v {
                assert_eq!(v, "new");
            } else {
                unreachable!()
            }
        });
    }
}
