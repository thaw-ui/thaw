use std::collections::HashSet;

use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let checked = create_rw_signal(false);
    let value = create_rw_signal(HashSet::new());
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

            <Demo>
                <CheckboxGroup value>
                    <CheckboxItem label="apple" value="a" />
                    <CheckboxItem label="b" value="b" />
                    <CheckboxItem label="c" value="c" />
                </CheckboxGroup>
                <div style="margin-top: 1rem">
                    "value: " { move || format!("{:?}", value.get()) }
                </div>
                <DemoCode slot>
                    {
                        indoc! {r#"
                            let value = create_rw_signal(HashSet::new());

                            <CheckboxGroup value>
                                <CheckboxItem label="apple" value="a" />
                                <CheckboxItem label="b" value="b" />
                                <CheckboxItem label="c" value="c" />
                            </CheckboxGroup>
                        "#}
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}
