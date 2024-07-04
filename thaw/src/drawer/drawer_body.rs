use crate::Scrollbar;
use leptos::prelude::*;

#[component]
pub fn DrawerBody(children: Children) -> impl IntoView {
    view! {
        <Scrollbar>
            <div class="thaw-drawer-body">
                {children()}
            </div>
        </Scrollbar>
    }
}
