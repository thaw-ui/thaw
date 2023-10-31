use crate::{
    components::{Demo, DemoCode},
    pages::MobilePage,
};
use leptos::*;
use melt_ui::mobile::*;
use prisms::highlight_str;

#[component]
pub fn TabbarPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <div style="width: 896px; margin: 0 auto;">
                <h1>"Tabbar"</h1>
                <Demo>
                    ""
                    <DemoCode
                        slot
                        html=highlight_str!(
                            r#"
                        let value = create_rw_signal(String::from("o"));
                                
                        <Tabbar value>
                            <TabbarItem name="a">
                                "and"
                            </TabbarItem>
                            <TabbarItem name="i">
                                "if"
                            </TabbarItem>
                            <TabbarItem name="o" icon=icondata::AiIcon::AiCloseOutlined>
                                "or"
                            </TabbarItem>
                        </Tabbar>
                    "#,
                            "rust"
                        )
                    >

                        ""
                    </DemoCode>
                </Demo>
            </div>
            <div>
                <MobilePage path="/melt-ui?path=/mobile/tabbar"/>
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
                <TabbarItem key="o" icon=icondata::AiIcon::AiCloseOutlined>
                    "or"
                </TabbarItem>
            </Tabbar>
        </div>
    }
}
