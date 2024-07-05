use leptos::prelude::*;
use thaw_utils::{class_list, OptionalProp};

#[component]
pub fn LayoutHeader(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class_list!["thaw-layout-header", class.map(| c | move || c.get())]
        >
            {children()}
        </div>
    }
}
