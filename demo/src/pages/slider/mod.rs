use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    let value = create_rw_signal(0.0);
    let stepped_value = create_rw_signal(0.0);
    let labeled_value = create_rw_signal(0.0);
    
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Slider"</h1>
            <Demo>
                <Slider value/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(0.0);
                            
                        <Slider value/>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"step"</h3>
            <Demo>
                <Slider step=10 value=stepped_value/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(0.0);
                            
                        <Slider step=10 value/>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h2>"SliderLabel"</h2>
            <Demo>
                <Slider value=labeled_value max=10.0 step=1>
                    <SliderLabel value=0.0>
                        "0"
                    </SliderLabel>
                    <SliderLabel value=5.0>
                        "5"
                    </SliderLabel>
                    <SliderLabel value=10.0>
                        "10"
                    </SliderLabel>
                </Slider>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(0.0);
                            
                        <Slider value max=10.0 step=1>
                            <SliderLabel value=0>
                                "0"
                            </SliderLabel>
                            <SliderLabel value=5>
                                "5"
                            </SliderLabel>
                            <SliderLabel value=10>
                                "10"
                            </SliderLabel>
                        </Slider>
                    "#,
                        "rust"
                    )}

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
                    <tr>
                        <td>"step"</td>
                        <td>"MaybeSignal<u32>"</td>
                        <td></td>
                        <td>"The step in which value is incremented."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Option<Children>"</td>
                        <td></td>
                        <td>"Slider labels."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"SliderLabel Props"</h3>
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
                        <td>"ReadSignal<f64>"</td>
                        <td></td>
                        <td>"Value at which label will be placed."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Content of the lable."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
