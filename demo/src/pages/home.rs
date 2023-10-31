use leptos::*;
use leptos_router::{use_navigate, use_query_map, Url};
use melt_ui::*;

#[component]
pub fn Home() -> impl IntoView {
    let query_map = use_query_map().get_untracked();
    // mobile page
    if let Some(path) = query_map.get("path") {
        let navigate = use_navigate();
        if let Some((_, search)) = path.split_once("?") {
            if let Some((key, value)) = search.split_once("=") {
                if key == "theme" {
                    let theme = use_rw_theme();
                    let theme_name = theme.with_untracked(|theme| theme.name.clone());
                    if value == "light" && theme_name != "light" {
                        theme.set(Theme::light())
                    } else if value == "dark" && theme_name != "dark" {
                        theme.set(Theme::dark())
                    }
                }
            }
        }
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
