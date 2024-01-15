use super::TabbarMdPage;
use crate::pages::MobilePage;
use leptos::*;
use thaw::mobile::{Tabbar, TabbarItem};

#[component]
pub fn TabbarPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <TabbarMdPage />
            <div>
                <MobilePage path="/thaw?path=/mobile/tabbar"/>
            </div>
        </div>
    }
}

#[component]
pub fn TabbarDemoPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {
        <div style="height: 100vh;">
            {move || value.get()} <Tabbar value>
                <TabbarItem key="a">"and"</TabbarItem>
                <TabbarItem key="i">"if"</TabbarItem>
                <TabbarItem key="o" icon=icondata::AiCloseOutlined>
                    "or"
                </TabbarItem>
            </Tabbar>
        </div>
    }
}
