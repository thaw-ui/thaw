use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn AlertPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Alert"</h1>
            <Demo>
                <Space vertical=true>
                    <Alert variant=AlertVariant::Success title="title">
                        "success"
                    </Alert>
                    <Alert variant=AlertVariant::Warning title="title">
                        "warning"
                    </Alert>
                    <Alert variant=AlertVariant::Error title="title">
                        "error"
                    </Alert>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        <Alert variant=AlertVariant::Success title="title">"success"</Alert>
                        <Alert variant=AlertVariant::Warning title="title">"warning"</Alert>
                        <Alert variant=AlertVariant::Error title="title">"error"</Alert>
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
