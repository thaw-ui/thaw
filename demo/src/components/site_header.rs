use super::switch_version::SwitchVersion;
use leptos::*;
use leptos_meta::Style;
use leptos_router::{use_location, use_navigate};
use thaw::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let theme = use_rw_theme();
    let theme_name = create_memo(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
                "Dark".to_string()
            } else {
                "Light".to_string()
            }
        })
    });
    let change_theme = Callback::new(move |_| {
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::light());
        } else {
            theme.set(Theme::dark());
        }
    });
    let style = create_memo(move |_| {
        theme.with(|theme| format!("border-bottom: 1px solid {}", theme.common.border_color))
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

    let menu_value = use_menu_value(change_theme);
    view! {
        <Style id="demo-header">
            "
                .demo-header {
                    height: 64px;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0 20px;
                }
                .demo-name {
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    height: 100%;
                    font-weight: 600;
                    font-size: 20px;
                }
                .demo-header__menu-mobile {
                    display: none !important;
                }
                .demo-header__menu-popover-mobile {
                    padding: 0;
                }
                .demo-header__right-btn .thaw-select {
                    width: 60px;
                }
                @media screen and (max-width: 600px) {
                    .demo-header {
                        padding: 0 8px;
                    }
                    .demo-name {
                        display: none;
                    }
                }
                @media screen and (max-width: 1200px) {
                    .demo-header__right-btn {
                        display: none !important;
                    }
                    .demo-header__menu-mobile {
                        display: block !important;
                    }
                }
            "
        </Style>
        <LayoutHeader class="demo-header" style>
            <Space
                on:click=move |_| {
                let navigate = use_navigate();
                navigate("/", Default::default());
            }>
                <img src="/logo.svg" style="width: 36px"/>
                <div class="demo-name">
                    "Thaw UI"
                </div>
            </Space>
            <Space>
                <AutoComplete
                    value=search_value
                    placeholder="Type '/' to search"
                    options=search_options
                    clear_after_select=true
                    blur_after_select=true
                    on_select=on_search_select
                    comp_ref=auto_complete_ref
                >
                    <AutoCompletePrefix slot>
                        <Icon icon=icondata::AiSearchOutlined style="font-size: 18px; color: var(--thaw-placeholder-color);"/>
                    </AutoCompletePrefix>
                </AutoComplete>
                <Popover placement=PopoverPlacement::BottomEnd class="demo-header__menu-popover-mobile">
                    <PopoverTrigger slot class="demo-header__menu-mobile">
                        <Button
                            appearance=ButtonAppearance::Subtle
                            icon=icondata::AiUnorderedListOutlined
                            style="font-size: 22px; padding: 0px 6px;"
                        />
                    </PopoverTrigger>
                    <div style="height: 70vh; overflow: auto;">
                        <Menu value=menu_value>
                            <MenuItem key=theme_name label=theme_name />
                            <MenuItem key="github" label="Github" />
                            {
                                use crate::pages::{gen_guide_menu_data, gen_menu_data};
                                vec![
                                    gen_guide_menu_data().into_view(),
                                    gen_menu_data().into_view(),
                                ]
                            }
                        </Menu>
                    </div>
                </Popover>
                <Space class="demo-header__right-btn" align=SpaceAlign::Center>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        on_click=move |_| {
                            let navigate = use_navigate();
                            navigate("/guide/installation", Default::default());
                        }
                    >

                        "Guide"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        on_click=move |_| {
                            let navigate = use_navigate();
                            navigate("/components/button", Default::default());
                        }
                    >

                        "Components"
                    </Button>
                    <Button appearance=ButtonAppearance::Subtle on_click=Callback::new(move |_| change_theme.call(()))>
                        {move || theme_name.get()}
                    </Button>
                    <SwitchVersion />
                    <Button
                        appearance=ButtonAppearance::Subtle
                        icon=icondata::AiGithubOutlined
                        round=true
                        style="font-size: 22px; padding: 0px 6px;"
                        on_click=move |_| {
                            _ = window().open_with_url("http://github.com/thaw-ui/thaw");
                        }
                    />
                </Space>
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

fn use_menu_value(change_theme: Callback<()>) -> RwSignal<String> {
    use crate::pages::gen_guide_menu_data;
    let guide = store_value(gen_guide_menu_data());
    let navigate = use_navigate();
    let loaction = use_location();

    let menu_value = create_rw_signal({
        let mut pathname = loaction.pathname.get_untracked();
        if pathname.starts_with("/components/") {
            pathname.drain(12..).collect()
        } else if pathname.starts_with("/guide/") {
            pathname.drain(7..).collect()
        } else {
            String::new()
        }
    });

    _ = menu_value.watch(move |name| {
        if name == "Dark" || name == "Light" {
            change_theme.call(());
            return;
        } else if name == "github" {
            _ = window().open_with_url("http://github.com/thaw-ui/thaw");
            return;
        }
        let pathname = loaction.pathname.get_untracked();
        if guide.with_value(|menu| {
            menu.iter()
                .any(|group| group.children.iter().any(|item| &item.value == name))
        }) {
            if !pathname.eq(&format!("/guide/{name}")) {
                navigate(&format!("/guide/{name}"), Default::default());
            }
        } else if !pathname.eq(&format!("/components/{name}")) {
            navigate(&format!("/components/{name}"), Default::default());
        }
    });

    menu_value
}
