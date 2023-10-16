use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn SpacePage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Space"</h1>
            <Demo>
                <Space>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Space>
                        <Button>"1"</Button>
                        <Button>"2"</Button>
                        <Button>"3"</Button>
                    </Space>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"vertical"</h3>
            <Demo>
                <Space vertical=true>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Space vertical=true>
                        <Button>"1"</Button>
                        <Button>"2"</Button>
                        <Button>"3"</Button>
                    </Space>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"gap"</h3>
            <Demo>
                <Space gap=SpaceGap::Large>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <Space gap=SpaceGap::WH(36, 36)>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Space gap=SpaceGap::Large>
                        <Button>"1"</Button>
                        <Button>"2"</Button>
                        <Button>"3"</Button>
                    </Space>
                    <Space gap=SpaceGap::WH(36, 36)>
                        <Button>"1"</Button>
                        <Button>"2"</Button>
                        <Button>"3"</Button>
                    </Space>
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
