use crate::components::SiteHeader;
use leptos::prelude::*;
use leptos_meta::Style;
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
        <Style>
            "   .main { width: 1100px; margin: calc(50vh - 200px) auto 0; display: flex; }
                .main > div { width: 50%; }
                @media screen and (max-width: 1100px) {
                    .main { width: 100%; margin: calc(50vh - 280px) 0 0; flex-direction: column-reverse; }
                    .main > div { width: auto; text-align: center; }
                    .main > div > .thaw-space { justify-content: center; }
                }
            "
        </Style>
        <Layout position=LayoutPosition::Absolute>
            <SiteHeader />
            <Layout position=LayoutPosition::Absolute attr:style="top: 64px">
                <div class="main">
                    <div>
                        <h2 style="line-height: 64px; font-size: 56px; margin: 0 0 18px">
                            "Thaw UI"
                        </h2>
                        <p style="line-height: 36px; font-size: 24px">
                            "An easy to use leptos component library"
                        </p>
                        <Space>
                            <Button
                                appearance=ButtonAppearance::Primary
                                on_click=move |_| {
                                    navigate("/guide/installation", Default::default());
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
                    </div>
                    <div style="text-align: center">
                        <img src="/logo.svg" style="width: 200px" />
                    </div>
                </div>
            </Layout>
        </Layout>
    }
}
