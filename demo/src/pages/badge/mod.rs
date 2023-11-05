use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

#[component]
pub fn BadgePage() -> impl IntoView {
    let value = create_rw_signal(0);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Badge"</h1>
            <Demo>
                <Space>
                    <Badge value=value max_value=10>
                        <Avatar/>
                    </Badge>
                    <Badge color=BadgeColor::Success value=value max_value=10>
                        <Avatar/>
                    </Badge>
                    <Badge color=BadgeColor::Warning value=value max_value=10>
                        <Avatar/>
                    </Badge>
                    <Badge color=BadgeColor::Warning dot=true>
                        <Avatar/>
                    </Badge>
                    <Button on_click=move |_| value.update(|v| *v += 1)>"+1"</Button>
                    <Button on_click=move |_| {
                        value
                            .update(|v| {
                                if *v != 0 {
                                    *v -= 1;
                                }
                            })
                    }>"-1"</Button>
                    "value:"
                    {move || value.get()}
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let value = create_rw_signal(0);
                    view! {
                        <Space>
                            <Badge value=value max_value=10>
                                <Avatar />
                            </Badge>
                            <Badge color=BadgeColor::Success value=value max_value=10>
                                <Avatar />
                            </Badge>
                            <Badge color=BadgeColor::Warning value=value max_value=10>
                                <Avatar />
                            </Badge>
                            <Badge color=BadgeColor::Warning dot=true>
                                <Avatar />
                            </Badge>
                            <Button on_click=move |_| value.update(|v| *v += 1)>"+1"</Button>
                            <Button on_click=move |_| value.update(|v| if *v != 0 { *v -= 1 })>"-1"</Button>
                            "value:"
                            {move || value.get()}
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
