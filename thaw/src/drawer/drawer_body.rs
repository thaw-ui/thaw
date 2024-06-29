use crate::Scrollbar;
use leptos::*;

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
