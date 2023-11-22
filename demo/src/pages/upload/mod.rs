use crate::components::{Demo, DemoCode};
use leptos::*;
use prisms::highlight_str;
use thaw::*;

#[component]
pub fn UploadPage() -> impl IntoView {
    let message = use_message();
    let custom_request = move |file_list: FileList| {
        message.create(
            format!("Number of uploaded files: {}", file_list.length()),
            MessageVariant::Success,
            Default::default(),
        );
    };
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Upload"</h1>
            <Demo>
                <Upload custom_request>
                    <Button>"Upload"</Button>
                </Upload>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let message = use_message();
                        let custom_request = move |file_list: FileList| {
                            message.create(
                                format!("Number of uploaded files: {}", file_list.length()),
                                MessageVariant::Success,
                                Default::default(),
                            );
                        };
                        view!{ 
                            <Upload>
                                <Button>
                                    "upload"
                                </Button>
                            </Upload>
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Drag to upload"</h3>
            <Demo>
                <Upload custom_request>
                    <UploadDragger>"Click or drag a file to this area to upload"</UploadDragger>
                </Upload>
                <DemoCode slot>

                    {highlight_str!(
                        r#"
                        let message = use_message();
                        let custom_request = move |file_list: FileList| {
                            message.create(
                                format!("Number of uploaded files: {}", file_list.length()),
                                MessageVariant::Success,
                                Default::default(),
                            );
                        };
                        view! {
                            <Upload custom_request>
                                <UploadDragger>
                                    "Click or drag a file to this area to upload"
                                </UploadDragger>
                            </Upload>
                        }
                    "#,
                        "rust"
                    )}

                </DemoCode>
            </Demo>
            <h3>"Upload Props"</h3>
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
                        <td>"accept"</td>
                        <td>"MaybeSignal<String>"</td>
                        <td>"Default::default()"</td>
                        <td>"The accept type of upload."</td>
                    </tr>
                    <tr>
                        <td>"multiple"</td>
                        <td>"MaybeSignal<bool>"</td>
                        <td>"false"</td>
                        <td>"Allow multiple files to be selected."</td>
                    </tr>
                    <tr>
                        <td>"custom_request"</td>
                        <td>"Option<Callback<FileList, ()>>"</td>
                        <td>"Default::default()"</td>
                        <td>"Customize upload request."</td>
                    </tr>
                    <tr>
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"Upload's content."</td>
                    </tr>
                </tbody>
            </Table>
            <h3>"UploadDragger Props"</h3>
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
                        <td>"children"</td>
                        <td>"Children"</td>
                        <td></td>
                        <td>"UploadDragger's content."</td>
                    </tr>
                </tbody>
            </Table>
        </div>
    }
}
