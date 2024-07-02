use leptos::*;
use std::collections::HashSet;
use thaw_utils::Model;

#[component]
pub fn CheckboxGroup(
    #[prop(optional, into)] value: Model<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value=CheckboxGroupInjection(value)>
            <div class="thaw-checkbox-group" role="group">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct CheckboxGroupInjection(pub Model<HashSet<String>>);

impl Copy for CheckboxGroupInjection {}

impl CheckboxGroupInjection {
    pub fn use_() -> Option<Self> {
        use_context()
    }
}
