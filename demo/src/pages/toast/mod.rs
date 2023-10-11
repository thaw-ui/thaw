use crate::{
    components::{Demo, DemoCode},
    pages::MobilePage,
};
use leptos::*;
use melt_ui::mobile::*;
use melt_ui::*;
use prisms::highlight_str;
use std::time::Duration;

#[component]
pub fn ToastPage() -> impl IntoView {
    view! {
        <div style="display: flex">
            <div style="width: 896px; margin: 0 auto;">
                <h1>"Toast"</h1>
                <Demo>
                    ""
                    <DemoCode
                        slot
                        html=highlight_str!(
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
                        )
                    >

                        ""
                    </DemoCode>
                </Demo>
            </div>
            <div>
                <MobilePage path="/melt-ui?path=/mobile/toast"/>
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
