use leptos::*;
use melt_ui::*;

#[component]
pub fn DemoButton(cx: Scope) -> impl IntoView {
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
            <Button color=ButtonColor::ERROR icon=leptos_icons::AiIcon::AiCloseOutlined>
                "ERROR Color Icon"
            </Button>
        </Space>
    }
}