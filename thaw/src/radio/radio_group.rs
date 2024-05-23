use leptos::*;
use thaw_utils::Model;

#[component]
pub fn RadioGroup(
    #[prop(optional, into)] value: Model<Option<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value=RadioGroupInjection(value)>
            <div class="thaw-radio-group" role="radiogroup" style="display: flex;align-items: flex-start">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct RadioGroupInjection(pub Model<Option<String>>);

impl RadioGroupInjection {
    pub fn use_() -> RadioGroupInjection {
        expect_context()
    }
}
