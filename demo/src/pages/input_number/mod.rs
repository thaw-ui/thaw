use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn InputNumberPage() -> impl IntoView {
    let value = create_rw_signal(0);
    let value_f64 = create_rw_signal(0.0);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"InputNumber"</h1>
            <Demo>
                <Space vertical=true>
                    <InputNumber value step=1/>
                    <InputNumber value=value_f64 step=1.0/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(0);
                        let value_f64 = create_rw_signal(0.0);
                        view! {
                            <Space vertical=true>
                                <InputNumber value step=1/>
                                <InputNumber value=value_f64 step=1.0/>
                            </Space>
                        }
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"disabled"</h3>
            <Demo>
                <Space vertical=true>
                    <InputNumber value step=1 disabled=true/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <InputNumber value step=1 disabled=true/>
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"invalid"</h3>
            <Demo>
                <Space vertical=true>
                    <InputNumber value step=1 invalid=true/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <InputNumber value step=1 invalid=true/>
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"InputNumber Props"</h3>
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
                        <td>"value"</td>
                        <td>"RwSignal<T>"</td>
                        <td>"T::default()"</td>
                        <td>"Set the input value."</td>
                    </tr>
                    <tr>
                        <td>"placeholder"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Placeholder of input number."</td>
                    </tr>
                    <tr>
                        <td>"step"</td>
                        <td>"MaybeSignal<T>"</td>
                        <td></td>
                        <td>
                            "The number which the current value is increased or decreased on key or button press."
                        </td>
                    </tr>
                    <tr>
                        <td>"disabled"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Whether the input is disabled."</td>
                    </tr>
                    <tr>
                        <td>"invalid"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Whether the input is invalid."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the input element."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"T impl"</h3>
            <p>
                "T: Add<Output = T> + Sub<Output = T> + Default + Clone + FromStr + ToString + 'static"
            </p>
        </div>
    }
}
