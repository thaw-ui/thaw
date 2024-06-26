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
    Effect::new(move |_| {
        select_name.set(loaction.pathname.get());
    });

    _ = select_name.watch(move |name| {
        let pathname = loaction.pathname.get_untracked();
        if &pathname != name {
            navigate(name, Default::default());
        }
    });
    view! {
        <Style>
            "
            .demo-components__component {
                width: 896px;
                margin: 0 auto;
            }
            .demo-components__toc {
                width: 190px;
                margin: 12px 2px 12px 12px;
            }
            .demo-components__toc > .thaw-anchor {
                position: sticky;
                top: 36px;
            }
            .demo-md-table-box {
                overflow: auto;
            }
            @media screen and (max-width: 1200px) {
                .demo-components__toc,
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
                <div class="demo-components__sider">
                    <NavDrawer selected_value=select_name>

                        {gen_menu_data().into_view()}

                    </NavDrawer>
                </div>
                <Layout content_style="padding: 8px 12px 28px; display: flex;" class="doc-content">
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
            <Caption1Strong style="margin-inline-start: 10px">
                {label}
            </Caption1Strong>
            {children.into_iter().map(|v| v.into_view()).collect_view()}
        }
        .into_view()
    }
}

pub(crate) struct MenuItemOption {
    pub label: String,
    pub value: String,
}

impl IntoView for MenuItemOption {
    fn into_view(self) -> View {
        let Self { label, value } = self;
        view! { <NavItem value>{label}</NavItem> }
    }
}

pub(crate) fn gen_menu_data() -> Vec<MenuGroupOption> {
    vec![
        MenuGroupOption {
            label: "Getting Started".into(),
            children: vec![
                MenuItemOption {
                    value: "/guide/installation".into(),
                    label: "Installation".into(),
                },
                MenuItemOption {
                    value: "/guide/usage".into(),
                    label: "Usage".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Guides".into(),
            children: vec![MenuItemOption {
                value: "/guide/server-sider-rendering".into(),
                label: "Server Sider Rendering".into(),
            }],
        },
        MenuGroupOption {
            label: "Development".into(),
            children: vec![
                MenuItemOption {
                    value: "/guide/development/guide".into(),
                    label: "Guide".into(),
                },
                MenuItemOption {
                    value: "/guide/development/components".into(),
                    label: "Components".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Components".into(),
            children: vec![
                MenuItemOption {
                    value: "/components/accordion".into(),
                    label: "Accordion".into(),
                },
                MenuItemOption {
                    value: "/components/alert".into(),
                    label: "Alert".into(),
                },
                MenuItemOption {
                    value: "/components/anchor".into(),
                    label: "Anchor".into(),
                },
                MenuItemOption {
                    value: "/components/auto-complete".into(),
                    label: "Auto Complete".into(),
                },
                MenuItemOption {
                    value: "/components/avatar".into(),
                    label: "Avatar".into(),
                },
                MenuItemOption {
                    value: "/components/back-top".into(),
                    label: "Back Top".into(),
                },
                MenuItemOption {
                    value: "/components/badge".into(),
                    label: "Badge".into(),
                },
                MenuItemOption {
                    value: "/components/breadcrumb".into(),
                    label: "Breadcrumb".into(),
                },
                MenuItemOption {
                    value: "/components/button".into(),
                    label: "Button".into(),
                },
                MenuItemOption {
                    value: "/components/calendar".into(),
                    label: "Calendar".into(),
                },
                MenuItemOption {
                    value: "/components/card".into(),
                    label: "Card".into(),
                },
                MenuItemOption {
                    value: "/components/checkbox".into(),
                    label: "Checkbox".into(),
                },
                MenuItemOption {
                    value: "/components/color-picker".into(),
                    label: "Color Picker".into(),
                },
                MenuItemOption {
                    value: "/components/config-provider".into(),
                    label: "Config Provider".into(),
                },
                MenuItemOption {
                    value: "/components/date-picker".into(),
                    label: "Date Picker".into(),
                },
                MenuItemOption {
                    value: "/components/divider".into(),
                    label: "Divider".into(),
                },
                MenuItemOption {
                    value: "/components/drawer".into(),
                    label: "Drawer".into(),
                },
                MenuItemOption {
                    value: "/components/grid".into(),
                    label: "Grid".into(),
                },
                MenuItemOption {
                    value: "/components/icon".into(),
                    label: "Icon".into(),
                },
                MenuItemOption {
                    value: "/components/image".into(),
                    label: "Image".into(),
                },
                MenuItemOption {
                    value: "/components/input".into(),
                    label: "Input".into(),
                },
                MenuItemOption {
                    value: "/components/layout".into(),
                    label: "Layout".into(),
                },
                MenuItemOption {
                    value: "/components/loading-bar".into(),
                    label: "Loading Bar".into(),
                },
                MenuItemOption {
                    value: "/components/message".into(),
                    label: "Message".into(),
                },
                MenuItemOption {
                    value: "/components/modal".into(),
                    label: "Modal".into(),
                },
                MenuItemOption {
                    value: "/components/popover".into(),
                    label: "Popover".into(),
                },
                MenuItemOption {
                    value: "/components/progress".into(),
                    label: "Progress".into(),
                },
                MenuItemOption {
                    value: "/components/radio".into(),
                    label: "Radio".into(),
                },
                MenuItemOption {
                    value: "/components/scrollbar".into(),
                    label: "Scrollbar".into(),
                },
                MenuItemOption {
                    value: "/components/select".into(),
                    label: "Select".into(),
                },
                MenuItemOption {
                    value: "/components/skeleton".into(),
                    label: "Skeleton".into(),
                },
                MenuItemOption {
                    value: "/components/slider".into(),
                    label: "Slider".into(),
                },
                MenuItemOption {
                    value: "/components/space".into(),
                    label: "Space".into(),
                },
                MenuItemOption {
                    value: "/components/spin-button".into(),
                    label: "Spin Button".into(),
                },
                MenuItemOption {
                    value: "/components/spinner".into(),
                    label: "Spinner".into(),
                },
                MenuItemOption {
                    value: "/components/switch".into(),
                    label: "Switch".into(),
                },
                MenuItemOption {
                    value: "/components/tab-list".into(),
                    label: "Tab List".into(),
                },
                MenuItemOption {
                    value: "/components/table".into(),
                    label: "Table".into(),
                },
                MenuItemOption {
                    value: "/components/tag".into(),
                    label: "Tag".into(),
                },
                MenuItemOption {
                    value: "/components/text".into(),
                    label: "Text".into(),
                },
                MenuItemOption {
                    value: "/components/textarea".into(),
                    label: "Textarea".into(),
                },
                MenuItemOption {
                    value: "/components/time-picker".into(),
                    label: "Time Picker".into(),
                },
                MenuItemOption {
                    value: "/components/upload".into(),
                    label: "Upload".into(),
                },
            ],
        },
        // MenuGroupOption {
        //     label: "Mobile Components".into(),
        //     children: vec![
        //         MenuItemOption {
        //             value: "/components/nav-bar".into(),
        //             label: "Nav Bar".into(),
        //         },
        //         MenuItemOption {
        //             value: "/components/tabbar".into(),
        //             label: "Tabbar".into(),
        //         },
        //         MenuItemOption {
        //             value: "/components/toast".into(),
        //             label: "Toast".into(),
        //         },
        //     ],
        // },
    ]
}
