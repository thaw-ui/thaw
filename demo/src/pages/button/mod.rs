use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Button"</h1>
            <Demo>
                <Space>
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                    <Button variant=ButtonVariant::Solid>"Solid"</Button>
                    <Button variant=ButtonVariant::Text>"Text"</Button>
                    <Button variant=ButtonVariant::Link>"Link"</Button>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Button variant=ButtonVariant::Primary>
                            "Primary"
                        </Button>
                        <Button variant=ButtonVariant::Solid>
                            "Solid"
                        </Button>
                        <Button variant=ButtonVariant::Text>
                            "Text"
                        </Button>
                        <Button variant=ButtonVariant::Link>
                            "Link"
                        </Button>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"color"</h3>
            <Demo>
                <Space>
                    <Button color=ButtonColor::Primary>"Primary Color"</Button>
                    <Button color=ButtonColor::Success>"Success Color"</Button>
                    <Button color=ButtonColor::Warning>"Warning Color"</Button>
                    <Button color=ButtonColor::Error>"Error Color"</Button>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Button color=ButtonColor::Primary>
                            "Primary Color"
                        </Button>
                        <Button color=ButtonColor::Success>
                            "Success Color"
                        </Button>
                        <Button color=ButtonColor::Warning>
                            "Warning Color"
                        </Button>
                        <Button color=ButtonColor::Error>
                            "Error Color"
                        </Button>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"icon"</h3>
            <Demo>
                <Space>
                    <Button color=ButtonColor::Error icon=icondata::AiIcon::AiCloseOutlined>
                        "Error Color Icon"
                    </Button>
                    <Button
                        color=ButtonColor::Error
                        icon=icondata::AiIcon::AiCloseOutlined
                        round=true
                    />
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                <Button color=ButtonColor::Error icon=icondata::AiIcon::AiCloseOutlined>
                    "Error Color Icon"
                </Button>
                <Button color=ButtonColor::Error icon=icondata::AiIcon::AiCloseOutlined round=true>
                </Button>
            "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <LoadingButton/>
            <h3>"disabled"</h3>
            <Demo>
                <Space>
                    <Button variant=ButtonVariant::Primary disabled=true>
                        "Primary"
                    </Button>
                    <Button variant=ButtonVariant::Solid disabled=true>
                        "Solid"
                    </Button>
                    <Button variant=ButtonVariant::Text disabled=true>
                        "Text"
                    </Button>
                    <Button variant=ButtonVariant::Link disabled=true>
                        "Link"
                    </Button>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        <Button variant=ButtonVariant::Primary disabled=true>
                            "Primary"
                        </Button>
                        <Button variant=ButtonVariant::Solid disabled=true>
                            "Solid"
                        </Button>
                        <Button variant=ButtonVariant::Text disabled=true>
                            "Text"
                        </Button>
                        <Button variant=ButtonVariant::Link disabled=true>
                            "Link"
                        </Button>
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"style"</h3>
            <Demo>
                <Space>
                    <Button style="background: blue;">"style blue"</Button>
                    <Button style="width: 40px; height: 20px">"size"</Button>
                </Space>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                    <Button style="background: blue;">"style blue"</Button>
                    <Button style="width: 40px; height: 20px">"size"</Button>
                "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Button Props"</h3>
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
                        <td>"style"</td>
                        <td>
                            <Text code=true>"MaybeSignal<String>"</Text>
                        </td>
                        <td>
                            <Text code=true>"Default::default()"</Text>
                        </td>
                        <td>"Button's style."</td>
                    </tr>
                    <tr>
                        <td>"variant"</td>
                        <td>
                            <Text code=true>"MaybeSignal<ButtonVariant>"</Text>
                        </td>
                        <td>
                            <Text code=true>"ButtonVariant::Primary"</Text>
                        </td>
                        <td>"Button's variant."</td>
                    </tr>
                    <tr>
                        <td>"color"</td>
                        <td>
                            <Text code=true>"MaybeSignal<ButtonColor>"</Text>
                        </td>
                        <td>
                            <Text code=true>"ButtonColor::Primary"</Text>
                        </td>
                        <td>"Button's color."</td>
                    </tr>
                    <tr>
                        <td>"round"</td>
                        <td>
                            <Text code=true>"MaybeSignal<bool>"</Text>
                        </td>
                        <td>
                            <Text code=true>"false"</Text>
                        </td>
                        <td>"Whether the button shows rounded corners."</td>
                    </tr>
                    <tr>
                        <td>"icon"</td>
                        <td>
                            <Text code=true>"Option<Icon>"</Text>
                        </td>
                        <td>
                            <Text code=true>"None"</Text>
                        </td>
                        <td>"The icon of the button."</td>
                    </tr>
                    <tr>
                        <td>"loading"</td>
                        <td>
                            <Text code=true>"MaybeSignal<bool>"</Text>
                        </td>
                        <td>
                            <Text code=true>"false"</Text>
                        </td>
                        <td>"Whether the button shows the loading status."</td>
                    </tr>
                    <tr>
                        <td>"disabled"</td>
                        <td>
                            <Text code=true>"MaybeSignal<bool>"</Text>
                        </td>
                        <td>
                            <Text code=true>"false"</Text>
                        </td>
                        <td>"Whether the button is disabled."</td>
                    </tr>
                    <tr>
                        <td>"on_click"</td>
                        <td>
                            <Text code=true>"Option<Callback<ev::MouseEvent>>"</Text>
                        </td>
                        <td>
                            <Text code=true>"None"</Text>
                        </td>
                        <td>"Listen for button click events."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>
                            <Text code=true>"Children"</Text>
                        </td>
                        <td></td>
                        <td>"Button's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}

#[component]
fn LoadingButton() -> impl IntoView {
    let loading = create_rw_signal(false);
    let on_click = move |_| {
        loading.set(true);
        set_timeout(
            move || {
                loading.set(false);
            },
            std::time::Duration::new(2, 0),
        );
    };
    view! {
        <h3>"Loading"</h3>
        <Demo>
            <Space>
                <Button loading on_click icon=icondata::AiIcon::AiCloseOutlined>
                    "Click Me"
                </Button>
                <Button loading on_click>
                    "Click Me"
                </Button>
            </Space>
            <DemoCode slot>

                {highlight_str!(
                    r#"
                    let loading = create_rw_signal(false);
                    let on_click = move |_| {
                        loading.set(true);
                        set_timeout(
                            move || {
                                loading.set(false);
                            },
                            std::time::Duration::new(2, 0),
                        );
                    };
                    view! {
                        <Space>
                            <Button loading on_click icon=icondata::AiIcon::AiCloseOutlined>
                                "Click Me"
                            </Button>
                            <Button loading on_click>
                                "Click Me"
                            </Button>
                        </Space>
                    }
                "#,
                    "rust"
                )}

            </DemoCode>
        </Demo>
    }
}
