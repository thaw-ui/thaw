use leptos::prelude::*;

#[component]
pub fn Skeleton(children: Children) -> impl IntoView {
    view! {
        <div
            role="progressbar"
            aria-busy="true"
            class="thaw-skeleton"
        >
            {children()}
        </div>
    }
}
