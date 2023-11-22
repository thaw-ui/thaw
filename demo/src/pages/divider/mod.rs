use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn DividerPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Divider"</h1>
            <Demo>
                "top" <Divider/> "bottom"
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        "top"
                        <Divider />
                        "bottom"
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
        </div>
    }
}
