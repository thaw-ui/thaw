use leptos::*;
use leptos_router::use_navigate;
use thaw::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let theme = use_rw_theme();
    let theme_name = create_memo(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
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
            let search_value = search_value.to_lowercase().replace([' ', '-'], "");
            let search_value = if search_value.len() > 20 {
                search_value.split_at(20).0
            } else {
                &search_value
            };
            options
                .iter()
                .filter(|option| {
                    let label = option.label.to_lowercase().replace([' ', '-'], "");
                    match_value(search_value, label)
                })
                .cloned()
                .collect()
        })
    });
    let on_search_select = move |path: String| {
        let navigate = use_navigate();
        navigate(&path, Default::default());
    };
    let auto_complete_ref = create_component_ref::<AutoCompleteRef>();
    let handle = window_event_listener(ev::keydown, move |event| {
        let key = event.key();
        if key == *"/" {
            if let Some(auto_complete_ref) = auto_complete_ref.get_untracked() {
                event.prevent_default();
                auto_complete_ref.focus();
            }
        }
    });
    on_cleanup(move || handle.remove());
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
                <AutoComplete
                    value=search_value
                    placeholder="Type '/' to search"
                    options=search_options
                    clear_after_select=true
                    on_select=on_search_select
                    comp_ref=auto_complete_ref
                >
                    <AutoCompletePrefix slot>
                        <Icon icon=icondata::Icon::from(icondata::AiIcon::AiSearchOutlined) style="font-size: 18px; color: var(--thaw-placeholder-color);"/>
                    </AutoCompletePrefix>
                </AutoComplete>
                <Button
                    variant=ButtonVariant::Text
                    on_click=move |_| {
                        let navigate = use_navigate();
                        navigate("/guide/installation", Default::default());
                    }
                >

                    "Guide"
                </Button>
                <Button
                    variant=ButtonVariant::Text
                    on_click=move |_| {
                        let navigate = use_navigate();
                        navigate("/components/button", Default::default());
                    }
                >

                    "Components"
                </Button>
                <Button variant=ButtonVariant::Text on_click=on_theme>
                    {move || theme_name.get()}
                </Button>
                <Button
                    variant=ButtonVariant::Text
                    icon=icondata::AiIcon::AiGithubOutlined
                    round=true
                    style="font-size: 22px; padding: 0px 6px;"
                    on_click=move |_| {
                        _ = window().open_with_url("http://github.com/thaw-ui/thaw");
                    }
                />

            </Space>

        </LayoutHeader>
    }
}

fn gen_search_all_options() -> Vec<AutoCompleteOption> {
    use crate::pages::{gen_guide_menu_data, gen_menu_data};
    let mut options: Vec<_> = gen_menu_data()
        .into_iter()
        .flat_map(|group| {
            group.children.into_iter().map(|item| AutoCompleteOption {
                value: format!("/components/{}", item.value),
                label: item.label,
            })
        })
        .collect();
    options.extend(gen_guide_menu_data().into_iter().flat_map(|group| {
        group.children.into_iter().map(|item| AutoCompleteOption {
            value: format!("/guide/{}", item.value),
            label: item.label,
        })
    }));
    options
}
