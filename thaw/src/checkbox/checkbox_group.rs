use leptos::*;
use std::collections::HashSet;
use thaw_utils::Model;

#[component]
pub fn CheckboxGroup(
    #[prop(optional, into)] value: Model<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    view! { <Provider value=CheckboxGroupInjection(value) children/> }
}

#[derive(Clone)]
pub(crate) struct CheckboxGroupInjection(pub Model<HashSet<String>>);

pub(crate) fn use_checkbox_group() -> CheckboxGroupInjection {
    expect_context()
}
