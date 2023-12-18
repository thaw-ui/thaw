use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn SwitchPage() -> impl IntoView {
    let value = create_rw_signal(false);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Switch"</h1>
            <Demo>
                <Switch value/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(false);
                        view! {
                            <Switch value />
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Swith Props"</h3>
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
                        <td>"Swith's value."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the switch element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
