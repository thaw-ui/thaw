use leptos::*;
use std::collections::HashSet;

#[component]
pub fn CheckboxGroup(
    #[prop(into)] value: RwSignal<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    let injection_key = CheckboxGroupInjectionKey::new(value);
    provide_context(injection_key);

    children()
}

#[derive(Clone)]
pub struct CheckboxGroupInjectionKey {
    pub value: RwSignal<HashSet<String>>,
}

impl CheckboxGroupInjectionKey {
    pub fn new(value: RwSignal<HashSet<String>>) -> Self {
        Self { value }
    }
}

pub fn use_checkbox_group() -> CheckboxGroupInjectionKey {
    expect_context::<CheckboxGroupInjectionKey>()
}
