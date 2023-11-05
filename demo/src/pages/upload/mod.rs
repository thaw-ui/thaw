use crate::components::{Demo, DemoCode};
use leptos::*;
use thaw::*;
use prisms::highlight_str;

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
                    <Button>
                        "Upload"
                    </Button>
                </Upload>
                <DemoCode
                    slot
                    html=highlight_str!(
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
                    )
                >

                    ""
                </DemoCode>
            </Demo>
            <h3>"Drag to upload"</h3>
            <Demo>
                <Upload>
                    <UploadDragger>
                        "Click or drag a file to this area to upload"
                    </UploadDragger>
                </Upload>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Upload>
                        <UploadDragger>
                            "Click or drag a file to this area to upload"
                        </UploadDragger>
                    </Upload>
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
