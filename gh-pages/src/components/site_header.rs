use leptos::*;
use leptos_router::use_navigate;
use melt_ui::*;

#[component]
pub fn SiteHeader(cx: Scope) -> impl IntoView {
    view! { cx,
        <LayoutHeader
            style="height: 54px; display: flex; align-items: center; justify-content: space-between; padding: 0 20px; border-bottom: 1px solid #efeff6"
        >
            <span style="cursor: pointer" on:click=move |_| {
                let navigate = use_navigate(cx);
                _ = navigate("/", Default::default());
            }>
                "Melt UI"
            </span>
            <Button type_=ButtonType::TEXT on:click=move |_| {
                _ = window().open_with_url("http://github.com/luoxiaozero/melt-ui");
            }>
                "Github"
            </Button>
        </LayoutHeader>
    }
}
