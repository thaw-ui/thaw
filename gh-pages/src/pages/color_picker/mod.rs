use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;

#[component]
pub fn ColorPickerPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Color Picker"</h1>
            <Demo>
                <ColorPicker/>
                <DemoCode slot>""</DemoCode>
            </Demo>
        </div>
    }
}
