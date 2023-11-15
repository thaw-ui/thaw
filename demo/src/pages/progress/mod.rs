use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn ProgressPage() -> impl IntoView {
    let percentage = create_rw_signal(0.0f32);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Progress"</h1>
            <Demo>
                <Space vertical=true>
                    <Progress percentage />
                    <Progress percentage indicator_placement=ProgressIndicatorPlacement::Inside/>
                    <Space>
                        <Button on_click=move |_| percentage.update(|v| *v -= 10.0)>
                            "-10%"
                        </Button>
                        <Button on_click=move |_| percentage.update(|v| *v += 10.0)>
                            "+10%"
                        </Button>
                    </Space>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let percentage = create_rw_signal(0.0f32);

                    view! {
                        <Space vertical=true>
                            <Progress percentage />
                            <Progress percentage indicator_placement=ProgressIndicatorPlacement::Inside/>
                            <Space>
                                <Button on_click=move |_| percentage.update(|v| *v -= 10.0)>
                                    "-10%"
                                </Button>
                                <Button on_click=move |_| percentage.update(|v| *v += 10.0)>
                                    "+10%"
                                </Button>
                            </Space>
                        </Space>
                    }
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
