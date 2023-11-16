use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn TablePage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Table"</h1>
            <Demo>
                <Table>
                    <thead>
                        <tr>
                            <th>"tag"</th>
                            <th>"count"</th>
                            <th>"date"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>"div"</td>
                            <td>"2"</td>
                            <td>"2023-10-08"</td>
                        </tr>
                        <tr>
                            <td>"span"</td>
                            <td>"2"</td>
                            <td>"2023-10-08"</td>
                        </tr>
                    </tbody>
                </Table>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                        <Table>
                            <thead>
                                <tr>
                                    <th>"tag"</th>
                                    <th>"count"</th>
                                    <th>"date"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>"div"</td>
                                    <td>"2"</td>
                                    <td>"2023-10-08"</td>
                                </tr>
                                <tr>
                                    <td>"span"</td>
                                    <td>"2"</td>
                                    <td>"2023-10-08"</td>
                                </tr>
                            </tbody>
                        </Table>
                        "#,
                        "rust"
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"Table Props"</h3>
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
                        <td>"single_row"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"true"</td>
                        <td>""</td>
                    </tr>
                    <tr>
                        <td>"single_column"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>""</td>
                    </tr>
                    <tr>
                        <td>"style"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>r#""""#</td>
                        <td>"Table's style."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Table's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
