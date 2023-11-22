use crate::components::{Demo, DemoCode};
use leptos::*;

#[component]
pub fn InstallationPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Installation"</h1>
            <p>"Installation thaw"</p>
            <Demo>
                ""
                <DemoCode slot>

                    "cargo add thaw"
                </DemoCode>
            </Demo>
        </div>
    }
}
