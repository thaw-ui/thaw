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
                    <Progress percentage show_indicator=false/>
                    <Progress percentage />
                    <Progress percentage indicator_placement=ProgressIndicatorPlacement::Inside/>
                    <Progress percentage color=ProgressColor::Success/>
                    <Progress percentage color=ProgressColor::Warning/>
                    <Progress percentage color=ProgressColor::Error/>
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
                            <Progress percentage show_indicator=false/>
                            <Progress percentage />
                            <Progress percentage indicator_placement=ProgressIndicatorPlacement::Inside/>
                            <Progress percentage color=ProgressColor::Success/>
                            <Progress percentage color=ProgressColor::Warning/>
                            <Progress percentage color=ProgressColor::Error/>
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
            <h3>"Progress Props"</h3>
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
                        <td>"percentage"</td>
                        <td>"MaybeSignal<f32>"</td>
                        <td>"0"</td>
                        <td>"Percentage value."</td>
                    </tr>
                    <tr>
                        <td>"color"</td>
                        <td>"MaybeSignal<ProgressColor>"</td>
                        <td>"ProgressColor::Primary"</td>
                        <td>"Progress color."</td>
                    </tr>
                    <tr>
                        <td>"show_indicator"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"true"</td>
                        <td>"Whether to display indicators."</td>
                    </tr>
                    <tr>
                        <td>"indicator_placement"</td>
                        <td>"MaybeSignal<ProgressIndicatorPlacement>"</td>
                        <td>"ProgressIndicatorPlacement::Outside"</td>
                        <td>"Indicator placement."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
