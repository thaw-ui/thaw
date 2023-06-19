use leptos::*;

#[component]
pub fn Code(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <code class="melt-code">
            { children(cx) }
        </code>
    }
}
