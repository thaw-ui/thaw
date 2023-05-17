use crate::demo_button::*;
use crate::demo_checkbox::*;
use crate::demo_modal::*;
use crate::demo_slider::*;
use leptos::*;
use melt_ui::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let (theme, set_theme) = create_signal(cx, Theme::light());
    provide_context(cx, theme);
    let (count, set_count) = create_signal(cx, 0.0);
    let (button_type, set_button_type) = create_signal(cx, ButtonType::TEXT);

    let count_string = create_memo(cx, move |_| {
        log!("sd");
        count.get().to_string()
    });
    let on_input = SignalSetter::map(cx, move |value: String| {
        set_count.set(value.parse().unwrap());
    });
    view! { cx,
        <Space>
            <Input value=count_string on_input=on_input/>
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
        <DemoModal/>
        <hr />
        <DemoCheckout />
        <hr />
        <DemoSlider />
    }
}
