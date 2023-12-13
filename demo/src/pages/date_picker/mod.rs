use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::chrono::prelude::*;
use thaw::*;

#[component]
pub fn DatePickerPage() -> impl IntoView {
    let value = create_rw_signal(Some(Local::now().date_naive()));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Date Picker"</h1>
            <Demo>
                <DatePicker value/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        use thaw::chrono::prelude::*;

                        let value = create_rw_signal(Some(Local::now().date_naive()));
                        view! {
                            <DatePicker value/>
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"DatePicker Props"</h3>
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
                            <Text code=true>"RwSignal<Option<NaiveDate>>"</Text>
                        </td>
                        <td>
                            <Text code=true>"Default::default()"</Text>
                        </td>
                        <td>
                            "Set the date picker value"
                        </td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
