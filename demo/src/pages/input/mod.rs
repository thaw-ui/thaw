use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn InputPage() -> impl IntoView {
    let value = create_rw_signal(String::from("o"));
    let input_ref = create_component_ref::<InputRef>();
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Input"</h1>
            <Demo>
                <Space vertical=true>
                    <Input value/>
                    <Input value variant=InputVariant::Password placeholder="Password"/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                            let value = create_rw_signal(String::from("o"));
    
                            view! {
                                <Space vertical=true>
                                    <Input value/>
                                    <Input value variant=InputVariant::Password placeholder="Password"/>
                                </Space>
                            }
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>disabled</h3>
            <Demo>
                <Space vertical=true>
                    <Input value disabled=true/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Input value disabled=true/>
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>invalid</h3>
            <Demo>
                <Space vertical=true>
                    <Input value invalid=true/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Input value invalid=true/>
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Imperative handle"</h3>
            <Demo>
                <Space vertical=true>
                    <Space>
                        <Button on_click=move |_| input_ref.get_untracked().unwrap().focus()>
                            "Focus"
                        </Button>
                        <Button on_click=move |_| input_ref.get_untracked().unwrap().blur()>
                            "Blur"
                        </Button>
                    </Space>
                    <Input value comp_ref=input_ref/>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(String::from("o"));
                        let input_ref = create_component_ref::<InputRef>();

                        view! {
                            <Space vertical=true>
                                <Space>
                                    <Button on_click=move |_| input_ref.get_untracked().unwrap().focus()>
                                        "Focus"
                                    </Button>
                                    <Button on_click=move |_| input_ref.get_untracked().unwrap().blur()>
                                        "Blur"
                                    </Button>
                                </Space>
                                <Input value comp_ref=input_ref/>
                            </Space>
                        }
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h2>"Prefix & Suffix"</h2>
            <Demo>
                <Space vertical=true>
                    <Input value>
                        <InputPrefix slot>
                            <Icon icon=icondata::Icon::from(icondata::AiIcon::AiUserOutlined)/>
                        </InputPrefix>
                    </Input>
                    <Input value>
                        <InputSuffix slot>"$"</InputSuffix>
                    </Input>
                    <Input value>
                        <InputSuffix slot>
                            <Icon icon=icondata::Icon::from(icondata::AiIcon::AiGithubOutlined)/>
                        </InputSuffix>
                    </Input>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                            let value = create_rw_signal(String::from("o"));
    
                            view! {
                                <Space vertical=true>
                                    <Input value>
                                        <InputPrefix slot>
                                            <Icon icon=icondata::Icon::from(icondata::AiIcon::AiUserOutlined)/>
                                        </InputPrefix>
                                    </Input>
                                    <Input value>
                                        <InputSuffix slot>
                                            "$"
                                        </InputSuffix>
                                    </Input>
                                    <Input value>
                                        <InputSuffix slot>
                                            <Icon icon=icondata::Icon::from(icondata::AiIcon::AiGithubOutlined)/>
                                        </InputSuffix>
                                    </Input>
                                </Space>
                            }
                        "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Input Props"</h3>
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
                        <td>"Set the input value"</td>
                    </tr>
                    <tr>
                        <td>"variant"</td>
                        <td>"MaybeSignal<InputVariant>"</td>
                        <td>"InputVariant::Text"</td>
                        <td>"Input's variant."</td>
                    </tr>
                    <tr>
                        <td>"placeholder"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Placeholder of input."</td>
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
                        <td>"allow_value"</td>
                        <td>"Option<Callback<String, bool>>"</td>
                        <td>"None"</td>
                        <td>
                            "Check the incoming value, if it returns false, input will not be accepted."
                        </td>
                    </tr>
                    <tr>
                        <td>"on_focus"</td>
                        <td>"Option<Callback<ev::FocusEvent>>"</td>
                        <td>"None"</td>
                        <td>"Callback triggered when the input is focussed on."</td>
                    </tr>
                    <tr>
                        <td>"on_blur"</td>
                        <td>"Option<Callback<ev::FocusEvent>>"</td>
                        <td>"None"</td>
                        <td>"Callback triggered when the input is blurred."</td>
                    </tr>
                    <tr>
                        <td>"class"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"Addtional classes for the input element."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"Input Slots"</h3>
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
                        <td>"InputPrefix"</td>
                        <td>"None"</td>
                        <td>"InputPrefix content."</td>
                    </tr>
                    <tr>
                        <td>"InputSuffix"</td>
                        <td>"None"</td>
                        <td>"InputSuffix content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"Input Ref"</h3>
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
