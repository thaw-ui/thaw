use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    let value = create_rw_signal(String::from("apple"));
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Tabs"</h1>
            <Demo>
                <Tabs value>
                    <Tab key="apple" label="Apple">
                        "apple"
                    </Tab>
                    <Tab key="pear" label="Pear">
                        "pear"
                    </Tab>
                </Tabs>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(String::from("apple"));

                        view! {
                            <Tabs value>
                                <Tab key="apple" label="Apple">
                                    "apple"
                                </Tab>
                                <Tab key="pear" label="Pear">
                                    "pear"
                                </Tab>
                            </Tabs>
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Tabs Props"</h3>
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
                            <Text code=true>"RwSignal<String>"</Text>
                        </td>
                        <td>
                            <Text code=true>"Default::default()"</Text>
                        </td>
                        <td>"Tabs value."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>
                            <Text code=true>"Children"</Text>
                        </td>
                        <td></td>
                        <td>"Tabs content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"Tab Props"</h3>
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
                        <td>
                            <Text code=true>"String"</Text>
                        </td>
                        <td></td>
                        <td>"The indentifier of the tab."</td>
                    </tr>
                    <tr>
                        <td>"label"</td>
                        <td>
                            <Text code=true>"String"</Text>
                        </td>
                        <td></td>
                        <td>"The label of the tab."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>
                            <Text code=true>"Children"</Text>
                        </td>
                        <td></td>
                        <td>"Tab's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
