use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn TagPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Tag"</h1>
            <Demo>
                <Space>
                    <Tag>
                        "default"
                    </Tag>
                    <Tag variant=TagVariant::Success>
                        "success"
                    </Tag>
                    <Tag variant=TagVariant::Warning>
                        "warning"
                    </Tag>
                    <Tag variant=TagVariant::Error>
                        "error"
                    </Tag>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Space>
                        <Tag>
                            "default"
                        </Tag>
                        <Tag variant=TagVariant::Success>
                            "success"
                        </Tag>
                        <Tag variant=TagVariant::Warning>
                            "warning"
                        </Tag>
                        <Tag variant=TagVariant::Error>
                            "error"
                        </Tag>
                    </Space>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"Tag Props"</h3>
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
                        <td>"variant"</td>
                        <td>"MaybeSignal<TagVariant>"</td>
                        <td>"TagVariant::Default"</td>
                        <td>"Tag's variant."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Tag's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
