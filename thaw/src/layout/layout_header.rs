use crate::utils::class_list::class_list;
use leptos::*;

#[component]
pub fn LayoutHeader(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(optional, into)] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-layout-header", move || class.get()] style=move || style.get()>
            {children()}
        </div>
    }
}
