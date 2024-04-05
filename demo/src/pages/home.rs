use leptos::*;
use leptos_router::{use_navigate, use_query_map};
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {
    let query_map = use_query_map().get_untracked();
    // mobile page
    if let Some(path) = query_map.get("path") {
        let navigate = use_navigate();
        navigate(path, Default::default());
    }
    view! {
        <Layout
            position=LayoutPosition::Absolute
            content_style="display: flex; align-items: center; justify-content: center; flex-direction: column;"
        >
            <h1 style="font-size: 80px; line-height: 1;margin: 0 0 18px;">"Thaw UI"</h1>
            <p>"An easy to use leptos component library"</p>
            <Space>
                <Button on_click=move |_| {
                    let navigate = use_navigate();
                    navigate("/components/button", Default::default());
                }>"Read the docs"</Button>
                <Button
                    variant=ButtonVariant::Text
                    on_click=move |_| {
                        _ = window().open_with_url("http://github.com/thaw-ui/thaw");
                    }
                >

                    "Github"
                </Button>
            </Space>
        </Layout>
    }
}
