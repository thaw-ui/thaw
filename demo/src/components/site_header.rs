use super::switch_version::SwitchVersion;
use leptos::{ev::MouseEvent, prelude::*};
use leptos_meta::Style;
use leptos_router::hooks::use_navigate;
// use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use thaw::*;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let navigate = use_navigate();
    let navigate_signal = RwSignal::new(use_navigate());
    let dir = ConfigInjection::expect_context().dir;
    let theme = Theme::use_rw_theme();
    let theme_name = Memo::new(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
                "Dark".to_string()
            } else {
                "Light".to_string()
            }
        })
    });
    // let (_, write_theme, _) = use_local_storage::<String, FromToStringCodec>("theme");
    let change_theme = move |_| {
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::light());
            // write_theme.set("light".to_string());
        } else {
            theme.set(Theme::dark());
            // write_theme.set("dark".to_string());
        }
    };

    let search_value = RwSignal::new(String::new());
    let search_all_options = StoredValue::new(gen_search_all_options());

    let search_options = Memo::new(move |_| {
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
    let on_search_select = {
        let navigate = navigate.clone();
        move |path: String| {
            navigate(&path, Default::default());
        }
    };
    let auto_complete_ref = ComponentRef::<AutoCompleteRef>::new();
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::ev;
        let handle = window_event_listener(ev::keydown, move |e| {
            if js_sys::Reflect::has(&e, &js_sys::wasm_bindgen::JsValue::from_str("key"))
                .unwrap_or_default()
            {
                let key = e.key();
                if key == *"/" {
                    if let Some(auto_complete_ref) = auto_complete_ref.get_untracked() {
                        e.prevent_default();
                        auto_complete_ref.focus();
                    }
                }
            }
        });
        on_cleanup(move || handle.remove());
    }

    // let menu_value = use_menu_value(change_theme);
    view! {
        <Style id="demo-header">
            "
                .demo-header {
                    height: 64px;
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0 20px;
                    z-index: 1000;
                    position: relative;
                    border-bottom: 1px solid var(--colorNeutralStroke2);
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
        <LayoutHeader class="demo-header">
            <Space on:click=move |_| {
                navigate("/", Default::default());
            }>
                <img src="/logo.svg" style="width: 36px"/>
                <div class="demo-name">"Thaw UI"</div>
            </Space>
            <Space align=SpaceAlign::Center>
                <AutoComplete
                    value=search_value
                    placeholder="Type '/' to search"
                    clear_after_select=true
                    blur_after_select=true
                    on_select=on_search_select
                    comp_ref=auto_complete_ref
                >
                    <For each=move || search_options.get() key=|option| option.label.clone() let:option>
                        <AutoCompleteOption value=option.value>{option.label}</AutoCompleteOption>
                    </For>
                    <AutoCompletePrefix slot>
                        <Icon
                            icon=icondata::AiSearchOutlined
                            style="font-size: 18px; color: var(--thaw-placeholder-color);"
                        />
                    </AutoCompletePrefix>
                </AutoComplete>
                <Menu
                    position=MenuPosition::BottomEnd
                    on_select=move |value : String| match value.as_str() {
                        "Dark" => change_theme(MouseEvent::new("click").unwrap()),
                        "Light" => change_theme(MouseEvent::new("click").unwrap()),
                        "github" => { _ = window().open_with_url("http://github.com/thaw-ui/thaw"); },
                        "discord" => { _ = window().open_with_url("https://discord.com/channels/1031524867910148188/1270735289437913108"); },
                        _ => navigate_signal.get()(&value, Default::default())

                    }
                >
                    <MenuTrigger slot class="demo-header__menu-mobile">
                    <Button
                            appearance=ButtonAppearance::Subtle
                            icon=icondata::AiUnorderedListOutlined
                            attr:style="font-size: 22px; padding: 0px 6px;"
                        />
                    </MenuTrigger>
                    <MenuItem value=theme_name>{theme_name}</MenuItem>
                    <MenuItem icon=icondata::AiGithubOutlined value="github">"Github"</MenuItem>
                    <MenuItem icon=icondata::BiDiscordAlt value="discord">"Discord"</MenuItem>
                    {
                        use crate::pages::{gen_nav_data, NavGroupOption, NavItemOption};
                        gen_nav_data().into_iter().map(|data| {
                            let NavGroupOption { label, children } = data;
                            view! {
                                <Caption1Strong style="margin-inline-start: 10px; margin-top: 10px; display: block">
                                {label}
                                </Caption1Strong>
                                {
                                    children.into_iter().map(|item| {
                                        let NavItemOption { label, value, .. } = item;
                                        view! {
                                            <MenuItem value=value>{label}</MenuItem>
                                        }
                                    }).collect_view()
                                }
                            }
                        }).collect_view()
                    }
                </Menu>
                <Space class="demo-header__right-btn" align=SpaceAlign::Center>
                    <SwitchVersion/>
                    <Button
                        icon=Memo::new(move |_| {
                            theme.with(|theme| {
                                if theme.name == "light" {
                                    icondata::BiMoonRegular
                                } else {
                                    icondata::BiSunRegular
                                }
                            })
                        })
                        on_click=change_theme
                    >
                        {move || theme_name.get()}
                    </Button>
                    <Button
                        on_click=move |_| {
                            let Some(dir) = dir else {
                                return;
                            };
                            dir.update(move |dir| {
                                *dir = match dir {
                                    ConfigDirection::Auto => ConfigDirection::Ltr,
                                    ConfigDirection::Ltr => ConfigDirection::Rtl,
                                    ConfigDirection::Rtl => ConfigDirection::Ltr,
                                };
                            });
                        }
                    >
                        {move || {
                            let Some(dir) = dir else {
                                return None;
                            };

                            match dir.get() {
                                ConfigDirection::Auto => Some("Auto"),
                                ConfigDirection::Ltr => Some("LTR"),
                                ConfigDirection::Rtl => Some("RTL"),
                            }
                        }}
                    </Button>
                    <Button
                        icon=icondata::BiDiscordAlt
                        on_click=move |_| {
                            _ = window().open_with_url("https://discord.com/channels/1031524867910148188/1270735289437913108");
                        }
                    />

                    <Button
                        icon=icondata::AiGithubOutlined
                        on_click=move |_| {
                            _ = window().open_with_url("http://github.com/thaw-ui/thaw");
                        }
                    />

                </Space>
            </Space>

        </LayoutHeader>
    }
}

#[derive(Clone, PartialEq)]
struct AutoCompleteOption {
    pub label: String,
    pub value: String,
}

fn gen_search_all_options() -> Vec<AutoCompleteOption> {
    crate::pages::gen_nav_data()
        .into_iter()
        .flat_map(|group| {
            group.children.into_iter().map(|item| AutoCompleteOption {
                value: item.value.to_string(),
                label: item.label.to_string(),
            })
        })
        .collect()
}

// TODO
// fn use_menu_value(change_theme: Callback<()>) -> RwSignal<String> {
//     use crate::pages::gen_guide_menu_data;
//     let guide = store_value(gen_guide_menu_data());
//     let navigate = use_navigate();
//     let loaction = use_location();

//     let menu_value = create_rw_signal({
//         let mut pathname = loaction.pathname.get_untracked();
//         if pathname.starts_with("/components/") {
//             pathname.drain(12..).collect()
//         } else if pathname.starts_with("/guide/") {
//             pathname.drain(7..).collect()
//         } else {
//             String::new()
//         }
//     });

//     _ = menu_value.watch(move |name| {
//         if name == "Dark" || name == "Light" {
//             change_theme.call(());
//             return;
//         } else if name == "github" {
//             _ = window().open_with_url("http://github.com/thaw-ui/thaw");
//             return;
//         }
//         let pathname = loaction.pathname.get_untracked();
//         if guide.with_value(|menu| {
//             menu.iter()
//                 .any(|group| group.children.iter().any(|item| &item.value == name))
//         }) {
//             if !pathname.eq(&format!("/guide/{name}")) {
//                 navigate(&format!("/guide/{name}"), Default::default());
//             }
//         } else if !pathname.eq(&format!("/components/{name}")) {
//             navigate(&format!("/components/{name}"), Default::default());
//         }
//     });

//     menu_value
// }
