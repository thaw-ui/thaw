mod model;
mod option_model;
mod vec_model;

pub use model::*;
pub use option_model::{OptionModel, OptionModelWithValue};
pub use vec_model::{VecModel, VecModelWithValue};

use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use reactive_stores::{ArcField, Field, StoreField};

#[derive(Clone, Copy)]
enum ReadModel<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T>,
{
    Signal(Signal<T, S>),
    Field(Field<T, S>),
}

impl<T, S> DefinedAt for ReadModel<T, S>
where
    S: Storage<T>,
{
    fn defined_at(&self) -> Option<&'static std::panic::Location<'static>> {
        match self {
            Self::Signal(signal) => signal.defined_at(),
            Self::Field(field) => field.defined_at(),
        }
    }
}

impl<T, S> With for ReadModel<T, S>
where
    S: Storage<ArcField<T>> + Storage<SignalTypes<T, S>> + Storage<T>,
{
    type Value = T;

    fn try_with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        match self {
            Self::Signal(signal) => signal.try_with(fun),
            Self::Field(field) => field.try_with(fun),
        }
    }
}

impl<T, S> WithUntracked for ReadModel<T, S>
where
    S: Storage<ArcField<T>> + Storage<SignalTypes<T, S>> + Storage<T>,
{
    type Value = T;

    fn try_with_untracked<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> Option<U> {
        match self {
            Self::Signal(signal) => signal.try_with_untracked(fun),
            Self::Field(field) => field.try_with_untracked(fun),
        }
    }
}

#[derive(Clone, Copy)]
enum WriteModel<T, S = SyncStorage>
where
    T: 'static,
    S: Storage<T>,
{
    WriteSignal(WriteSignal<T, S>),
    SignalSetter(ReadModel<T, S>, SignalSetter<T, S>),
    Field(Field<T, S>),
}

impl<T, S> Update for WriteModel<T, S>
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
        match self {
            Self::WriteSignal(write_signal) => write_signal.try_maybe_update(fun),
            Self::SignalSetter(read, signal_setter) => {
                if let Some(mut data) = read.try_get_untracked() {
                    let (did_update, val) = fun(&mut data);
                    if did_update {
                        signal_setter.set(data);
                    }
                    Some(val)
                } else {
                    None
                }
            }
            Self::Field(field) => {
                if let Some(mut writer) = field.writer() {
                    let (did_update, val) = fun(&mut *writer);
                    if !did_update {
                        writer.untrack();
                    }
                    drop(writer);
                    Some(val)
                } else {
                    None
                }
            }
        }
    }
}

impl<T, S> IsDisposed for WriteModel<T, S>
where
    S: Storage<T>,
{
    fn is_disposed(&self) -> bool {
        match self {
            Self::WriteSignal(write_signal) => write_signal.is_disposed(),
            Self::SignalSetter(_, _) => false,
            Self::Field(field) => field.is_disposed(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ReadModel;
    use leptos::prelude::*;

    #[test]
    fn read() {
        let signal = Signal::derive(move || 1);
        let read_model = ReadModel::Signal(signal);
        let _value = read_model.get();
        let _value = read_model.with(|value| *value);
    }
}
