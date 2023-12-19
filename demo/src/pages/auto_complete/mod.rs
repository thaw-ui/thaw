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
                <DemoCode slot>

                    {highlight_str!(
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
                    )}

                </DemoCode>
            </Demo>
            <h3>"disabled"</h3>
            <Demo>
                <AutoComplete value options placeholder="Email" disabled=true/>
                <DemoCode slot>
                    {highlight_str!(
                        r#"
                        <AutoComplete value options placeholder="Email" disabled=true/>
                "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"invalid"</h3>
            <Demo>
                <AutoComplete value options placeholder="Email" invalid=true/>
                <DemoCode slot>
                    {highlight_str!(
                        r#"
                        <AutoComplete value options placeholder="Email" invalid=true/>
                "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h2>"Prefix & Suffix"</h2>
            <Demo>
                <Space vertical=true>
                    <AutoComplete value options>
                        <AutoCompletePrefix slot>
                            <Icon icon=icondata::Icon::from(icondata::AiIcon::AiUserOutlined)/>
                        </AutoCompletePrefix>
                    </AutoComplete>
                    <AutoComplete value options>
                        <AutoCompleteSuffix slot>
                            <Icon icon=icondata::Icon::from(icondata::AiIcon::AiGithubOutlined)/>
                        </AutoCompleteSuffix>
                    </AutoComplete>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Space vertical=true>
                            <AutoComplete value options>
                                <AutoCompletePrefix slot>
                                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiUserOutlined)/>
                                </AutoCompletePrefix>
                            </AutoComplete>
                            <AutoComplete value options>
                                <AutoCompleteSuffix slot>
                                    <Icon icon=icondata::Icon::from(icondata::AiIcon::AiGithubOutlined)/>
                                </AutoCompleteSuffix>
                            </AutoComplete>
                        </Space>
                        "#,
                        "rust"
                    )}

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
                        <td>"Default::default()"</td>
                        <td>"Input of autocomplete."</td>
                    </tr>
                    <tr>
                        <td>"placeholder"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Autocomplete's placeholder."</td>
                    </tr>
                    <tr>
                        <td>"options"</td>
                        <td>"MaybeSignal<Vec<AutoCompleteOption>>"</td>
                        <td>"Vec::new()"</td>
                        <td>"Options to autocomplete from."</td>
                    </tr>
                    <tr>
                        <td>"disabled"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Whether the input is disabled."</td>
                    </tr>
                    <tr>
                        <td>"invalid"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Whether the input is invalid."</td>
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
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Additional classes for the autocomplete element."</td>
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
            <h3>"AutoComplete Slots"</h3>
            <Table single_column=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Default"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"AutoCompletePrefix"</td>
                        <td>
                            <Text code=true>"None"</Text>
                        </td>
                        <td>"AutoCompletePrefix content."</td>
                    </tr>
                    <tr>
                        <td>"AutoCompleteSuffix"</td>
                        <td>
                            <Text code=true>"None"</Text>
                        </td>
                        <td>"AutoCompleteSuffix content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"AutoComplete Ref"</h3>
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
                        <td>"focus"</td>
                        <td>
                            <Text code=true>"Fn(&self)"</Text>
                        </td>
                        <td>"Focus the input element."</td>
                    </tr>
                    <tr>
                        <td>"blur"</td>
                        <td>
                            <Text code=true>"Fn(&self)"</Text>
                        </td>
                        <td>"Blur the input element."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
