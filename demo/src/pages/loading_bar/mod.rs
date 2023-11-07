use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn LoadingBarPage() -> impl IntoView {
    let loading_bar = use_loading_bar();
    let start = move |_| {
        loading_bar.start();
    };
    let finish = move |_| {
        loading_bar.finish();
    };
    let error = move |_| {
        loading_bar.error();
    };
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Loading Bar"</h1>
            <Alert variant=AlertVariant::Warning title="Prerequisite">
                "If you want to use loading bar, you need to wrap the component where you call related methods inside LoadingBarProvider and use use_loading_bar to get the API."
            </Alert>
            <Demo>
                <Space>
                    <Button on_click=start>"start"</Button>
                    <Button on_click=finish>"finish"</Button>
                    <Button on_click=error>"error"</Button>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let loading_bar = use_loading_bar();
                    let start = move |_| {
                        loading_bar.start();
                    };
                    let finish = move |_| {
                        loading_bar.finish();
                    };
                    let error = move |_| {
                        loading_bar.error();
                    };
                    view! {
                        <Space>
                            <Button on_click=start>"start"</Button>
                            <Button on_click=finish>"finish"</Button>
                            <Button on_click=error>"error"</Button>
                        </Space>
                    }
                    "#,
                        "rust"
                    )
                >

                    ""
                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}
