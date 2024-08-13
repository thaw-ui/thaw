use leptos::{context::Provider, prelude::*};
use thaw_utils::{class_list, OptionModel};

#[component]
pub fn RadioGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The selected Radio item in this group.
    #[prop(optional, into)]
    value: OptionModel<String>,
    /// The name of this radio group.
    #[prop(optional, into)]
    name: Option<String>,
    children: Children,
) -> impl IntoView {
    let name = name.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    view! {
        <Provider value=RadioGroupInjection { value, name }>
            <div
                class=class_list!["thaw-radio-group", class]
                role="radiogroup"
                style="display: flex;align-items: flex-start"
            >
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct RadioGroupInjection {
    pub value: OptionModel<String>,
    pub name: String,
}

impl RadioGroupInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
