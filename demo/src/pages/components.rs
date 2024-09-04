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
                padding: 8px 12px 28px;
                box-sizing: border-box;
            }
            .demo-components__toc {
                width: 190px;
                margin: 12px;
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
                    width: 100vw;
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
                <Layout content_style="display: flex" attr:class=("doc-content", true)>
                    <Outlet/>
                </Layout>
            </Layout>
        </Layout>
    }
}

pub(crate) struct NavGroupOption {
    pub label: &'static str,
    pub children: Vec<NavItemOption>,
}

pub(crate) struct NavItemOption {
    pub label: &'static str,
    pub value: &'static str,
}

pub(crate) fn gen_nav_data() -> Vec<NavGroupOption> {
    vec![
        NavGroupOption {
            label: "Getting Started",
            children: vec![
                NavItemOption {
                    value: "/guide/installation",
                    label: "Installation",
                },
                NavItemOption {
                    value: "/guide/server-sider-rendering",
                    label: "Server Sider Rendering",
                },
            ],
        },
        // NavGroupOption {
        //     label: "Development",
        //     children: vec![
        //         NavItemOption {
        //             value: "/guide/development/components",
        //             label: "Components",
        //         },
        //     ],
        // },
        NavGroupOption {
            label: "Components",
            children: vec![
                NavItemOption {
                    value: "/components/accordion",
                    label: "Accordion",
                },
                NavItemOption {
                    value: "/components/anchor",
                    label: "Anchor",
                },
                NavItemOption {
                    value: "/components/auto-complete",
                    label: "Auto Complete",
                },
                NavItemOption {
                    value: "/components/avatar",
                    label: "Avatar",
                },
                NavItemOption {
                    value: "/components/back-top",
                    label: "Back Top",
                },
                NavItemOption {
                    value: "/components/badge",
                    label: "Badge",
                },
                NavItemOption {
                    value: "/components/breadcrumb",
                    label: "Breadcrumb",
                },
                NavItemOption {
                    value: "/components/button",
                    label: "Button",
                },
                NavItemOption {
                    value: "/components/calendar",
                    label: "Calendar",
                },
                NavItemOption {
                    value: "/components/card",
                    label: "Card",
                },
                NavItemOption {
                    value: "/components/checkbox",
                    label: "Checkbox",
                },
                NavItemOption {
                    value: "/components/color-picker",
                    label: "Color Picker",
                },
                NavItemOption {
                    value: "/components/combobox",
                    label: "Combobox",
                },
                NavItemOption {
                    value: "/components/config-provider",
                    label: "Config Provider",
                },
                NavItemOption {
                    value: "/components/date-picker",
                    label: "Date Picker",
                },
                NavItemOption {
                    value: "/components/dialog",
                    label: "Dialog",
                },
                NavItemOption {
                    value: "/components/divider",
                    label: "Divider",
                },
                NavItemOption {
                    value: "/components/drawer",
                    label: "Drawer",
                },
                NavItemOption {
                    value: "/components/field",
                    label: "Field",
                },
                NavItemOption {
                    value: "/components/grid",
                    label: "Grid",
                },
                NavItemOption {
                    value: "/components/icon",
                    label: "Icon",
                },
                NavItemOption {
                    value: "/components/image",
                    label: "Image",
                },
                NavItemOption {
                    value: "/components/input",
                    label: "Input",
                },
                NavItemOption {
                    value: "/components/layout",
                    label: "Layout",
                },
                NavItemOption {
                    value: "/components/link",
                    label: "Link",
                },
                NavItemOption {
                    value: "/components/loading-bar",
                    label: "Loading Bar",
                },
                NavItemOption {
                    value: "/components/menu",
                    label: "Menu",
                },
                NavItemOption {
                    value: "/components/message-bar",
                    label: "Message Bar",
                },
                NavItemOption {
                    value: "/components/nav",
                    label: "Nav",
                },
                NavItemOption {
                    value: "/components/pagination",
                    label: "Pagination",
                },
                NavItemOption {
                    value: "/components/popover",
                    label: "Popover",
                },
                NavItemOption {
                    value: "/components/progress-bar",
                    label: "ProgressBar",
                },
                NavItemOption {
                    value: "/components/radio",
                    label: "Radio",
                },
                NavItemOption {
                    value: "/components/scrollbar",
                    label: "Scrollbar",
                },
                NavItemOption {
                    value: "/components/select",
                    label: "Select",
                },
                NavItemOption {
                    value: "/components/skeleton",
                    label: "Skeleton",
                },
                NavItemOption {
                    value: "/components/slider",
                    label: "Slider",
                },
                NavItemOption {
                    value: "/components/space",
                    label: "Space",
                },
                NavItemOption {
                    value: "/components/spin-button",
                    label: "Spin Button",
                },
                NavItemOption {
                    value: "/components/spinner",
                    label: "Spinner",
                },
                NavItemOption {
                    value: "/components/switch",
                    label: "Switch",
                },
                NavItemOption {
                    value: "/components/tab-list",
                    label: "Tab List",
                },
                NavItemOption {
                    value: "/components/table",
                    label: "Table",
                },
                NavItemOption {
                    value: "/components/tag",
                    label: "Tag",
                },
                NavItemOption {
                    value: "/components/tag-picker",
                    label: "TagPicker",
                },
                NavItemOption {
                    value: "/components/text",
                    label: "Text",
                },
                NavItemOption {
                    value: "/components/textarea",
                    label: "Textarea",
                },
                NavItemOption {
                    value: "/components/time-picker",
                    label: "Time Picker",
                },
                NavItemOption {
                    value: "/components/toast",
                    label: "Toast",
                },
                NavItemOption {
                    value: "/components/tooltip",
                    label: "Tooltip",
                },
                NavItemOption {
                    value: "/components/upload",
                    label: "Upload",
                },
            ],
        },
    ]
}
