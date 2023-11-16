use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    let value = create_rw_signal(0.0);

    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Slider"</h1>
            <Demo>
                <Slider value/>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(0.0);
                        
                    <Slider value/>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"Slider Props"</h3>
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
                        <td>"value"</td>
                        <td>"RwSignal<f64>"</td>
                        <td>"0"</td>
                        <td>"Value of the slider."</td>
                    </tr>
                    <tr>
                        <td>"max"</td>
                        <td>"MaybeSignal<f64>"</td>
                        <td>"100"</td>
                        <td>"Max value of the slider."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
