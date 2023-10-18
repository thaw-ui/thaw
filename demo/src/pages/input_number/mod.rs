use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn InputNumberPage() -> impl IntoView {
    let value = create_rw_signal(0);
    let value_f64 = create_rw_signal(0.0);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"InputNumber"</h1>
            <Demo>
                <Space vertical=true>
                    <InputNumber value step=1/>
                    <InputNumber value=value_f64 step=1.0/>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(0);
                    view! {
                        <Space vertical=true>
                            <InputNumber value step=1/>
                            <InputNumber value=value_f64 step=1.0/>
                        </Space>
                    }
                    "#,
                        "rust"
                    )
                >

                    ""
                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}
