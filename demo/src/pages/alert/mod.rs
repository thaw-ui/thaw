use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn AlertPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Alert"</h1>
            <Demo>
                <Space vertical=true>
                    <Alert variant=AlertVariant::Success title="title">
                        "success"
                    </Alert>
                    <Alert variant=AlertVariant::Warning title="title">
                        "warning"
                    </Alert>
                    <Alert variant=AlertVariant::Error title="title">
                        "error"
                    </Alert>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Alert variant=AlertVariant::Success title="title">"success"</Alert>
                        <Alert variant=AlertVariant::Warning title="title">"warning"</Alert>
                        <Alert variant=AlertVariant::Error title="title">"error"</Alert>
                "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Alert Props"</h3>
            <Table single_column=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Type"</th>
                        <th>"Default"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Additional classes for the alert element."</td>
                    </tr>
                    <tr>
                        <td>"title"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Title of the alert."</td>
                    </tr>
                    <tr>
                        <td>"variant"</td>
                        <td>"AlertVariant"</td>
                        <td></td>
                        <td>"Alert variant."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"The content of the alert."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
