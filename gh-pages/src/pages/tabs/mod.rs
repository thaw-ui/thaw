use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    let active_key = create_rw_signal("apple");
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Tabs"</h1>
            <Demo>
                <Tabs active_key>
                    <Tab key="apple" label="Apple">
                        "apple"
                    </Tab>
                    <Tab key="pear" label="Pear">
                        "pear"
                    </Tab>
                </Tabs>
                <DemoCode slot>
                    {
                        indoc! {r#"
                        let active_key = create_rw_signal("apple");
                        <Tabs active_key>
                            <Tab key="apple" label="Apple">
                                "apple"
                            </Tab>
                            <Tab key="pear" label="Pear">
                                "pear"
                            </Tab>
                        </Tabs>
                        "#}
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}
