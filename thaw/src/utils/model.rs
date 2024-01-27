use leptos::{
    Memo, ReadSignal, RwSignal, Signal, SignalGet, SignalGetUntracked, SignalSet, SignalSetter,
    WriteSignal,
};

pub struct Model<T>
where
    T: 'static,
{
    read: Signal<T>,
    write: SignalSetter<T>,
    on_write: Option<SignalSetter<T>>,
}

impl<T: Default> Default for Model<T> {
    fn default() -> Self {
        RwSignal::new(Default::default()).into()
    }
}

impl<T> Model<T> {
    fn new(value: T) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }
}

impl<T: Clone> SignalGet for Model<T> {
    type Value = T;

    fn get(&self) -> Self::Value {
        self.read.get()
    }

    fn try_get(&self) -> Option<Self::Value> {
        self.read.try_get()
    }
}

impl<T: Clone> SignalGetUntracked for Model<T> {
    type Value = T;

    fn get_untracked(&self) -> Self::Value {
        self.read.get_untracked()
    }

    fn try_get_untracked(&self) -> Option<Self::Value> {
        self.read.try_get_untracked()
    }
}

impl<T: Clone> SignalSet for Model<T> {
    type Value = T;

    fn set(&self, new_value: Self::Value) {
        if let Some(on_write) = self.on_write.as_ref() {
            on_write.set(new_value.clone());
        }
        self.write.set(new_value);
    }

    fn try_set(&self, new_value: Self::Value) -> Option<Self::Value> {
        if let Some(on_write) = self.on_write.as_ref() {
            on_write.try_set(new_value.clone());
        }
        self.write.try_set(new_value)
    }
}

impl<T> From<T> for Model<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> From<RwSignal<T>> for Model<T> {
    fn from(rw_signal: RwSignal<T>) -> Self {
        let (read, write) = rw_signal.split();
        Self {
            read: read.into(),
            write: write.into(),
            on_write: None,
        }
    }
}

impl<T> From<(ReadSignal<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self {
            read: read.into(),
            write: write.into(),
            on_write: None,
        }
    }
}

impl<T, F> From<(ReadSignal<T>, F)> for Model<T>
where
    F: Fn(T) + 'static,
{
    fn from((read, write): (ReadSignal<T>, F)) -> Self {
        Self {
            read: read.into(),
            write: SignalSetter::map(write),
            on_write: None,
        }
    }
}

impl<T> From<(Memo<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self {
            read: read.into(),
            write: write.into(),
            on_write: None,
        }
    }
}

impl<T, F> From<(Memo<T>, F)> for Model<T>
where
    F: Fn(T) + 'static,
{
    fn from((read, write): (Memo<T>, F)) -> Self {
        Self {
            read: read.into(),
            write: SignalSetter::map(write),
            on_write: None,
        }
    }
}

impl<T: Default> From<(Option<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Option<T>, WriteSignal<T>)) -> Self {
        let mut modal = Self::new(read.unwrap_or_default());
        modal.on_write = Some(write.into());
        modal
    }
}

impl<T: Default, F> From<(Option<T>, F)> for Model<T>
where
    F: Fn(T) + 'static,
{
    fn from((read, write): (Option<T>, F)) -> Self {
        let mut modal = Self::new(read.unwrap_or_default());
        modal.on_write = Some(SignalSetter::map(write));
        modal
    }
}

#[cfg(test)]
mod test {
    use super::Model;
    use leptos::*;

    #[test]
    fn from() {
        let runtime = create_runtime();

        // T
        let modal: Model<i32> = 0.into();
        assert_eq!(modal.get_untracked(), 0);
        modal.set(1);
        assert_eq!(modal.get_untracked(), 1);

        // RwSignal
        let rw_signal = RwSignal::new(0);
        let modal: Model<i32> = rw_signal.into();
        assert_eq!(modal.get_untracked(), 0);
        modal.set(1);
        assert_eq!(modal.get_untracked(), 1);

        // Read Write
        let (read, write) = create_signal(0);
        let modal: Model<i32> = (read, write).into();
        assert_eq!(modal.get_untracked(), 0);
        modal.set(1);
        assert_eq!(modal.get_untracked(), 1);

        // Read Fn
        let (read, _) = create_signal(0);
        let modal: Model<i32> = (read, move |_| {}).into();
        assert_eq!(modal.get_untracked(), 0);
        modal.set(1);
        assert_eq!(modal.get_untracked(), 0);

        // Memo Fn
        let modal: Model<i32> = (Memo::new(move |_| 0), move |_| {}).into();
        assert_eq!(modal.get_untracked(), 0);

        // Option<T> Fn
        let modal: Model<i32> = (Some(1), move |_| {}).into();
        assert_eq!(modal.get_untracked(), 1);

        // Option<T> Fn
        let modal: Model<i32> = (None, move |_| {}).into();
        assert_eq!(modal.get_untracked(), 0);

        runtime.dispose();
    }
}
