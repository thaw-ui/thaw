use super::{ReadModel, WriteModel};
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use reactive_stores::{ArcField, Field, StoreField, Subfield};

#[derive(Clone, Copy)]
pub struct Model<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T>,
{
    read: ReadModel<T, S>,
    write: WriteModel<T, S>,
    on_write: Option<WriteSignal<T, S>>,
}

impl<T: Default + Send + Sync> Default for Model<T> {
    fn default() -> Self {
        RwSignal::new(Default::default()).into()
    }
}

impl<T: Send + Sync> Model<T> {
    fn new(value: T) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }
}

impl Model<bool> {
    pub fn signal(&self) -> Signal<bool> {
        match self.read {
            ReadModel::Signal(signal) => signal.clone(),
            ReadModel::Field(field) => Signal::derive(move || field.get()),
        }
    }
}

impl<T, S> DefinedAt for Model<T, S>
where
    S: Storage<T>,
{
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        self.read.defined_at()
    }
}

impl<T, S> With for Model<T, S>
where
    S: Storage<ArcField<T>> + Storage<SignalTypes<T, S>> + Storage<T>,
{
    type Value = T;

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.read.try_with(f)
    }
}

impl<T, S> WithUntracked for Model<T, S>
where
    S: Storage<ArcField<T>> + Storage<SignalTypes<T, S>> + Storage<T>,
{
    type Value = T;

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.read.try_with_untracked(f)
    }
}

impl<T, S> Update for Model<T, S>
where
    T: Clone,
    S: Storage<SignalTypes<T, S>>,
    S: Storage<ArcField<T>>
        + Storage<ArcWriteSignal<T>>
        + Storage<Box<dyn Fn(T) + Send + Sync>>
        + Storage<T>,
{
    type Value = T;

    fn try_maybe_update<U>(&self, fun: impl FnOnce(&mut Self::Value) -> (bool, U)) -> Option<U> {
        let value = self.write.try_maybe_update(fun);

        if value.is_some() {
            // TODO After the fun function returns false, this should not be executed
            if let Some(on_write) = self.on_write.as_ref() {
                on_write.set(self.read.with_untracked(|read| read.clone()));
            }
        }

        value
    }
}

impl<T, S> IsDisposed for Model<T, S>
where
    S: Storage<T>,
{
    fn is_disposed(&self) -> bool {
        self.write.is_disposed()
    }
}

impl<T: Send + Sync> From<T> for Model<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> From<RwSignal<T>> for Model<T>
where
    T: Sync + Send,
{
    fn from(rw_signal: RwSignal<T>) -> Self {
        let (read, write) = rw_signal.split();
        Self {
            read: ReadModel::Signal(read.into()),
            write: WriteModel::WriteSignal(write),
            on_write: None,
        }
    }
}

impl<T> From<RwSignal<T, LocalStorage>> for Model<T, LocalStorage> {
    fn from(rw_signal: RwSignal<T, LocalStorage>) -> Self {
        let (read, write) = rw_signal.split();
        Self {
            read: ReadModel::Signal(read.into()),
            write: WriteModel::WriteSignal(write),
            on_write: None,
        }
    }
}

impl<T, S> From<Field<T, S>> for Model<T, S>
where
    S: Storage<T>,
{
    fn from(field: Field<T, S>) -> Self {
        Self {
            read: ReadModel::Field(field.clone()),
            write: WriteModel::Field(field),
            on_write: None,
        }
    }
}

impl<T, S, Inner, Prev> From<Subfield<Inner, Prev, T>> for Model<T, S>
where
    T: Send + Sync,
    S: Storage<T> + Storage<ArcField<T>>,
    Subfield<Inner, Prev, T>: Clone,
    Inner: StoreField<Value = Prev> + Send + Sync + 'static,
    Prev: 'static,
{
    fn from(subfield: Subfield<Inner, Prev, T>) -> Self {
        let field: Field<T, S> = subfield.into();
        Self {
            read: ReadModel::Field(field.clone()),
            write: WriteModel::Field(field),
            on_write: None,
        }
    }
}

