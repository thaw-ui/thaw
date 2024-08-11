use crate::components::SiteHeader;
use leptos::prelude::*;
use leptos_meta::Style;
use leptos_router::{
    components::Outlet,
    hooks::{use_location, use_navigate},
};
use thaw::*;

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let navigate = use_navigate();
    let loaction = use_location();

    let select_name = RwSignal::new(String::new());
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
            <Layout has_sider=true position=LayoutPosition::Absolute attr:style="top: 64px;">
                <div class="demo-components__sider">
                    <NavDrawer selected_value=select_name>
                        {
                            gen_nav_data().into_iter().map(|data| {
                                let NavGroupOption { label, children } = data;
                                view! {
                                    <Caption1Strong style="margin-inline-start: 10px; margin-top: 10px; display: inline-block">
                                        {label}
                                    </Caption1Strong>
                                    {
                                        children.into_iter().map(|item| {
                                            let NavItemOption { label, value } = item;
                                            view! {
                                                <NavItem value>{label}</NavItem>
                                            }
                                        }).collect_view()
                                    }
                                }
                            }).collect_view()
                        }
                    </NavDrawer>
                </div>
                <Layout content_style="padding: 8px 12px 28px; display: flex;" attr:class=("doc-content", true)>
                    <Outlet/>
                </Layout>
            </Layout>
        </Layout>
    }
}

pub(crate) struct NavGroupOption {
    pub label: String,
    pub children: Vec<NavItemOption>,
}

pub(crate) struct NavItemOption {
    pub label: String,
    pub value: String,
}

pub(crate) fn gen_nav_data() -> Vec<NavGroupOption> {
    vec![
        NavGroupOption {
            label: "Getting Started".into(),
            children: vec![
                NavItemOption {
                    value: "/guide/installation".into(),
                    label: "Installation".into(),
                },
                NavItemOption {
                    value: "/guide/server-sider-rendering".into(),
                    label: "Server Sider Rendering".into(),
                },
            ],
        },
        // NavGroupOption {
        //     label: "Development".into(),
        //     children: vec![
        //         NavItemOption {
        //             value: "/guide/development/components".into(),
        //             label: "Components".into(),
        //         },
        //     ],
        // },
        NavGroupOption {
            label: "Components".into(),
            children: vec![
                NavItemOption {
                    value: "/components/accordion".into(),
                    label: "Accordion".into(),
                },
                NavItemOption {
                    value: "/components/anchor".into(),
                    label: "Anchor".into(),
                },
                NavItemOption {
                    value: "/components/auto-complete".into(),
                    label: "Auto Complete".into(),
                },
                NavItemOption {
                    value: "/components/avatar".into(),
                    label: "Avatar".into(),
                },
                NavItemOption {
                    value: "/components/back-top".into(),
                    label: "Back Top".into(),
                },
                NavItemOption {
                    value: "/components/badge".into(),
                    label: "Badge".into(),
                },
                NavItemOption {
                    value: "/components/breadcrumb".into(),
                    label: "Breadcrumb".into(),
                },
                NavItemOption {
                    value: "/components/button".into(),
                    label: "Button".into(),
                },
                NavItemOption {
                    value: "/components/calendar".into(),
                    label: "Calendar".into(),
                },
                NavItemOption {
                    value: "/components/card".into(),
                    label: "Card".into(),
                },
                NavItemOption {
                    value: "/components/checkbox".into(),
                    label: "Checkbox".into(),
                },
                NavItemOption {
                    value: "/components/color-picker".into(),
                    label: "Color Picker".into(),
                },
                NavItemOption {
                    value: "/components/combobox".into(),
                    label: "Combobox".into(),
                },
                NavItemOption {
                    value: "/components/config-provider".into(),
                    label: "Config Provider".into(),
                },
                NavItemOption {
                    value: "/components/date-picker".into(),
                    label: "Date Picker".into(),
                },
                NavItemOption {
                    value: "/components/dialog".into(),
                    label: "Dialog".into(),
                },
                NavItemOption {
                    value: "/components/divider".into(),
                    label: "Divider".into(),
                },
                NavItemOption {
                    value: "/components/drawer".into(),
                    label: "Drawer".into(),
                },
                NavItemOption {
                    value: "/components/grid".into(),
                    label: "Grid".into(),
                },
                NavItemOption {
                    value: "/components/icon".into(),
                    label: "Icon".into(),
                },
                NavItemOption {
                    value: "/components/image".into(),
                    label: "Image".into(),
                },
                NavItemOption {
                    value: "/components/input".into(),
                    label: "Input".into(),
                },
                NavItemOption {
                    value: "/components/layout".into(),
                    label: "Layout".into(),
                },
                NavItemOption {
                    value: "/components/loading-bar".into(),
                    label: "Loading Bar".into(),
                },
                NavItemOption {
                    value: "/components/menu".into(),
                    label: "Menu".into(),
                },
                NavItemOption {
                    value: "/components/message-bar".into(),
                    label: "Message Bar".into(),
                },
                NavItemOption {
                    value: "/components/nav".into(),
                    label: "Nav".into(),
                },
                NavItemOption {
                    value: "/components/pagination".into(),
                    label: "Pagination".into(),
                },
                NavItemOption {
                    value: "/components/popover".into(),
                    label: "Popover".into(),
                },
                NavItemOption {
                    value: "/components/progress-bar".into(),
                    label: "ProgressBar".into(),
                },
                NavItemOption {
                    value: "/components/radio".into(),
                    label: "Radio".into(),
                },
                NavItemOption {
                    value: "/components/scrollbar".into(),
                    label: "Scrollbar".into(),
                },
                NavItemOption {
                    value: "/components/skeleton".into(),
                    label: "Skeleton".into(),
                },
                NavItemOption {
                    value: "/components/slider".into(),
                    label: "Slider".into(),
                },
                NavItemOption {
                    value: "/components/space".into(),
                    label: "Space".into(),
                },
                NavItemOption {
                    value: "/components/spin-button".into(),
                    label: "Spin Button".into(),
                },
                NavItemOption {
                    value: "/components/spinner".into(),
                    label: "Spinner".into(),
                },
                NavItemOption {
                    value: "/components/switch".into(),
                    label: "Switch".into(),
                },
                NavItemOption {
                    value: "/components/tab-list".into(),
                    label: "Tab List".into(),
                },
                NavItemOption {
                    value: "/components/table".into(),
                    label: "Table".into(),
                },
                NavItemOption {
                    value: "/components/tag".into(),
                    label: "Tag".into(),
                },
                NavItemOption {
                    value: "/components/text".into(),
                    label: "Text".into(),
                },
                NavItemOption {
                    value: "/components/textarea".into(),
                    label: "Textarea".into(),
                },
                NavItemOption {
                    value: "/components/time-picker".into(),
                    label: "Time Picker".into(),
                },
                NavItemOption {
                    value: "/components/toast".into(),
                    label: "Toast".into(),
                },
                NavItemOption {
                    value: "/components/upload".into(),
                    label: "Upload".into(),
                },
            ],
        },
    ]
}
