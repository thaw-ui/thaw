use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn DrawerBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into, default = true)] scrollable: Signal<bool>,
    children: Children,
) -> impl IntoView {
    move || match scrollable.get() {
        true => view! {
            <Scrollbar>
                <div class=class_list!["thaw-drawer-body", class]>{children()}</div>
            </Scrollbar>
        },
        false => view! {
            <div class=class_list!["thaw-drawer-body", class]>{children()}</div>
        },
    }
}
