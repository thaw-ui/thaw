use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn DividerPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Checkbox"</h1>
            <Demo>
                "top"
                <Divider />
                "bottom"
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    "top"
                    <Divider />
                    "bottom"
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
