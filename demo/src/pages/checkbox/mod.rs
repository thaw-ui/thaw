use std::collections::HashSet;

use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let checked = create_rw_signal(false);
    let value = create_rw_signal(HashSet::new());
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Checkbox"</h1>
            <Demo>
                <Checkbox value=checked>"Click"</Checkbox>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(false);

                    view! {
                        <Checkbox value>
                            "Click"
                        </Checkbox>
                    }
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"group"</h3>
            <Demo>
                <CheckboxGroup value>
                    <CheckboxItem label="apple" key="a"/>
                    <CheckboxItem label="b" key="b"/>
                    <CheckboxItem label="c" key="c"/>
                </CheckboxGroup>
                <div style="margin-top: 1rem">"value: " {move || format!("{:?}", value.get())}</div>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(HashSet::new());

                    view! {
                        <CheckboxGroup value>
                            <CheckboxItem label="apple" key="a" />
                            <CheckboxItem label="b" key="b" />
                            <CheckboxItem label="c" key="c" />
                        </CheckboxGroup>
                    }
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"Checkbox Props"</h3>
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
                        <td>"Whether the checkbox is being checked."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Checkbox's content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"CheckboxGroup Props"</h3>
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
                        <td>"RwSignal<HashSet<String>>,"</td>
                        <td>"false"</td>
                        <td>"Sets the value of the checkbox group."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"CheckboxGroup's content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"CheckboxItem Props"</h3>
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
                        <td>"key"</td>
                        <td>"String"</td>
                        <td></td>
                        <td>"The key of the checkbox to be used in a checkbox group."</td>
                    </tr>
                    <tr>
                        <td>"label"</td>
                        <td>"Option<String>"</td>
                        <td>"None"</td>
                        <td>"Checkbox's label."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
