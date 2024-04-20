use crate::components::SiteHeader;
use leptos::*;
use leptos_meta::Style;
use leptos_router::{use_location, use_navigate, Outlet};
use thaw::*;

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let navigate = use_navigate();
    let loaction = use_location();

    let select_name = create_rw_signal(String::new());
    create_effect(move |_| {
        let mut pathname = loaction.pathname.get();
        let name = if pathname.starts_with("/components/") {
            pathname.drain(12..).collect()
        } else {
            String::new()
        };
        select_name.set(name);
    });

    _ = select_name.watch(move |name| {
        let pathname = loaction.pathname.get_untracked();
        if !pathname.eq(&format!("/components/{name}")) {
            navigate(&format!("/components/{name}"), Default::default());
        }
    });
    view! {
        <Style>
            "
            .demo-components__component {
                width: 896px;
                margin: 0 auto;
            }
            .demo-md-table-box {
                overflow: auto;
            }
            @media screen and (max-width: 1200px) {
                .demo-components__sider {
                    display: none;
                }
                .demo-components__component {
                    width: 100%;
                }
            }
            "
        </Style>
        <Layout position=LayoutPosition::Absolute>
            <SiteHeader/>
            <Layout has_sider=true position=LayoutPosition::Absolute style="top: 64px;">
                <LayoutSider class="demo-components__sider">
                    <Menu value=select_name>

                        {gen_menu_data().into_view()}

                    </Menu>
                </LayoutSider>
                <Layout content_style="padding: 8px 12px 28px;">
                    <Outlet/>
                </Layout>
            </Layout>
        </Layout>
    }
}

pub(crate) struct MenuGroupOption {
    pub label: String,
    pub children: Vec<MenuItemOption>,
}

impl IntoView for MenuGroupOption {
    fn into_view(self) -> View {
        let Self { label, children } = self;
        view! {
            <MenuGroup label=format!(
                "{label} ({})", children.len()
            )>

                {children.into_iter().map(|v| v.into_view()).collect_view()}

            </MenuGroup>
        }
    }
}

pub(crate) struct MenuItemOption {
    pub label: String,
    pub value: String,
}

impl IntoView for MenuItemOption {
    fn into_view(self) -> View {
        let Self { label, value } = self;
        view! { <MenuItem key=value label/> }
    }
}

pub(crate) fn gen_menu_data() -> Vec<MenuGroupOption> {
    vec![
        MenuGroupOption {
            label: "Common Components".into(),
            children: vec![
                MenuItemOption {
                    value: "avatar".into(),
                    label: "Avatar".into(),
                },
                MenuItemOption {
                    value: "button".into(),
                    label: "Button".into(),
                },
                MenuItemOption {
                    value: "card".into(),
                    label: "Card".into(),
                },
                MenuItemOption {
                    value: "collapse".into(),
                    label: "Collapse".into(),
                },
                MenuItemOption {
                    value: "divider".into(),
                    label: "Divider".into(),
                },
                MenuItemOption {
                    value: "icon".into(),
                    label: "Icon".into(),
                },
                MenuItemOption {
                    value: "tag".into(),
                    label: "Tag".into(),
                },
                MenuItemOption {
                    value: "typography".into(),
                    label: "Typography".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Data Input Components".into(),
            children: vec![
                MenuItemOption {
                    value: "auto-complete".into(),
                    label: "Auto Complete".into(),
                },
                MenuItemOption {
                    value: "color-picker".into(),
                    label: "Color Picker".into(),
                },
                MenuItemOption {
                    value: "checkbox".into(),
                    label: "Checkbox".into(),
                },
                MenuItemOption {
                    value: "date-picker".into(),
                    label: "Date Picker".into(),
                },
                MenuItemOption {
                    value: "input".into(),
                    label: "Input".into(),
                },
                MenuItemOption {
                    value: "input-number".into(),
                    label: "Input Number".into(),
                },
                MenuItemOption {
                    value: "radio".into(),
                    label: "Radio".into(),
                },
                MenuItemOption {
                    value: "select".into(),
                    label: "Select".into(),
                },
                MenuItemOption {
                    value: "slider".into(),
                    label: "Slider".into(),
                },
                MenuItemOption {
                    value: "switch".into(),
                    label: "Switch".into(),
                },
                MenuItemOption {
                    value: "time-picker".into(),
                    label: "Time Picker".into(),
                },
                MenuItemOption {
                    value: "upload".into(),
                    label: "Upload".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Data Display Components".into(),
            children: vec![
                MenuItemOption {
                    value: "calendar".into(),
                    label: "Calendar".into(),
                },
                MenuItemOption {
                    value: "image".into(),
                    label: "Image".into(),
                },
                MenuItemOption {
                    value: "table".into(),
                    label: "Table".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Navigation Components".into(),
            children: vec![
                MenuItemOption {
                    value: "breadcrumb".into(),
                    label: "Breadcrumb".into(),
                },
                MenuItemOption {
                    value: "loading-bar".into(),
                    label: "Loading Bar".into(),
                },
                MenuItemOption {
                    value: "menu".into(),
                    label: "Menu".into(),
                },
                MenuItemOption {
                    value: "tabs".into(),
                    label: "Tabs".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Feedback Components".into(),
            children: vec![
                MenuItemOption {
                    value: "alert".into(),
                    label: "Alert".into(),
                },
                MenuItemOption {
                    value: "badge".into(),
                    label: "Badge".into(),
                },
                MenuItemOption {
                    value: "drawer".into(),
                    label: "Drawer".into(),
                },
                MenuItemOption {
                    value: "message".into(),
                    label: "Message".into(),
                },
                MenuItemOption {
                    value: "modal".into(),
                    label: "Modal".into(),
                },
                MenuItemOption {
                    value: "popover".into(),
                    label: "Popover".into(),
                },
                MenuItemOption {
                    value: "progress".into(),
                    label: "Progress".into(),
                },
                MenuItemOption {
                    value: "spinner".into(),
                    label: "Spinner".into(),
                },
                MenuItemOption {
                    value: "skeleton".into(),
                    label: "Skeleton".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Layout Components".into(),
            children: vec![
                MenuItemOption {
                    value: "layout".into(),
                    label: "Layout".into(),
                },
                MenuItemOption {
                    value: "grid".into(),
                    label: "Grid".into(),
                },
                MenuItemOption {
                    value: "space".into(),
                    label: "Space".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Utility Components".into(),
            children: vec![MenuItemOption {
                value: "scrollbar".into(),
                label: "Scrollbar".into(),
            }],
        },
        MenuGroupOption {
            label: "Config Components".into(),
            children: vec![MenuItemOption {
                value: "theme".into(),
                label: "Theme".into(),
            }],
        },
        MenuGroupOption {
            label: "Mobile Components".into(),
            children: vec![
                MenuItemOption {
                    value: "nav-bar".into(),
                    label: "Nav Bar".into(),
                },
                MenuItemOption {
                    value: "tabbar".into(),
                    label: "Tabbar".into(),
                },
                MenuItemOption {
                    value: "toast".into(),
                    label: "Toast".into(),
                },
            ],
        },
    ]
}
