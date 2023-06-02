use leptos::*;
use melt_ui::*;

#[component]
pub fn ModalPage(cx: Scope) -> impl IntoView {
    let show = create_rw_signal(cx, false);
    view! { cx,
        <Button on:click=move |_| show.set(true)>
            "open modal"
        </Button>
        <Modal title="标题" show>
            "sd"
        </Modal>
    }
}