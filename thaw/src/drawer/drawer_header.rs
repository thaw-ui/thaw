use leptos::*;

#[component]
pub fn DrawerHeader(children: Children) -> impl IntoView {
    view! {
        <header class="thaw-drawer-header">
            {children()}
        </header>
    }
}