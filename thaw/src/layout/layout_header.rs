use leptos::prelude::*;
use thaw_utils::{class_list, OptionalProp};

#[component]
pub fn LayoutHeader(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] style: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class_list!["thaw-layout-header", class.map(| c | move || c.get())]
            style=style.map(|s| move || s.get())
        >
            {children()}
        </div>
    }
}
