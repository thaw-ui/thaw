use crate::components::{Demo, DemoCode};
use leptos::*;

#[component]
pub fn ServerSiderRenderingPage() -> impl IntoView {
    let ssr_config = r#"thaw = { ..., default-features = false, features = ["ssr"] }"#;
    let hydrate_config = r#"thaw = { ..., default-features = false, features = ["hydrate"] }"#;
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Server Sider Rendering"</h1>
            <p>"To enable the ssr mode, the following configurations are required:"</p>
            <Demo>
                ""
                <DemoCode slot>

                    {ssr_config}

                </DemoCode>
            </Demo>
            <p>"To enable the hydrate mode, the following configurations are required:"</p>
            <Demo>
                ""
                <DemoCode slot>

                    {hydrate_config}

                </DemoCode>
            </Demo>
        </div>
    }
}
