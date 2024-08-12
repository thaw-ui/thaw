use leptos::{context::Provider, prelude::*};
use std::collections::HashSet;
use thaw_utils::{class_list, Model};

#[component]
pub fn CheckboxGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Sets the value of the checkbox group.
    #[prop(optional, into)]
    value: Model<HashSet<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value=CheckboxGroupInjection(value)>
            <div class=class_list!["thaw-checkbox-group", class] role="group">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct CheckboxGroupInjection(pub Model<HashSet<String>>);

impl Copy for CheckboxGroupInjection {}

impl CheckboxGroupInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
