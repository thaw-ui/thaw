use crate::components::SiteHeader;
use leptos::*;
use leptos_router::{use_location, use_navigate, Outlet};
use melt_ui::*;
use regex::Regex;

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let loaction = use_location();
    let navigate = use_navigate();
    let selected = create_rw_signal(String::from(""));
    create_effect(move |_| {
        let pathname = loaction.pathname.get();

        let re = Regex::new(r"^/melt-ui/components/(.+)$").unwrap();
        let Some(caps) = re.captures(&pathname) else {
            return;
        };
        let Some(path) = caps.get(1) else {
            return;
        };
        let path = path.as_str().to_string();
        selected.set(path);
    });

    create_effect(move |value| {
        let selected = selected.get();
        if value.is_some() {
            navigate(&format!("/components/{selected}"), Default::default());
        }
        selected
    });
    view! {
        <Layout position=LayoutPosition::ABSOLUTE>
            <SiteHeader/>
            <Layout has_sider=true position=LayoutPosition::ABSOLUTE style="top: 54px;">
                <LayoutSider>
                    <Menu selected>
                        <MenuGroup label="Common Components">
                            <MenuItem key="menu" label="Menu"/>
                            <MenuItem key="slider" label="Slider"/>
                            <MenuItem key="input" label="Input"/>
                            <MenuItem key="image" label="Image"/>
                            <MenuItem key="modal" label="Modal"/>
                            <MenuItem key="button" label="Button"/>
                            <MenuItem key="checkbox" label="Checkbox"/>
                            <MenuItem key="tabs" label="Tabs"/>
                            <MenuItem key="select" label="Select"/>
                            <MenuItem key="space" label="Space"/>
                        </MenuGroup>
                        <MenuGroup label="Mobile Components">
                            <MenuItem key="tabbar" label="Tabbar"/>
                            <MenuItem key="nav-bar" label="Nav Bar"/>
                            <MenuItem key="toast" label="Toast"/>
                        </MenuGroup>
                    </Menu>
                </LayoutSider>
                <Layout style="padding: 8px 12px 28px; overflow-y: scroll;">
                    <Outlet/>
                </Layout>
            </Layout>
        </Layout>
    }
}
