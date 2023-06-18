use leptos::*;

#[component]
pub fn LayoutHeader(
    cx: Scope,
    #[prop(optional, into)] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <div class="melt-layout-header" style=move || style.get()>
            { children(cx) }
        </div>
    }
}
