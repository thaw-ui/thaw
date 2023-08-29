use super::Fallback;
use leptos::*;

#[slot]
pub struct Then {
    children: ChildrenFn,
}

#[slot]
pub struct ElseIf {
    cond: MaybeSignal<bool>,
    children: ChildrenFn,
}

#[component]
pub fn If(
    #[prop(into)] cond: MaybeSignal<bool>,
    then: Then,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView {
    move || {
        if cond.get() {
            (then.children)().into_view()
        } else if let Some(else_if) = else_if.iter().find(|i| i.cond.get()) {
            (else_if.children)().into_view()
        } else if let Some(fallback) = &fallback {
            (fallback.children)().into_view()
        } else {
            ().into_view()
        }
    }
}
