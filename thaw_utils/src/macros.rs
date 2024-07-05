#[macro_export]
macro_rules! with {
    (|$ident:ident $(,)?| $body:expr) => {
        $crate::macros::__private::Withable::call_with(&$ident, |$ident| $body)
    };
    (move |$ident:ident $(,)?| $body:expr) => {
        $crate::macros::__private::Withable::call_with(&$ident, move |$ident| $body)
    };
    (|$first:ident, $($rest:ident),+ $(,)? | $body:expr) => {
        $crate::macros::__private::Withable::call_with(
            &$first,
            |$first| with!(|$($rest),+| $body)
        )
    };
    (move |$first:ident, $($rest:ident),+ $(,)? | $body:expr) => {
        $crate::macros::__private::Withable::call_with(
            &$first,
            move |$first| with!(|$($rest),+| $body)
        )
    };
}

#[macro_export]
macro_rules! update {
    (|$ident:ident $(,)?| $body:expr) => {
        $crate::macros::__private::Updatable::call_update(&$ident, |$ident| $body)
    };
    (move |$ident:ident $(,)?| $body:expr) => {
        $crate::macros::__private::Updatable::call_update(&$ident, move |$ident| $body)
    };
    (|$first:ident, $($rest:ident),+ $(,)? | $body:expr) => {
        $crate::macros::__private::Updatable::call_update(
            &$first,
            |$first| update!(|$($rest),+| $body)
        )
    };
    (move |$first:ident, $($rest:ident),+ $(,)? | $body:expr) => {
        $crate::macros::__private::Updatable::call_update(
            &$first,
            move |$first| update!(|$($rest),+| $body)
        )
    };
}

pub mod __private {
    use leptos::prelude::{Update, With};

    pub trait Withable {
        type Value;

        #[track_caller]
        fn call_with<O>(item: &Self, f: impl FnOnce(&Self::Value) -> O) -> O;
    }

    impl<S: With> Withable for S
    where
        <S as With>::Value: Sized,
    {
        type Value = S::Value;

        #[inline(always)]
        fn call_with<O>(item: &Self, f: impl FnOnce(&Self::Value) -> O) -> O {
            item.with(f)
        }
    }

    pub trait Updatable {
        type Value;

        #[track_caller]
        fn call_update(item: &Self, f: impl FnOnce(&mut Self::Value));
    }

    impl<S: Update> Updatable for S {
        type Value = S::Value;

        #[inline(always)]
        fn call_update(item: &Self, f: impl FnOnce(&mut Self::Value)) {
            item.update(f)
        }
    }
}
