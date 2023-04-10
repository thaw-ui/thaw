use leptos::*;
use melt_ui::*;

#[component]
pub fn DemoModal(cx: Scope) -> impl IntoView {
    let (open, set_open) = create_signal(cx, false);
    let on_cancel = SignalSetter::map(cx, move |_| {
        set_open.set(false);
    });
    view! { cx,
        <Button on:click=move |_| set_open.update(move |value| *value = !*value)>
            "open modal"
        </Button>
        <Modal title="" open=open on_cancel=on_cancel>
            "sd"
        </Modal>
    }
}