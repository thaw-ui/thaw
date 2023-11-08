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
    let search_value = create_rw_signal(String::new());
    let search_all_options = store_value(gen_search_all_options());
    let search_options = create_memo(move |_| {
        let search_value = search_value.get();
        if search_value.is_empty() {
            return vec![];
        }
        fn match_value(pattern: &str, value: String) -> bool {
            if pattern.is_empty() {
                return true;
            }
            if value.is_empty() {
                return false;
            }
            if pattern.chars().next() == value.chars().next() {
                return match_value(pattern.split_at(1).1, value.split_at(1).1.to_string());
            }
            match_value(pattern, value.split_at(1).1.to_string())
        }
        search_all_options.with_value(|options| {
            let search_value = search_value
                .to_lowercase()
                .replace(" ", "")
                .replace("-", "");
            let search_value = if search_value.len() > 20 {
                search_value.split_at(20).0
            } else {
                &search_value
            };
            options
                .iter()
                .filter(|option| {
                    let label = option
                        .label
                        .to_lowercase()
                        .replace(" ", "")
                        .replace("-", "");
                    match_value(search_value, label)
                })
                .cloned()
                .collect()
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
                <AutoComplete value=search_value placeholder="Search" options=search_options/>
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

fn gen_search_all_options() -> Vec<AutoCompleteOption> {
    use crate::pages::{gen_guide_menu_data, gen_menu_data};
    let mut options: Vec<_> = gen_menu_data()
        .into_iter()
        .map(|group| {
            group.children.into_iter().map(|item| AutoCompleteOption {
                value: format!("/components/{}", item.value),
                label: item.label,
            })
        })
        .flatten()
        .collect();
    options.extend(
        gen_guide_menu_data()
            .into_iter()
            .map(|group| {
                group.children.into_iter().map(|item| AutoCompleteOption {
                    value: format!("/guide/{}", item.value),
                    label: item.label,
                })
            })
            .flatten(),
    );
    options
}
