use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn DividerPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Divider"</h1>
            <Demo>
                "top" <Divider/> "bottom"
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        "top"
                        <Divider />
                        "bottom"
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Divider Props"</h3>
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
                        <td>"Addtional classes for the divider element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
