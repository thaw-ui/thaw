use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn ColorPickerPage() -> impl IntoView {
    let value = create_rw_signal(RGBA::default());

    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Color Picker"</h1>
            <Demo>
                <ColorPicker value/>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = RGBA::default();

                    view! {
                        <ColorPicker value/>
                    }
                    "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"ColorPicker Props"</h3>
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
                        <td>"RwSignal<RGBA>"</td>
                        <td>"RwSignal<RGBA>"</td>
                        <td>"Value of the picker."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
