use super::Fallback;
use leptos::{either::EitherOf3, prelude::*};

#[component]
pub fn OptionComp<T, CF, IV>(
    value: Option<T>,
    children: CF,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView
where
    CF: FnOnce(T) -> IV + 'static,
    IV: IntoView + 'static,
{
    if let Some(value) = value {
        EitherOf3::A(children(value))
    } else if let Some(fallback) = fallback {
        EitherOf3::B((fallback.children)())
    } else {
        EitherOf3::C(())
    }
}
