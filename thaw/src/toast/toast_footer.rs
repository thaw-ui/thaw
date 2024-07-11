use leptos::prelude::*;

#[component]
pub fn ToastFooter(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-toast-footer">
            {children()}
        </div>
    }
}
