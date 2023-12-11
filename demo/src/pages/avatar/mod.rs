use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Avatar"</h1>
            <Demo>
                <Space>
                    <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg"/>
                    <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" round=true/>
                    <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" size=50/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space>
                            <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg"/>
                            <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" round=true/>
                            <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" size=50/>
                        </Space>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Avatar Props"</h3>
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
                        <td>"src"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Avatar's image source."</td>
                    </tr>
                    <tr>
                        <td>"round"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Whether to display a rounded avatar."</td>
                    </tr>
                    <tr>
                        <td>"size"</td>
                        <td>"MaybeSignal<u16>"</td>
                        <td>"30"</td>
                        <td>"Avatar's size."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the avatar element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
