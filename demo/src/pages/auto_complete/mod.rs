use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn AutoCompletePage() -> impl IntoView {
    let value = create_rw_signal(String::new());
    let options = create_memo(move |_| {
        let prefix = value
            .get()
            .split_once('@')
            .map_or(value.get(), |v| v.0.to_string());
        vec!["@gmail.com", "@163.com"]
            .into_iter()
            .map(|suffix| AutoCompleteOption {
                label: format!("{prefix}{suffix}"),
                value: format!("{prefix}{suffix}"),
            })
            .collect()
    });

    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"AutoComplete"</h1>
            <Demo>
                <AutoComplete value options placeholder="Email"/>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        let value = create_rw_signal(String::new());
                        let options =create_memo(|_| {
                            let prefix = value
                                .get()
                                .split_once('@')
                                .map_or(value.get(), |v| v.0.to_string());
                            vec!["@gmail.com", "@163.com"]
                                .into_iter()
                                .map(|suffix| AutoCompleteOption {
                                    label: format!("{prefix}{suffix}"),
                                    value: format!("{prefix}{suffix}"),
                                })
                                .collect()
                        });
                        view! {
                            <AutoComplete value options placeholder="Email"/>
                        }
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"AutoComplete Props"</h3>
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
                        <td>"RwSignal<String>"</td>
                        <td>r#""""#</td>
                        <td>"Input of autocomplete."</td>
                    </tr>
                    <tr>
                        <td>"placeholder"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>r#""""#</td>
                        <td>"Autocomplete's placeholder."</td>
                    </tr>
                    <tr>
                        <td>"options"</td>
                        <td>"MaybeSignal<Vec<AutoCompleteOption>>"</td>
                        <td>"Vec::new()"</td>
                        <td>"Options to autocomplete from."</td>
                    </tr>
                    <tr>
                        <td>"clear_after_select"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Whether to clear after selection."</td>
                    </tr>
                    <tr>
                        <td>"on_select"</td>
                        <td>"Option<Callback<String>>"</td>
                        <td>"None"</td>
                        <td>"On select callback function."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"AutoCompleteOption Properties"</h3>
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
                        <td>"value"</td>
                        <td>"String"</td>
                        <td>"Option ID."</td>
                    </tr>
                    <tr>
                        <td>"label"</td>
                        <td>"String"</td>
                        <td>"Option label value."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
