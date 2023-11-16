use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn LayoutPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Layout"</h1>
            <Demo>
                <Layout>
                    <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                    <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                </Layout>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Layout>
                        <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                        <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                    </Layout>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"sider"</h3>
            <Demo>
                <Layout has_sider=true>
                    <LayoutSider style="background-color: #0078ff99; padding: 20px;">"Sider"</LayoutSider>
                    <Layout>
                        <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                        <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                    </Layout>
                </Layout>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Layout has_sider=true>
                        <LayoutSider style="background-color: #0078ff99; padding: 20px;">"Sider"</LayoutSider>
                        <Layout>
                            <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">"Header"</LayoutHeader>
                            <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
                        </Layout>
                    </Layout>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"Layout Props"</h3>
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
                        <td>"style"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>r#""""#</td>
                        <td>"Layout's style."</td>
                    </tr>
                    <tr>
                        <td>"position"</td>
                        <td>"LayoutPosition"</td>
                        <td>"LayoutPosition::Static"</td>
                        <td>"static position will make it css position set to static. absolute position will make it css position set to absolute and left, right, top, bottom to 0. absolute position is very useful when you want to make content scroll in a fixed container or make the whole page's layout in a fixed position. You may need to change the style of the component to make it display as you expect."</td>
                    </tr>
                    <tr>
                        <td>"has_sider"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Whether the component has sider inside. If so it must be true."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Layout's content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"LayoutHeader, LayoutSider Props"</h3>
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
                        <td>"style"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>r#""""#</td>
                        <td>"LayoutHeader's style."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"LayoutHeader's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
