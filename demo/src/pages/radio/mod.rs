use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn RadioPage() -> impl IntoView {
    let checked = create_rw_signal(false);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Radio"</h1>
            <Demo>
                <Radio value=checked>"Click"</Radio>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(false);

                    view! {
                        <Radio value>
                            "Click"
                        </Radio>
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
