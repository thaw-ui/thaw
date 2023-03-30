use leptos::*;
use melt_ui::*;

fn main() {
    mount_to_body(|cx| view! { cx,  <App /> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (open, set_open) = create_signal(cx, true);
    view! { cx,
        <div class="root">
            <Button on:click=move |_| set_count.update(move |value| *value += 1)>"click"</Button>
            {move || count.get()}
            <Modal title=Some("".to_string()) open=open on_cancel=Some(Box::new(move || { set_open.set(false) }))>
                "sd" {move || count.get()}
            </Modal>
        </div>
    }
}

