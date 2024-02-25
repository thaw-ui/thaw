use crate::utils::Model;
use leptos::*;

#[component]
pub fn RadioGroup(
    #[prop(optional, into)] value: Model<Option<String>>,
    children: Children,
) -> impl IntoView {
    view! { <Provider value=RadioGroupInjection(value) children/> }
}

#[derive(Clone)]
pub(crate) struct RadioGroupInjection(pub Model<Option<String>>);

pub(crate) fn use_radio_group() -> RadioGroupInjection {
    expect_context()
}
