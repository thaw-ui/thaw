use leptos::*;
use melt_ui::*;

#[component]
pub fn ModalPage() -> impl IntoView {
    let show = create_rw_signal(false);
    view! {
        <Button on:click=move |_| show.set(true)>
            "open modal"
        </Button>
        <Modal title="title" show>
            "sd"
        </Modal>
    }
}
