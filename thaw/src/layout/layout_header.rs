use leptos::prelude::*;

#[component]
pub fn LayoutHeader(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-layout-header">
            {children()}
        </div>
    }
}
