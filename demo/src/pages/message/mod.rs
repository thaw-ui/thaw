use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn MessagePage() -> impl IntoView {
    let message = use_message();
    let success = move |_| {
        message.create(
            "Success".into(),
            MessageVariant::Success,
            Default::default(),
        );
    };
    let warning = move |_| {
        message.create(
            "Warning".into(),
            MessageVariant::Warning,
            Default::default(),
        );
    };
    let error = move |_| {
        message.create("Error".into(), MessageVariant::Error, Default::default());
    };
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Message"</h1>
            <Alert variant=AlertVariant::Warning title="Prerequisite">
                "If you want to use message, you need to wrap the component where you call related methods inside MessageProvider and use use_message to get the API."
            </Alert>
            <Demo>
                <Space>
                    <Button on:click=success>"Success"</Button>
                    <Button on:click=warning>"Warning"</Button>
                    <Button on:click=error>"Error"</Button>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        let message = use_message();
                        let success = move |_| {
                            message.create(
                                "Success".into(),
                                MessageVariant::Success,
                                Default::default(),
                            );
                        };
                        let warning = move |_| {
                            message.create(
                                "Warning".into(),
                                MessageVariant::Warning,
                                Default::default(),
                            );
                        };
                        let error = move |_| {
                            message.create("Error".into(), MessageVariant::Error, Default::default());
                        };
                        view! {
                            <Space>
                                <Button on:click=success>"Success"</Button>
                                <Button on:click=warning>"Warning"</Button>
                                <Button on:click=error>"Error"</Button>
                            </Space>
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
