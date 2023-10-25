use leptos::*;
use leptos_router::use_navigate;
use melt_ui::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let theme = use_rw_theme();
    let theme_name = create_memo(move |_| theme.with(|theme| theme.name.clone()));
    let on_theme = move |_| {
        if theme_name.get_untracked() != "Light".to_string() {
            theme.set(Theme::light())
        } else {
            theme.set(Theme::dark())
        }
    };
    view! {
        <LayoutHeader style="height: 54px; display: flex; align-items: center; justify-content: space-between; padding: 0 20px; border-bottom: 1px solid #e5e8eb">
            <span
                style="cursor: pointer"
                on:click=move |_| {
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                }
            >

                "Melt UI"
            </span>
            <Space>
                <Button
                    variant=ButtonVariant::Text
                    on:click=on_theme
                >
                    {move || theme_name.get()}
                </Button>
                <Button
                    variant=ButtonVariant::Text
                    on:click=move |_| {
                        _ = window().open_with_url("http://github.com/luoxiaozero/melt-ui");
                    }
                >

                    "Github"
                </Button>
            </Space>

        </LayoutHeader>
    }
}
