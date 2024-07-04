use leptos::{context::Provider, prelude::*};
use thaw_utils::Model;

#[component]
pub fn RadioGroup(
    #[prop(optional, into)] value: Model<Option<String>>,
    /// The name of this radio group.
    #[prop(optional, into)]
    name: Option<String>,
    children: Children,
) -> impl IntoView {
    let name = name.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    view! {
        <Provider value=RadioGroupInjection{ value, name }>
            <div class="thaw-radio-group" role="radiogroup" style="display: flex;align-items: flex-start">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct RadioGroupInjection {
    pub value: Model<Option<String>>,
    pub name: String,
}

impl RadioGroupInjection {
    pub fn use_() -> RadioGroupInjection {
        expect_context()
    }
}
