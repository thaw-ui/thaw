use crate::Scrollbar;
use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn DrawerBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <Scrollbar>
            <div class=class_list!["thaw-drawer-body", class]>
                {children()}
            </div>
        </Scrollbar>
    }
}
