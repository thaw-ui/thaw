use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn RadioPage() -> impl IntoView {
    let checked = create_rw_signal(false);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Radio"</h1>
            <Demo>
                <Radio value=checked>"Click"</Radio>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(false);
    
                        view! {
                            <Radio value>
                                "Click"
                            </Radio>
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Radio Props"</h3>
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
                        <td>"RwSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Checked value."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Radio's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
