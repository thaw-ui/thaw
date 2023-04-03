use leptos::*;
use melt_ui::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0.0);
    let (open, set_open) = create_signal(cx, true);
    let (button_type, set_button_type) = create_signal(cx, ButtonType::TEXT);
    view! { cx,
        <Space>
            <Button on:click=move |_| set_button_type.update(move |value| *value = ButtonType::PRIMARY)>"click"</Button>
            <Button on:click=move |_| set_count.update(move |value| *value += 1.0) type_=button_type>"click"</Button>
            {move || count.get()}
            <Modal title=Some("".to_string()) open=open on_cancel=Some(Box::new(move || { set_open.set(false) }))>
                "sd" {move || count.get()}
            </Modal>
            <Progress percentage=count/>
        </Space>
    }
}

