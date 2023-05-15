use super::Fallback;
use leptos::*;

#[component]
pub fn OptionComp<T, VF, IV>(
    cx: Scope,
    value: Option<T>,
    view: VF,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView
where
    VF: Fn(Scope, T) -> IV + 'static,
    IV: IntoView,
{
    if let Some(value) = value {
        view(cx, value).into_view(cx)
    } else if let Some(fallback) = &fallback {
        (fallback.children)(cx).into_view(cx)
    } else {
        ().into_view(cx)
    }
}
