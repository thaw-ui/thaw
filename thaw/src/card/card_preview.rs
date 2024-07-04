use leptos::prelude::*;

#[component]
pub fn CardPreview(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-card-preview" style="position: relative">
            {children()}
        </div>
    }
}
