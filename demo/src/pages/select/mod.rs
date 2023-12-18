use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    let selected_value = create_rw_signal(Some(String::from("apple")));

    let options = vec![SelectOption {
        label: String::from("apple"),
        value: String::from("apple"),
    }];
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Select"</h1>
            <Demo>
                <Select value=selected_value options/>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let selected_value = create_rw_signal(Some(String::from("apple")));
                        let options = vec![SelectOption {
                            label: String::from("apple"),
                            value: String::from("apple"),
                        }];
    
                        <Select value=selected_value options/>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Select Props"</h3>
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
                        <td>"RwSignal<Option<T>>"</td>
                        <td>"None"</td>
                        <td>"Checked value."</td>
                    </tr>
                    <tr>
                        <td>"options"</td>
                        <td>"MaybeSignal<Vec<SelectOption<T>>>"</td>
                        <td>"vec![]"</td>
                        <td>"Options that can be selected."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the select element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
