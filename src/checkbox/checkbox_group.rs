use crate::utils::Provider;
use leptos::*;
use std::collections::HashSet;

#[component]
pub fn CheckboxGroup(
    #[prop(optional, into)] value: RwSignal<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    view! { <Provider value=CheckboxGroupInjection(value) children/> }
}

#[derive(Clone)]
pub(crate) struct CheckboxGroupInjection(pub RwSignal<HashSet<String>>);

pub(crate) fn use_checkbox_group() -> CheckboxGroupInjection {
    expect_context()
}
