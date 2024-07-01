use super::Fallback;
use leptos::{prelude::*, tachys::view::any_view::IntoAny};

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
        children(value).into_any()
    } else if let Some(fallback) = fallback {
        (fallback.children)().into_any()
    } else {
        ().into_any()
    };
}
