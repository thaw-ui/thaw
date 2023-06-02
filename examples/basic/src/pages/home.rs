use crate::demo_button::*;
use crate::demo_checkbox::*;
use crate::demo_slider::*;
use leptos::*;
use leptos_router::use_navigate;
use melt_ui::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let (theme, set_theme) = create_signal(cx, Theme::light());
    provide_context(cx, theme);
    let (count, set_count) = create_signal(cx, 0.0);
    let (button_type, set_button_type) = create_signal(cx, ButtonType::TEXT);
    view! { cx,
        <Button on:click=move |_| {
            let navigate = use_navigate(cx);
            _ = navigate("/components/menu", Default::default());
        }>
            "components"
        </Button>
        <hr />
        <Space>
            <Button
                on:click=move |_| set_theme.update(move |value| *value = Theme::dark())
                type_=button_type
            >
                "theme"
            </Button>
            <Button on:click=move |_| set_button_type.update(move |value| *value = ButtonType::PRIMARY)>
                "click"
            </Button>
            <Button
                on:click=move |_| set_count.update(move |value| *value += 1.0)
                type_=button_type
            >
                "click"
            </Button>
            {move || count.get()}

            <Progress percentage=count/>
        </Space>
        <hr />
        <DemoButton />
        <hr />
        <DemoCheckout />
        <hr />
        <DemoSlider />
    }
}
