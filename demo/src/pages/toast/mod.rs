use crate::{
    components::{Demo, DemoCode},
    pages::MobilePage,
};
use leptos::*;
use prisms::highlight_str;
use std::time::Duration;
use thaw::mobile::*;
use thaw::*;

#[component]
pub fn ToastPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <div style="width: 896px; margin: 0 auto;">
                <h1>"Toast"</h1>
                <Demo>
                    ""
                    <DemoCode slot>

                        {highlight_str!(
                            r#"
                            let count = create_rw_signal(0u32);
                            let onclick = move |_| {
                                show_toast(ToastOptions {
                                    message: format!("Hello {}", count.get_untracked()),
                                    duration: Duration::from_millis(2000),
                                });
                                count.set(count.get_untracked() + 1);
                            };
                        "#,
                            "rust"
                        )}

                    </DemoCode>
                </Demo>
                <h3>"Toast Methods"</h3>
                <Table single_column=true>
                    <thead>
                        <tr>
                            <th>"Name"</th>
                            <th>"Type"</th>
                            <th>"Description"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"show_toast"</td>
                            <td>"fn(options: ToastOptions))"</td>
                            <td>"Show toast."</td>
                        </tr>
                    </tbody>
                </Table>
                <h3>"ToastOptions Properties"</h3>
                <Table single_column=true>
                    <thead>
                        <tr>
                            <th>"Name"</th>
                            <th>"Type"</th>
                            <th>"Description"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"message"</td>
                            <td>"String"</td>
                            <td>"message."</td>
                        </tr>
                        <tr>
                            <td>"duration"</td>
                            <td>"std::time::Duration"</td>
                            <td>"show duration."</td>
                        </tr>
                    </tbody>
                </Table>
            </div>
            <div>
                <MobilePage path="/thaw?path=/mobile/toast"/>
            </div>
        </div>
    }
}

#[component]
pub fn ToastDemoPage() -> impl IntoView {
    let count = create_rw_signal(0u32);
    let onclick = move |_| {
        show_toast(ToastOptions {
            message: format!("Hello {}", count.get_untracked()),
            duration: Duration::from_millis(2000),
        });
        count.set(count.get_untracked() + 1);
    };
    view! {
        <div style="margin: 20px">
            <Button on:click=onclick>"hi"</Button>
        </div>
    }
}
