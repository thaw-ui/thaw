use leptos::{MaybeSignal, Memo, ReadSignal, RwSignal, Signal};
use std::ops::{Deref, DerefMut};

pub struct OptionalProp<T>(Option<T>);

impl<T> Default for OptionalProp<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> OptionalProp<T> {
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        match self.0 {
            Some(x) => Some(f(x)),
            None => None,
        }
    }

    pub fn into_option(self) -> Option<T> {
        self.0
    }
}

impl<T> Deref for OptionalProp<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OptionalProp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for OptionalProp<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl From<&str> for OptionalProp<String> {
    fn from(value: &str) -> Self {
        Self(Some(value.to_string()))
    }
}

impl From<&str> for OptionalProp<MaybeSignal<String>> {
    fn from(value: &str) -> Self {
        Self(Some(MaybeSignal::from(value.to_string())))
    }
}

impl From<String> for OptionalProp<MaybeSignal<String>> {
    fn from(value: String) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T> From<ReadSignal<T>> for OptionalProp<MaybeSignal<T>> {
    fn from(value: ReadSignal<T>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T> From<RwSignal<T>> for OptionalProp<MaybeSignal<T>> {
    fn from(value: RwSignal<T>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T> From<Memo<T>> for OptionalProp<MaybeSignal<T>> {
    fn from(value: Memo<T>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T> From<Signal<T>> for OptionalProp<MaybeSignal<T>> {
    fn from(value: Signal<T>) -> Self {
        Self(Some(MaybeSignal::from(value)))
    }
}

impl<T> From<Option<T>> for OptionalProp<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::OptionalProp;
    use leptos::MaybeSignal;

    #[test]
    fn from() {
        let _prop: OptionalProp<MaybeSignal<String>> = "prop".into();
        let _prop: OptionalProp<MaybeSignal<String>> = "prop".to_string().into();
        let _prop: OptionalProp<String> = "prop".into();
    }
}
