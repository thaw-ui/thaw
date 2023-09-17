use crate::components::{Demo, DemoCode};
use indoc::indoc;
use leptos::*;
use melt_ui::*;

#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Button"</h1>
            <Demo>
                <Button type_=ButtonType::PRIMARY>
                    "PRIMARY"
                </Button>
                <Button type_=ButtonType::SOLID>
                    "SOLID"
                </Button>
                <Button type_=ButtonType::TEXT>
                    "TEXT"
                </Button>
                <Button type_=ButtonType::LINK>
                    "LINK"
                </Button>
                <DemoCode slot>
                    {
                        indoc!(r#"
                            <Button type_=ButtonType::PRIMARY>
                                "PRIMARY"
                            </Button>
                            <Button type_=ButtonType::SOLID>
                                "SOLID"
                            </Button>
                            <Button type_=ButtonType::TEXT>
                                "TEXT"
                            </Button>
                            <Button type_=ButtonType::LINK>
                                "LINK"
                            </Button>
                        "#)
                    }
                </DemoCode>
            </Demo>
            <h3>"color"</h3>
            <Demo>
                <Button color=ButtonColor::PRIMARY>
                    "PRIMARY Color"
                </Button>
                <Button color=ButtonColor::SUCCESS>
                    "SUCCESS Color"
                </Button>
                <Button color=ButtonColor::WARNING>
                    "WARNING Color"
                </Button>
                <Button color=ButtonColor::ERROR>
                    "ERROR Color"
                </Button>
                <DemoCode slot>
                    {
                        indoc!(r#"
                            <Button color=ButtonColor::PRIMARY>
                                "PRIMARY Color"
                            </Button>
                            <Button color=ButtonColor::SUCCESS>
                                "SUCCESS Color"
                            </Button>
                            <Button color=ButtonColor::WARNING>
                                "WARNING Color"
                            </Button>
                            <Button color=ButtonColor::ERROR>
                                "ERROR Color"
                            </Button>
                        "#)
                    }
                </DemoCode>
            </Demo>
            <h3>"icon"</h3>
            <Demo>
                <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined>
                    "ERROR Color Icon"
                </Button>
                <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined round=true>
                </Button>
                <DemoCode slot>
                    {
                        indoc! {r#"
                            <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined>
                                "ERROR Color Icon"
                            </Button>
                            <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined round=true>
                            </Button>
                        "#}
                    }
                </DemoCode>
            </Demo>
            <h3>"style"</h3>
            <Demo>
                <Button style="background: blue;">"style blue"</Button>
                <Button style="width: 40px; height: 20px">"size"</Button>
                <DemoCode slot>
                    {
                        indoc! {r#"
                            <Button style="background: blue;">"style blue"</Button>
                            <Button style="width: 40px; height: 20px">"size"</Button>
                        "#}
                    }
                </DemoCode>
            </Demo>
        </div>
    }
}
