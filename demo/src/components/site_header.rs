use leptos::*;
use leptos_router::use_navigate;
use thaw::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let theme = use_rw_theme();
    let theme_name = create_memo(move |_| {
        theme.with(|theme| {
            if theme.name == "light".to_string() {
                "Dark"
            } else {
                "Light"
            }
        })
    });
    let on_theme = move |_| {
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::light())
        } else {
            theme.set(Theme::dark())
        }
    };
    let style = create_memo(move |_| {
        theme.with(|theme| {
            format!("height: 64px; display: flex; align-items: center; justify-content: space-between; padding: 0 20px; border-bottom: 1px solid {}", theme.common.border_color)
        })
    });
    view! {
        <LayoutHeader style>
            <Space>
                <img src="/thaw/logo.svg" style="width: 36px"/>
                <div
                    style="cursor: pointer; display: flex; align-items: center; height: 100%; font-weight: 600; font-size: 20px"
                    on:click=move |_| {
                        let navigate = use_navigate();
                        navigate("/", Default::default());
                    }
                >

                    "Thaw UI"
                </div>
            </Space>
            <Space>
                <Button
                    variant=ButtonVariant::Text
                    on:click=move |_| {
                        let navigate = use_navigate();
                        navigate("/guide/installation", Default::default());
                    }
                >
                    "Guide"
                </Button>
                <Button
                    variant=ButtonVariant::Text
                    on:click=move |_| {
                        let navigate = use_navigate();
                        navigate("/components/button", Default::default());
                    }
                >
                    "Components"
                </Button>
                <Button
                    variant=ButtonVariant::Text
                    on:click=on_theme
                >
                    {move || theme_name.get()}
                </Button>
                <Button
                    variant=ButtonVariant::Text
                    icon=icondata::AiIcon::AiGithubOutlined
                    round=true
                    style="font-size: 22px; padding: 0px 6px;"
                    on:click=move |_| {
                        _ = window().open_with_url("http://github.com/thaw-ui/thaw");
                    }
                >
                </Button>
            </Space>

        </LayoutHeader>
    }
}
