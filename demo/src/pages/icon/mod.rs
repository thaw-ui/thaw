use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn IconPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Icon"</h1>
            <Demo>
                <Space>
                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCloseOutlined)/>
                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCheckOutlined)/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                    <Space>
                        <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCloseOutlined) />
                        <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCheckOutlined) />
                    </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Icon Props"</h3>
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
                        <td>"icon"</td>
                        <td>"MaybeSignal<Icon>"</td>
                        <td></td>
                        <td>"The icon to show."</td>
                    </tr>
                    <tr>
                        <td>"width"</td>
                        <td>"Option<MaybeSignal<String>>"</td>
                        <td>"1em"</td>
                        <td>"The width of the icon."</td>
                    </tr>
                    <tr>
                        <td>"height"</td>
                        <td>"Option<MaybeSignal<String>>"</td>
                        <td>"1em"</td>
                        <td>"The height of the icon."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"Option<MaybeSignal<String>>"</td>
                        <td>"None"</td>
                        <td>"HTML class attribute."</td>
                    </tr>
                    <tr>
                        <td>"style"</td>
                        <td>"Option<MaybeSignal<String>>"</td>
                        <td>"None"</td>
                        <td>"HTML style attribute."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
