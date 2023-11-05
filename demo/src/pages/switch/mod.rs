use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn SwitchPage() -> impl IntoView {
    let value = create_rw_signal(false);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Switch"</h1>
            <Demo>
                <Switch value />
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(false);
                    view! {
                        <Switch value />
                    }
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
