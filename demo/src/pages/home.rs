use leptos::prelude::*;
use leptos_router::hooks::{use_navigate, use_query_map};
use thaw::*;

#[component]
pub fn Home() -> impl IntoView {
    let query_map = use_query_map().get_untracked();
    let navigate = use_navigate();

    // mobile page
    if let Some(path) = query_map.get("path") {
        navigate(&path, Default::default());
    }
    view! {
        <Layout
            position=LayoutPosition::Absolute
            content_style="display: flex; align-items: center; justify-content: center; flex-direction: column;"
        >
            <h1 style="font-size: 80px; line-height: 1;margin: 0 0 18px;">"Thaw UI"</h1>
            <p>"An easy to use leptos component library"</p>
            <Space>
                <Button
                    appearance=ButtonAppearance::Primary
                    on_click=move |_| {
                        navigate("/components/button", Default::default());
                    }
                >
                    "Read the docs"
                </Button>
                <Button
                    appearance=ButtonAppearance::Subtle
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
