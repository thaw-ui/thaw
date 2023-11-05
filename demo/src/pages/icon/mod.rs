use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn IconPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Icon"</h1>
            <Demo>
                <Space>
                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCloseOutlined) />
                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCheckOutlined) />
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                <Space>
                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCloseOutlined) />
                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCheckOutlined) />
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
