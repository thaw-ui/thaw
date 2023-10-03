use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

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
                <DemoCode slot>
                    {
                        indoc!(r#"
                        <Space>
                            <Button>"1"</Button>
                            <Button>"2"</Button>
                            <Button>"3"</Button>
                        </Space>
                        "#)
                    }
                </DemoCode>
            </Demo>
            <h3>"gap"</h3>
            <Demo>
                <Space gap=SpaceGap::LARGE>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <Space gap=SpaceGap::TUPLE(36, 36)>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <DemoCode slot>
                    {
                        indoc!(r#"
                        <Space gap=SpaceGap::LARGE>
                            <Button>"1"</Button>
                            <Button>"2"</Button>
                            <Button>"3"</Button>
                        </Space>
                        <Space gap=SpaceGap::TUPLE(36, 36)>
                            <Button>"1"</Button>
                            <Button>"2"</Button>
                            <Button>"3"</Button>
                        </Space>
                        "#)
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}