impl<T, S> From<(Signal<T, S>, WriteSignal<T, S>)> for Model<T, S>
where
    S: Storage<T>,
{
    fn from((read, write): (Signal<T, S>, WriteSignal<T, S>)) -> Self {
        Self {
            read: ReadModel::Signal(read),
            write: WriteModel::WriteSignal(write),
            on_write: None,
        }
    }
}

impl<T, S> From<(Signal<T, S>, SignalSetter<T, S>)> for Model<T, S>
where
    S: Storage<T>,
{
    fn from((read, write): (Signal<T, S>, SignalSetter<T, S>)) -> Self {
        Self {
            read: ReadModel::Signal(read.clone()),
            write: WriteModel::SignalSetter(ReadModel::Signal(read), write),
            on_write: None,
        }
    }
}

impl<T> From<(ReadSignal<T>, WriteSignal<T>)> for Model<T>
where
    T: Send + Sync,
{
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self {
            read: ReadModel::Signal(read.into()),
            write: WriteModel::WriteSignal(write),
            on_write: None,
        }
    }
}

impl<T> From<(ReadSignal<T, LocalStorage>, WriteSignal<T, LocalStorage>)>
    for Model<T, LocalStorage>
{
    fn from((read, write): (ReadSignal<T, LocalStorage>, WriteSignal<T, LocalStorage>)) -> Self {
        Self {
            read: ReadModel::Signal(read.into()),
            write: WriteModel::WriteSignal(write),
            on_write: None,
        }
    }
}

impl<T> From<(Memo<T>, WriteSignal<T>)> for Model<T>
where
    T: Send + Sync,
{
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self {
            read: ReadModel::Signal(read.into()),
            write: WriteModel::WriteSignal(write),
            on_write: None,
        }
    }
}

impl<T> From<(Memo<T, LocalStorage>, WriteSignal<T, LocalStorage>)> for Model<T, LocalStorage> {
    fn from((read, write): (Memo<T, LocalStorage>, WriteSignal<T, LocalStorage>)) -> Self {
        Self {
            read: ReadModel::Signal(read.into()),
            write: WriteModel::WriteSignal(write),
            on_write: None,
        }
    }
}

impl<T> From<(Option<T>, WriteSignal<T>)> for Model<T>
where
    T: Default + Send + Sync,
{
    fn from((read, write): (Option<T>, WriteSignal<T>)) -> Self {
        let mut model = Self::new(read.unwrap_or_default());
        model.on_write = Some(write);
        model
    }
}

#[cfg(test)]
mod test {
    use super::Model;
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

        // T
        let model: Model<i32> = 0.into();
        assert_eq!(model.get_untracked(), 0);
        model.set(1);
        assert_eq!(model.get_untracked(), 1);

        // RwSignal
        let rw_signal = RwSignal::new(0);
        let model: Model<i32> = rw_signal.into();
        assert_eq!(model.get_untracked(), 0);
        model.set(1);
        assert_eq!(model.get_untracked(), 1);

        // Read Write
        let (read, write) = signal(0);
        let model: Model<i32> = (read, write).into();
        assert_eq!(model.get_untracked(), 0);
        model.set(1);
        assert_eq!(model.get_untracked(), 1);

        // Signal SignalSetter
        let (read, write) = signal(0);
        let signal = Signal::from(read);
        let setter = SignalSetter::map(move |v: i32| { write.set(v); });
        let model: Model<i32> = (signal, setter).into();
        assert_eq!(model.get_untracked(), 0);
        model.set(1);
        assert_eq!(model.get_untracked(), 1);

        // Store Subfield
        let store = Store::new(StoreInfo {
            my_str: "old".to_string(),
        });
        let model: Model<String> = store.my_str().into();
        assert_eq!(model.get_untracked(), "old".to_string());
        model.set("new".to_string());
        assert_eq!(model.get_untracked(), "new".to_string());
    }
}
