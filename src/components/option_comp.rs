use leptos::*;

#[component]
pub fn OptionComp<T, VF, IV>(
    cx: Scope,
    value: Option<T>,
    view: VF,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    VF: FnOnce(Scope, T) -> IV + 'static,
    IV: IntoView,
{
    if let Some(value) = value {
        view(cx, value).into_view(cx)
    } else if let Some(children) = children {
        children(cx).into_view(cx)
    } else {
        ().into_view(cx)
    }
}
