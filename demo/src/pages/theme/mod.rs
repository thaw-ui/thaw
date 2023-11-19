use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn ThemePage() -> impl IntoView {
    let theme = create_rw_signal(Theme::light());
    let customize_theme = create_rw_signal(Theme::light());
    let on_customize_theme = move |_| {
        customize_theme.update(|theme| {
            theme.common.color_primary = "#f5222d".to_string();
            theme.common.color_primary_hover = "#ff4d4f".to_string();
            theme.common.color_primary_active = "#cf1322".to_string();
        });
    };
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Theme"</h1>
            <h3>"ThemeProvider"</h3>
            <Demo>
                <ThemeProvider theme>
                    <Card>
                        <Space>
                            <Button on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                            <Button on_click=move |_| theme.set(Theme::dark())>"Dark"</Button>
                        </Space>
                    </Card>
                </ThemeProvider>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let theme = create_rw_signal(Theme::light());

                    view! {
                        <ThemeProvider theme>
                            <Card>
                                <Space>
                                    <Button on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                                    <Button on_click=move |_| theme.set(Theme::dark())>"Dark"</Button>
                                </Space>
                            </Card>
                        </ThemeProvider>
                    }
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"GlobalStyle"</h3>
            <p>"You can use GlobalStyle to sync common global style to the body element."</p>
            <Demo>
                ""
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let theme = create_rw_signal(Theme::light());
                    
                    view! {
                        <ThemeProvider theme>
                            <GlobalStyle />
                            "..."
                        </ThemeProvider>
                    }
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"CustomizeTheme"</h3>
            <Demo>
                <ThemeProvider theme=customize_theme>
                    <Card>
                        <Space>
                            <Button on_click=move |_| {
                                customize_theme.set(Theme::light())
                            }>"Light"</Button>
                            <Button on_click=on_customize_theme>"Customize Theme"</Button>
                        </Space>
                    </Card>
                </ThemeProvider>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r##"
                    let customize_theme = create_rw_signal(Theme::light());
                    let on_customize_theme = move |_| {
                        customize_theme.update(|theme| {
                            theme.common.color_primary = "#f5222d".to_string();
                            theme.common.color_primary_hover = "#ff4d4f".to_string();
                            theme.common.color_primary_active = "#cf1322".to_string();
                        });
                    };
                    
                    view! {
                        <ThemeProvider theme=customize_theme>
                            <Card>
                                <Space>
                                    <Button on_click=move |_| customize_theme.set(Theme::light())>"Light"</Button>
                                    <Button on_click=on_customize_theme>"Customize Theme"</Button>
                                </Space>
                            </Card>
                        </ThemeProvider>
                    }
                "##,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"ThemeProvider Props"</h3>
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
                        <td>"theme"</td>
                        <td>"RwSignal<Theme>"</td>
                        <td></td>
                        <td>"Theme."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
