use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::chrono::prelude::*;
use thaw::*;

#[component]
pub fn TimePickerPage() -> impl IntoView {
    let value = create_rw_signal(Some(Local::now().time()));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Time Picker"</h1>
            <Demo>
                <TimePicker value/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        use thaw::chrono::prelude::*;

                        let value = create_rw_signal(Local::now().time());
                        view! {
                            <TimePicker value />
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"TimePicker Props"</h3>
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
                        <td>
                            <Text code=true>"RwSignal<Time>"</Text>
                        </td>
                        <td></td>
                        <td></td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
