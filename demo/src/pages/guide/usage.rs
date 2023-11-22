use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;

#[component]
pub fn UsagePage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Usage"</h1>
            <p>"You just need to import thaw and use it."</p>
            <Demo>
                ""
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                            // Import all
                            use thaw::*;
                            // Import on Demand
                            use thaw::{Button, ButtonVariant};
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <p>"A small example:"</p>
            <Demo>
                ""
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                            use leptos::*;
                            use thaw::*;
                            
                            fn main() {
                                mount_to_body(App)
                            }
                            #[component]
                            pub fn App() -> impl IntoView {
                                view! {
                                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                                }
                            }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
        </div>
    }
}
