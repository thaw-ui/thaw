use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn TabsPage() -> impl IntoView {
    let value = create_rw_signal(String::from("apple"));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Tabs2"</h1>
            <Demo>
                <Tabs value>
                    <Tab key="apple" label="Apple">
                        "apple"
                    </Tab>
                    <Tab key="pear" label="Pear">
                        "pear"
                    </Tab>
                </Tabs>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal("apple");
                    <Tabs value>
                        <Tab key="apple" label="Apple">
                            "apple"
                        </Tab>
                        <Tab key="pear" label="Pear">
                            "pear"
                        </Tab>
                    </Tabs>
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
