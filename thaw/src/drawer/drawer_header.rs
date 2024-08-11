use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn DrawerHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <header class=class_list!["thaw-drawer-header", class]>
            {children()}
        </header>
    }
}
