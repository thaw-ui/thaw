use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn SliderPage() -> impl IntoView {
    let value = create_rw_signal(0.0);

    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Slider"</h1>
            <Demo>
                <Slider value/>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(0.0);
                        
                    <Slider value/>
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
