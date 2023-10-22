use crate::components::{Demo, DemoCode};
use leptos::*;
use melt_ui::*;
use prisms::highlight_str;

#[component]
pub fn UploadPage() -> impl IntoView {
    view! {
        <div style="width: 896px; margin: 0 auto;">
            <h1>"Upload(TODO)"</h1>
            <Demo>
                <Upload>
                    <Button>
                        "upload"
                    </Button>
                </Upload>
                <DemoCode
                    slot
                    html=highlight_str!(
                        r#"
                    <Upload>
                        <Button>
                            "upload"
                        </Button>
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
