use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Avatar"</h1>
            <Demo>
                <Space>
                    <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg"/>
                    <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" circle=true/>
                    <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" size=50/>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Space>
                        <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg"/>
                        <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" circle=true/>
                        <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" size=50/>
                    </Space>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}
