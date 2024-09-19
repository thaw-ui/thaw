use std::vec;

use crate::components::SiteHeader;
use leptos::prelude::*;
use leptos_meta::Style;
use leptos_router::{
    components::Outlet,
    hooks::{use_location, use_navigate},
};
use tachys::view::any_view::AnyView;
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
                                    {VecIntoView::into_view(children)}
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
    pub group: Option<&'static str>,
    pub label: &'static str,
    pub value: &'static str,
}

trait VecIntoView {
    fn into_view(self) -> Vec<AnyView<Dom>>;
}

impl VecIntoView for Vec<NavItemOption> {
    fn into_view(self) -> Vec<AnyView<Dom>> {
        let mut iter = self.into_iter().peekable();
        let mut views = vec![];
        while let Some(item) = iter.next() {
            let NavItemOption {
                group,
                label,
                value,
            } = item;
            if let Some(group) = group {
                let mut sub_views = vec![];
                while let Some(item) = iter.peek() {
                    if item.group != Some(group) {
                        break;
                    }
                    let NavItemOption { label, value, .. } = iter.next().unwrap();
                    sub_views.push(view! {
                        <NavSubItem value=value>
                            {label}
                        </NavSubItem>
                    });
                }
                views.push(
                    view! {
                        <NavCategory value=group>
                            <NavCategoryItem slot>
                                {group}
                            </NavCategoryItem>
                            <NavSubItem value=value>
                                {label}
                            </NavSubItem>
                            {sub_views}
                        </NavCategory>
                    }
                    .into_any(),
                );
            } else {
                views.push(
                    view! {
                        <NavItem value>{label}</NavItem>
                    }
                    .into_any(),
                );
            }
        }
        views
    }
}

