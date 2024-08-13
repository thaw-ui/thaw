use super::{toaster::Toaster, ToastPosition, ToasterInjection};
use leptos::{context::Provider, prelude::*};

#[component]
pub fn ToasterProvider(
    /// The position the toast should render.
    #[prop(optional)] position: ToastPosition,
    children: Children,
) -> impl IntoView {
    let (injection, receiver) = ToasterInjection::channel();
    view! {
        <Toaster receiver position/>
        <Provider value=injection>
            {children()}
        </Provider>
    }
}
