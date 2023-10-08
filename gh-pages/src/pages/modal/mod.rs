use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn ModalPage() -> impl IntoView {
    let show = create_rw_signal(false);
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Modal"</h1>
            <Demo>
                <Button on:click=move |_| show.set(true)>"Open Modal"</Button>
                <Modal title="title" show>
                    "hello"
                </Modal>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    let show = create_rw_signal(false);

                    <Button on:click=move |_| show.set(true)>
                        "open modal"
                    </Button>
                    <Modal title="title" show>
                        "hello"
                    </Modal>
                "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
        </div>
    }
}
