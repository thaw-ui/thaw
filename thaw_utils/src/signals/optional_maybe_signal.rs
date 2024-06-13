use leptos::prelude::*;
use std::ops::Deref;

pub struct OptionalMaybeSignal<T: 'static>(MaybeSignal<Option<T>>);

impl<T> Default for OptionalMaybeSignal<T> {
    fn default() -> Self {
        Self(MaybeSignal::Static(None))
    }
}

impl<T: Copy> Copy for OptionalMaybeSignal<T> {}

impl<T: Clone> Clone for OptionalMaybeSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for OptionalMaybeSignal<T> {
    type Target = MaybeSignal<Option<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for OptionalMaybeSignal<T> {
    fn from(value: T) -> Self {
        Self(MaybeSignal::Static(Some(value)))
    }
}

impl<T> From<Option<T>> for OptionalMaybeSignal<T> {
    fn from(value: Option<T>) -> Self {
        Self(MaybeSignal::Static(value))
    }
}

impl<T: Send + Sync> From<ReadSignal<Option<T>>> for OptionalMaybeSignal<T> {
    fn from(value: ReadSignal<Option<T>>) -> Self {
        Self(MaybeSignal::Dynamic(value.into()))
    }
}

impl<T: Send + Sync> From<RwSignal<Option<T>>> for OptionalMaybeSignal<T> {
    fn from(value: RwSignal<Option<T>>) -> Self {
        Self(MaybeSignal::Dynamic(value.into()))
    }
}

impl<T: Send + Sync> From<Memo<Option<T>>> for OptionalMaybeSignal<T> {
    fn from(value: Memo<Option<T>>) -> Self {
        Self(MaybeSignal::Dynamic(value.into()))
    }
}

impl<T> From<Signal<Option<T>>> for OptionalMaybeSignal<T> {
    fn from(value: Signal<Option<T>>) -> Self {
        Self(MaybeSignal::Dynamic(value))
    }
}

impl<T> From<MaybeSignal<Option<T>>> for OptionalMaybeSignal<T> {
    fn from(value: MaybeSignal<Option<T>>) -> Self {
        Self(value)
    }
}

// TODO
// #[cfg(test)]
// mod test {
//     use super::OptionalMaybeSignal;
//     use leptos::{create_runtime, MaybeSignal};

//     #[test]
//     fn into() {
//         let runtime = create_runtime();

//         let _: MaybeSignal<i32> = 12.into();
//         let _: OptionalMaybeSignal<i32> = Some(12).into();
//         let _: OptionalMaybeSignal<i32> = MaybeSignal::Static(Some(12)).into();

//         runtime.dispose();
//     }
// }
