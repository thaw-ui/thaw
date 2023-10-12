use crate::components::SiteHeader;
use leptos::*;
use leptos_router::{use_location, use_navigate, Outlet};
use melt_ui::*;

#[component]
pub fn ComponentsPage() -> impl IntoView {
    let navigate = use_navigate();
    let selected = create_rw_signal({
        let loaction = use_location();
        let mut pathname = loaction.pathname.get_untracked();

        if pathname.starts_with("/melt-ui/components/") {
            pathname.drain(20..).collect()
        } else {
            String::new()
        }
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
                    <Menu value=selected>
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
                            <MenuItem key="table" label="Table"/>
                            <MenuItem key="color-picker" label="Color Picker"/>
                            <MenuItem key="alert" label="Alert"/>
                            <MenuItem key="grid" label="Grid"/>
                        </MenuGroup>
                        <MenuGroup label="Mobile Components">
                            <MenuItem key="tabbar" label="Tabbar"/>
                            <MenuItem key="nav-bar" label="Nav Bar"/>
                            <MenuItem key="toast" label="Toast"/>
                        </MenuGroup>
                    </Menu>
                </LayoutSider>
                <Layout style="padding: 8px 12px 28px; overflow-y: auto;">
                    <Outlet/>
                </Layout>
            </Layout>
        </Layout>
    }
}
