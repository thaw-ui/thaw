use super::{toaster::Toaster, ToastPosition, ToasterInjection};
use leptos::*;

#[component]
pub fn ToasterProvider(
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
