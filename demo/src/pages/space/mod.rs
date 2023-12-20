use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn SpacePage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Space"</h1>
            <Demo>
                <Space>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space>
                            <Button>"1"</Button>
                            <Button>"2"</Button>
                            <Button>"3"</Button>
                        </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"vertical"</h3>
            <Demo>
                <Space vertical=true>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space vertical=true>
                            <Button>"1"</Button>
                            <Button>"2"</Button>
                            <Button>"3"</Button>
                        </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"gap"</h3>
            <Demo>
                <Space gap=SpaceGap::Large>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <Space gap=SpaceGap::WH(36, 36)>
                    <Button>"1"</Button>
                    <Button>"2"</Button>
                    <Button>"3"</Button>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space gap=SpaceGap::Large>
                            <Button>"1"</Button>
                            <Button>"2"</Button>
                            <Button>"3"</Button>
                        </Space>
                        <Space gap=SpaceGap::WH(36, 36)>
                            <Button>"1"</Button>
                            <Button>"2"</Button>
                            <Button>"3"</Button>
                        </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Space Props"</h3>
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
                        <td>"gap"</td>
                        <td>"SpaceGap"</td>
                        <td>"SpaceGap::Medium"</td>
                        <td>"Space's gap."</td>
                    </tr>
                    <tr>
                        <td>"vertical"</td>
                        <td>"MaybeSignal<f64>"</td>
                        <td>"false"</td>
                        <td>"Whether to lay out vertically."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Space's content."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the space element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
