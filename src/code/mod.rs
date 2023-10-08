use leptos::*;

#[component]
pub fn Code(children: Children) -> impl IntoView {
    view! { <code class="melt-code">{children()}</code> }
}
