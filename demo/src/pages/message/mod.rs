use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

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
                <DemoCode slot>

                    {highlight_str!(
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
                    )}

                </DemoCode>
            </Demo>
            <h3>"MessageProvider Injection Methods"</h3>
            <Table single_column=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Type"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"create"</td>
                        <td>
                            "fn(&self, content: String, variant: MessageVariant, options: MessageOptions)"
                        </td>
                        <td>"The label of the menu item."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
