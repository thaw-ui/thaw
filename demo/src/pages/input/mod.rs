use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn InputPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Input"</h1>
            <Demo>
                <Space vertical=true>
                    <Input value/>
                    <Input value variant=InputVariant::Password placeholder="Password"/>
                    <Input value>
                        <InputSuffix slot>
                            "$"
                        </InputSuffix>
                    </Input>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        let value = create_rw_signal(String::from("o"));

                        view! {
                            <Input value/>
                            <Input value variant=InputVariant::Password placeholder="Password"/>
                            <Input value>
                                <InputSuffix slot>
                                    "$"
                                </InputSuffix>
                            </Input>
                        }
                    "#,
                        "rust"
                    )
                >

                    ""
                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}
