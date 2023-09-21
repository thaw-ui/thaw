use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn ImagePage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Image"</h1>
            <Demo>
            <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="500px"/>
            <Image width="200px" height="200px"/>
                <DemoCode slot>
                    {
                        indoc! {r#"
                            <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="500px"/>
                            <Image width="200px" height="200px"/>
                        "#}
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}
