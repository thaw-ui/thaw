use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn TypographyPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Typography"</h1>
            <Demo>
                <Space>
                    <Text>"text"</Text>
                    <Text code=true>"code"</Text>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space>
                            <Text>
                                "text"
                            </Text>
                            <Text code=true>
                                "code"
                            </Text>
                        </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Text Props"</h3>
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
                        <td>"code"</td>
                        <td>
                            <Text code=true>"bool"</Text>
                        </td>
                        <td>
                            <Text code=true>"false"</Text>
                        </td>
                        <td>"Use the code tag and style."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>
                            <Text code=true>"Children"</Text>
                        </td>
                        <td></td>
                        <td>"Text's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
