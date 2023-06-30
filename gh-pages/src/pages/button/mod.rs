use leptos::*;
use melt_ui::*;

#[component]
pub fn ButtonPage(cx: Scope) -> impl IntoView {
    view! {cx,
        <Space>
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
            <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined>
                "ERROR Color Icon"
            </Button>
            <Button color=ButtonColor::ERROR icon=icondata::AiIcon::AiCloseOutlined round=true>
            </Button>
        </Space>
        <div style="padding-top: 12px">
            <Button style="background: blue;">"style blue"</Button>
            <Button style="width: 40px; height: 20px">"size"</Button>
        </div>
    }
}
