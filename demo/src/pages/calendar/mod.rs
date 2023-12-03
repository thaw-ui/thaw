use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn CalendarPage() -> impl IntoView {
    let value = create_rw_signal(OffsetDateTime::now_utc());
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Calendar"</h1>
            <Demo>
                <Calendar value />
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let value = create_rw_singal(OffsetDateTime::now_utc());

                        view! {
                            <Calendar value />
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Calendar Props"</h3>
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
                        <td>"value"</td>
                        <td>"RwSignal<OffsetDateTime>"</td>
                        <td></td>
                        <td></td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
