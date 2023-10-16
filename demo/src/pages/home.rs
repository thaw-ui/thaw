use leptos::*;
use leptos_router::{use_navigate, use_query_map};
use melt_ui::*;

#[component]
pub fn Home() -> impl IntoView {
    let query_map = use_query_map().get_untracked();
    if let Some(path) = query_map.get("path") {
        let navigate = use_navigate();
        navigate(path, Default::default());
    }
    view! {
        <Layout
            position=LayoutPosition::Absolute
            style="display: flex; align-items: center; justify-content: center; flex-direction: column;"
        >
            <h1 style="font-size: 80px; line-height: 1;margin: 0 0 18px;">"Melt UI"</h1>
            <p>"An easy to use leptos component library"</p>
            <Space>
                <Button on_click=move |_| {
                    let navigate = use_navigate();
                    navigate("/components/menu", Default::default());
                }>"Read the docs"</Button>
                <Button
                    variant=ButtonVariant::Text
                    on:click=move |_| {
                        _ = window().open_with_url("http://github.com/luoxiaozero/melt-ui");
                    }
                >

                    "Github"
                </Button>
            </Space>
        </Layout>
    }
}
