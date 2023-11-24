use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn BadgePage() -> impl IntoView {
    let value = create_rw_signal(0);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Badge"</h1>
            <Demo>
                <Space>
                    <Badge value=value max=10>
                        <Avatar/>
                    </Badge>
                    <Badge variant=BadgeVariant::Success value=value max=10>
                        <Avatar/>
                    </Badge>
                    <Badge variant=BadgeVariant::Warning value=value max=10>
                        <Avatar/>
                    </Badge>
                    <Badge variant=BadgeVariant::Warning dot=true>
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
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_signal(0);
                        view! {
                            <Space>
                                <Badge value=value max=10>
                                    <Avatar />
                                </Badge>
                                <Badge variant=BadgeVariant::Success value=value max=10>
                                    <Avatar />
                                </Badge>
                                <Badge variant=BadgeVariant::Warning value=value max=10>
                                    <Avatar />
                                </Badge>
                                <Badge variant=BadgeVariant::Warning dot=true>
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
                    )}

                </DemoCode>
            </Demo>
            <h3>"Badge Props"</h3>
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
                        <td>"MaybeSignal<u32>"</td>
                        <td>"0"</td>
                        <td>"Badge's value."</td>
                    </tr>
                    <tr>
                        <td>"max"</td>
                        <td>"MaybeSignal<u32>"</td>
                        <td>"u32::MAX"</td>
                        <td>"The maximum number of the badge when its value overflows."</td>
                    </tr>
                    <tr>
                        <td>"variant"</td>
                        <td>"MaybeSignal<BadgeVariant>"</td>
                        <td>"BadgeVariant::Error"</td>
                        <td>"Badge variant."</td>
                    </tr>
                    <tr>
                        <td>"dot"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Show badge as dot."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Badge's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
