use leptos::prelude::*;
use thaw_utils::class_list;

use crate::Scrollbar;

#[component]
pub fn DrawerBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = true)] scrollable: bool,
    children: Children,
) -> impl IntoView {
    match scrollable {
        true => view! {
            <Scrollbar>
                <div class=class_list!["thaw-drawer-body", class]>{children()}</div>
            </Scrollbar>
        }
        .into_any(),
        false => view! {
            <div class=class_list!["thaw-drawer-body", class]>{children()}</div>
        }
        .into_any(),
    }
}
