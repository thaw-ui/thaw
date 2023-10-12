use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn ColorPickerPage() -> impl IntoView {
    let value = create_rw_signal(RGBA::default());

    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Color Picker"</h1>
            <Demo>
                <ColorPicker value/>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = RGBA::default();

                    view! {
                        <ColorPicker value/>
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
