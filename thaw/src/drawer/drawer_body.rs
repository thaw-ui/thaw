use leptos::*;

#[component]
pub fn DrawerBody(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-drawer-body">
            {children()}
        </div>
    }
}