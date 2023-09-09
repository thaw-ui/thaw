use crate::components::*;
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
        <Layout position=LayoutPosition::ABSOLUTE>
            <SiteHeader />
            <Layout position=LayoutPosition::ABSOLUTE style="top: 54px; display: flex; align-items: center; justify-content: center; flex-direction: column;">
                <p>"A Leptos UI Library"</p>
                <Button on:click=move |_| {
                    let navigate = use_navigate();
                    navigate("/components/menu", Default::default());
                }>
                    "Read the docs"
                </Button>
            </Layout>
        </Layout>
    }
}
