use leptos::*;
use super::Fallback;

#[component]
pub fn OptionComp<T, CF, IV>(
    cx: Scope,
    value: Option<T>,
    children: CF,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView
where
    CF: Fn(Scope, T) -> IV + 'static,
    IV: IntoView,
{
    if let Some(value) = value {
        children(cx, value).into_view(cx)
    } else if let Some(fallback) = fallback {
        (fallback.children)(cx).into_view(cx)
    } else {
        ().into_view(cx)
    }
}
