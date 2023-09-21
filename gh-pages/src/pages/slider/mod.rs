use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    let value = create_rw_signal(0.0);

    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Slider"</h1>
            <Demo>
                <Slider value/>
                <DemoCode slot>
                    {
                        indoc!(r#"
                        let value = create_rw_signal(0.0);
                        
                        <Slider value/>
                        "#)
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}
