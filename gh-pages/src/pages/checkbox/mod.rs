use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let checked = create_rw_signal(false);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Checkbox"</h1>
            <Demo>
                <Checkbox checked>
                    "Click"
                </Checkbox>
                <DemoCode slot>
                    {
                        indoc! {r#"
                            let checked = create_rw_signal(false);

                            <Checkbox checked>
                                "Click"
                            </Checkbox>
                        "#}
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}
