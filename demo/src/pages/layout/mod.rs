use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn LayoutPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Layout"</h1>
            <Demo>
                <Layout>
                    <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                    <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                </Layout>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Layout>
                        <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                        <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                    </Layout>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"sider"</h3>
            <Demo>
                <Layout has_sider=true>
                    <LayoutSider style="background-color: #0078ff99; padding: 20px;">"Sider"</LayoutSider>
                    <Layout>
                        <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                        <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                    </Layout>
                </Layout>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Layout has_sider=true>
                        <LayoutSider style="background-color: #0078ff99; padding: 20px;">"Sider"</LayoutSider>
                        <Layout>
                            <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                            <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                        </Layout>
                    </Layout>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}
