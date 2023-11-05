use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

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
                <DemoCode
                    slot
                    html=highlight_str!(
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
                    )
                >

                    ""
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
                <DemoCode
                    slot
                    html=highlight_str!(
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
                    )
                >

                    ""
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
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Button color=ButtonColor::Error icon=icondata::AiIcon::AiCloseOutlined>
                        "Error Color Icon"
                    </Button>
                    <Button color=ButtonColor::Error icon=icondata::AiIcon::AiCloseOutlined round=true>
                    </Button>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <LoadingButton/>
            <h3>"style"</h3>
            <Demo>
                <Space>
                    <Button style="background: blue;">"style blue"</Button>
                    <Button style="width: 40px; height: 20px">"size"</Button>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Button style="background: blue;">"style blue"</Button>
                    <Button style="width: 40px; height: 20px">"size"</Button>
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

#[component]
pub fn LoadingButton() -> impl IntoView {
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
            <DemoCode
                slot
                html=highlight_str!(
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
                )
            >

                ""
            </DemoCode>
        </Demo>
    }
}
