use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

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
            <h3>"Modal Props"</h3>
            <Table single_column=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Type"</th>
                        <th>"Default"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"show"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td></td>
                        <td>"Whether to show modal."</td>
                    </tr>
                    <tr>
                        <td>"title"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>r#""""#</td>
                        <td>"Modal title."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Modal's content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"Modal Slots"</h3>
            <Table single_column=true>
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Type"</th>
                        <th>"Default"</th>
                        <th>"Description"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"ModalFooter"</td>
                        <td>"None"</td>
                        <td>"Footer content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