pub(crate) fn gen_nav_data() -> Vec<NavGroupOption> {
    vec![
        NavGroupOption {
            label: "Getting Started",
            children: vec![
                NavItemOption {
                    group: None,
                    value: "/guide/installation",
                    label: "Installation",
                },
                NavItemOption {
                    group: None,
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
                    group: None,
                    value: "/components/accordion",
                    label: "Accordion",
                },
                NavItemOption {
                    group: None,
                    value: "/components/anchor",
                    label: "Anchor",
                },
                NavItemOption {
                    group: None,
                    value: "/components/auto-complete",
                    label: "Auto Complete",
                },
                NavItemOption {
                    group: None,
                    value: "/components/avatar",
                    label: "Avatar",
                },
                NavItemOption {
                    group: None,
                    value: "/components/back-top",
                    label: "Back Top",
                },
                NavItemOption {
                    group: None,
                    value: "/components/badge",
                    label: "Badge",
                },
                NavItemOption {
                    group: None,
                    value: "/components/breadcrumb",
                    label: "Breadcrumb",
                },
                NavItemOption {
                    group: None,
                    value: "/components/button",
                    label: "Button",
                },
                NavItemOption {
                    group: None,
                    value: "/components/calendar",
                    label: "Calendar",
                },
                NavItemOption {
                    group: None,
                    value: "/components/card",
                    label: "Card",
                },
                NavItemOption {
                    group: None,
                    value: "/components/checkbox",
                    label: "Checkbox",
                },
                NavItemOption {
                    group: None,
                    value: "/components/color-picker",
                    label: "Color Picker",
                },
                NavItemOption {
                    group: None,
                    value: "/components/combobox",
                    label: "Combobox",
                },
                NavItemOption {
                    group: None,
                    value: "/components/config-provider",
                    label: "Config Provider",
                },
                NavItemOption {
                    group: None,
                    value: "/components/date-picker",
                    label: "Date Picker",
                },
                NavItemOption {
                    group: None,
                    value: "/components/dialog",
                    label: "Dialog",
                },
                NavItemOption {
                    group: None,
                    value: "/components/divider",
                    label: "Divider",
                },
                NavItemOption {
                    group: None,
                    value: "/components/drawer",
                    label: "Drawer",
                },
                NavItemOption {
                    group: None,
                    value: "/components/field",
                    label: "Field",
                },
                NavItemOption {
                    group: None,
                    value: "/components/flex",
                    label: "Flex",
                },
                NavItemOption {
                    group: None,
                    value: "/components/grid",
                    label: "Grid",
                },
                NavItemOption {
                    group: None,
                    value: "/components/icon",
                    label: "Icon",
                },
                NavItemOption {
                    group: None,
                    value: "/components/image",
                    label: "Image",
                },
                NavItemOption {
                    group: None,
                    value: "/components/input",
                    label: "Input",
                },
                NavItemOption {
                    group: None,
                    value: "/components/label",
                    label: "Label",
                },
                NavItemOption {
                    group: None,
                    value: "/components/layout",
                    label: "Layout",
                },
                NavItemOption {
                    group: None,
                    value: "/components/link",
                    label: "Link",
                },
                NavItemOption {
                    group: None,
                    value: "/components/loading-bar",
                    label: "Loading Bar",
                },
                NavItemOption {
                    group: None,
                    value: "/components/menu",
                    label: "Menu",
                },
                NavItemOption {
                    group: None,
                    value: "/components/message-bar",
                    label: "Message Bar",
                },
                NavItemOption {
                    group: None,
                    value: "/components/nav",
                    label: "Nav",
                },
                NavItemOption {
                    group: None,
                    value: "/components/pagination",
                    label: "Pagination",
                },
                NavItemOption {
                    group: None,
                    value: "/components/popover",
                    label: "Popover",
                },
                NavItemOption {
                    group: None,
                    value: "/components/progress-bar",
                    label: "ProgressBar",
                },
                NavItemOption {
                    group: None,
                    value: "/components/radio",
                    label: "Radio",
                },
                NavItemOption {
                    group: None,
                    value: "/components/scrollbar",
                    label: "Scrollbar",
                },
                NavItemOption {
                    group: None,
                    value: "/components/select",
                    label: "Select",
                },
                NavItemOption {
                    group: None,
                    value: "/components/skeleton",
                    label: "Skeleton",
                },
                NavItemOption {
                    group: None,
                    value: "/components/slider",
                    label: "Slider",
                },
                NavItemOption {
                    group: None,
                    value: "/components/space",
                    label: "Space",
                },
                NavItemOption {
                    group: None,
                    value: "/components/spin-button",
                    label: "Spin Button",
                },
                NavItemOption {
                    group: None,
                    value: "/components/spinner",
                    label: "Spinner",
                },
                NavItemOption {
                    group: None,
                    value: "/components/switch",
                    label: "Switch",
                },
                NavItemOption {
                    group: None,
                    value: "/components/tab-list",
                    label: "Tab List",
                },
                NavItemOption {
                    group: None,
                    value: "/components/table",
                    label: "Table",
                },
                NavItemOption {
                    group: Some("Tag"),
                    value: "/components/tag",
                    label: "Tag",
                },
                NavItemOption {
                    group: Some("Tag"),
                    value: "/components/tag-group",
                    label: "Tag Group",
                },
                NavItemOption {
                    group: None,
                    value: "/components/tag-picker",
                    label: "Tag Picker",
                },
                NavItemOption {
                    group: None,
                    value: "/components/text",
                    label: "Text",
                },
                NavItemOption {
                    group: None,
                    value: "/components/textarea",
                    label: "Textarea",
                },
                NavItemOption {
                    group: None,
                    value: "/components/time-picker",
                    label: "Time Picker",
                },
                NavItemOption {
                    group: None,
                    value: "/components/toast",
                    label: "Toast",
                },
                NavItemOption {
                    group: None,
                    value: "/components/tooltip",
                    label: "Tooltip",
                },
                NavItemOption {
                    group: None,
                    value: "/components/upload",
                    label: "Upload",
                },
            ],
        },
    ]
}
