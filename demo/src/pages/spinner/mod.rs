use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn SpinnerPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Spinner"</h1>
            <Demo>
                <Space>
                    <Spinner/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Spinner/>
                "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"size"</h3>
            <Demo>
                <Space>
                    <Spinner size=SpinnerSize::Tiny/>
                    <Spinner size=SpinnerSize::Small/>
                    <Spinner size=SpinnerSize::Medium/>
                    <Spinner size=SpinnerSize::Large/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Spinner size=SpinnerSize::Tiny/>
                        <Spinner size=SpinnerSize::Small/>
                        <Spinner size=SpinnerSize::Medium/>
                        <Spinner size=SpinnerSize::Large/>
                "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Spinner Props"</h3>
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
                        <td>
                            <Text code=true>"MaybeSignal<String>"</Text>
                        </td>
                        <td>
                            <Text code=true>"Default::default()"</Text>
                        </td>
                        <td>"Additional classes for the spinner element."</td>
                    </tr>
                    <tr>
                        <td>"size"</td>
                        <td>
                            <Text code=true>"MaybeSignal<SpinnerSize>"</Text>
                        </td>
                        <td>
                            <Text code=true>"SpinnerSize::Medium"</Text>
                        </td>
                        <td>"Spinner size."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
