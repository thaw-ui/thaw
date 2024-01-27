use leptos::{
    Memo, ReadSignal, RwSignal, Signal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate,
    SignalWith, SignalWithUntracked, WriteSignal,
};

pub struct Model<T>
where
    T: 'static,
{
    read: Signal<T>,
    write: WriteSignal<T>,
    on_write: Option<WriteSignal<T>>,
}

impl<T: Default> Default for Model<T> {
    fn default() -> Self {
        RwSignal::new(Default::default()).into()
    }
}

impl<T> Clone for Model<T> {
    fn clone(&self) -> Self {
        Self {
            read: self.read.clone(),
            write: self.write.clone(),
            on_write: self.on_write.clone(),
        }
    }
}

impl<T> Copy for Model<T> {}

impl<T> Model<T> {
    fn new(value: T) -> Self {
        let rw_signal = RwSignal::new(value);
        rw_signal.into()
    }

    pub fn signal(&self) -> Signal<T> {
        self.read.clone()
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

impl<T> SignalWith for Model<T> {
    type Value = T;

    fn with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.read.with(f)
    }

    fn try_with<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.read.try_with(f)
    }
}

impl<T> SignalWithUntracked for Model<T> {
    type Value = T;

    fn with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> O {
        self.read.with_untracked(f)
    }

    fn try_with_untracked<O>(&self, f: impl FnOnce(&Self::Value) -> O) -> Option<O> {
        self.read.try_with_untracked(f)
    }
}

impl<T> SignalUpdate for Model<T> {
    type Value = T;

    fn update(&self, f: impl FnOnce(&mut Self::Value)) {
        self.write.update(f);
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut Self::Value) -> O) -> Option<O> {
        self.write.try_update(f)
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
            write,
            on_write: None,
        }
    }
}

impl<T> From<(ReadSignal<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Self {
            read: read.into(),
            write,
            on_write: None,
        }
    }
}

impl<T> From<(Memo<T>, WriteSignal<T>)> for Model<T> {
    fn from((read, write): (Memo<T>, WriteSignal<T>)) -> Self {
        Self {
            read: read.into(),
            write,
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

        runtime.dispose();
    }
}
