use crate::components::SiteHeader;
use leptos::*;
use leptos_meta::Style;
use leptos_router::{use_location, use_navigate, Outlet};
use thaw::*;

#[component]
pub fn GuidePage() -> impl IntoView {
    let navigate = use_navigate();
    let selected = create_rw_signal({
        let loaction = use_location();
        let mut pathname = loaction.pathname.get_untracked();

        if pathname.starts_with("/guide/") {
            pathname.drain(7..).collect()
        } else {
            String::new()
        }
    });

    create_effect(move |value| {
        let selected = selected.get();
        if value.is_some() {
            navigate(&format!("/guide/{selected}"), Default::default());
        }
        selected
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
                .demo-guide__sider {
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
                <LayoutSider class="demo-guide__sider">
                    <Menu value=selected>

                        {gen_guide_menu_data().into_view()}

                    </Menu>
                </LayoutSider>
                <Layout style="padding: 8px 12px 28px; overflow-y: auto;">
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
            <MenuGroup label=label>

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

pub(crate) fn gen_guide_menu_data() -> Vec<MenuGroupOption> {
    vec![
        MenuGroupOption {
            label: "Getting Started".into(),
            children: vec![
                MenuItemOption {
                    value: "installation".into(),
                    label: "Installation".into(),
                },
                MenuItemOption {
                    value: "usage".into(),
                    label: "Usage".into(),
                },
            ],
        },
        MenuGroupOption {
            label: "Guides".into(),
            children: vec![MenuItemOption {
                value: "server-sider-rendering".into(),
                label: "Server Sider Rendering".into(),
            }],
        },
        MenuGroupOption {
            label: "Development".into(),
            children: vec![
                MenuItemOption {
                    value: "development/guide".into(),
                    label: "Guide".into(),
                },
                MenuItemOption {
                    value: "development/components".into(),
                    label: "Components".into(),
                },
            ],
        },
    ]
}
