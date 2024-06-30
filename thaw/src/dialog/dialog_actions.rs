use leptos::*;

#[component]
pub fn DialogActions(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-dialog-actions">
            {children()}
        </div>
    }
}
