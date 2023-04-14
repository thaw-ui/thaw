use leptos::*;
use melt_ui::*;
mod demo_button;
mod demo_checkbox;
mod demo_modal;
pub use demo_button::*;
pub use demo_checkbox::*;
pub use demo_modal::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
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
    }
}

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
