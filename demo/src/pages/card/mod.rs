use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Card"</h1>
            <Demo>
                <Space vertical=true>
                    <Card title="title">
                        "content"
                    </Card>
                    <Card title="title">
                        <CardHeaderExtra slot>
                            "header-extra"
                        </CardHeaderExtra>
                        "content"
                    </Card>
                    <Card title="title">
                        <CardHeader slot>
                            "header"
                        </CardHeader>
                        "content"
                    </Card>
                    <Card title="title">
                        <CardHeaderExtra slot>
                            "header-extra"
                        </CardHeaderExtra>
                        "content"
                        <CardFooter slot>
                            "footer"
                        </CardFooter>
                    </Card>
                </Space>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Space vertical=true> 
                        <Card title="title">
                            "content"
                        </Card>
                        <Card title="title">
                            <CardHeaderExtra slot>
                                "header-extra"
                            </CardHeaderExtra>
                            "content"
                        </Card>
                        <Card title="title">
                            <CardHeader slot>
                                "header"
                            </CardHeader>
                            "content"
                        </Card>
                        <Card title="title">
                            <CardHeaderExtra slot>
                                "header-extra"
                            </CardHeaderExtra>
                            "content"
                            <CardFooter slot>
                                "footer"
                            </CardFooter>
                        </Card>
                    </Space>
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
