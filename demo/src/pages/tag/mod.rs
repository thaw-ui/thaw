use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn TagPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Tag"</h1>
            <Demo>
                <Space>
                    <Tag>
                        "default"
                    </Tag>
                    <Tag variant=TagVariant::Success>
                        "success"
                    </Tag>
                    <Tag variant=TagVariant::Warning>
                        "warning"
                    </Tag>
                    <Tag variant=TagVariant::Error>
                        "error"
                    </Tag>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Space>
                        <Tag>
                            "default"
                        </Tag>
                        <Tag variant=TagVariant::Success>
                            "success"
                        </Tag>
                        <Tag variant=TagVariant::Warning>
                            "warning"
                        </Tag>
                        <Tag variant=TagVariant::Error>
                            "error"
                        </Tag>
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
